use std::{fs::File, io::Cursor};

pub use nyvo::*;

#[test]
fn test_create_simple_empty() {
    let mut buffer = Cursor::new(vec![]);
    let archive = ArchiveBuilder::new();
    archive.build(&mut buffer).unwrap();
}

#[test]
fn test_create_encrypted_empty() {
    let mut buffer = Cursor::new(vec![]);
    let mut archive = ArchiveBuilder::new();
    archive.encrypt(b"Password");
    archive.build(&mut buffer).unwrap();
}

#[test]
fn test_create_simple_1f() {
    let mut buffer = File::create("./test.nyvo").unwrap();
    let mut archive = ArchiveBuilder::new();
    let method = 0;

    let index = archive.add_index(method);
    let block = archive.add_content_block(method);

    let mut file = Cursor::new(b"Hello, world!");
    let content = archive.add_content(block, &mut file, 13);
    archive.index(index, "test.txt".into(), content);

    archive.build(&mut buffer).unwrap();
}

#[test]
fn test_create_encrypted_1f() {
    let mut buffer = Cursor::new(vec![]);
    let mut archive = ArchiveBuilder::new();
    let encryption = archive.encrypt(b"Password");
    let method = archive.add_store_method(encryption, CompressionAlgorithm::None, 0);

    let index = archive.add_index(method);
    let block = archive.add_content_block(method);

    let mut file = Cursor::new(b"Hello, world!");
    let content = archive.add_content(block, &mut file, 13);
    archive.index(index, "test.txt".into(), content);

    archive.build(&mut buffer).unwrap();
}
