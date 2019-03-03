use parse_hosts::HostsFile;
use regex::Regex;
use std::fmt;

const HOSTS_FILE: &str = "/etc/hosts";
// const HOSTS_FILE: &str = "./hosts";

pub enum MatchType {
  Partial,
  Exact,
}

impl MatchType {
  pub fn from_bool(exact: bool) -> MatchType {
    return match exact {
      true => MatchType::Exact,
      false => MatchType::Partial,
    };
  }
}

pub struct ManagedHostsFile<'a> {
  lines: Vec<parse_hosts::Line<'a>>,
}

impl<'a> ManagedHostsFile<'a> {
  pub fn load() -> Result<ManagedHostsFile<'a>, std::io::Error> {
    let contents = read_hosts();
    let hf = HostsFile::read_buffered(contents.as_bytes());
    return Ok(ManagedHostsFile {
      lines: hf.lines().map(|l| l.unwrap()).collect(),
    });
  }

  pub fn must_load() -> ManagedHostsFile<'a> {
    return ManagedHostsFile::load().unwrap();
  }

  pub fn get_matches(&self, host: &str, exact: MatchType) -> Vec<String> {
    let lines: Vec<String> = self.lines.iter().map(|l| format!("{}", l)).collect();
    lines
      .into_iter()
      .filter(|line| match exact {
        MatchType::Exact => exact_match(host, line),
        MatchType::Partial => line.contains(host),
      })
      .collect()
  }

  pub fn has_host(&self, host: &str) -> bool {
    self.lines.iter().any(|l| l.hosts().any(|h| h == host))
  }

  pub fn has_disabled_host(&self, host: &str) -> bool {
    self.lines.iter().any(|l| match l.comment() {
      Some(c) => c.contains(host),
      _ => false,
    })
  }

  pub fn add_line(&mut self, line: &'a str) {
    let l = parse_hosts::Line::new(line).unwrap();
    self.lines.push(l);
  }

  pub fn remove_host(&mut self, host: &str) {
    let index = self
      .lines
      .iter()
      .position(|l| l.hosts().any(|h| h == host))
      .unwrap();
    self.lines.remove(index);
  }

  pub fn disable_host(&mut self, host: &str) {
    let position = self
      .lines
      .iter()
      .position(|l: &parse_hosts::Line| l.hosts().any(|h| h == host));

    if let Some(index) = position {
      let comment = format!("{}", self.lines[index]);
      let new_line = parse_hosts::Line::from_comment(&comment).into_owned();
      self.lines[index] = new_line;
    } else {
      println!("Error, line not found for {}", host)
    }
  }

  pub fn enable_host(&mut self, host: &str) {
    let position = self.lines.iter().position(|l| match l.comment() {
      Some(c) => c.contains(host),
      _ => false,
    });

    if let Some(index) = position {
      let comment = self.lines[index].comment().unwrap();
      let new_line = parse_hosts::Line::new(&comment).unwrap().into_owned();
      self.lines[index] = new_line;
    } else {
      println!("Error, line not found for {}", host)
    }
  }

  pub fn without_comments(&self) -> Vec<String> {
    self
      .lines
      .iter()
      .filter(|l| match l.data() {
        Some(_) => true,
        None => false,
      })
      .map(|l| format!("{}", l))
      .collect::<Vec<String>>()
  }

  pub fn save(&self) {
    let file_content = format!("{}", self);
    write_hosts(&file_content);
  }
}

fn exact_match(needle: &str, haystack: &str) -> bool {
  let exact_matcher = Regex::new(format!(r"(^| ){}( |$)", needle).as_str()).unwrap();
  return exact_matcher.is_match(haystack);
}

fn read_hosts() -> String {
  return std::fs::read_to_string(HOSTS_FILE).expect("could not read file");
}

fn backup_hosts() {
  let file_name = format!("{}.bak", HOSTS_FILE);
  if std::fs::copy(HOSTS_FILE, &file_name).is_err() {
    eprintln!("Error: cannot write to backup file: {}", file_name);
    std::process::exit(1);
  }
}

fn write_hosts(contents: &str) {
  backup_hosts();
  if std::fs::write(HOSTS_FILE, contents).is_err() {
    eprintln!("Error: cannot write hosts file: {}", HOSTS_FILE);
    std::process::exit(1);
  }
}

impl<'a> fmt::Display for ManagedHostsFile<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let hf = &self.lines;
    let lines: Vec<String> = hf.iter().map(|l| format!("{}", l)).collect();
    write!(f, "{}\n", lines.join("\n"))
  }
}
