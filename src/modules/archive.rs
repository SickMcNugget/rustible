use expanduser::expanduser;

use super::{ModuleError, Result};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

// consists of ArchiveType, magic, and offset of magic
const MAGIC: &[(ArchiveType, &[u8], usize)] = &[
    // 50 4B 03 04
    // 50 4B 05 06 (empty)
    // 50 4B 07 08 (spanned?)
    (ArchiveType::Zip, b"PK", 0),
    // 52 61 72 21 1A 07 00
    // 52 61 72 21 1A 07 01 00
    (ArchiveType::Rar, b"Rar!", 0),
    // 37 7A BC AF 27 1C
    (ArchiveType::SevenZip, b"7z", 0),
    // 75 73 74 61 72 00 30 30
    // 75 73 74 61 72 20 20 00
    (ArchiveType::Tar, b"ustar", 257),
    // 42 5A 68
    (ArchiveType::TarBzip2, b"BZh", 0),
    // 1F 8B
    (ArchiveType::TarGzip, &[0x1f, 0x8B], 0),
    // 4C 5A 49 50
    (ArchiveType::TarLzip, b"LZIP", 0),
    // 89 4c 5a 4f 00 0d 0a 1a 0a
    (
        ArchiveType::TarLzop,
        &[0x89, 0x4c, 0x5a, 0x4f, 0x00, 0x0d, 0x0a, 0x1a, 0x0a],
        0,
    ),
    // FD 37 7A 58 5A 00
    (ArchiveType::TarXz, &[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00], 0),
    // 1F 9D
    // 1F A0
    (ArchiveType::TarCompress, &[0x1F, 0x9D], 0),
    // 28 b5 2f fd
    (ArchiveType::TarZstd, &[0x28, 0xb5, 0x2f, 0xfd], 0),
];

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArchiveType {
    Zip,
    Rar,
    SevenZip,
    Tar,
    TarBzip2,
    TarGzip,
    TarLzip,
    TarLzop,
    TarXz,
    TarCompress,
    TarZstd,
}

impl ArchiveType {
    fn match_extension(filepath: &Path) -> Result<Self> {
        // Since there can be multiple extensions, like
        // .tar.gz, it's easier to match against the full filename,
        // excluding everything before the first '.'
        let extension = filepath
            .file_name()
            .expect("Filename somehow ended with '..'")
            .to_str()
            .expect("Filename contained invalid UTF-8")
            .split_once(".")
            .expect("Filename does not have an extension")
            .1;

        match extension {
            "zip" => Ok(Self::Zip),
            "rar" => Ok(Self::Rar),
            "7z" => Ok(Self::SevenZip),
            "tar" => Ok(Self::Tar),
            "tar.bz2" => Ok(Self::TarBzip2),
            "tar.gz" => Ok(Self::TarGzip),
            "tar.lz" => Ok(Self::TarLzip),
            "tar.lzo" => Ok(Self::TarLzop),
            "tar.xz" => Ok(Self::TarXz),
            "tar.Z" => Ok(Self::TarCompress),
            "tar.zst" => Ok(Self::TarZstd),
            _ => Err(ModuleError::PlainMessage(format!(
                "Unknown archive file extension: {extension}"
            ))),
        }
    }
}

fn archive_magic(filepath: &Path) -> Result<ArchiveType> {
    let bytes_to_read = MAGIC
        .iter()
        .map(|(_, magic, offset)| magic.len() + offset)
        .max()
        .unwrap_or_default();

    let mut buffer = vec![0u8; bytes_to_read];
    let n_read = File::open(filepath)?.read(&mut buffer)?;

    for (archive_type, magic, offset) in MAGIC {
        if n_read >= (magic.len() + offset)
            && &buffer[*offset..(magic.len() + offset)] == *magic
        {
            return Ok(archive_type.clone());
        }
    }

    Err(ModuleError::PlainMessage(format!(
        "Unable to determine archive type of {}",
        filepath.display()
    )))
}

pub fn determine_archive_type(filename: &str) -> Result<ArchiveType> {
    let filepath = expanduser(filename)?.canonicalize()?;
    if !filepath.is_file() {
        return Err(ModuleError::PlainMessage(format!(
            "{} is not a file",
            filepath.display()
        )));
    }

    let archive_type = ArchiveType::match_extension(&filepath);
    if archive_type.is_ok() {
        archive_type
    } else {
        archive_magic(&filepath)
    }
}

impl fmt::Display for ArchiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// pub fn unarchive()

// fn expand_home(archive: &mut String) -> Result<String, &'static str> {
//     match env::var("HOME") {
//         Ok(home) => Ok(archive.replacen("~", home.as_str(), 1)),
//         Err(_) => Err("HOME environment variable not found"),
//     }
// }

