use std::env;
use std::process::Command;

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
    let archive_type = archive_type(archive.clone())?;

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

fn archive_type(archive: std::path::PathBuf) -> Result<ArchiveType, &'static str> {
    let extension = archive.extension();

    let second_ext = match archive.file_stem() {
        None => return Err("Invalid archive name"),
        Some(basename) => std::path::PathBuf::from(basename),
    };

    match extension {
        None => Err("No extension found for path"),
        Some(first_ext) => match first_ext.to_str() {
            Some("zip") | Some("zipx") | Some("ZIP") => Ok(ArchiveType::Zip),
            Some("rar") => Ok(ArchiveType::Rar),
            Some("7z") => Ok(ArchiveType::SevenZip),
            Some("tar") => Ok(ArchiveType::Tar),
            Some("tb2") | Some("tbz") | Some("tbz2") | Some("tz2") => Ok(ArchiveType::TarBzip2),
            Some("bz2") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarBzip2),
                Some(&_) => Err("Invalid archive type for `bzip2` compressed data"),
                None => Err("Could not determine archive type for `bzip2` compressed data"),
            },
            Some("tgz") | Some("taz") => Ok(ArchiveType::TarGzip),
            Some("gz") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarGzip),
                Some(&_) => Err("Invalid archive type for `gzip` compressed data"),
                None => Err("Could not determine archive type for `gzip` compressed data"),
            },
            Some("lz") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarLzip),
                Some(&_) => Err("Invalid archive type for `lzip` compressed data"),
                None => Err("Could not determine archive type for `lzip` compressed data"),
            },
            Some("tlz") => Ok(ArchiveType::TarLzma),
            Some("lzma") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarLzma),
                Some(&_) => Err("Invalid archive type for `lzma` compressed data"),
                None => Err("Could not determine archive type for `lzma` compressed data"),
            },
            Some("lzo") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarLzop),
                Some(&_) => Err("Invalid archive type for `lzop` compressed data"),
                None => Err("Could not determine archive type for `lzop` compressed data"),
            },
            Some("txz") => Ok(ArchiveType::TarXz),
            Some("xz") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarXz),
                Some(&_) => Err("Invalid archive type for `xz` compressed data"),
                None => Err("Could not determine archive type for `xz` compressed data"),
            },
            Some("tZ") | Some("taZ") => Ok(ArchiveType::TarCompress),
            Some("Z") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarCompress),
                Some(&_) => Err("Invalid archive type for `Compress` compressed data"),
                None => Err("Could not determine archive type for `Compress` compressed data"),
            },
            Some("tzst") => Ok(ArchiveType::TarZstd),
            Some("zst") => match second_ext.to_str() {
                Some("tar") => Ok(ArchiveType::TarZstd),
                Some(&_) => Err("Invalid archive type for `zstd` compressed data"),
                None => Err("Could not determine archive type for `zstd` compressed data"),
            },
            Some(&_) => Err("Unknown archive extension"),
            None => Err("Invalid archive string"),
        },
    }
}
