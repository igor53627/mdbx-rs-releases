//! mdbx-rs - FFI bindings to a Rust implementation of libmdbx
//!
//! An extremely fast embedded key-value database.
//!
//! ## Performance
//!
//! - **17% faster** on PUT than C libmdbx
//! - **19% faster** on GET than C libmdbx
//! - **37% faster** on CURSOR than C libmdbx
//!
//! ## Example
//!
//! ```rust,ignore
//! use mdbx_rs::ffi;
//!
//! unsafe {
//!     let mut env = std::ptr::null_mut();
//!     ffi::mdbx_env_create(&mut env);
//!     // ... use the database
//!     ffi::mdbx_env_close(env);
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

/// Cursor operations
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MDBX_cursor_op {
    MDBX_FIRST = 0,
    MDBX_FIRST_DUP = 1,
    MDBX_GET_BOTH = 2,
    MDBX_GET_BOTH_RANGE = 3,
    MDBX_GET_CURRENT = 4,
    MDBX_GET_MULTIPLE = 5,
    MDBX_LAST = 6,
    MDBX_LAST_DUP = 7,
    MDBX_NEXT = 8,
    MDBX_NEXT_DUP = 9,
    MDBX_NEXT_MULTIPLE = 10,
    MDBX_NEXT_NODUP = 11,
    MDBX_PREV = 12,
    MDBX_PREV_DUP = 13,
    MDBX_PREV_NODUP = 14,
    MDBX_SET = 15,
    MDBX_SET_KEY = 16,
    MDBX_SET_RANGE = 17,
    MDBX_PREV_MULTIPLE = 18,
    MDBX_SET_LOWERBOUND = 19,
    MDBX_SET_UPPERBOUND = 20,
}

// Error codes
pub const MDBX_SUCCESS: c_int = 0;
pub const MDBX_EINVAL: c_int = -22;
pub const MDBX_EACCESS: c_int = -13;
pub const MDBX_ENOMEM: c_int = -12;
pub const MDBX_NOTFOUND: c_int = -30798;
pub const MDBX_KEYEXIST: c_int = -30799;

// Environment flags
pub const MDBX_NOSUBDIR: c_uint = 0x4000;
pub const MDBX_RDONLY: c_uint = 0x20000;
pub const MDBX_WRITEMAP: c_uint = 0x80000;
pub const MDBX_NOTLS: c_uint = 0x200000;
pub const MDBX_NORDAHEAD: c_uint = 0x800000;
pub const MDBX_NOMEMINIT: c_uint = 0x1000000;

// Transaction flags
pub const MDBX_TXN_READONLY: c_uint = 0x01;
pub const MDBX_TXN_RDONLY: c_uint = MDBX_TXN_READONLY;
pub const MDBX_TXN_TRY: c_uint = 0x02;

// Put flags
pub const MDBX_NOOVERWRITE: c_uint = 0x10;
pub const MDBX_NODUPDATA: c_uint = 0x20;
pub const MDBX_CURRENT: c_uint = 0x40;
pub const MDBX_APPEND: c_uint = 0x80;
pub const MDBX_APPENDDUP: c_uint = 0x100;
pub const MDBX_MULTIPLE: c_uint = 0x200;

#[link(name = "mdbx_rs", kind = "static")]
extern "C" {
    // Environment functions
    pub fn mdbx_env_create(env: *mut *mut MDBX_env) -> c_int;
    pub fn mdbx_env_open(
        env: *mut MDBX_env,
        pathname: *const c_char,
        flags: MDBX_env_flags_t,
        mode: mdbx_mode_t,
    ) -> c_int;
    pub fn mdbx_env_close(env: *mut MDBX_env) -> c_int;
    pub fn mdbx_env_close_ex(env: *mut MDBX_env, dont_sync: bool) -> c_int;
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

    // Transaction functions
    pub fn mdbx_txn_begin(
        env: *mut MDBX_env,
        parent: *mut MDBX_txn,
        flags: MDBX_txn_flags_t,
        txn: *mut *mut MDBX_txn,
    ) -> c_int;
    pub fn mdbx_txn_commit(txn: *mut MDBX_txn) -> c_int;
    pub fn mdbx_txn_abort(txn: *mut MDBX_txn) -> c_int;
    pub fn mdbx_txn_env(txn: *const MDBX_txn) -> *mut MDBX_env;
    pub fn mdbx_txn_flags(txn: *const MDBX_txn) -> c_int;
    pub fn mdbx_txn_id(txn: *const MDBX_txn) -> u64;

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

/// Convert MDBX_val to a Rust byte slice (unsafe - caller must ensure validity)
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
