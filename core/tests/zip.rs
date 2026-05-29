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
fn test_zip_simple() {
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

    let file_id = index
        .iter()
        .position(|i| i.path.to_str().unwrap() == "test.txt")
        .unwrap();
    assert_eq!(file_id, 0);
    let block_id = index[file_id].block;
    assert_eq!(block_id, 0);

    let mut block = Cursor::new(Vec::new());
    z.extract_block(block_id, &mut block).unwrap();

    let mut file = Vec::new();
    z.extract_file(file_id, &mut block, &mut file).unwrap();

    assert_eq!(content, file.as_slice());
}

#[test]
fn test_zip_2files() {
    let content = b"Hello, world!";
    let content2 = b"Hello, world! 2";
    let mut zip = ZipWriter::new(Cursor::new(vec![]));
    zip.start_file("test.txt", SimpleFileOptions::default())
        .unwrap();
    zip.write_all(content).unwrap();
    zip.start_file("test2.txt", SimpleFileOptions::default())
        .unwrap();
    zip.write_all(content2).unwrap();
    let mut zip = zip.finish().unwrap();

    let env = Rc::new(nyvo_core::env::Environment::new(Box::new(|_: Warning| {})).unwrap());

    let mut z = ZipFormat::new(env, &mut zip);
    z.index_blocks().unwrap();
    let index = z.index().unwrap();

    let file_id = index
        .iter()
        .position(|i| i.path.to_str().unwrap() == "test.txt")
        .unwrap();
    assert_eq!(file_id, 0);
    let block_id = index[file_id].block;
    assert_eq!(block_id, 0);

    let file2_id = index
        .iter()
        .position(|i| i.path.to_str().unwrap() == "test2.txt")
        .unwrap();
    assert_eq!(file2_id, 1);
    let block2_id = index[file2_id].block;
    assert_eq!(block2_id, 1);

    let mut block = Cursor::new(Vec::new());
    z.extract_block(block_id, &mut block).unwrap();
    let mut block2 = Cursor::new(Vec::new());
    z.extract_block(block2_id, &mut block2).unwrap();

    let mut file = Vec::new();
    z.extract_file(file_id, &mut block, &mut file).unwrap();
    let mut file2 = Vec::new();
    z.extract_file(file2_id, &mut block2, &mut file2).unwrap();

    assert_eq!(content, file.as_slice());
    assert_eq!(content2, file2.as_slice());
}
