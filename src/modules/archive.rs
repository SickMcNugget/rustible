use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
// use std::io;
use std::process::Command;

// FK it, just use the magic bytes at the beginning of the files to check for type.
fn archive_type(archive: &std::path::PathBuf) -> Result<ArchiveType, &'static str> {
    let archive_file = File::open(archive).unwrap();
    let f = BufReader::new(archive_file);
    for byte in f.bytes() {
        print!("{:#04x} ", byte.unwrap())
    }
    println!();
    // let buf = vec![];
    // archive_file.read(&buf);
    Ok(ArchiveType::Zip)
}

#[derive(Debug, PartialEq)]
enum ArchiveType {
    Zip,
    Rar,
    SevenZip,
    Tar,
    TarBzip2,
    TarGzip,
    TarLzip,
    TarLzma,
    TarLzop,
    TarXz,
    TarCompress,
    TarZstd,
}

impl fmt::Display for ArchiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn expand_home(archive: String) -> Result<String, &'static str> {
    match env::var("HOME") {
        Ok(home) => Ok(archive.replacen("~", home.as_str(), 1)),
        Err(_) => Err("HOME environment variable not found"),
    }
}

fn unzip(archive: std::path::PathBuf) -> Result<(), &'static str> {
    let status = Command::new("unzip").arg(archive).status();
    let status = match status {
        Ok(ret) => ret,
        Err(_) => return Err("Failed to execute `unzip` command"),
    };

    if !status.success() {
        return Err("Unable to run `unzip` on archive");
    }

    Ok(())
}
fn unrar(archive: std::path::PathBuf) -> Result<(), &'static str> {
    let status = { Command::new("rar").arg("e").arg(archive).status() };
    let status = match status {
        Ok(ret) => ret,
        Err(_) => return Err("Failed to execute `rar` command"),
    };

    if !status.success() {
        return Err("Unable to run `rar` on archive");
    }

    Ok(())
}
fn un7z(archive: std::path::PathBuf) -> Result<(), &'static str> {
    let status = { Command::new("7z").arg("e").arg(archive).status() };
    let status = match status {
        Ok(ret) => ret,
        Err(_) => return Err("Failed to execute `7z` command"),
    };

    if !status.success() {
        return Err("Unable to run `7z` on archive");
    }

    Ok(())
}
fn untar(archive: std::path::PathBuf) -> Result<(), &'static str> {
    let status = { Command::new("tar").arg("-xf").arg(archive).status() };
    let status = match status {
        Ok(ret) => ret,
        Err(_) => return Err("Failed to execute `tar` command"),
    };

    if !status.success() {
        return Err("Unable to run `tar` on archive");
    }

    Ok(())
}

pub fn unarchive(mut archive: String) -> Result<(), &'static str> {
    archive = expand_home(archive)?;
    let archive = std::path::PathBuf::from(archive);
    println!("{:?}", archive);
    let archive = match archive.canonicalize() {
        Ok(path) => path,
        Err(_) => return Err("archive does not exist"),
    };
    let archive_type = archive_type(&archive)?;

    let current_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => return Err("Could not determine the current directory"),
    };

    let archive_dir = match archive.parent() {
        Some(dir) => dir,
        None => return Err("Could not determine the parent directory of the archive"),
    };

    if std::env::set_current_dir(archive_dir).is_err() {
        return Err("Unable to change to the archive directory");
    };

    let result = match archive_type {
        ArchiveType::Zip => unzip(archive),
        ArchiveType::Rar => unrar(archive),
        ArchiveType::SevenZip => un7z(archive),
        ArchiveType::Tar
        | ArchiveType::TarBzip2
        | ArchiveType::TarGzip
        | ArchiveType::TarLzip
        | ArchiveType::TarLzma
        | ArchiveType::TarLzop
        | ArchiveType::TarXz
        | ArchiveType::TarCompress
        | ArchiveType::TarZstd => untar(archive),
    };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    match std::env::set_current_dir(current_dir) {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to return to the starting directory"),
    }
}

///Gets all the file extensions of a file
// fn file_extensions(file: std::path::PathBuf) -> Result<Vec<&'static str>, &'static str> {
//     let extensions: Vec<&str> = Vec::new();
//
//     loop {
//         let extension = file.extension();
//         let stem = file.file_stem();
//         if extension.is_some() {
//             extensions.push(extension.unwrap().to_str());
//         }
//         break;
//     }
//     let extension = file.extension();
//     let stem = file.file_stem();
//     return Ok(extensions);
// }

