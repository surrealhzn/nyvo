use fakefs::{FakeFs, FsContext};

#[test]
#[cfg(unix)]
fn unix_dirs() {
    let fs = FakeFs::root();

    let dirs = fs.here().unwrap();
    assert!(dirs.contains(&"bin".into()));

    let bin = fs.list(&"bin".into()).unwrap();
    assert!(bin.contains(&"cat".into()));
}
