use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    pub dir: Option<PathBuf>,

    #[arg(
    num_args = 1..,
    conflicts_with_all = ["remove", "list", "get"]
    )]
    pub key: Vec<String>,

    #[arg(
    short,
    long,
    alias = "get-all",
    conflicts_with_all = ["dir", "key", "remove", "get"]
    )]
    /// List all dirmarks
    pub list: bool,

    #[arg(
    short,
    long,
    num_args = 1..,
    value_name = "KEY",
    conflicts_with_all = ["dir", "key", "remove", "list"]
    )]
    /// Get the corresponding dirmark for the given key
    pub get: Vec<String>,

    #[arg(
    short,
    long,
    num_args = 1..,
    value_name = "KEY",
    conflicts_with_all = ["dir", "key", "list", "get"]
    )]
    /// Remove the dirmark specified by the key
    pub remove: Vec<String>,

    #[arg(
    long,
    conflicts_with_all = ["dir", "key", "list", "get", "remove"]
    )]
    /// Reset or recreate the config file. WARNING: This overwrites the current config, including all dirmarks!
    pub reset_config: bool,
}

pub mod functionality {
    use std::process::exit;

    use crate::config;
    use crate::dirmark::Dirmarks;

    use super::*;

    pub fn reset_config(config_file: &PathBuf) {
        config::reset_config(&Dirmarks::new(), &config_file);
    }

    pub fn add_dirmark(cli: &Cli, config_file: &PathBuf, dir: PathBuf, dirmarks: &mut Dirmarks) {
        if cli.key.is_empty() {
            eprintln!("Cannot add dirmark, no key specified!");
            exit(1);
        }

        dirmarks.insert(&cli.key, dir);

        if let Err(err) = config::try_save_config(&dirmarks, &config_file) {
            eprintln!("Failed to save config: {}", err);
            exit(1);
        }
    }

    pub fn list_dirmarks(dirmarks: &Dirmarks) {
        let max_key_length = dirmarks.get_all()
            .keys()
            .map(|key| key.len())
            .max()
            .unwrap_or(0);

        for (key, dirmark) in dirmarks.get_all() {
            println!("{}{}  =>  {}",
                     key.replace(".", " "),
                     " ".repeat(max_key_length - key.len()),
                     dirmark.path.display()
            );
        }
    }

    pub fn get_dirmark(cli: &Cli, dirmarks: &Dirmarks) {
        if let Some(dirmark) = dirmarks.get(&cli.get) {
            println!("{}", dirmark.path.display());
        } else {
            exit(1)
        }
        return;
    }

    pub fn remove_dirmark(cli: &Cli, config_file: &PathBuf, dirmarks: &mut Dirmarks) {
        if let Some(_) = dirmarks.delete(&cli.remove) {
            if let Err(err) = config::try_save_config(&dirmarks, &config_file) {
                eprintln!("Failed to save config: {}", err);
                exit(1);
            }
        } else {
            eprintln!("Cannot remove, key does not exist!");
            exit(1)
        }
        return;
    }
}