// fn archive_type(archive: std::path::PathBuf) -> Result<ArchiveType, &'static str> {
//     // if !valid_filename(archive) {
//     //     return Err(format!(
//     //         "Invalid filename for archive: {}",
//     //         archive.to_string_lossy()
//     //     ));
//     // }
//     let extension = archive.extension();
//
//     let stem = match archive.file_stem() {
//         None => return Err("Invalid archive name"),
//         Some(basename) => std::path::PathBuf::from(basename),
//     };
//
//     match extension {
//         None => Err("No extension found for path"),
//         Some(first_ext) => match first_ext.to_str() {
//             Some("zip") | Some("zipx") | Some("ZIP") => Ok(ArchiveType::Zip),
//             Some("rar") => Ok(ArchiveType::Rar),
//             Some("7z") => Ok(ArchiveType::SevenZip),
//             Some("tar") => Ok(ArchiveType::Tar),
//             Some("tb2") | Some("tbz") | Some("tbz2") | Some("tz2") => Ok(ArchiveType::TarBzip2),
//             Some("bz2") => match stem.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarBzip2),
//                 Some(&_) => Err("Invalid archive type for `bzip2` compressed data"),
//                 None => Err("Could not determine archive type for `bzip2` compressed data"),
//             },
//             Some("tgz") | Some("taz") => Ok(ArchiveType::TarGzip),
//             Some("gz") => match stem.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarGzip),
//                 Some(&_) => Err("Invalid archive type for `gzip` compressed data"),
//                 None => Err("Could not determine archive type for `gzip` compressed data"),
//             },
//             Some("lz") => match second_ext.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarLzip),
//                 Some(&_) => Err("Invalid archive type for `lzip` compressed data"),
//                 None => Err("Could not determine archive type for `lzip` compressed data"),
//             },
//             Some("tlz") => Ok(ArchiveType::TarLzma),
//             Some("lzma") => match second_ext.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarLzma),
//                 Some(&_) => Err("Invalid archive type for `lzma` compressed data"),
//                 None => Err("Could not determine archive type for `lzma` compressed data"),
//             },
//             Some("lzo") => match second_ext.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarLzop),
//                 Some(&_) => Err("Invalid archive type for `lzop` compressed data"),
//                 None => Err("Could not determine archive type for `lzop` compressed data"),
//             },
//             Some("txz") => Ok(ArchiveType::TarXz),
//             Some("xz") => match second_ext.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarXz),
//                 Some(&_) => Err("Invalid archive type for `xz` compressed data"),
//                 None => Err("Could not determine archive type for `xz` compressed data"),
//             },
//             Some("tZ") | Some("taZ") => Ok(ArchiveType::TarCompress),
//             Some("Z") => match second_ext.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarCompress),
//                 Some(&_) => Err("Invalid archive type for `Compress` compressed data"),
//                 None => Err("Could not determine archive type for `Compress` compressed data"),
//             },
//             Some("tzst") => Ok(ArchiveType::TarZstd),
//             Some("zst") => match second_ext.to_str() {
//                 Some("tar") => Ok(ArchiveType::TarZstd),
//                 Some(&_) => Err("Invalid archive type for `zstd` compressed data"),
//                 None => Err("Could not determine archive type for `zstd` compressed data"),
//             },
//             Some(&_) => Err("Unknown archive extension"),
//             None => Err("Invalid archive string"),
//         },
//     }
// }

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn archive_types() {
        // let extensions: [(&'static str, ArchiveType); 19] = [
        //     ("test.tar", ArchiveType::Tar),
        //     ("test.tb2", ArchiveType::TarBzip2),
        //     ("test.tbz", ArchiveType::TarBzip2),
        //     ("test.tbz2", ArchiveType::TarBzip2),
        //     ("test.tz2", ArchiveType::TarBzip2),
        //     ("test.tar.bz2", ArchiveType::TarBzip2),
        //     ("test.tgz", ArchiveType::TarGzip),
        //     ("test.taz", ArchiveType::TarGzip),
        //     ("test.tar.gz", ArchiveType::TarGzip),
        //     ("test.tar.lz", ArchiveType::TarLzip),
        //     ("test.tlz", ArchiveType::TarLzma),
        //     ("test.tar.lzma", ArchiveType::TarLzma),
        //     ("test.tar.lzo", ArchiveType::TarLzop),
        //     ("test.txz", ArchiveType::TarXz),
        //     ("test.tar.xz", ArchiveType::TarXz),
        //     ("test.tZ", ArchiveType::TarCompress),
        //     ("test.tar.Z", ArchiveType::TarCompress),
        //     ("test.tzst", ArchiveType::TarZstd),
        //     ("test.tar.zst", ArchiveType::TarZstd),
        // ];
        let extensions: [(String, ArchiveType); 1] = [("test.zip".to_owned(), ArchiveType::Zip)];

        for (name, expected_type) in extensions {
            let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            root.push(format!("resources/{name}"));
            let archive_file = std::path::PathBuf::from(&root);

            // println!("{}", archive_file.to_str().unwrap());
            let result = archive_type(&archive_file);
            assert!(result.is_ok(), "archive {} was not a valid name", name);
            let unwrapped = result.unwrap();
            assert_eq!(
                expected_type, unwrapped,
                "expected type={}, got={}",
                expected_type, unwrapped
            );
        }
        //     let result = archive_type(std::path::PathBuf::from(name));
        //     assert!(result.is_ok(), "archive {} was not a valid name", name);
        //     let unwrapped = result.unwrap();
        //     assert_eq!(
        //         expected_type, unwrapped,
        //         "expected type={}, got={}",
        //         expected_type, unwrapped
        //     );
    }

    // #[test]
    // fn file_extensions() {
    //     let extensions = super::file_extensions(std::path::PathBuf::from("test.tar.gz"));
    //     let expected: Vec<&str> = Vec::from(["gz", "tar"]);
    //     assert!(extensions.is_ok());
    //     let extensions = extensions.unwrap();
    //     let matching = extensions
    //         .iter()
    //         .zip(&expected)
    //         .filter(|&(extensions, expected)| extensions == expected)
    //         .count();
    //
    //     assert!(matching == extensions.len() && matching == expected.len())
    // }
    //
    // #[test]
    // fn zip_extensions() {
    //     let mut result = archive_type(std::path::PathBuf::from("test.zip"));
    //     assert!(result.is_ok());
    //     assert_eq!(ArchiveType::Zip, result.unwrap());
    //
    //     result = archive_type(std::path::PathBuf::from("test.zipx"));
    //     assert!(result.is_ok());
    //     assert_eq!(ArchiveType::Zip, result.unwrap());
    //
    //     result = archive_type(std::path::PathBuf::from("test.ZIP"));
    //     assert!(result.is_ok());
    //     assert_eq!(ArchiveType::Zip, result.unwrap());
    // }
    //
    // #[test]
    // fn rar_extensions() {
    //     let result = archive_type(std::path::PathBuf::from("test.rar"));
    //     assert!(result.is_ok());
    //     assert_eq!(ArchiveType::Rar, result.unwrap());
    // }
    //
    // #[test]
    // fn sevenzip_extensions() {
    //     let result = archive_type(std::path::PathBuf::from("test.7z"));
    //     assert!(result.is_ok());
    //     assert_eq!(ArchiveType::SevenZip, result.unwrap());
    // }
    //
    // #[test]
    // fn tar_extensions() {
    //     let extensions = [
    //         ("test.tar", ArchiveType::Tar),
    //         ("test.tb2", ArchiveType::TarBzip2),
    //         ("test.tbz", ArchiveType::TarBzip2),
    //         ("test.tbz2", ArchiveType::TarBzip2),
    //         ("test.tz2", ArchiveType::TarBzip2),
    //         ("test.tar.bz2", ArchiveType::TarBzip2),
    //         ("test.tgz", ArchiveType::TarGzip),
    //         ("test.taz", ArchiveType::TarGzip),
    //         ("test.tar.gz", ArchiveType::TarGzip),
    //         ("test.tar.lz", ArchiveType::TarLzip),
    //         ("test.tlz", ArchiveType::TarLzma),
    //         ("test.tar.lzma", ArchiveType::TarLzma),
    //         ("test.tar.lzo", ArchiveType::TarLzop),
    //         ("test.txz", ArchiveType::TarXz),
    //         ("test.tar.xz", ArchiveType::TarXz),
    //         ("test.tZ", ArchiveType::TarCompress),
    //         ("test.tar.Z", ArchiveType::TarCompress),
    //         ("test.tzst", ArchiveType::TarZstd),
    //         ("test.tar.zst", ArchiveType::TarZstd),
    //     ];
    //
    //     for (name, expected_type) in extensions {
    //         let result = archive_type(std::path::PathBuf::from(name));
    //         assert!(result.is_ok(), "archive {} was not a valid name", name);
    //         let unwrapped = result.unwrap();
    //         assert_eq!(
    //             expected_type, unwrapped,
    //             "expected type={}, got={}",
    //             expected_type, unwrapped
    //         );
    //     }
    // }
}
