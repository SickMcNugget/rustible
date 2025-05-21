use rustible::modules;

use clap::Parser;
// use std::fmt;
// use std::env;
// use std::process::Command;
// use std::vec::Vec;
// use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(Parser)]
struct Cli {
    playbook: std::path::PathBuf,
}

fn main() -> modules::Result<()> {
    Ok(())
}

// struct Task {
//     name: String,
//     module: RustibleModule,
// }
//
// impl Task {
//     fn new(name: String, module: RustibleModule) -> Task {
//         Task {
//             name: name,
//             module: module,
//         }
//     }
// }
//
// struct RustibleModule {
//     repo: String,
//     dest: String,
//     version: String,
// }
//
// impl RustibleModule {
//     fn new(repo: String, dest: String, version: String) -> RustibleModule {
//         RustibleModule {
//             repo: repo,
//             dest: dest,
//             version: version,
//         }
//     }
// }
//
// struct Playbook {
//     name: String,
//     hosts: String,
//     remote_usr: String,
//     tasks: Vec<Task>,
// }
//
// impl Playbook {
//     fn new(name: String, hosts: String, remote_usr: String, tasks: Vec<Task>) -> Playbook {
//         Playbook {
//             name: name,
//             hosts: hosts,
//             remote_usr: remote_usr,
//             tasks: tasks,
//         }
//     }
// }
//

// fn parse_playbook(_file: std::path::PathBuf) -> Playbook {
//     // let content = std::fs::read_to_string(file);
//     // for line in content.lines() {
//     //
//     // }
//
//     let output = {
//         Command::new("sh")
//             .arg("-c")
//             .arg("echo hello")
//             .output()
//             .expect("failed to execute sh")
//     };
//
//     // let ta
//
//     let hello = String::from_utf8(output.stdout).expect("Unable to unwrap command output");
//
//     return Playbook {
//         name: hello.clone(),
//         hosts: hello.clone(),
//         remote_usr: hello.clone(),
//         tasks: vec![Task {
//             name: hello.clone(),
//             module: RustibleModule {
//                 repo: hello.clone(),
//                 dest: hello.clone(),
//                 version: hello.clone(),
//             },
//         }],
//     };
// }
/*
- name: Git checkout
  tasks:
      rustible.builtin.git:
        repo: "https://github.com/SickMcNugget/Waste_Detection_New.git"
        dest: Waste_Detection_New
        version: detr_setup
*/
