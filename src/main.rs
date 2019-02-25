extern crate regex;
use structopt::StructOpt;
mod commands;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Manage /etc/hosts
pub enum Cli {
    #[structopt(name = "show")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Show current configuration.
    Show {},

    #[structopt(name = "add")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Add host to /etc/hosts.
    Add {
        // command
        ip: String,
        names: String,
        comment: Vec<String>,
    },

    #[structopt(name = "remove", alias = "rm")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Remove host from /etc/hosts (alias: rm).
    Remove { host: String },

    #[structopt(name = "disable", alias = "dis")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Disable host without removing (alias: dis).
    Disable { host: String },

    #[structopt(name = "enable", alias = "en")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Enable previously disabled host (alias: en).
    Enable { host: String },

    #[structopt(name = "check")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// (or rm) Remove host from /etc/hosts.
    Check {
        host: String,
        #[structopt(long = "exact", short = "e")]
        exact: bool,
    },
}

// use quicli::prelude::*;

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);

    match args {
        Cli::Show {} => commands::show::run(),
        Cli::Check { host, exact } => commands::check::run(&host, exact),
        Cli::Add { ip, names, comment } => commands::add::run(&ip, &names, &comment.join(" ")),
        _ => println!("Not implemented"),
    }
}
