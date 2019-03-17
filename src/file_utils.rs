use regex::Regex;

pub fn exact_match(needle: &str, haystack: &str) -> bool {
  let exact_matcher = Regex::new(format!(r"(^| ){}( |$)", needle).as_str()).unwrap();
  exact_matcher.is_match(haystack)
}

pub fn read_hosts(file_name: &str) -> String {
  std::fs::read_to_string(file_name).expect("could not read file")
}

pub fn backup_hosts(file_name: &str) {
  let backup_name = format!("{}.bak", file_name);
  if std::fs::copy(file_name, &backup_name).is_err() {
    eprintln!("Error: cannot write to backup file: {}", backup_name);
    std::process::exit(1);
  }
}

pub fn write_hosts(file_name: &str, contents: &str) {
  backup_hosts(file_name);
  if std::fs::write(file_name, contents).is_err() {
    eprintln!("Error: cannot write hosts file: {}", file_name);
    std::process::exit(1);
  }
}
