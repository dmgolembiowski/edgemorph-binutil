use binrw::{
    binread,
    io::{Cursor, SeekFrom},
    prelude::*,
    NullString 
};


use std::io::BufReader;
use std::path::Path;
//mod reader;

#[binread]
struct EdbDumpFile {
    /// Dump file format marker
    fmt_marker: FmtMarker,

    /// Format version number (8 bytes)
    #[br(temp)]
    version_number: i64,

    dump_header: DumpHeader,
}

#[binread]
#[br(little, magic = b"\xFF\xD8\x00\x00\xD8EDGEDB\x00DUMP\x00")]
struct FmtMarker (#[br(default)] u32);

#[binread]
struct DumpHeader {
    #[br(temp)]
    mtype: i8,

    #[br(temp)]
    checksum: [u8; 20],

    #[br(temp, big)]
    msg_len: i32,

    #[br(
        count = msg_len - 4 as i32,
        restore_position,
        seek_before = SeekFrom::Start(msg_len as u64),
        map = replace_packets,
    )]
    data: Vec<u8>,
}

fn replace_packets(byte: &u8) -> u8 {
    match *byte {
        0x40 => { 0x48 },
        0x3d => { 0x44 },
        rem  => { rem  }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
