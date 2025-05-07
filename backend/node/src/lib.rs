#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod auth;
pub mod command;
pub mod communication;
pub mod config;
pub mod eject;
pub mod encryption;
pub mod ghost_shares;
pub mod key_info;
pub mod keygen;
pub mod logging;
pub mod node;
pub mod recovery;
mod security;
pub mod signing;
pub mod storage;
pub mod user_recovery;

use crate::{ config::*, node::NodeIdentity, logging::GridlockLogInitializer };
use anyhow::{ anyhow, bail, Result };
use keygen::eddsa;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use tracing::{ info, warn };
use std::env;

#[derive(Clone)]
pub struct App {
    pub nc: nats::Connection,
    pub node: NodeIdentity,
}

pub static NATS_CONNECTED: AtomicBool = AtomicBool::new(false);

impl App {
    pub fn new() -> Result<App> {
        let node = match node::NodeIdentity::load() {
            Ok(node) => node,
            Err(_) => { create_new_node_identity()? }
        };
        info!("Version: {}", env!("CARGO_PKG_VERSION"));
        info!("-----------------------------------");
        info!("Hello, you can call me \x1b[34m\x1b[1m{}\x1b[0m", node.name);
        info!("Node ID: \x1b[34m\x1b[1m{}\x1b[0m", &node.node_id);
        info!("Networking Public Key: \x1b[34m\x1b[1m{}\x1b[0m", &node.networking_public_key);
        info!("E2E Public Key: \x1b[34m\x1b[1m{}\x1b[0m", &node.e2e_public_key);
        info!("-----------------------------------");
        info!(
            "\n{{\n  \"name\": \"{}\",\n  \"nodeId\": \"{}\",\n  \"networkingPublicKey\": \"{}\",\n  \"e2ePublicKey\": \"{}\"\n}}",
            node.name,
            node.node_id,
            node.networking_public_key,
            node.e2e_public_key
        );
        info!("-----------------------------------");
        let nc = get_nats_connection()?;

        Ok(App { nc, node })
    }

    pub fn try_reconnect(&mut self) -> Result<()> {
        warn!("Try reconnect NATs");
        self.nc = get_nats_connection()?;
        Ok(())
    }
}

pub fn start() -> Result<App> {
    if Config::create_data_dirs().is_err() {
        bail!("Failed to create application data directories");
    }
    GridlockLogInitializer::init();
    App::new()
}

pub fn get_nats_connection() -> Result<nats::Connection> {
    let address = Config::get_nats_address();
    let NATS_USER = env
        ::var("NATS_USER")
        .map_err(|_| anyhow!("NATS_USER environment variable is not set"))?;
    let NATS_PASSWORD = env
        ::var("NATS_PASSWORD")
        .map_err(|_| anyhow!("NATS_PASSWORD environment variable is not set"))?;

    // Add retry logic with exponential backoff
    let mut retry_count = 0;
    let max_retries = 5;
    let mut wait_time = Duration::from_secs(1);

    loop {
        match
            nats::Options
                ::with_user_pass(&NATS_USER, &NATS_PASSWORD)
                .disconnect_callback(|| {
                    warn!("NATs disconnected");
                    NATS_CONNECTED.store(false, Ordering::Relaxed);
                })
                .reconnect_callback(|| {
                    warn!("NATs reconnected");
                    NATS_CONNECTED.store(true, Ordering::Relaxed);
                })
                .retry_on_failed_connect()
                .connect(&address)
        {
            Ok(conn) => {
                NATS_CONNECTED.store(true, Ordering::Relaxed);
                info!("Connected to NATS successfully at: {:?}", &address);
                return Ok(conn);
            }
            Err(err) => {
                if retry_count >= max_retries {
                    return Err(
                        anyhow!(
                            "Failed to connect to NATS at \"{}\" after {} attempts. Last error: {}. Please check that:\n1. The NATS server is running\n2. The address is correct\n3. Network connectivity is available",
                            address,
                            max_retries,
                            err
                        )
                    );
                }
                warn!(
                    "Failed to connect to NATS (attempt {}/{}). Retrying in {} seconds...",
                    retry_count + 1,
                    max_retries,
                    wait_time.as_secs()
                );
                std::thread::sleep(wait_time);
                wait_time *= 2; // Exponential backoff
                retry_count += 1;
            }
        }
    }
}

pub fn create_new_node_identity() -> Result<NodeIdentity> {
    //no json file exists
    info!("No pre-existing data, creating new node identity");
    let node = NodeIdentity::new();
    node.save()?;
    Ok(node)
}

pub fn handle_message(app: &App, message: nats::Message) {
    info!("Received a message with subject \"{}\"", message.subject);

    if message.subject.starts_with("network.gridlock.nodes.keyGen.") {
        info!("start keygen process");
        keygen::ecdsa::session::handle_new_session_message(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.keySign.") {
        signing::ecdsa::session::handle_new_session_message(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.KeyGenEdDSA.") {
        eddsa::session::handle_new_session_message(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.KeySignEdDSA.") {
        signing::eddsa::session::handle_new_session_message(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.KeySignSr25519.") {
        signing::sr25519_musign::handle_new_session_message(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.Message.") {
        // To be able manage partner, user and gridlock nodes
        let _ = command::handle_nats_command(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.KeyShareRecovery.") {
        recovery::recovery_session::handle_new_session_message(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.UserRecovery.") {
        user_recovery::session::handle_new_session_message(app, message);
    } else if message.subject.starts_with("network.gridlock.nodes.UserRecoveryConfirm.") {
        user_recovery::confirm::handle_new_session_message(app, message);
    } else {
        warn!("Received message with an unrecognized subject: {}", message.subject);
    }
}

pub fn start_sending_ready_as_cancellable_task_on_thread(
    conn: nats::Connection,
    node_id: String,
    rx: mpsc::Receiver<()>,
    interval_duration: Duration
) -> Result<()> {
    let subject = format!("network.gridlock.nodes.ready.{}", &node_id);
    let _ = std::thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            let _ = conn.publish(&subject, &node_id);
            std::thread::sleep(interval_duration);
        }
    });
    Ok(())
}
