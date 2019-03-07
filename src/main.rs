#[macro_use]
extern crate self_update;
extern crate parse_hosts;
extern crate regex;

use structopt::StructOpt;
mod commands;
mod hostsfile;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Manage /etc/hosts
pub enum Cli {
    #[structopt(name = "show", alias = "s")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Show current configuration (alias: s).
    Show {
        #[structopt(long = "summary", short = "s")]
        summary: bool,
    },

    #[structopt(name = "add", alias = "a")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Add host to /etc/hosts (alias: a).
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

    #[structopt(name = "check", alias = "c")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Check whether host is in hosts file (alias: c).
    Check {
        host: String,
        #[structopt(long = "exact", short = "e")]
        exact: bool,
    },

    #[structopt(name = "update", alias = "up")]
    /// Update hostman.
    Update {},
}

// use quicli::prelude::*;

fn main() {
    let args = Cli::from_args();

    match args {
        Cli::Show { summary } => commands::show(summary),
        Cli::Check { host, exact } => commands::check(&host, exact),
        Cli::Add { ip, names, comment } => commands::add(&ip, &names, &comment.join(" ")),
        Cli::Remove { host } => commands::remove(&host),
        Cli::Disable { host } => commands::disable(&host),
        Cli::Enable { host } => commands::enable(&host),
        Cli::Update {} => commands::update(),
        // _ => println!("Not implemented"),
    }
}
