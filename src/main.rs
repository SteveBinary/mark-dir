use std::process::exit;

use clap::Parser;

use config::load_or_create_config;

use crate::cli::Cli;

mod config;
mod cli;
mod dirmark;

static CONFIG_FILE_NAME: &str = "config.json";
static APP_NAME: &str = "mark-dir";

fn main() {
    let Some(base_directories) = directories::BaseDirs::new() else {
        eprintln!("Failed to get base directories!");
        exit(1);
    };

    let config_directory = base_directories.config_local_dir().join(APP_NAME);
    let config_file = config_directory.join(CONFIG_FILE_NAME);

    let cli = Cli::parse();

    if cli.reset_config {
        cli::functionality::reset_config(&config_file);
        return;
    }

    let mut dirmarks = load_or_create_config(&config_file);

    if let Some(dir) = cli.dir.clone() {
        cli::functionality::add_dirmark(&cli, &config_file, dir, &mut dirmarks);
        return;
    }

    if cli.list {
        cli::functionality::list_dirmarks(&dirmarks);
        return;
    }

    if !cli.get.is_empty() {
        cli::functionality::get_dirmark(&cli, &dirmarks);
        return;
    }

    if !cli.remove.is_empty() {
        cli::functionality::remove_dirmark(&cli, &config_file, &mut dirmarks);
        return;
    }
}
