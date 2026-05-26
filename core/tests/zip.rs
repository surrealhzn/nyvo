#![cfg(feature = "zip")]

use std::{
    io::{Cursor, Write},
    rc::Rc,
};

use nyvo_core::{
    env::Warning,
    formats::{ArchiveFormat, zip::ZipFormat},
};
use zip::{ZipWriter, write::SimpleFileOptions};

#[test]
fn test_zip() {
    let content = b"Hello, world!";
    let mut zip = ZipWriter::new(Cursor::new(vec![]));
    zip.start_file("test.txt", SimpleFileOptions::default())
        .unwrap();
    zip.write_all(content).unwrap();
    let mut zip = zip.finish().unwrap();

    let env = Rc::new(nyvo_core::env::Environment::new(Box::new(|_: Warning| {})).unwrap());

    let mut z = ZipFormat::new(env, &mut zip);
    z.index_blocks().unwrap();
    let index = z.index().unwrap();

    let file_id = 0;
    let block_id = index[file_id].block;

    let mut block = Cursor::new(Vec::new());
    z.extract_block(block_id, &mut block).unwrap();

    let mut file = Vec::new();
    z.extract_file(file_id, &mut block, &mut file).unwrap();

    assert_eq!(content, file.as_slice());
}
