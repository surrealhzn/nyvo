use nyvo::*;
use std::{
    fs::OpenOptions,
    io::{Cursor, Seek},
};

#[test]
fn test_create_simple_empty() {
    let mut buffer = Cursor::new(vec![]);
    let archive = ArchiveBuilder::new();
    archive.build(&mut buffer).unwrap();

    buffer.rewind().unwrap();
    let mut archive = load_archive(&mut buffer).unwrap();
    assert!(archive.check_integrity().unwrap());
    assert_eq!(archive.encryption_methods.len(), 0);
    assert_eq!(archive.store_methods, vec![StoreMethod::default()]);
    assert_eq!(archive.content.len(), 0);
    let archive = IndexedArchive::try_from(archive).unwrap();
    assert_eq!(archive.index.len(), 0);
}

#[test]
fn test_create_encrypted_empty() {
    let mut buffer = Cursor::new(vec![]);
    let mut archive = ArchiveBuilder::new();
    archive.encrypt(b"Password");
    archive.build(&mut buffer).unwrap();

    buffer.rewind().unwrap();
    let mut archive = load_archive(&mut buffer).unwrap();
    assert!(archive.check_integrity().unwrap());
    assert_eq!(archive.encryption_methods.len(), 1);
    assert_eq!(archive.store_methods, vec![StoreMethod::default()]);
    assert!(!archive.unlock(b"password").unwrap());
    assert!(archive.unlock(b"Password").unwrap());
    assert_eq!(archive.content.len(), 0);
    let archive = IndexedArchive::try_from(archive).unwrap();
    assert_eq!(archive.index.len(), 0);
}

#[test]
fn test_create_simple_1f() {
    let mut buffer = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open("./test.nyvo")
        .unwrap();
    let mut archive = ArchiveBuilder::new();

    let index = archive.add_index(Default::default());
    let block = archive.add_content_block(Default::default());

    let mut file = Cursor::new(b"Hello, world!");
    let content = archive.add_content(block, &mut file, 13);
    archive.index(index, "test.txt".into(), content);

    archive.build(&mut buffer).unwrap();

    buffer.rewind().unwrap();
    let mut archive = load_archive(&mut buffer).unwrap();
    assert!(archive.check_integrity().unwrap());
    assert_eq!(archive.encryption_methods.len(), 0);
    assert_eq!(archive.store_methods, vec![StoreMethod::default()]);
    assert_eq!(archive.content.len(), 2);
    assert!(archive.content[0].is_index);
    assert_eq!(archive.content[0].store_method, 0);
    assert!(!archive.content[1].is_index);
    assert_eq!(archive.content[1].store_method, 0);
    let archive = IndexedArchive::try_from(archive).unwrap();
    assert_eq!(archive.index.len(), 1);
    assert_eq!(
        archive.index.get("test.txt").unwrap(),
        &IndexEntry {
            block: 0,
            offset: 0,
            len: 13,
        }
    );
}

#[test]
fn test_create_encrypted_1f() {
    let mut buffer = Cursor::new(vec![]);
    let mut archive = ArchiveBuilder::new();
    let encryption = archive.encrypt(b"Password");
    let method = archive.add_store_method(encryption, CompressionAlgorithm::None, 0);

    let index = archive.add_index(&method);
    let block = archive.add_content_block(&method);

    let mut file = Cursor::new(b"Hello, world!");
    let content = archive.add_content(block, &mut file, 13);
    archive.index(index, "test.txt".into(), content);

    archive.build(&mut buffer).unwrap();

    buffer.rewind().unwrap();
    let mut archive = load_archive(&mut buffer).unwrap();
    assert!(archive.check_integrity().unwrap());
    assert_eq!(archive.encryption_methods.len(), 1);
    assert_eq!(
        archive.store_methods,
        vec![StoreMethod {
            encryption: 1,
            compression: CompressionAlgorithm::None
        }]
    );
    assert!(archive.unlock(b"Password").unwrap());
    assert_eq!(archive.content.len(), 2);
    assert!(archive.content[0].is_index);
    assert_eq!(archive.content[0].store_method, 0);
    assert!(!archive.content[1].is_index);
    assert_eq!(archive.content[1].store_method, 0);
    let archive = IndexedArchive::try_from(archive).unwrap();
    assert_eq!(archive.index.len(), 1);
    assert_eq!(
        archive.index.get("test.txt").unwrap(),
        &IndexEntry {
            block: 0,
            offset: 0,
            len: 13,
        }
    );
}

#[test]
fn test_create_compressed_1f() {
    let mut buffer = Cursor::new(vec![]);
    let mut archive = ArchiveBuilder::new();
    let method = archive.add_store_method(Default::default(), CompressionAlgorithm::Zstd, 1);

    let index = archive.add_index(&method);
    let block = archive.add_content_block(&method);

    let mut file = Cursor::new(b"Hello, world!");
    let content = archive.add_content(block, &mut file, 13);
    archive.index(index, "test.txt".into(), content);

    archive.build(&mut buffer).unwrap();

    buffer.rewind().unwrap();
    let mut archive = load_archive(&mut buffer).unwrap();
    assert!(archive.check_integrity().unwrap());
    assert_eq!(archive.encryption_methods.len(), 0);
    assert_eq!(
        archive.store_methods,
        vec![StoreMethod {
            encryption: 0,
            compression: CompressionAlgorithm::Zstd
        }]
    );
    assert_eq!(archive.content.len(), 2);
    assert!(archive.content[0].is_index);
    assert_eq!(archive.content[0].store_method, 0);
    assert!(!archive.content[1].is_index);
    assert_eq!(archive.content[1].store_method, 0);
    let archive = IndexedArchive::try_from(archive).unwrap();
    assert_eq!(archive.index.len(), 1);
    assert_eq!(
        archive.index.get("test.txt").unwrap(),
        &IndexEntry {
            block: 0,
            offset: 0,
            len: 13,
        }
    );
}

#[test]
fn test_create_encrypted_compressed_1f() {
    let mut buffer = Cursor::new(vec![]);
    let mut archive = ArchiveBuilder::new();
    let encryption = archive.encrypt(b"Password");
    let method = archive.add_store_method(encryption, CompressionAlgorithm::Zstd, 1);

    let index = archive.add_index(&method);
    let block = archive.add_content_block(&method);

    let mut file = Cursor::new(b"Hello, world!");
    let content = archive.add_content(block, &mut file, 13);
    archive.index(index, "test.txt".into(), content);

    archive.build(&mut buffer).unwrap();

    buffer.rewind().unwrap();
    let mut archive = load_archive(&mut buffer).unwrap();
    assert!(archive.check_integrity().unwrap());
    assert_eq!(archive.encryption_methods.len(), 1);
    assert_eq!(
        archive.store_methods,
        vec![StoreMethod {
            encryption: 1,
            compression: CompressionAlgorithm::Zstd
        }]
    );
    assert!(archive.unlock(b"Password").unwrap());
    assert_eq!(archive.content.len(), 2);
    assert!(archive.content[0].is_index);
    assert_eq!(archive.content[0].store_method, 0);
    assert!(!archive.content[1].is_index);
    assert_eq!(archive.content[1].store_method, 0);
    let archive = IndexedArchive::try_from(archive).unwrap();
    assert_eq!(archive.index.len(), 1);
    assert_eq!(
        archive.index.get("test.txt").unwrap(),
        &IndexEntry {
            block: 0,
            offset: 0,
            len: 13,
        }
    );
}
