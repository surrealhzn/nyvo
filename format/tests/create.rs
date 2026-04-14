pub use nyvo::*;

#[test]
fn test_create_simple_empty() {
    let archive = CreateArchive {
        version: 1,
        encryption_methods: vec![],
        indexes: vec![],
        store_options: vec![],
    };
}
