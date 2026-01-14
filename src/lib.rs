//! mdbx-rs - A pure Rust reimplementation of libmdbx
//!
//! An extremely fast embedded key-value database with memory safety guarantees.
//!
//! ## Benefits
//!
//! - **Memory safe** - No undefined behavior from C FFI
//! - **Binary compatible** - Reads/writes same database files as C libmdbx
//! - **Native Rust** - No external C dependencies
//!
//! ## Example
//!
//! ```rust,ignore
//! use mdbx_rs::*;
//!
//! unsafe {
//!     let mut env = std::ptr::null_mut();
//!     mdbx_env_create(&mut env);
//!     // ... use the database
//!     mdbx_env_close(env);
//! }
//! ```
//!
//! ## Binary Compatibility
//!
//! This crate is binary compatible with C libmdbx databases.
//! No migration required for existing databases.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int, c_uint, c_void};

// Re-export all constants
pub mod constants;
pub use constants::*;

/// Database handle type
pub type MDBX_dbi = u32;

/// File mode type
pub type mdbx_mode_t = u32;

/// File handle type
#[cfg(unix)]
pub type mdbx_filehandle_t = c_int;

/// Environment flags type
pub type MDBX_env_flags_t = c_uint;

/// Transaction flags type  
pub type MDBX_txn_flags_t = c_uint;

/// Opaque environment handle
#[repr(C)]
pub struct MDBX_env {
    _private: [u8; 0],
}

/// Opaque transaction handle
#[repr(C)]
pub struct MDBX_txn {
    _private: [u8; 0],
}

/// Opaque cursor handle
#[repr(C)]
pub struct MDBX_cursor {
    _private: [u8; 0],
}

/// Key-value data structure (matches C struct iovec layout)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MDBX_val {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

impl Default for MDBX_val {
    fn default() -> Self {
        Self {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }
    }
}

/// Database statistics
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_stat {
    pub ms_psize: u32,
    pub ms_depth: u32,
    pub ms_branch_pages: u64,
    pub ms_leaf_pages: u64,
    pub ms_overflow_pages: u64,
    pub ms_entries: u64,
    pub ms_mod_txnid: u64,
}

/// Environment geometry info
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_envinfo_geo {
    pub lower: u64,
    pub upper: u64,
    pub current: u64,
    pub shrink: u64,
    pub grow: u64,
}

/// Boot ID inner struct
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_envinfo_bootid_inner {
    pub x: u64,
    pub y: u64,
}

/// Boot ID struct
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_envinfo_bootid {
    pub current: MDBX_envinfo_bootid_inner,
    pub meta: [MDBX_envinfo_bootid_inner; 3],
}

/// Page operation statistics
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_envinfo_pgop_stat {
    pub newly: u64,
    pub cow: u64,
    pub clone: u64,
    pub split: u64,
    pub merge: u64,
    pub spill: u64,
    pub unspill: u64,
    pub wops: u64,
    pub prefault: u64,
    pub mincore: u64,
    pub msync: u64,
    pub fsync: u64,
}

/// Database ID struct
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_envinfo_dxbid {
    pub x: u64,
    pub y: u64,
}

/// Environment information
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_envinfo {
    pub mi_geo: MDBX_envinfo_geo,
    pub mi_mapsize: u64,
    pub mi_dxb_fsize: u64,
    pub mi_dxb_fallocated: u64,
    pub mi_last_pgno: u64,
    pub mi_recent_txnid: u64,
    pub mi_latter_reader_txnid: u64,
    pub mi_self_latter_reader_txnid: u64,
    pub mi_meta_txnid: [u64; 3],
    pub mi_meta_sign: [u64; 3],
    pub mi_maxreaders: u32,
    pub mi_numreaders: u32,
    pub mi_dxb_pagesize: u32,
    pub mi_sys_pagesize: u32,
    pub mi_sys_upcblk: u32,
    pub mi_sys_ioblk: u32,
    pub mi_bootid: MDBX_envinfo_bootid,
    pub mi_unsync_volume: u64,
    pub mi_autosync_threshold: u64,
    pub mi_since_sync_seconds16dot16: u32,
    pub mi_autosync_period_seconds16dot16: u32,
    pub mi_since_reader_check_seconds16dot16: u32,
    pub mi_mode: u32,
    pub mi_pgop_stat: MDBX_envinfo_pgop_stat,
    pub mi_dxbid: MDBX_envinfo_dxbid,
}

/// Commit latency information
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MDBX_commit_latency {
    pub preparation: u32,
    pub gc_wallclock: u32,
    pub audit: u32,
    pub write: u32,
    pub sync: u32,
    pub ending: u32,
    pub whole: u32,
    pub gc_cputime: u32,
}

