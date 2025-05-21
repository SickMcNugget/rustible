// Written by Joren Regan
// Date: 2025-05-21
//
// Collects facts that rustible needs to perform operations on a host
use std::collections::HashMap;
use std::env;
use std::io::BufRead;
use std::process::Command;

type Facts = HashMap<&'static str, &'static str>;

// If we don't have permission, also treat apparmor as disabled
fn apparmor_enabled() -> bool {
    std::fs::exists("/sys/kernel/security/apparmor").unwrap_or_default()
}

// fn systemcaps() -> Facts {
//     let mut facts = Facts::default();
//
//     facts.insert("capabilities_enforced", "N/A");
//     facts.insert("capabilities", "N/A");
//
//     let stdout = Command::new("capsh")
//         .arg("--print")
//         .output()
//         .expect("Failed to execute capsh")
//         .stdout;
//
//     for result_line in stdout.lines() {
//         let line = result_line.expect("Error reading line from stdout");
//         if line.starts_with("Current: ") {
//             if line.split(":").nth(1).unwrap().trim() == "=ep" {
//                 facts.insert("capabilies_enforced", "false");
//             } else {
//                 facts.insert("capabilies_enforced", "true");
//                 let caps = line.split(":").nth(1).unwrap();
//
//                 facts.insert("capabilies", caps);
//             }
//             println!("{}", line);
//         }
//     }
//     facts
// }
//
// fn is_chroot() -> bool {
//     if
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facts_apparmor() {
        assert!(apparmor_enabled());
    }
    // #[test]
    // fn facts_systemcaps() {
    //     systemcaps();
    // }
}
