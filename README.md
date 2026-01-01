# mdbx-rs

A pure Rust reimplementation of [libmdbx](https://gitflic.ru/project/erthink/libmdbx) - an extremely fast embedded key-value database.

[![Crates.io](https://img.shields.io/crates/v/mdbx-rs.svg)](https://crates.io/crates/mdbx-rs)
[![Documentation](https://docs.rs/mdbx-rs/badge.svg)](https://docs.rs/mdbx-rs)

## Acknowledgments

This project is a Rust reimplementation based on [libmdbx](https://gitflic.ru/project/erthink/libmdbx) by **Leonid Yuriev (Леонид Юрьев)**. We extend our sincere thanks for creating such an excellent database engine that served as the foundation for this work.

The original libmdbx is a remarkably well-engineered embedded database with exceptional performance characteristics. This Rust implementation aims to provide the same capabilities with memory safety guarantees.

## How This Crate Works

This crate provides low-level C-style FFI bindings. At build time, it downloads prebuilt static libraries from GitHub Releases:

- **Repository:** [igor53627/mdbx-rs-releases](https://github.com/igor53627/mdbx-rs-releases)
- **Artifacts:** `mdbx-rs-{platform}.tar.gz`
- **License:** Apache-2.0

## Source Code Availability

The Rust source code is currently in a private repository while we work toward production quality. Once the implementation is stable and well-tested, we plan to open-source the full codebase to enable community contributions.

In the meantime, this crate provides prebuilt binaries that are binary-compatible with C libmdbx databases.

## Performance

**Pure Rust mdbx-rs now outperforms C libmdbx on all operations.**

Benchmarks on x86_64 Linux (AMD EPYC) with 1M entries (20-byte keys, 32-byte values):

| Operation | Rust mdbx-rs | C libmdbx | Rust Advantage |
|-----------|-------------|-----------|----------------|
| **PUT** | 2.1M/sec | 2.1M/sec | Parity |
| **GET** | **4.5M/sec** | 4.0M/sec | **15% faster** |
| **CURSOR** | **55.7M/sec** | 40.8M/sec | **37% faster** |

These results are achieved with Profile-Guided Optimization (PGO), which is enabled by default in release builds.

### Key Optimizations

1. **Zero-copy reads** - Returns borrowed slices directly into mmap
2. **Unchecked page access** - Removes bounds checking in release mode
3. **Optimized binary search** - SIMD key comparison for fixed-size keys
4. **Byte-balanced page splits** - Optimal page fill reduces DB size

## Installation

```toml
[dependencies]
mdbx-rs = "0.2"
```

## Quick Start

```rust,ignore
use mdbx_rs::*;
use std::ffi::CString;

fn main() {
    unsafe {
        // Create environment
        let mut env = std::ptr::null_mut();
        mdbx_env_create(&mut env);

        // Set geometry (optional)
        mdbx_env_set_geometry(env, -1, -1, 1024*1024*1024, -1, -1, 4096);

        // Open database
        let path = CString::new("./mydb").unwrap();
        mdbx_env_open(env, path.as_ptr(), MDBX_NOSUBDIR | MDBX_NOTLS, 0o644);

        // Begin transaction
        let mut txn = std::ptr::null_mut();
        mdbx_txn_begin(env, std::ptr::null_mut(), 0, &mut txn);

        // Open database handle
        let mut dbi = 0;
        mdbx_dbi_open(txn, std::ptr::null(), 0, &mut dbi);

        // Put key-value
        let key = bytes_to_val(b"hello");
        let mut val = bytes_to_val(b"world");
        mdbx_put(txn, dbi, &key, &mut val, 0);

        // Commit
        mdbx_txn_commit(txn);

        // Close
        mdbx_env_close(env);
    }
}
```

## Configuring Database Size

By default, the database will grow automatically but may hit size limits. Use `mdbx_env_set_geometry` **before** `mdbx_env_open` to configure:

```rust,ignore
use mdbx_rs::*;

unsafe {
    let mut env = std::ptr::null_mut();
    mdbx_env_create(&mut env);

    // Set geometry: (env, size_lower, size_now, size_upper, growth_step, shrink_threshold, pagesize)
    // Use -1 for any parameter to keep default/auto value

    // Example: Allow database to grow up to 100GB
    let size_100gb: isize = 100 * 1024 * 1024 * 1024;
    mdbx_env_set_geometry(
        env,
        -1,           // size_lower: auto
        -1,           // size_now: auto
        size_100gb,   // size_upper: 100GB max
        -1,           // growth_step: auto (typically 16MB)
        -1,           // shrink_threshold: auto
        4096,         // pagesize: 4KB (standard)
    );

    // Now open the database
    let path = std::ffi::CString::new("./large_db").unwrap();
    mdbx_env_open(env, path.as_ptr(), MDBX_NOSUBDIR, 0o644);

    // ... use database ...

    mdbx_env_close(env);
}
```

If you encounter `MDBX_MAP_FULL` (-30797), increase `size_upper` before opening.

## Troubleshooting

### SIGBUS on Large Databases (1TB+)

**Fixed in v0.2.21**. Earlier versions could crash with SIGBUS when writing to large databases because tree splits can allocate 1000+ pages at once. If the file wasn't pre-extended, writes to mmap'd regions beyond EOF caused SIGBUS.

**Solution**: Upgrade to v0.2.21 or later.

**What changed:**
1. Pager now uses `geometry.size_upper` for the mmap virtual address range
2. File is extended at `env_open` if `metadata.size_now > file_size`
3. Before allocating pages, file is pre-extended by `growth_step` (default 16MB)

### MDBX_CORRUPTED (-30796)

If your database was corrupted by previous SIGBUS crashes, you'll need to restore from backup or resync from scratch.

## Supported Platforms

| Platform | Artifact |
|----------|----------|
| Linux x86_64 | `mdbx-rs-linux-x86_64.tar.gz` |
| macOS Apple Silicon | `mdbx-rs-macos-aarch64.tar.gz` |

## Offline Builds

By default, `mdbx-rs` downloads prebuilt binaries during `cargo build`.

To use a locally built library (for offline/air-gapped builds):

```bash
export MDBX_RS_LIB_DIR=/path/to/dir/containing/libmdbx_rs.a
cargo build
```

## Compatibility

- Binary compatible with C libmdbx databases (format version 0.13.x)
- No migration required for existing databases

## API Level

This crate exposes the **low-level C-style FFI API**. All database operations require `unsafe` blocks. A higher-level safe Rust wrapper may be provided in a future release.

## License

Apache 2.0

## Prebuilt Artifacts

Release archives contain only C-compatible libraries:

- `libmdbx_rs.a` - Static library (staticlib)
- `libmdbx_rs.so` / `libmdbx_rs.dylib` - Dynamic library (cdylib)

**Note:** `.rlib` files are **not** included. The `.rlib` format is Rust-internal and not stable across compiler versions. If you need to use mdbx-rs from Rust code with full optimization (inlining, LTO), depend on the source crate and let Cargo build from source.

### Supported Platforms

| Platform | Artifact |
|----------|----------|
| Linux x86_64 | `mdbx-rs-linux-x86_64.tar.gz` |
| macOS ARM64 | `mdbx-rs-macos-aarch64.tar.gz` |

### Usage from C/C++

```c
// Link with: -lmdbx_rs -lpthread -ldl -lm
#include <stdint.h>

extern int mdbx_env_create(void **env);
extern int mdbx_env_open(void *env, const char *path, unsigned flags, unsigned mode);
// ... see mdbx.h for full API
```

### Usage from Rust (via this crate)

```toml
[dependencies]
mdbx-rs = "0.2"
```

The build script automatically downloads and links the appropriate prebuilt library.
