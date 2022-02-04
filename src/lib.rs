#![allow(dead_code)]
#![allow(unused)]
#![forbid(unsafe_code)]

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

#[binrw]
#[brw(big, magic = b"\xFF\xD8\x00\x00\xD8EDGEDB\x00DUMP\x00")]
struct FmtMarker (#[brw(default)] u32);

#[binrw]
struct GeneralDumpBlock {
    #[brw(temp)]
    mtype: i8,

    #[brw(temp)]
    sha1sum: [u8; 20],

    #[brw(temp)]
    msg_len: i32,

    #[brw(
        count = msg_len - 4 as i32,
        restore_position,
        seek_before = SeekFrom::Start(msg_len as u64),
        map = replace_packets,
    )]
    data: Vec<u8>,
}

#[binrw]
#[brw(magic = 0x48)]
struct HeaderMsgType ( #[br(default)] i8 );

#[binrw]
enum Headers {
    #[brw(default, magic = )]
}
#[binrw]
struct DumpHeader {
    #[brw(temp)]
    mtype: HeaderMsgType,

    #[brw(temp)]
    sha1sum: [u8; 20],

    #[brw(temp)]
    msg_len: i32,

    #[brw(temp)]
    major_ver: i16,

    #[brw(temp)]
    minor_ver: i16,

    #[brw(temp)]
    num_types: i32,

    #[brw(
        count = num_types - 4 as i32,
        restore_position,
        seek_before = SeekFrom::Start(num_types as u64),
        map = replace_packets,
     )]
    types: Vec<TypeInfo>,

    #[brw(temp)]
    num_descriptors: i32,

    #[brw(
        count = num_types - 4 as i32,
        restore_position,
        seek_before = SeekFrom::Start(num_descriptors as u64),
        map = replace_packets,
     )]
    descriptors: Vec<ObjectDesc>,
}

#[binrw]
struct TypeInfo {
    #[brw(temp)] 
    type_name: String,

    #[brw(temp)]
    type_class: String,

    #[brw(temp)]
    type_id: [u8; 16],
}

#[binrw]
struct ObjectDesc {
    #[brw(temp)]
    object_id: [u8; 16],

    #[brw(temp)]
    description: Vec<u8>,

    #[brw(temp)]
    num_dependencies: i16,

    #[brw(
        count = num_dependencies - 2 as i32,
        restore_position,
        seek_before = SeekFrom::Start(num_dependencies as u64),
        map = replace_packets,
    )]
    dependency_id: [u8; 16]
}




fn replace_packets(bytes: Vec<u8>) -> Vec<u8> {
    bytes
        .into_iter()
        .map(|byte| match byte {
            0x40 => { 0x48 },
            0x3d => { 0x44 },
            rem  => { rem  }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
