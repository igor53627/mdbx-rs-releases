# mdbx-rs Releases

Binary releases for **mdbx-rs** - a pure Rust implementation of [libmdbx](https://gitflic.ru/project/erthink/libmdbx).

## Downloads

See [Releases](https://github.com/igor53627/mdbx-rs-releases/releases) for pre-built binaries.

### Available Builds

| Platform | File |
|----------|------|
| Linux x86_64 | `mdbx-rs-linux-x86_64.tar.gz` |
| macOS Apple Silicon | `mdbx-rs-macos-aarch64.tar.gz` |

## Performance vs C libmdbx

| Operation | Rust mdbx-rs | C libmdbx |
|-----------|-------------|-----------|
| **PUT** | **143ms** | 167ms |
| **GET** | **74ms** | 88ms |
| **CURSOR** | **12ms** | 16.5ms |

## Compatibility

- Binary compatible with C libmdbx databases
- No migration required for existing databases
- Supported platforms: Linux x86_64, macOS Apple Silicon

## License

Apache 2.0
