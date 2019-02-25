// const HOSTS_FILE: &str = "/etc/hosts";
const HOSTS_FILE: &str = "./hosts";

fn read_hosts() -> String {
  return std::fs::read_to_string(HOSTS_FILE).expect("could not read file");
}

fn backup_hosts() {
  std::fs::copy(HOSTS_FILE, format!("{}.bak", HOSTS_FILE)).expect("could not backup file");
}

fn write_hosts(contents: &str) {
  backup_hosts();
  return std::fs::write(HOSTS_FILE, contents).expect("could not write to file");
}

pub mod show {
  pub fn run() {
    let content = super::read_hosts();
    println!("{}", content);
  }
}

pub mod check {
  use regex::Regex;

  // let substring_matcher = Regex::new(r"").unwrap();
  fn exact_match(needle: &str, haystack: &str) -> bool {
    let exact_matcher = Regex::new(format!(r"(^| ){}( |$)", needle).as_str()).unwrap();
    return exact_matcher.is_match(haystack);
  }

  pub fn get_matches<'a>(content: &'a str, host: &'a str, exact: bool) -> Vec<&'a str> {
    return content
      .split("\n")
      .collect::<Vec<&str>>()
      .into_iter()
      .filter(|line| {
        if exact {
          exact_match(host, line)
        } else {
          line.contains(host)
        }
      })
      .collect::<Vec<&str>>();
  }
  pub fn run(host: &str, exact: bool) {
    let content = super::read_hosts();
    // let found = content.split('\n').filter(|line: &str| line.matches(host)).collect<Vec<&str>>().join("\n");
    let found = get_matches(&content, host, exact);

    println!("{}", found.join("\n"));
  }
}

pub mod add {
  pub fn run(ip: &str, names: &str, comment: &str) {
    let content = super::read_hosts();
    let all_names = names.split(',').collect::<Vec<&str>>();
    for name in &all_names {
      let matches = super::check::get_matches(&content, name, true);
      if matches.len() > 0 {
        println!(
          "The requested host is already present: \n{}",
          matches.join("\n")
        );
        return;
      }
    }
    let name = all_names.join(" ");
    println!("Adding {} {} to /etc/hosts", ip, name);

    let computed_comment = match comment {
      "" => &name,
      _ => comment,
    };
    let content = format!("{}\n#{}\n{} {}", content, computed_comment, ip, name);
    super::write_hosts(&content);
    println!("{}", content);
  }
}