// fn unzip(archive: std::path::PathBuf) -> Result<(), &'static str> {
//     let status = Command::new("unzip").arg(archive).status();
//     let status = match status {
//         Ok(ret) => ret,
//         Err(_) => return Err("Failed to execute `unzip` command"),
//     };
//
//     if !status.success() {
//         return Err("Unable to run `unzip` on archive");
//     }
//
//     Ok(())
// }
// fn unrar(archive: std::path::PathBuf) -> Result<(), &'static str> {
//     let status = { Command::new("rar").arg("e").arg(archive).status() };
//     let status = match status {
//         Ok(ret) => ret,
//         Err(_) => return Err("Failed to execute `rar` command"),
//     };
//
//     if !status.success() {
//         return Err("Unable to run `rar` on archive");
//     }
//
//     Ok(())
// }
// fn un7z(archive: std::path::PathBuf) -> Result<(), &'static str> {
//     let status = { Command::new("7z").arg("e").arg(archive).status() };
//     let status = match status {
//         Ok(ret) => ret,
//         Err(_) => return Err("Failed to execute `7z` command"),
//     };
//
//     if !status.success() {
//         return Err("Unable to run `7z` on archive");
//     }
//
//     Ok(())
// }
// fn untar(archive: std::path::PathBuf) -> Result<(), &'static str> {
//     let status = { Command::new("tar").arg("-xf").arg(archive).status() };
//     let status = match status {
//         Ok(ret) => ret,
//         Err(_) => return Err("Failed to execute `tar` command"),
//     };
//
//     if !status.success() {
//         return Err("Unable to run `tar` on archive");
//     }
//
//     Ok(())
// }
//

// pub fn unarchive(archive: &mut String) -> Result<(), &'static str> {
//     archive = expand_home(archive)?;
//     let archive = std::path::PathBuf::from(archive);
//     println!("{:?}", archive);
//     let archive = match archive.canonicalize() {
//         Ok(path) => path,
//         Err(_) => return Err("archive does not exist"),
//     };
//     let archive_type = archive_type(&archive)?;
//
//     let current_dir = match std::env::current_dir() {
//         Ok(path) => path,
//         Err(_) => return Err("Could not determine the current directory"),
//     };
//
//     let archive_dir = match archive.parent() {
//         Some(dir) => dir,
//         None => return Err("Could not determine the parent directory of the archive"),
//     };
//
//     if std::env::set_current_dir(archive_dir).is_err() {
//         return Err("Unable to change to the archive directory");
//     };
//
//     let result = match archive_type {
//         ArchiveType::Zip => unzip(archive),
//         ArchiveType::Rar => unrar(archive),
//         ArchiveType::SevenZip => un7z(archive),
//         ArchiveType::Tar
//         | ArchiveType::TarBzip2
//         | ArchiveType::TarGzip
//         | ArchiveType::TarLzip
//         | ArchiveType::TarLzma
//         | ArchiveType::TarLzop
//         | ArchiveType::TarXz
//         | ArchiveType::TarCompress
//         | ArchiveType::TarZstd => untar(archive),
//     };
//
//     if result.is_err() {
//         return Err(result.unwrap_err());
//     }
//
//     match std::env::set_current_dir(current_dir) {
//         Ok(_) => Ok(()),
//         Err(_) => Err("Unable to return to the starting directory"),
//     }
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
    use super::*;

    const TESTTABLE: &[(&str, ArchiveType)] = &[
        ("zip", ArchiveType::Zip),
        ("rar", ArchiveType::Rar),
        ("7z", ArchiveType::SevenZip),
        ("tar", ArchiveType::Tar),
        ("tar.bz2", ArchiveType::TarBzip2),
        ("tar.gz", ArchiveType::TarGzip),
        ("tar.lz", ArchiveType::TarLzip),
        ("tar.lzo", ArchiveType::TarLzop),
        ("tar.xz", ArchiveType::TarXz),
        ("tar.zst", ArchiveType::TarZstd),
        ("tar.Z", ArchiveType::TarCompress),
    ];

    #[test]
    fn archive_magic() {
        for (ext, archive_type) in TESTTABLE {
            let filepath = PathBuf::from(format!("resources/test.{ext}"));
            let result = super::archive_magic(filepath.as_path());
            assert!(
                result.is_ok(),
                "{} did not match magic",
                filepath.display()
            );
            let result = result.unwrap();
            assert_eq!(result, *archive_type);
        }
    }

    #[test]
    fn archive_extension() {
        for (ext, archive_type) in TESTTABLE {
            let filepath = PathBuf::from(format!("resources/test.{ext}"));
            let result =
                super::ArchiveType::match_extension(filepath.as_path());
            assert!(
                result.is_ok(),
                "{} for file {}",
                result.unwrap_err(),
                filepath.display()
            );
            let result = result.unwrap();
            assert_eq!(result, *archive_type);
        }
    }

    #[test]
    fn archive_type() {
        for (ext, archive_type) in TESTTABLE {
            let file = format!("resources/test.{ext}");
            let result = super::determine_archive_type(&file);
            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(result, *archive_type);
        }
    }
}
