use crate::hostsfile::MatchType;

pub fn show(summary: bool) {
  let hosts_file = crate::hostsfile::ManagedHostsFile::must_load();
  match summary {
    false => println!("{}", hosts_file),
    true => println!("{}", hosts_file.without_comments().join("\n")),
  }
}

pub fn check(host: &str, exact: bool) {
  let hosts_file = crate::hostsfile::ManagedHostsFile::must_load();
  let found = hosts_file.get_matches(host, MatchType::from_bool(exact));
  println!("{}", found.join("\n"));
}

pub fn add(ip: &str, names: &str, comment: &str) {
  let all_names = names.split(',').collect::<Vec<&str>>();
  let mut hosts_file = crate::hostsfile::ManagedHostsFile::must_load();
  for name in &all_names {
    let matches = hosts_file.get_matches(name, MatchType::Exact);
    if matches.len() > 0 {
      println!(
        "The requested host is already present: \n{}",
        matches.join("\n")
      );
      return;
    }
  }
  let names = all_names.join(" ");
  println!("Adding {} {} to /etc/hosts", ip, names);

  let computed_comment = match comment {
    "" => &names,
    _ => comment,
  };
  let host_line = format!("{} {} #{}", ip, names, computed_comment);
  hosts_file.add_line(&host_line);
  hosts_file.save();
}

pub fn remove(host: &str) {
  let mut hosts_file = crate::hostsfile::ManagedHostsFile::must_load();
  if !hosts_file.has_host(host) {
    println!("{} not in hosts file.", host);
    return;
  }
  println!("Removing host {}", host);
  hosts_file.remove_host(host);
  hosts_file.save();
}

pub fn disable(host: &str) {
  let mut hosts_file = crate::hostsfile::ManagedHostsFile::must_load();
  if !hosts_file.has_host(host) {
    if hosts_file.has_disabled_host(host) {
      println!("{} is already disabled in hosts file.", host);
    } else {
      println!("{} is not in hosts file.", host);
    }
    return;
  }
  println!("Disabling host {}", host);
  hosts_file.disable_host(host);
  hosts_file.save();
}

pub fn enable(host: &str) {
  let mut hosts_file = crate::hostsfile::ManagedHostsFile::must_load();
  if !hosts_file.has_disabled_host(host) {
    if hosts_file.has_host(host) {
      println!("{} is already enabled in hosts file.", host);
    } else {
      println!("{} is not in hosts file.", host);
    }
    return;
  }
  println!("Enabling host {}", host);
  hosts_file.enable_host(host);
  hosts_file.save();
}

pub fn update() {
  let target = self_update::get_target().expect("Error getting self-update target");
  let status = self_update::backends::github::Update::configure()
    .expect("error configuring backend")
    .repo_owner("lucascaro")
    .repo_name("hostman")
    .target(&target)
    .bin_name("hostman")
    .show_download_progress(true)
    .current_version(cargo_crate_version!())
    .build()
    .expect("cannot build")
    .update()
    .expect("cannot update");
  println!("Update status: `{}`!", status.version());
}
