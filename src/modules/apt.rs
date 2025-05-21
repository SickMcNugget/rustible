use std::process::Command;
use std::string::FromUtf8Error;
use std::{error, io};
use std::{fmt, result};

enum Subcommand {
    Update,
}

type Result<T> = result::Result<T, AptError>;

#[derive(Debug)]
struct AptError {
    details: String,
}

impl AptError {
    fn new(details: String) -> Self {
        AptError { details }
    }
}

impl error::Error for AptError {}

impl fmt::Display for AptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error during `apt-get` operation: {}", self.details)
    }
}

impl From<FromUtf8Error> for AptError {
    fn from(err: FromUtf8Error) -> Self {
        AptError::new(err.to_string())
    }
}

impl From<io::Error> for AptError {
    fn from(err: io::Error) -> Self {
        AptError::new(err.to_string())
    }
}

impl From<&str> for AptError {
    fn from(str: &str) -> Self {
        AptError::new(str.to_string())
    }
}

// impl From<Utf8Error> for AptError {
//     fn from(err: Utf8Error) -> AptError {
//         AptError::Parse(err)
//     }
// }
//

fn check_apt_return(output: std::process::Output) -> Result<()> {
    match output.status.success() {
        true => Ok(()),
        false => {
            let stderr = String::from_utf8(output.stderr)?;
            if stderr.contains("are you root?") {
                return Err(AptError::from("Insufficient permissions"));
            } else {
                return Err(AptError::new(stderr));
            }
        }
    }
}

fn update_apt_cache() -> Result<()> {
    let output = Command::new("apt-get").arg("update").arg("-y").output()?;

    check_apt_return(output)
}

fn install(packages: Vec<String>) -> Result<()> {
    let output = Command::new("apt-get")
        .arg("install")
        .arg("-y")
        .args(packages)
        .output()?;

    check_apt_return(output)
}

fn remove(packages: Vec<String>) -> Result<()> {
    let output = Command::new("apt-get")
        .arg("remove")
        .arg("-y")
        .args(packages)
        .output()?;

    check_apt_return(output)
}

fn upgrade(packages: Vec<String>) -> Result<()> {
    let output = Command::new("apt-get")
        .arg("upgrade")
        .arg("-y")
        .args(packages)
        .output()?;

    check_apt_return(output)
}

// #[cfg(test)]
// mod tests {
//
//     use std::{thread::sleep, time::Duration};
//
//     use super::*;
//
//     #[test]
//     fn install_and_remove() {
//         let package = "sl";
//         let install_result = install_package(package);
//         assert!(
//             install_result.is_ok(),
//             "result was not okay from install, got: {}",
//             install_result.unwrap_err()
//         );
//         sleep(Duration::new(1, 0));
//         let remove_result = remove_package(package);
//         assert!(
//             remove_result.is_ok(),
//             "result was not okay from remove, got: {}",
//             remove_result.unwrap_err()
//         );
//     }
//
//     #[test]
//     fn update_cache() {
//         let update_result = update_apt_cache();
//         assert!(
//             update_result.is_ok(),
//             "result was not okay from update, got: {}",
//             update_result.unwrap_err()
//         );
//     }
// }
