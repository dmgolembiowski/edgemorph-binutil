use binrw::{self, BinRead, BinReaderExt, derive_binread, NullString, io::{Read, Seek, SeekFrom}};
use std::io::BufReader;
use std::path::Path;

pub use binread::Error;
pub use binread::BinResult;

mod reader;

#[derive(BinRead, Debug, PartialEq)]
pub struct EdbDumpFile {

    /// Dump file format marker
    pub fmt_marker: FmtMarker,

    /// Format version number (8 bytes)
    #[br(temp)]
    pub version_number: u32,

    pub dump_header: DumpHeader,

    #[br(count = file_count)]
    pub files: Vec<FileEntry>,
}

#[derive(BinRead, Debug, PartialEq)]
#[br(magic = b"\xFF\xD8\x00\x00\xD8EDGEDB\x00DUMP\x00")]
pub struct FmtMarker (
    #[br(temp)]
    u32,
);


#[derive(BinRead, Debug, PartialEq)]
pub struct DumpHeader {
    /// Message type
    #[br(temp)]
    pub mtype: i8,

    /// SHA1 hash sum of block data
    #[br(temp)]
    pub checksum: [u8; 20],

    /// Length of message contents in bytes,
    /// including self
    #[br(temp)]
    pub msg_len: i32,

    /// Block data
    /// Should be treated in opaque way by a client.
    #[br(
        restore_position,
        seek_after = SeekFrom::Start(msg_len as u64),
        try_map = replace_packets,
        count = msg_len - 4
    )]
    pub data: Vec<u8>
}


#[derive(BinRead, Debug, PartialEq)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
