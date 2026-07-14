# Nyvo archive format specification

Important links:

- [Reference implementation](..) (Rust)
- [Source code of this page](https://github.com/surrealhzn/nyvo/blob/main/format/spec/spec.md)
- [Nyvo format website](https://nyvo.rocks/)
- [Nyvo archiver website](https://surrealhorizon.com/nyvo)

## 0. About this specification

This specification should serve as a detailed description of the Nyvo archive format.
It is currently **incomplete** and should therefore not be your single source of information about the format.

In the following, the Nyvo archive format will be called "Nyvo" for simplicity.

## 1. General information

### 1.1. File extensions

It is recommended to use the one of the following file extensions for Nyvo:

- `.nyvo` - This should be used most of the time for consistency purposes.
- `.n0` - Alternative extension, may not be recognizedcorrectly by some OS or userspace applications.
- `.nyv` - For compability only, as some software requires the file extension not to be longer than 3 bytes.

### 1.2. Endianess

Nyvo uses **Little-endian** (LE) byte order everywhere without exceptions.

### 1.3. Variable-length integers

Also, Nyvo makes extensive use of variable-length integers (unsigned LEB128 encoding), which are also used in the
[RAR 5.0 archive format](https://www.rarlab.com/technote.htm#vint) and a wide range of other projects,
including [WebAssembly](https://webassembly.github.io/spec/core/binary/values.html#integers),
[LLVM](https://llvm.org/doxygen/LEB128_8h_source.html) and
[Android](https://source.android.com/docs/core/runtime/dex-format).

In the following (and the reference implementation), this kind of variable-length unsigned integers
is referred to as `vu8`. The theoretical limit of this encoding is infinite, but the reference implementation
cannot handle more bits than the target architecture supports (e.g. 64 bits for amd64/x86_64 or other 64-bit
architectures, 32 bits for i386/x86_32 and so on) due to internal memory adressing by the Rust programming language.

As this encoding is widely used, it is considered _trivial_ and will not be documented here.

&rarr; [LEB128 on Wikipedia](https://en.wikipedia.org/wiki/LEB128)

## 2. Content structure

| Section            | Notes                    |
| ------------------ | ------------------------ |
| Magic value        | always the first 8 bytes |
| Archive metadata   |                          |
| Encryption methods |                          |
| Store methods      |                          |

// TODO: continue specification, add section details, ...
