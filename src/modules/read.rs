use super::Result;
use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Debug, Clone)]
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
    pub fn extension(&self) -> &str {
        match self {
            Self::Zip => ".zip",
            Self::Rar => ".rar",
            Self::SevenZip => ".7z",
            Self::Tar => ".tar",
            Self::TarBzip2 => ".tar.bz2",
            Self::TarGzip => ".tar.gz",
            Self::TarLzip => ".tar.lz",
            Self::TarLzop => ".tar.lzo",
            Self::TarXz => ".tar.xz",
            Self::TarCompress => ".tar.Z",
            Self::TarZstd => ".tar.zstd",
        }
    }
}

// consists of ArchiveType, magic byte signature, and offset of magic
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

pub fn archive_magic(filename: String) -> Result<ArchiveType> {
    // let mut f = std::fs::File::open(filename)?;
    // let mut bytes = [0; 10];
    // let n = f.read(&mut bytes)?;
    let bytes_to_read = MAGIC
        .iter()
        .map(|(_, magic, offset)| magic.len() + offset)
        .max()
        .unwrap_or_default();

    let mut buffer = vec![0u8; bytes_to_read];
    // let f = File::open(filename)?;
    let n_read = File::open(filename)?.read(&mut buffer)?;

    for (archive_type, magic, offset) in MAGIC {
        if n_read >= (magic.len() + offset) && &buffer[*offset..(magic.len() + offset)] == *magic {
            return Ok(archive_type.clone());
        }
    }

    // let file = std::fs::File::open(filename)?;
    // file.read(buf)

    // if &bytes[..6] == [0x37, 0x7a, 0xbc, 0xaf, 0x27, 0x1c] {
    //     println!("7z {:x?}", &bytes[..6]);
    //     return Ok(ArchiveType::SevenZip);
    // } else if &bytes[..9] == [0x89, 0x4c, 0x5a, 0x4f, 0x00, 0x0d, 0x0a, 0x1a, 0x0a] {
    //     println!("lzop {:x?}", &bytes[..9]);
    //     return Ok(ArchiveType::TarLzop);
    // } else if &bytes[..10] == [0x42, 0x5a, 0x68, 0x39, 0x31, 0x41, 0x59, 0x26, 0x53, 0x59] {
    //     println!("bz2 {:x?}", &bytes[..10]);
    //     return Ok(ArchiveType::TarBzip2);
    // } else if &bytes[..12]
    //     == [
    //         0x4c, 0x5a, 0x49, 0x50, 0x01, 0xce, 0x00, 0x3a, 0x19, 0x4a, 0xce, 0x1d,
    //     ]
    // {
    //     println!("lzip {:x?}", &bytes[..12]);
    //     return Ok(ArchiveType::TarLzip);
    // } else if &bytes[257..257 + 5] == [0x75, 0x73, 0x74, 0x61, 0x72] {
    //     println!("tar {:x?}", &bytes[257..262]);
    //     return Ok(ArchiveType::Tar);
    // }

    // println!("{:x?}", &bytes[..n]);
    println!("Whaddya want from me, camman!");
    Ok(ArchiveType::Zip)
}
