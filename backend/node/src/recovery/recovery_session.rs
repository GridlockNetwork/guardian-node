use crate::communication::nats_session::Nats;
use crate::communication::protocol::Topic;
use crate::config::ConfigProvider;
use crate::node::NodeIdentity;
use crate::recovery::encryption::{ NKeyHelperEncryptor, NKeyTargetEncryptor };
use crate::recovery::helper_role::{
    ECDSABehaviourHelperRole,
    EdDSABehaviourHelperRole,
    KeyshareRecoveryHelper,
};
use crate::recovery::target_role::{
    ECDSABehaviourTargetRole,
    EdDSABehaviourTargetRole,
    KeyshareRecoveryTarget,
    Sr25519BehaviourTargetRole,
};
use crate::recovery::{ Key, Party, RecoveryRole };
use crate::storage::{ KeyshareAccessor, ECDSA, EDDSA };
use crate::App;
use anyhow::{ anyhow, bail, Result };
use serde::{ Deserialize, Serialize };
use shared::recovery::PublicKeysEnum;
use std::collections::HashMap;
use std::thread;
use tracing::{ error, info };

#[derive(Clone, Serialize, Deserialize)]
pub struct NewKeyShareRecoverySession {
    #[serde(flatten)]
    pub kind: Key,
    pub key_id: String,
    pub session_id: String,
    pub recovery_index: usize,
    pub threshold: usize,
    pub public_keys: PublicKeysEnum,
    pub role: RecoveryRole,
    #[serde(default)]
    pub email: Option<String>,
}

impl NewKeyShareRecoverySession {
    pub fn handle(&self, conn: nats::Connection) -> Result<()> {
        let key_id = self.key_id.clone();
        let session_id = self.session_id.clone();

        // Get email from struct or find it if not provided
        let email = match &self.email {
            Some(email) => email.clone(),
            None => Self::find_email_for_key(&key_id)?,
        };

        let node = NodeIdentity::load()?;
        let private_key = node.networking_private_key.clone();
        info!("Retrieved node identity");

        let public_keys: HashMap<usize, String> = self.public_keys.clone().into();

        let topic = Topic::KeyShareRecovery;

        match self.kind {
            Key::Sr25519 => {}
            _ if self.recovery_index == 0 => {
                bail!(
                    "Recovery of the zero index keyshare (the underlying secret key) is restricted to Sr25519 recoveries"
                );
            }
            _ => {}
        }

        match (&self.role, &self.kind) {
            //Recovery of a EdCSA keyshare by a helper guardian
            (RecoveryRole::Helper, Key::EDDSA) => {
                let key_accessor = KeyshareAccessor::<EDDSA>::read_only_with_email(
                    &key_id,
                    &email
                )?;
                let party_index = key_accessor.key.party_index;

                let (messenger, peers) = Nats::new_session(
                    conn,
                    &session_id,
                    &node,
                    &key_id,
                    party_index,
                    topic
                )?;

                let key_behaviour = EdDSABehaviourHelperRole::from_key_accessor(key_accessor);

                let encryptor = NKeyHelperEncryptor::new(
                    &public_keys,
                    self.recovery_index,
                    party_index,
                    &peers,
                    private_key
                ).map_err(|err| anyhow!("Unable to create encryptor: {}", err))?;

                let mut recoverer = KeyshareRecoveryHelper::new(
                    messenger,
                    encryptor,
                    key_behaviour
                );

                recoverer.try_recovery(self.recovery_index, Party {
                    party_index,
                    all_parties: peers,
                })
            }
            //Recovery of a EdCSA keyshare by a helper guardian
            (RecoveryRole::Helper, Key::Sr25519) => {
                let key_accessor = KeyshareAccessor::<EDDSA>::read_only_with_email(
                    &key_id,
                    &email
                )?;
                let party_index = key_accessor.key.party_index;

                let (messenger, peers) = Nats::new_session(
                    conn,
                    &session_id,
                    &node,
                    &key_id,
                    party_index,
                    topic
                )?;

                let key_behaviour = EdDSABehaviourHelperRole::from_key_accessor(key_accessor);

                let encryptor = NKeyHelperEncryptor::new(
                    &public_keys,
                    self.recovery_index,
                    party_index,
                    &peers,
                    private_key
                ).map_err(|err| anyhow!("Unable to create encryptor: {}", err))?;

                let mut recoverer = KeyshareRecoveryHelper::new(
                    messenger,
                    encryptor,
                    key_behaviour
                );

                recoverer.try_recovery(self.recovery_index, Party {
                    party_index,
                    all_parties: peers,
                })
            }
            //Recovery procedure followed by target of EdDSA key recovery to receive and validate their new keyshare
            (RecoveryRole::Target, Key::EDDSA) => {
                let party_index = self.recovery_index;

                let (messenger, peers) = Nats::new_session(
                    conn,
                    &session_id,
                    &node,
                    &key_id,
                    party_index,
                    topic
                )?;

                let key_behaviour = EdDSABehaviourTargetRole::new(&key_id);

                let encryptor = NKeyTargetEncryptor::new(&public_keys, &peers, private_key).map_err(
                    |err| anyhow!("Unable to create encryptor: {}", err)
                )?;

                let recoverer = KeyshareRecoveryTarget::new(messenger, encryptor, key_behaviour);

                let encrypted_packages = recoverer.try_recieve_encrypted_packages()?;

                let result = recoverer.recover_keyshare(
                    self.recovery_index,
                    self.threshold,
                    encrypted_packages
                )?;

                recoverer.broadcast_result(result)
            }
            //Recovery of a ECDSA keyshare by a helper guardian
            (RecoveryRole::Helper, Key::ECDSA) => {
                let key_accessor = KeyshareAccessor::<ECDSA>::modifiable_with_email(
                    &key_id,
                    &email
                )?;
                let party_index = key_accessor.key.party_index;

                let (messenger, peers) = Nats::new_session(
                    conn,
                    &session_id,
                    &node,
                    &key_id,
                    party_index,
                    topic
                )?;

                let key_behaviour = ECDSABehaviourHelperRole::from_key_accessor(key_accessor);

                let encryptor = NKeyHelperEncryptor::new(
                    &public_keys,
                    self.recovery_index,
                    party_index,
                    &peers,
                    private_key
                ).map_err(|err| anyhow!("Unable to create encryptor: {}", err))?;

                let mut recoverer = KeyshareRecoveryHelper::new(
                    messenger,
                    encryptor,
                    key_behaviour
                );

                recoverer.try_recovery(self.recovery_index, Party {
                    party_index,
                    all_parties: peers,
                })
            }
            //Recovery procedure followed by target of ECDSA key recovery to receive and validate their new keyshare
            (RecoveryRole::Target, Key::ECDSA) => {
                let party_index = self.recovery_index;

                let (messenger, peers) = Nats::new_session(
                    conn,
                    &session_id,
                    &node,
                    &key_id,
                    party_index,
                    topic
                )?;

                let key_behaviour = ECDSABehaviourTargetRole::new(&key_id);

                let encryptor = NKeyTargetEncryptor::new(&public_keys, &peers, private_key).map_err(
                    |err| anyhow!("Unable to create encryptor: {}", err)
                )?;

                let recoverer = KeyshareRecoveryTarget::new(messenger, encryptor, key_behaviour);

                let encrypted_packages = recoverer.try_recieve_encrypted_packages()?;

                let result = recoverer.recover_keyshare(
                    self.recovery_index,
                    self.threshold,
                    encrypted_packages
                )?;

                recoverer.broadcast_result(result)
            }
            //Recovery procedure followed by target of 2fa key recovery to receive and validate their new keyshare
            (RecoveryRole::Target, Key::Sr25519) => {
                let party_index = self.recovery_index;

                let (messenger, peers) = Nats::new_session(
                    conn,
                    &session_id,
                    &node,
                    &key_id,
                    party_index,
                    topic
                )?;

                let key_behaviour = Sr25519BehaviourTargetRole::new(&key_id);

                let encryptor = NKeyTargetEncryptor::new(&public_keys, &peers, private_key).map_err(
                    |err| anyhow!("Unable to create encryptor: {}", err)
                )?;

                let recoverer = KeyshareRecoveryTarget::new(messenger, encryptor, key_behaviour);

                let encrypted_packages = recoverer.try_recieve_encrypted_packages()?;

                let result = recoverer.recover_keyshare(
                    self.recovery_index,
                    self.threshold,
                    encrypted_packages
                )?;

                recoverer.broadcast_result(result)
            }
        }
    }

