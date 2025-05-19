mod modules;

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

fn main() {
    // modules::archive::unarchive("~/test/test.7z".to_string()).expect("Failed to extract archive");
    for thing in &[
        "zip", "rar", "7z", "tar", "tar.bz2", "tar.gz", "tar.lz", "tar.lzma", "tar.lzo", "tar.xz",
        "tar.Z", "tar.zst",
    ] {
        let filename = format!("/home/joren/code/rust/rustible/resources/test.{}", thing);
        println!("{filename}");
        let archive_type = modules::read::archive_magic(filename);
        println!("{archive_type:?}");
    }
    // modules::git::clone_repo()
    // let args = Cli::parse();
    //
    // println!("playbook: {:?}", args.playbook);
    // let yamlfile_str = std::fs::read_to_string(&args.playbook).unwrap();
    // let yaml = YamlLoader::load_from_str(yamlfile_str.as_str()).unwrap();
    // let playbook = &yaml[0];
    // let play = playbook[0];
    // let play_name = play["name"];
    // let play_hosts = play["hosts"];
    // let play_remote_usr = play["remote_usr"];
    //
    // let tasks = play["tasks"];
    // let task = tasks[0];
    // let task_name = task["name"];
    // let module = task["rustible.builtin.git"];
    // let module_repo = module["repo"];
    // let module_dest = module["dest"];
    // let module_version = module["version"];

    // let mut out_str = String::new();
    // {
    //     let mut emitter = YamlEmitter::new(&mut out_str);
    //     emitter.dump(playbook).unwrap();
    // }
    //
    // println!("{}", out_str)

    // println!("{:?}", goodyaml);
    //     .expect("`args.playbook` should correspond to a valid yaml file");
    // for line in yml.lines() {
    //     println!("{}", line);
    // }
    // let _playbook = parse_playbook(args.playbook);
    // clone_repo()
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
