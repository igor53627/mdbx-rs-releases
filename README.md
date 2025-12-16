# mdbx-rs Releases

Binary releases for **mdbx-rs** - a pure Rust implementation of [libmdbx](https://gitflic.ru/project/erthink/libmdbx).

## Downloads

See [Releases](https://github.com/igor53627/mdbx-rs-releases/releases) for pre-built binaries.

### Available Builds

| Platform | PGO Build | Standard Build |
|----------|-----------|----------------|
| Linux x86_64 | `mdbx-rs-linux-x86_64-pgo.tar.gz` | `mdbx-rs-linux-x86_64-standard.tar.gz` |
| Linux aarch64 | `mdbx-rs-linux-aarch64-pgo.tar.gz` | - |
| macOS Intel | `mdbx-rs-macos-x86_64-pgo.tar.gz` | - |
| macOS Apple Silicon | `mdbx-rs-macos-aarch64-pgo.tar.gz` | `mdbx-rs-macos-aarch64-standard.tar.gz` |

**Recommended:** Use PGO (Profile-Guided Optimization) builds for production - they are 10-15% faster.

## Performance vs C libmdbx

| Operation | Rust mdbx-rs + PGO | C libmdbx |
|-----------|-------------------|-----------|
| **PUT** | **143ms** | 167ms |
| **GET** | **74ms** | 88ms |
| **CURSOR** | **12ms** | 16.5ms |

## Compatibility

- Binary compatible with C libmdbx databases
- No migration required for existing databases
- Supported platforms: Linux (x86_64, aarch64), macOS (Intel, Apple Silicon)

## Usage

```toml
[dependencies]
mdbx-rs = { git = "https://github.com/igor53627/mdbx-rs" }
```

## License

Apache 2.0
