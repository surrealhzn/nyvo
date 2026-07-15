# Nyvo archive format specification

Important links:

- [Reference implementation](https://github.com/surrealhzn/nyvo/tree/main/format) (Rust)
- [Source code of this page](https://github.com/surrealhzn/nyvo/blob/main/format/spec/spec.md)
- [Nyvo format website](https://nyvo.rocks/)
- [Nyvo archiver website](https://surrealhorizon.com/nyvo)

## 0. About this specification

This specification should serve as a detailed description of the Nyvo archive format.
It is currently **incomplete** and should therefore not be your single source of information about the format.

In the following, the Nyvo archive format will be called "Nyvo" for simplicity.

**Current specification version:** 1

## 1. General information

### 1.1. File extensions

It is recommended to use the one of the following file extensions for Nyvo:

- `.nyvo` - This should be used most of the time for consistency purposes.
- `.n0` - Alternative extension, may not be recognized correctly by some OS or userspace applications.
- `.nyv` - For compability only, as some software requires the file extension not to be longer than 3 bytes.

### 1.2. Endianness

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

> [!WARN]  
> Archives with variable integers larger than platform word size may cause unexpected errors.

As this encoding is widely used, it is considered _trivial_ and will not be documented here.

&rarr; [LEB128 on Wikipedia](https://en.wikipedia.org/wiki/LEB128)

## 2. Content structure

| Section            | Notes                    |
| ------------------ | ------------------------ |
| Magic value        | always the first 8 bytes |
| Archive metadata   |                          |
| Encryption methods |                          |
| Store methods      |                          |

### 2.1. Magic value

Always the first 8 bytes of the archive.
This should be used to detect the Nyvo format itself when checking the archive type.

| Type    | Name        | Content                   | Notes                                 |
| ------- | ----------- | ------------------------- | ------------------------------------- |
| `u8[8]` | magic value | `a8 28 4e 79 76 6f 28 a8` | escaped string:`\xa8\x28Nyvo\x28\xa8` |

### 2.2. Archive metadata

| Type  | Name                    | Content                                                                      | Notes           |
| ----- | ----------------------- | ---------------------------------------------------------------------------- | --------------- |
| `vu8` | Format version          | [Spec version](#0-about-this-specification) -1                               | `0` for Nyvo v1 |
| `vu8` | Encryption method count | Number of entries in the [encryption methods header](#23-encryption-methods) |                 |
| `vu8` | Store method count      | Number of entries in the store methods header                                |                 |

### 2.3. Encryption methods

This is an array of encryption methods with the length stored in the [archive metadata header](#22-archive-metadata).

An entry of this header is structured in this format:

| Type                              | Name                       | Content                                                                                  | Notes |
| --------------------------------- | -------------------------- | ---------------------------------------------------------------------------------------- | ----- |
| `vu8`                             | Encryption algorithm       | [Encryption algorithm](#231-encryption-algorithm-ids) used for content encryption        |       |
| `vu8`                             | Key derivation memory      | Memory cost for [Argon2](https://www.rfc-editor.org/info/rfc9106/)id key derivation      |       |
| `vu8`                             | Key derivation iterations  | Iteration cost for [Argon2](https://www.rfc-editor.org/info/rfc9106/)id key derivation   |       |
| `vu8`                             | Key derivation parallelism | Parallelism cost for [Argon2](https://www.rfc-editor.org/info/rfc9106/)id key derivation |       |
| `u8[32]`                          | Salt                       | Key derivation salt                                                                      |       |
| `vu8`                             | KEK count                  | Key encryption key count                                                                 |       |
| [Key](#232-key-data-structure)\[] | Keys                       | Key encryption keys                                                                      |       |

> [!WARN]  
> Key derivation fields may not exceed 32 bits.

#### 2.3.1. Encryption algorithm IDs

| ID  | Name                                                             |
| --- | ---------------------------------------------------------------- |
| `0` | [AES-256-GCM-SIV](https://datatracker.ietf.org/doc/html/rfc8452) |

More will be added in the future. For now, only AES-256-GCM-SIV is recommended and supported.

#### 2.3.2. Key

| Type     | Name  | Content                    | Notes                                     |
| -------- | ----- | -------------------------- | ----------------------------------------- |
| `u8[12]` | Nonce | DEK cipher nonce           |                                           |
| `u8[48]` | DEK   | Data encryption key cipher | 32-byte AES-256 key + 16-byte GCM-SIV tag |

The DEK will always be encrypted with AES-256-GCM-SIV, no matter what the "Encryption algorithm" fields says.

**Content decryption steps:**

1. Obtain a valid decryption passphrase of any length.
2. Pass the passphrase into the Argon2id key derivation function with correct parameters. This will return a KEK.
3. Try decrypting every DEK cipher using the KEK.

- If decryption succeeds, the passphrase is valid and the DEK can be used.
- If decryption fails, the passphrase might be invalid, try again with another DEK cipher.
- If every decryption attempt fails, the passphrase is invalid for this encryption method.

// TODO: continue
