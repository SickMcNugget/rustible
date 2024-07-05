use std::process::Command;

pub fn clone_repo() {
    let mut status = Command::new("git")
        .arg("clone")
        .arg("-q")
        .arg("https://github.com/SickMcNugget/Waste_Detection_New.git")
        .status()
        .expect("failed to execute git clone");

    if status.success() {
        println!("Successfully cloned to Waste_Detection_New");
        status = Command::new("rm")
            .arg("-rf")
            .arg("Waste_Detection_New")
            .status()
            .expect("Failed to remove cloned git repo");
        if status.success() {
            println!("Successfully deleted cloned git repo")
        } else {
            println!("Failed to delete cloned git repo")
        }
    } else {
        println!("Failed to clone to Waste_Detection_New {status}");
    }
}