#[link(name = "mdbx_rs", kind = "static")]
extern "C" {
    // Environment functions

    /// Create an MDBX environment handle.
    pub fn mdbx_env_create(env: *mut *mut MDBX_env) -> c_int;

    /// Open an environment handle.
    pub fn mdbx_env_open(
        env: *mut MDBX_env,
        pathname: *const c_char,
        flags: MDBX_env_flags_t,
        mode: mdbx_mode_t,
    ) -> c_int;

    /// Close the environment and release resources.
    pub fn mdbx_env_close(env: *mut MDBX_env) -> c_int;

    /// Close the environment with optional sync control.
    pub fn mdbx_env_close_ex(env: *mut MDBX_env, dont_sync: bool) -> c_int;

    /// Set the database size limits and geometry.
    ///
    /// Must be called **before** `mdbx_env_open()`. Use -1 for any parameter
    /// to keep the current/default value.
    ///
    /// # Parameters
    /// - `size_lower`: Minimum database size in bytes (-1 for auto)
    /// - `size_now`: Current/initial size in bytes (-1 for auto)
    /// - `size_upper`: Maximum database size in bytes (-1 for default ~1GB)
    /// - `growth_step`: Size increment when growing (-1 for auto, typically 16MB)
    /// - `shrink_threshold`: Threshold for shrinking (-1 for auto)
    /// - `pagesize`: Page size in bytes (256..65536, power of 2, typically 4096)
    ///
    /// # Returns
    /// - 0 on success
    /// - `MDBX_EINVAL` if parameters are invalid
    ///
    /// # Example
    /// ```ignore
    /// // Allow database to grow up to 100GB
    /// let max_size: isize = 100 * 1024 * 1024 * 1024;
    /// mdbx_env_set_geometry(env, -1, -1, max_size, -1, -1, 4096);
    /// ```
    pub fn mdbx_env_set_geometry(
        env: *mut MDBX_env,
        size_lower: isize,
        size_now: isize,
        size_upper: isize,
        growth_step: isize,
        shrink_threshold: isize,
        pagesize: isize,
    ) -> c_int;
    pub fn mdbx_env_set_maxdbs(env: *mut MDBX_env, dbs: MDBX_dbi) -> c_int;
    pub fn mdbx_env_set_maxreaders(env: *mut MDBX_env, readers: c_uint) -> c_int;
    pub fn mdbx_env_set_option(env: *mut MDBX_env, option: c_int, value: u64) -> c_int;
    pub fn mdbx_env_get_option(env: *const MDBX_env, option: c_int, value: *mut u64) -> c_int;
    pub fn mdbx_env_get_maxreaders(env: *const MDBX_env, readers: *mut c_uint) -> c_int;
    pub fn mdbx_env_get_maxdbs(env: *const MDBX_env, dbs: *mut MDBX_dbi) -> c_int;
    pub fn mdbx_env_sync_ex(env: *mut MDBX_env, force: bool, nonblock: bool) -> c_int;
    pub fn mdbx_env_set_flags(env: *mut MDBX_env, flags: c_uint, onoff: bool) -> c_int;
    pub fn mdbx_env_get_flags(env: *const MDBX_env, flags: *mut c_uint) -> c_int;
    pub fn mdbx_env_get_path(env: *const MDBX_env, path: *mut *mut c_char) -> c_int;
    pub fn mdbx_env_get_fd(env: *const MDBX_env, fd: *mut mdbx_filehandle_t) -> c_int;
    pub fn mdbx_env_stat_ex(
        env: *const MDBX_env,
        txn: *const MDBX_txn,
        stat: *mut MDBX_stat,
        bytes: usize,
    ) -> c_int;
    pub fn mdbx_env_info_ex(
        env: *const MDBX_env,
        txn: *const MDBX_txn,
        info: *mut MDBX_envinfo,
        bytes: usize,
    ) -> c_int;

    // Transaction functions
    pub fn mdbx_txn_begin(
        env: *mut MDBX_env,
        parent: *mut MDBX_txn,
        flags: MDBX_txn_flags_t,
        txn: *mut *mut MDBX_txn,
    ) -> c_int;
    pub fn mdbx_txn_commit(txn: *mut MDBX_txn) -> c_int;
    pub fn mdbx_txn_commit_ex(txn: *mut MDBX_txn, latency: *mut MDBX_commit_latency) -> c_int;
    pub fn mdbx_txn_abort(txn: *mut MDBX_txn) -> c_int;
    pub fn mdbx_txn_env(txn: *const MDBX_txn) -> *mut MDBX_env;
    pub fn mdbx_txn_flags(txn: *const MDBX_txn) -> c_int;
    pub fn mdbx_txn_id(txn: *const MDBX_txn) -> u64;
    pub fn mdbx_txn_reset(txn: *mut MDBX_txn) -> c_int;
    pub fn mdbx_txn_renew(txn: *mut MDBX_txn) -> c_int;

    // Database functions
    pub fn mdbx_dbi_open(
        txn: *mut MDBX_txn,
        name: *const c_char,
        flags: c_uint,
        dbi: *mut MDBX_dbi,
    ) -> c_int;
    pub fn mdbx_dbi_close(env: *mut MDBX_env, dbi: MDBX_dbi) -> c_int;
    pub fn mdbx_dbi_stat(
        txn: *const MDBX_txn,
        dbi: MDBX_dbi,
        stat: *mut MDBX_stat,
        bytes: usize,
    ) -> c_int;
    pub fn mdbx_dbi_flags_ex(
        txn: *const MDBX_txn,
        dbi: MDBX_dbi,
        flags: *mut c_uint,
        state: *mut c_uint,
    ) -> c_int;
    pub fn mdbx_drop(txn: *mut MDBX_txn, dbi: MDBX_dbi, del: bool) -> c_int;

    // Data functions
    pub fn mdbx_get(
        txn: *mut MDBX_txn,
        dbi: MDBX_dbi,
        key: *const MDBX_val,
        data: *mut MDBX_val,
    ) -> c_int;
    pub fn mdbx_put(
        txn: *mut MDBX_txn,
        dbi: MDBX_dbi,
        key: *const MDBX_val,
        data: *mut MDBX_val,
        flags: c_uint,
    ) -> c_int;
    pub fn mdbx_del(
        txn: *mut MDBX_txn,
        dbi: MDBX_dbi,
        key: *const MDBX_val,
        data: *const MDBX_val,
    ) -> c_int;

    // Cursor functions
    pub fn mdbx_cursor_open(
        txn: *mut MDBX_txn,
        dbi: MDBX_dbi,
        cursor: *mut *mut MDBX_cursor,
    ) -> c_int;
    pub fn mdbx_cursor_close(cursor: *mut MDBX_cursor);
    pub fn mdbx_cursor_create(context: *mut c_void) -> *mut MDBX_cursor;
    pub fn mdbx_cursor_renew(txn: *mut MDBX_txn, cursor: *mut MDBX_cursor) -> c_int;
    pub fn mdbx_cursor_txn(cursor: *const MDBX_cursor) -> *mut MDBX_txn;
    pub fn mdbx_cursor_dbi(cursor: *const MDBX_cursor) -> MDBX_dbi;
    pub fn mdbx_cursor_get(
        cursor: *mut MDBX_cursor,
        key: *mut MDBX_val,
        data: *mut MDBX_val,
        op: MDBX_cursor_op,
    ) -> c_int;
    pub fn mdbx_cursor_put(
        cursor: *mut MDBX_cursor,
        key: *const MDBX_val,
        data: *mut MDBX_val,
        flags: c_uint,
    ) -> c_int;
    pub fn mdbx_cursor_del(cursor: *mut MDBX_cursor, flags: c_uint) -> c_int;
    pub fn mdbx_cursor_count(cursor: *const MDBX_cursor, count: *mut usize) -> c_int;

    // Utility functions
    pub fn mdbx_cmp(
        txn: *const MDBX_txn,
        dbi: MDBX_dbi,
        a: *const MDBX_val,
        b: *const MDBX_val,
    ) -> c_int;
    pub fn mdbx_strerror(errnum: c_int) -> *const c_char;
}

/// Convert a Rust byte slice to MDBX_val
#[inline]
pub fn bytes_to_val(bytes: &[u8]) -> MDBX_val {
    MDBX_val {
        iov_base: bytes.as_ptr() as *mut c_void,
        iov_len: bytes.len(),
    }
}

/// Convert MDBX_val to a Rust byte slice.
///
/// # Safety
///
/// The caller must ensure that:
/// - `val.iov_base` points to valid memory if non-null
/// - The memory is valid for `val.iov_len` bytes
/// - The memory is not mutated during the lifetime `'a`
#[inline]
pub unsafe fn val_to_bytes<'a>(val: &MDBX_val) -> &'a [u8] {
    if val.iov_base.is_null() || val.iov_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(val.iov_base as *const u8, val.iov_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_val() {
        let data = b"hello";
        let val = bytes_to_val(data);
        assert_eq!(val.iov_len, 5);
    }
}
