use fakefs::FsPath;

#[test]
fn paths() {
    assert_eq!(FsPath::from("a/../b"), "b".into());
    assert_eq!(FsPath::from("a/../a\\b/."), "a/b".into());
    assert_eq!(FsPath::from("/a\\../a/b/.."), "a".into());
    assert_eq!(FsPath::from("a/../a/"), "a".into());
    assert_eq!(FsPath::from("a/../a/b/../.."), "".into());
}
