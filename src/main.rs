#[macro_use]
extern crate self_update;
// #[macro_use]
// extern crate nom;
extern crate hosts_parser;
extern crate regex;

use structopt::StructOpt;
mod cli;
use cli::{Cli, CliCmd};
mod commands;
mod file_utils;
mod hostsfile;
// mod nom_hosts;

fn main() {
    let args = Cli::from_args();

    match &args.cmd {
        CliCmd::Show { summary } => commands::show(*summary),
        CliCmd::Check { host, exact } => commands::check(&host, *exact),
        CliCmd::Add(sub_cmd) => commands::add(&args, sub_cmd),
        CliCmd::AddLocal(sub_cmd) => commands::add_local(&args, sub_cmd),
        CliCmd::Remove { host } => commands::remove(&args, &host),
        CliCmd::Disable { host } => commands::disable(&args, &host),
        CliCmd::Enable { host } => commands::enable(&args, &host),
        CliCmd::Update {} => commands::update(),
        // _ => println!("Not implemented"),
    }
}
