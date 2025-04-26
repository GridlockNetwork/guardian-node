use crate::config::ConfigProvider;
use std::sync::Once;
use std::path::PathBuf;
use dotenv::dotenv;

pub struct ConfigGridlock {}

static mut STORAGE_DIR: Option<&str> = None;
static INIT: Once = Once::new();

fn get_storage_dir() -> &'static str {
    unsafe {
        INIT.call_once(|| {
            // Load .env file
            dotenv().ok();

            let value = std::env::var("STORAGE_DIR").unwrap_or_else(|_| "./storage".to_string());
            STORAGE_DIR = Some(Box::leak(value.into_boxed_str()));
        });
        STORAGE_DIR.unwrap()
    }
}

impl ConfigProvider for ConfigGridlock {
    fn create_data_dirs() -> std::io::Result<()> {
        std::fs::create_dir_all(get_storage_dir())
    }

    fn get_nats_address() -> String {
        match std::env::var("NATS_ADDRESS") {
            Ok(addr) if !addr.is_empty() => {
                println!("Using NATS server address from environment: {}", addr);
                addr
            }
            _ => {
                // If NATS_ADDRESS is missing or empty in that file, it's a critical configuration error.
                panic!(
                    "NATS_ADDRESS environment variable is not set or is empty. Please ensure it is defined in your .env file."
                );
            }
        }
    }

    fn get_key_storage_path(key_id: &str, index: usize) -> PathBuf {
        let path_append = if index > 0 {
            format!("--{}", &index.to_string())
        } else {
            String::from("")
        };
        PathBuf::from(format!("{}/keys--{}{}.json", get_storage_dir(), key_id, path_append))
    }

    fn get_key_info_storage_path(key_id: &str) -> PathBuf {
        PathBuf::from(format!("{}/info--{}.json", get_storage_dir(), key_id))
    }

    fn get_gridlock_directory() -> PathBuf {
        PathBuf::from(get_storage_dir().to_string())
    }
}
