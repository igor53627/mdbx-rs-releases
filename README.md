# mdbx-rs

Pure Rust implementation of [libmdbx](https://gitflic.ru/project/erthink/libmdbx) - an extremely fast embedded key-value database.

[![Crates.io](https://img.shields.io/crates/v/mdbx-rs.svg)](https://crates.io/crates/mdbx-rs)
[![Documentation](https://docs.rs/mdbx-rs/badge.svg)](https://docs.rs/mdbx-rs)

## Performance

| Operation | Rust mdbx-rs | C libmdbx |
|-----------|-------------|-----------|
| **PUT** | **143ms** | 167ms |
| **GET** | **74ms** | 88ms |
| **CURSOR** | **12ms** | 16.5ms |

Benchmarked with 1 million entries (20-byte keys, 32-byte values).

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

## Supported Platforms

| Platform | File |
|----------|------|
| Linux x86_64 | `mdbx-rs-linux-x86_64.tar.gz` |
| macOS Apple Silicon | `mdbx-rs-macos-aarch64.tar.gz` |

## Compatibility

- Binary compatible with C libmdbx databases
- No migration required for existing databases

## License

Apache 2.0
