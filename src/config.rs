use std::io::{ErrorKind, Write};
use std::path::Path;
use std::process::exit;

use error::{ConfigLoadError, ConfigSaveError};

use crate::dirmark::Dirmarks;

pub fn try_save_config(dirmarks: &Dirmarks, path: &Path) -> Result<(), ConfigSaveError> {
    // ensure that the directory exists
    let mut path_dir = path.to_path_buf();
    if !path_dir.exists() {
        path_dir.pop();
        std::fs::create_dir_all(&path_dir)?;
    }

    let mut file = std::fs::File::create(path)?;
    let config_json_string = serde_json::to_string_pretty(dirmarks)?;

    file.write_all(config_json_string.as_bytes())?;

    Ok(())
}


pub fn try_load_config(path: &Path) -> Result<Dirmarks, ConfigLoadError> {
    let config_json_string = std::fs::read_to_string(path)?;
    let dirmarks: Dirmarks = serde_json::from_str(&config_json_string)?;

    Ok(dirmarks)
}

pub fn load_or_create_config(path: impl AsRef<Path>) -> Dirmarks {
    match try_load_config(path.as_ref()) {
        Ok(config) => return config,
        Err(err) => match err {
            ConfigLoadError::Io(err) => {
                if err.kind() != ErrorKind::NotFound {
                    eprintln!("Failed to read config: {}", err.kind());
                    exit(1);
                }
            }
            ConfigLoadError::Serialization(_) => {
                eprintln!("Failed to parse config!\nMake sure that the config file is formatted correctly, or execute 'mark-dir --reset-config' to reset the current config.");
                exit(1);
            }
        }
    }

    println!("No config file found, creating one.");

    let new_config = Dirmarks::new();
    reset_config(&new_config, path.as_ref());

    Dirmarks::new()
}

pub fn reset_config(dirmarks: &Dirmarks, path: impl AsRef<Path>) {
    if let Err(err) = try_save_config(&dirmarks, path.as_ref()) {
        eprintln!("Failed to create config file. {}! Location: {}", err, path.as_ref().display());
        exit(1);
    }
}

pub mod error {
    use std::fmt::{Display, Formatter};
    use std::io::ErrorKind;

    const fn error_message<'a>(error_kind: ErrorKind) -> &'a str {
        match error_kind {
            ErrorKind::NotFound => "File not found",
            ErrorKind::PermissionDenied => "Permission denied",
            _ => "Unknown error",
        }
    }

    #[derive(Debug)]
    pub enum ConfigSaveError {
        Io(std::io::Error),
        Serialization(serde_json::Error),
    }

    impl From<std::io::Error> for ConfigSaveError {
        fn from(value: std::io::Error) -> Self {
            ConfigSaveError::Io(value)
        }
    }

    impl From<serde_json::Error> for ConfigSaveError {
        fn from(value: serde_json::Error) -> Self {
            ConfigSaveError::Serialization(value)
        }
    }

    impl Display for ConfigSaveError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let error_message = match self {
                ConfigSaveError::Io(err) => error_message(err.kind()),
                ConfigSaveError::Serialization(err) => {
                    match err.io_error_kind() {
                        None => "Unknown error",
                        Some(error_kind) => error_message(error_kind),
                    }
                }
            };
            write!(f, "{}", error_message)
        }
    }

    #[derive(Debug)]
    pub enum ConfigLoadError {
        Io(std::io::Error),
        Serialization(serde_json::Error),
    }

    impl From<std::io::Error> for ConfigLoadError {
        fn from(value: std::io::Error) -> Self {
            ConfigLoadError::Io(value)
        }
    }

    impl From<serde_json::Error> for ConfigLoadError {
        fn from(value: serde_json::Error) -> Self {
            ConfigLoadError::Serialization(value)
        }
    }

    impl Display for ConfigLoadError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let error_message = match self {
                ConfigLoadError::Io(err) => error_message(err.kind()),
                ConfigLoadError::Serialization(err) => {
                    match err.io_error_kind() {
                        None => "Unknown error",
                        Some(error_kind) => error_message(error_kind),
                    }
                }
            };
            write!(f, "{}", error_message)
        }
    }
}
