use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Manage /etc/hosts
pub struct Cli {
  #[structopt(long = "dry-run", short = "d")]
  pub dry_run: bool,
  #[structopt(subcommand)]
  pub cmd: CliCmd,
}

#[derive(Debug, StructOpt)]
pub enum CliCmd {
  #[structopt(name = "show", alias = "s")]
  #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
  /// Show current configuration (alias: s).
  Show {
    #[structopt(long = "summary", short = "s")]
    summary: bool,
  },

  #[structopt(name = "local", alias = "l")]
  #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
  /// Add host to /etc/hosts (alias: a).
  AddLocal(CmdAddLocal),

  #[structopt(name = "add", alias = "a")]
  #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
  /// Add host to /etc/hosts (alias: a).
  Add(CmdAdd),

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

#[derive(Debug, StructOpt)]
pub struct CmdAddLocal {
  /// Update host if it already exists in the hosts file
  #[structopt(long = "update", short = "u")]
  pub update: bool,
  /// Host names to add, comma separated
  pub names: String,
  /// Optional comment
  pub comment: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct CmdAdd {
  /// Update host if it already exists in the hosts file
  #[structopt(long = "update", short = "u")]
  pub update: bool,
  /// IP Address of the line to add
  pub ip: String,
  /// Host names to add, comma separated
  pub names: String,
  /// Optional comment
  pub comment: Vec<String>,
}
