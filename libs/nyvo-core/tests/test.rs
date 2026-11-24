use nyvo_core::{
    ArchiveFeature, ArchiveFormat, CompressionAlgorithm, ConvertOptionsBuilder, NyvoCore,
    NyvoSource, OptimizationMode,
};

#[test]
fn extract() {
    let nyvo = NyvoCore::new();

    nyvo.extract(
        NyvoSource::File("tests/samples/test.zip"),
        "tests/out/test-extract",
    )
    .unwrap();
}

#[test]
fn extract_partial() {
    let nyvo = NyvoCore::new();

    let mut archive = nyvo
        .open(NyvoSource::File("tests/samples/test.zip"))
        .unwrap();
    archive.index().unwrap();

    let files = archive
        .select_files(|f| f.path.to_str().unwrap() == "test.txt")
        .unwrap();

    archive
        .extract(files, "tests/out/test-extract-partial")
        .unwrap();
}

#[test]
fn create() {
    let nyvo = NyvoCore::new();

    let mut archive = nyvo.create(ArchiveFormat::Nyvo);
    assert!(archive.supports(ArchiveFeature::Create));

    archive.add_file(NyvoSource::File("tests/samples/test.txt"), "test.txt");
    archive.build("tests/out/test-create.nyvo").unwrap();
}

#[test]
fn convert() {
    let nyvo = NyvoCore::new();

    let mut archive = nyvo
        .open(NyvoSource::File("tests/samples/test.zip"))
        .unwrap();
    archive.index().unwrap();

    archive
        .convert(
            ConvertOptionsBuilder::new()
                .format(ArchiveFormat::Nyvo)
                .optimize(OptimizationMode::WriteSpeed)
                .encoding_rewriter(|entry| entry.compression(CompressionAlgorithm::None))
                .build(),
            "tests/out/test-convert.nyvo",
        )
        .unwrap();
}
