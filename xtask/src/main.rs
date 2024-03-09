mod app;
mod build;
mod locale;
mod term;
mod tool;
mod ui;
use crate::app::{App, RouteId};

use app::{Bootstrap, Platform, StandardSbiEnabled};
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::error;
use once_cell::sync::Lazy;
use std::{error::Error, fs, io, path::Path};
use toml_edit::{value, Document};

#[derive(Parser)]
#[clap(name = "RustSBI Prototyping System")]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure this project
    Config,
    /// Make this project
    Make(BuildArgs),
    /// Build and flash output to board
    Flash(BuildArgs),
}

#[derive(Args)]
struct BuildArgs {}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut buf = String::new();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    match cli.command {
        Commands::Config => {
            let mut app = if config_file_exists()? {
                buf = read_config_file()?;
                let config: Config = toml_edit::de::from_str(&buf)?;
                App::from(config)
            } else {
                App::default()
            };
            term::terminal_main(&mut app)?;
            save_app_to_string(&app, &mut buf)?;
            write_config_file(&buf)?;
            Ok(())
        }
        Commands::Make(_args) => {
            if !config_file_exists()? {
                error!("No configuration files given and Xtask.toml does not exist!");
                error!("Configure RustSBI Prototyping System using:");
                error!("    cargo termconfig");
                return Err(Box::new(io::Error::from(io::ErrorKind::NotFound)));
            }
            buf = read_config_file()?;
            let config: Config = toml_edit::de::from_str(&buf)?;
            build::build_main(&config)?;
            Ok(())
        }
        Commands::Flash(_args) => {
            if !config_file_exists()? {
                error!("No configuration files given and Xtask.toml does not exist!");
                error!("Configure RustSBI Prototyping System using:");
                error!("    cargo termconfig");
                return Err(Box::new(io::Error::from(io::ErrorKind::NotFound)));
            }
            buf = read_config_file()?;
            let config: Config = toml_edit::de::from_str(&buf)?;
            build::build_main(&config)?;
            build::flash_main(&config)?;
            Ok(())
        }
    }
}

static PROJECT: Lazy<&'static Path> =
    Lazy::new(|| Path::new(std::env!("CARGO_MANIFEST_DIR")).parent().unwrap());

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    locale: Option<String>,
    bootstrap: Bootstrap,
    standard_sbi_enabled: Option<StandardSbiEnabled>,
    machine_fdt_ident_enabled: Option<bool>,
    platform: Platform,
}

fn save_app_to_string(app: &App, buf: &mut String) -> io::Result<()> {
    use serde_variant::to_variant_name;
    let mut doc = buf.parse::<Document>().expect("invalid doc");
    doc["locale"] = value(&app.locale);
    doc["bootstrap"] = value(to_variant_name(&app.bootstrap).unwrap());
    let StandardSbiEnabled {
        timer,
        ipi,
        rfence,
        hsm,
        srst,
        pmu,
        dbcn,
        susp,
        cppc,
        nacl,
        sta,
    } = app.standard_sbi_enabled;
    doc["standard-sbi-enabled"]["timer"] = value(timer);
    doc["standard-sbi-enabled"]["ipi"] = value(ipi);
    doc["standard-sbi-enabled"]["rfence"] = value(rfence);
    doc["standard-sbi-enabled"]["hsm"] = value(hsm);
    doc["standard-sbi-enabled"]["srst"] = value(srst);
    doc["standard-sbi-enabled"]["pmu"] = value(pmu);
    doc["standard-sbi-enabled"]["dbcn"] = value(dbcn);
    doc["standard-sbi-enabled"]["susp"] = value(susp);
    doc["standard-sbi-enabled"]["cppc"] = value(cppc);
    doc["standard-sbi-enabled"]["nacl"] = value(nacl);
    doc["standard-sbi-enabled"]["sta"] = value(sta);
    doc["machine-fdt-ident-enabled"] = value(app.machine_mode_fdt_ident_enabled);
    doc["platform"] = value(to_variant_name(&app.platform).unwrap());
    *buf = doc.to_string();
    Ok(())
}

fn config_file_exists() -> io::Result<bool> {
    PROJECT.join("Xtask.toml").try_exists()
}

fn read_config_file() -> io::Result<String> {
    fs::read_to_string(PROJECT.join("Xtask.toml"))
}

fn write_config_file(buf: &str) -> io::Result<()> {
    fs::write(PROJECT.join("Xtask.toml"), buf)
}
