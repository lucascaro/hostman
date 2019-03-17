use parse_hosts::HostsFile;
use std::fmt;

use crate::file_utils::*;

const SYSTEM_HOSTS_FILE: &str = "/etc/hosts";
// const HOSTS_FILE: &str = "./hosts";

pub enum MatchType {
    Partial,
    Exact,
}

impl MatchType {
    pub fn from_bool(exact: bool) -> MatchType {
        if exact {
            MatchType::Exact
        } else {
            MatchType::Partial
        }
    }
}

pub struct ManagedHostsFile<'a> {
    lines: Vec<parse_hosts::Line<'a>>,
    file_name: String,
}

impl<'a> ManagedHostsFile<'a> {
    pub fn load() -> Result<ManagedHostsFile<'a>, std::io::Error> {
        ManagedHostsFile::from_file(SYSTEM_HOSTS_FILE)
    }

    pub fn from_file(file_name: &str) -> Result<ManagedHostsFile<'a>, std::io::Error> {
        let contents = read_hosts(file_name);
        Ok(ManagedHostsFile::from_string(&contents, file_name))
    }

    pub fn from_string(contents: &str, file_name: &str) -> ManagedHostsFile<'a> {
        let hf = HostsFile::read_buffered(contents.as_bytes());
        ManagedHostsFile {
            lines: hf.lines().map(|l| l.unwrap()).collect(),
            file_name: String::from(file_name),
        }
    }

    pub fn must_load() -> ManagedHostsFile<'a> {
        ManagedHostsFile::load().unwrap()
    }

    pub fn get_matches(&self, host: &str, exact: &MatchType) -> Vec<String> {
        let lines: Vec<String> = self.lines.iter().map(|l| format!("{}", l)).collect();
        lines
            .into_iter()
            .filter(|line| match exact {
                MatchType::Exact => exact_match(host, line),
                MatchType::Partial => line.contains(host),
            })
            .collect()
    }

    pub fn get_multi_match(&self, hosts: &[&str], exact: &MatchType) -> Vec<String> {
        hosts
            .iter()
            .filter(|name| !self.get_matches(name, exact).is_empty())
            .map(|s| String::from(*s))
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
        self.lines
            .iter()
            .filter(|l| l.data().is_some())
            .map(|l| format!("{}", l))
            .collect::<Vec<String>>()
    }

    pub fn contents(&self) -> String {
        format!("{}", self)
    }

    pub fn save(&self) {
        let file_content = self.contents();
        write_hosts(&self.file_name, &file_content);
    }
}

impl<'a> fmt::Display for ManagedHostsFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hf = &self.lines;
        let lines: Vec<String> = hf.iter().map(|l| format!("{}", l)).collect();
        write!(f, "{}\n", lines.join("\n"))
    }
}
