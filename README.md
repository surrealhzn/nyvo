<p align="center">
    <img src="https://surrealhorizon.com/nyvo.svg" alt="Nyvo Icon" width="96"/>
</p>

<h1 align="center">Surreal Nyvo</h1>

## Features

### Compilation feature flags

These can be customized to your liking if you want to compile Nyvo yourself.

- `compression` - support for the [most important compression algorithms](#in-compression)
- `compression-full` - support for [all compression algorithms](#in-compression-full) Nyvo supports
- `formats` - support for the [most important archive formats](#in-formats)
- `formats-full` - support for [all archive formats](#in-formats-full) Nyvo supports
- `full` - all features except `gui` for a full CLI build
- `gui` - the Nyvo GUI version, accessible through `nyvo gui`
- `http` - allows Nyvo to perform connections via HTTP

#### Archive formats

##### in `formats`

- `nyvo` - Nyvo (.nyvo) archive format
- `zip` - PKZIP (.zip) archive format

##### in `formats-full`

- `sourceengine` - archive formats used by the Source and Source 2 game engines like VPK or BSP

#### Compression algorithms

##### in `compression`

- `bzip2` - bzip2
- `deflate` - DEFLATE (zlib, gzip, zip)
- `lzma` - LZMA
- `lzma2` - LZMA2
- `ppmd` - PPMd
- `zstd` - Zstandard

##### in `compression-full`

- `lz4` - LZ4
- `zpaq` - ZPAQ

## Build

If you want to compile Nyvo yourself, simply clone the repository and build the project with Rust's package manager Cargo.
You can add any feature flag you want from [this list](#compilation-feature-flags).

```sh
git clone https://github.com/surrealhzn/nyvo
cd nyvo

# full GUI build
cargo build --release --features "full gui"

# full build without GUI
cargo build --release --features full

# minimal build with only ZIP and DEFLATE support compiled in
cargo build --release --features "zip deflate"
```

## License

This project has been licensed under [MIT License](LICENSE).