    // Function to find the email for a key ID by searching the file system
    fn find_email_for_key(key_id: &str) -> Result<String> {
        use std::fs;

        use crate::config::Config;

        // Get the gridlock directory
        let gridlock_dir = Config::get_gridlock_directory();
        let accounts_dir = gridlock_dir.join("accounts");

        if !accounts_dir.exists() {
            return Err(anyhow!("Cannot find accounts directory"));
        }

        // List all email directories
        if let Ok(email_dirs) = fs::read_dir(&accounts_dir) {
            for email_entry in email_dirs.flatten() {
                let email_path = email_entry.path();
                if email_path.is_dir() {
                    let email = email_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .ok_or_else(|| anyhow!("Invalid email directory name"))?;

                    // Check if this email has the key we're looking for
                    let key_path = email_path.join("keys").join(key_id);
                    let keyshare_file = key_path.join(format!("keyshare-{}.json", key_id));

                    if keyshare_file.exists() {
                        return Ok(email.to_string());
                    }
                }
            }
        }

        Err(anyhow!("Could not find email associated with key_id: {}", key_id))
    }
}

pub fn handle_new_session_message(app: &App, message: nats::Message) {
    let session = match serde_json::from_slice::<NewKeyShareRecoverySession>(&message.data[..]) {
        Ok(session) => session,
        Err(err) => {
            error!("Incorrect keyshare recovery message format: {}", err);
            return;
        }
    };

    let nc = app.nc.clone();
    let session_id = session.session_id.clone();
    let thread_session_id = session_id.clone(); // Clone again for thread
    match
        thread::Builder
            ::new()
            .name(format!("keyshare_recovery_session_{}", &session_id))
            .spawn(move || {
                match session.handle(nc) {
                    Ok(_) => {
                        info!(
                            "Keyshare recovery was successful for session id {}",
                            &thread_session_id
                        );
                    }
                    Err(err) => {
                        error!(
                            "Keyshare recovery failed: session id: {}, error: {}",
                            &thread_session_id,
                            err
                        );
                    }
                };
            })
    {
        Ok(_) => {
            info!("Spawned a thread to handle keyshare recovery");
        }
        Err(_) => {
            error!("Failed to spawn thread for keyshare recovery");
        }
    };
}
