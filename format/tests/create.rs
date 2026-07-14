use std::io::Cursor;

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
    let encryption_id = archive.encrypt(b"Password");
    archive.build(&mut buffer).unwrap();
}
