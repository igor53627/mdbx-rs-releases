use libc::c_int;

pub const MDBX_SUCCESS: c_int = 0;
pub const MDBX_RESULT_FALSE: c_int = MDBX_SUCCESS;
pub const MDBX_RESULT_TRUE: c_int = -1;
pub const MDBX_KEYEXIST: c_int = -30799;
pub const MDBX_FIRST_LMDB_ERRCODE: c_int = MDBX_KEYEXIST;
pub const MDBX_NOTFOUND: c_int = -30798;
pub const MDBX_PAGE_NOTFOUND: c_int = -30797;
pub const MDBX_CORRUPTED: c_int = -30796;
pub const MDBX_PANIC: c_int = -30795;
pub const MDBX_VERSION_MISMATCH: c_int = -30794;
pub const MDBX_INVALID: c_int = -30793;
pub const MDBX_MAP_FULL: c_int = -30792;
pub const MDBX_DBS_FULL: c_int = -30791;
pub const MDBX_READERS_FULL: c_int = -30790;
pub const MDBX_TXN_FULL: c_int = -30788;
pub const MDBX_CURSOR_FULL: c_int = -30787;
pub const MDBX_PAGE_FULL: c_int = -30786;
pub const MDBX_UNABLE_EXTEND_MAPSIZE: c_int = -30785;
pub const MDBX_INCOMPATIBLE: c_int = -30784;
pub const MDBX_BAD_RSLOT: c_int = -30783;
pub const MDBX_BAD_TXN: c_int = -30782;
pub const MDBX_BAD_VALSIZE: c_int = -30781;
pub const MDBX_BAD_DBI: c_int = -30780;
pub const MDBX_PROBLEM: c_int = -30779;
pub const MDBX_LAST_LMDB_ERRCODE: c_int = MDBX_PROBLEM;
pub const MDBX_BUSY: c_int = -30778;
pub const MDBX_FIRST_ADDED_ERRCODE: c_int = MDBX_BUSY;
pub const MDBX_EMULTIVAL: c_int = -30421;
pub const MDBX_EBADSIGN: c_int = -30420;
pub const MDBX_WANNA_RECOVERY: c_int = -30419;
pub const MDBX_EKEYMISMATCH: c_int = -30418;
pub const MDBX_TOO_LARGE: c_int = -30417;
pub const MDBX_THREAD_MISMATCH: c_int = -30416;
pub const MDBX_TXN_OVERLAPPING: c_int = -30415;
pub const MDBX_BACKLOG_DEPLETED: c_int = -30414;
pub const MDBX_DUPLICATED_CLK: c_int = -30413;
pub const MDBX_DANGLING_DBI: c_int = -30412;
pub const MDBX_OUSTED: c_int = -30411;
pub const MDBX_MVCC_RETARDED: c_int = -30410;
pub const MDBX_LAST_ADDED_ERRCODE: c_int = MDBX_MVCC_RETARDED;

#[cfg(unix)]
pub const MDBX_ENODATA: c_int = libc::ENODATA;
#[cfg(not(unix))]
pub const MDBX_ENODATA: c_int = 38; // ERROR_HANDLE_EOF on Windows

#[cfg(unix)]
pub const MDBX_EINVAL: c_int = libc::EINVAL;
#[cfg(not(unix))]
pub const MDBX_EINVAL: c_int = 22;

#[cfg(unix)]
pub const MDBX_EACCESS: c_int = libc::EACCES;
#[cfg(not(unix))]
pub const MDBX_EACCESS: c_int = 13;

#[cfg(unix)]
pub const MDBX_ENOMEM: c_int = libc::ENOMEM;
#[cfg(not(unix))]
pub const MDBX_ENOMEM: c_int = 12;

#[cfg(unix)]
pub const MDBX_EROFS: c_int = libc::EROFS;
#[cfg(not(unix))]
pub const MDBX_EROFS: c_int = 30;

#[cfg(unix)]
/// Note: Uses ENOTSUP to match MDBX C library behavior
pub const MDBX_ENOSYS: c_int = libc::ENOTSUP;
#[cfg(not(unix))]
pub const MDBX_ENOSYS: c_int = 95;

#[cfg(unix)]
pub const MDBX_EIO: c_int = libc::EIO;
#[cfg(not(unix))]
pub const MDBX_EIO: c_int = 5;

#[cfg(unix)]
pub const MDBX_EPERM: c_int = libc::EPERM;
#[cfg(not(unix))]
pub const MDBX_EPERM: c_int = 1;

#[cfg(unix)]
pub const MDBX_EINTR: c_int = libc::EINTR;
#[cfg(not(unix))]
pub const MDBX_EINTR: c_int = 4;

#[cfg(unix)]
pub const MDBX_ENOFILE: c_int = libc::ENOENT;
#[cfg(not(unix))]
pub const MDBX_ENOFILE: c_int = 2;

#[cfg(unix)]
pub const MDBX_EDEADLK: c_int = libc::EDEADLK;
#[cfg(not(unix))]
pub const MDBX_EDEADLK: c_int = 35;

pub const MDBX_ENV_DEFAULTS: c_int = 0;
pub const MDBX_VALIDATION: c_int = 0x00002000;
pub const MDBX_NOSUBDIR: c_int = 0x4000;
pub const MDBX_RDONLY: c_int = 0x20000;
pub const MDBX_EXCLUSIVE: c_int = 0x400000;
pub const MDBX_ACCEDE: c_int = 0x40000000;
pub const MDBX_WRITEMAP: c_int = 0x80000;
pub const MDBX_NOSTICKYTHREADS: c_int = 0x200000;
pub const MDBX_NOTLS: c_int = MDBX_NOSTICKYTHREADS;
pub const MDBX_NORDAHEAD: c_int = 0x800000;
pub const MDBX_NOMEMINIT: c_int = 0x1000000;
pub const MDBX_COALESCE: c_int = 0x2000000;
pub const MDBX_LIFORECLAIM: c_int = 0x4000000;
pub const MDBX_PAGEPERTURB: c_int = 0x8000000;

pub const MDBX_SYNC_DURABLE: c_int = 0;
pub const MDBX_NOMETASYNC: c_int = 0x40000;
pub const MDBX_SAFE_NOSYNC: c_int = 0x10000;
pub const MDBX_MAPASYNC: c_int = MDBX_SAFE_NOSYNC;
pub const MDBX_UTTERLY_NOSYNC: c_int = MDBX_SAFE_NOSYNC | MDBX_NOMETASYNC;

pub const MDBX_TXN_READWRITE: c_int = 0;
pub const MDBX_TXN_RDONLY: c_int = MDBX_RDONLY;
pub const MDBX_TXN_RDONLY_PREPARE: c_int = MDBX_RDONLY | MDBX_NOMEMINIT;
pub const MDBX_TXN_TRY: c_int = 0x10000000;
pub const MDBX_TXN_NOMETASYNC: c_int = MDBX_NOMETASYNC;
pub const MDBX_TXN_NOSYNC: c_int = MDBX_SAFE_NOSYNC;

pub const MDBX_TXN_INVALID: c_int = i32::MIN;
pub const MDBX_TXN_FINISHED: c_int = 0x01;
pub const MDBX_TXN_ERROR: c_int = 0x02;
pub const MDBX_TXN_DIRTY: c_int = 0x04;
pub const MDBX_TXN_SPILLS: c_int = 0x08;
pub const MDBX_TXN_HAS_CHILD: c_int = 0x10;
pub const MDBX_TXN_PARKED: c_int = 0x20;
pub const MDBX_TXN_AUTOUNPARK: c_int = 0x40;
pub const MDBX_TXN_OUSTED: c_int = 0x80;
pub const MDBX_TXN_BLOCKED: c_int =
    MDBX_TXN_FINISHED | MDBX_TXN_ERROR | MDBX_TXN_HAS_CHILD | MDBX_TXN_PARKED;

pub const MDBX_DB_DEFAULTS: c_int = 0;
pub const MDBX_REVERSEKEY: c_int = 0x02;
pub const MDBX_DUPSORT: c_int = 0x04;
pub const MDBX_INTEGERKEY: c_int = 0x08;
pub const MDBX_DUPFIXED: c_int = 0x10;
pub const MDBX_INTEGERDUP: c_int = 0x20;
pub const MDBX_REVERSEDUP: c_int = 0x40;
pub const MDBX_CREATE: c_int = 0x40000;
pub const MDBX_DB_ACCEDE: c_int = MDBX_ACCEDE;

pub const MDBX_UPSERT: c_int = 0;
pub const MDBX_NOOVERWRITE: c_int = 0x10;
pub const MDBX_NODUPDATA: c_int = 0x20;
pub const MDBX_CURRENT: c_int = 0x40;
pub const MDBX_ALLDUPS: c_int = 0x80;
pub const MDBX_RESERVE: c_int = 0x10000;
pub const MDBX_APPEND: c_int = 0x20000;
pub const MDBX_APPENDDUP: c_int = 0x40000;
pub const MDBX_MULTIPLE: c_int = 0x80000;

pub const MDBX_CP_DEFAULTS: c_int = 0;
pub const MDBX_CP_COMPACT: c_int = 1;
pub const MDBX_CP_FORCE_DYNAMIC_SIZE: c_int = 2;
pub const MDBX_CP_DONT_FLUSH: c_int = 4;
pub const MDBX_CP_THROTTLE_MVCC: c_int = 8;
pub const MDBX_CP_DISPOSE_TXN: c_int = 16;
pub const MDBX_CP_RENEW_TXN: c_int = 32;
pub const MDBX_CP_OVERWRITE: c_int = 64;

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
    MDBX_TO_KEY_LESSER_THAN = 21,
    MDBX_TO_KEY_LESSER_OR_EQUAL = 22,
    MDBX_TO_KEY_EQUAL = 23,
    MDBX_TO_KEY_GREATER_OR_EQUAL = 24,
    MDBX_TO_KEY_GREATER_THAN = 25,
    MDBX_TO_EXACT_KEY_VALUE_LESSER_THAN = 26,
    MDBX_TO_EXACT_KEY_VALUE_LESSER_OR_EQUAL = 27,
    MDBX_TO_EXACT_KEY_VALUE_EQUAL = 28,
    MDBX_TO_EXACT_KEY_VALUE_GREATER_OR_EQUAL = 29,
    MDBX_TO_EXACT_KEY_VALUE_GREATER_THAN = 30,
    MDBX_TO_PAIR_LESSER_THAN = 31,
    MDBX_TO_PAIR_LESSER_OR_EQUAL = 32,
    MDBX_TO_PAIR_EQUAL = 33,
    MDBX_TO_PAIR_GREATER_OR_EQUAL = 34,
    MDBX_TO_PAIR_GREATER_THAN = 35,
    MDBX_SEEK_AND_GET_MULTIPLE = 36,
}

pub const MDBX_LOG_FATAL: c_int = 0;
pub const MDBX_LOG_ERROR: c_int = 1;
pub const MDBX_LOG_WARN: c_int = 2;
pub const MDBX_LOG_NOTICE: c_int = 3;
pub const MDBX_LOG_VERBOSE: c_int = 4;
pub const MDBX_LOG_DEBUG: c_int = 5;
pub const MDBX_LOG_TRACE: c_int = 6;
pub const MDBX_LOG_EXTRA: c_int = 7;
pub const MDBX_LOG_DONTCHANGE: c_int = -1;

pub const MDBX_DBG_NONE: c_int = 0;
pub const MDBX_DBG_ASSERT: c_int = 1;
pub const MDBX_DBG_AUDIT: c_int = 2;
pub const MDBX_DBG_JITTER: c_int = 4;
pub const MDBX_DBG_DUMP: c_int = 8;
pub const MDBX_DBG_LEGACY_MULTIOPEN: c_int = 16;
pub const MDBX_DBG_LEGACY_OVERLAP: c_int = 32;
pub const MDBX_DBG_DONT_UPGRADE: c_int = 64;
pub const MDBX_DBG_DONTCHANGE: c_int = -1;

pub const MDBX_DBI_DIRTY: c_int = 0x01;
pub const MDBX_DBI_STALE: c_int = 0x02;
pub const MDBX_DBI_FRESH: c_int = 0x04;
pub const MDBX_DBI_CREAT: c_int = 0x08;

pub const MDBX_MAX_DBI: u32 = 32765;
pub const MDBX_MAXDATASIZE: u32 = 0x7fff0000;
pub const MDBX_MIN_PAGESIZE: u32 = 256;
pub const MDBX_MAX_PAGESIZE: u32 = 65536;

pub const MDBX_ENV_JUST_DELETE: c_int = 0;
pub const MDBX_ENV_ENSURE_UNUSED: c_int = 1;
pub const MDBX_ENV_WAIT_FOR_UNUSED: c_int = 2;

pub const MDBX_WARMUP_DEFAULT: c_int = 0;
pub const MDBX_WARMUP_FORCE: c_int = 1;
pub const MDBX_WARMUP_OOMSAFE: c_int = 2;
pub const MDBX_WARMUP_LOCK: c_int = 4;
pub const MDBX_WARMUP_TOUCHLIMIT: c_int = 8;
pub const MDBX_WARMUP_RELEASE: c_int = 16;

pub const MDBX_opt_max_db: c_int = 0;
pub const MDBX_opt_max_readers: c_int = 1;
pub const MDBX_opt_sync_bytes: c_int = 2;
pub const MDBX_opt_sync_period: c_int = 3;
pub const MDBX_opt_rp_augment_limit: c_int = 4;
pub const MDBX_opt_loose_limit: c_int = 5;
pub const MDBX_opt_dp_reserve_limit: c_int = 6;
pub const MDBX_opt_txn_dp_limit: c_int = 7;
pub const MDBX_opt_txn_dp_initial: c_int = 8;
pub const MDBX_opt_spill_max_denominator: c_int = 9;
pub const MDBX_opt_spill_min_denominator: c_int = 10;
pub const MDBX_opt_spill_parent4child_denominator: c_int = 11;
pub const MDBX_opt_merge_threshold_16dot16_percent: c_int = 12;
pub const MDBX_opt_writethrough_threshold: c_int = 13;
pub const MDBX_opt_prefault_write_enable: c_int = 14;
pub const MDBX_opt_gc_time_limit: c_int = 15;
pub const MDBX_opt_prefer_waf_insteadof_balance: c_int = 16;
pub const MDBX_opt_subpage_limit: c_int = 17;
pub const MDBX_opt_subpage_room_threshold: c_int = 18;
pub const MDBX_opt_subpage_reserve_prereq: c_int = 19;
pub const MDBX_opt_subpage_reserve_limit: c_int = 20;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_values() {
        assert_eq!(MDBX_SUCCESS, 0);
        assert_eq!(MDBX_RESULT_FALSE, 0);
        assert_eq!(MDBX_RESULT_TRUE, -1);
        assert_eq!(MDBX_KEYEXIST, -30799);
        assert_eq!(MDBX_NOTFOUND, -30798);
        assert_eq!(MDBX_PAGE_NOTFOUND, -30797);
        assert_eq!(MDBX_CORRUPTED, -30796);
        assert_eq!(MDBX_PANIC, -30795);
        assert_eq!(MDBX_VERSION_MISMATCH, -30794);
        assert_eq!(MDBX_INVALID, -30793);
        assert_eq!(MDBX_MAP_FULL, -30792);
        assert_eq!(MDBX_DBS_FULL, -30791);
        assert_eq!(MDBX_READERS_FULL, -30790);
        assert_eq!(MDBX_TXN_FULL, -30788);
        assert_eq!(MDBX_CURSOR_FULL, -30787);
        assert_eq!(MDBX_PAGE_FULL, -30786);
        assert_eq!(MDBX_UNABLE_EXTEND_MAPSIZE, -30785);
        assert_eq!(MDBX_INCOMPATIBLE, -30784);
        assert_eq!(MDBX_BAD_RSLOT, -30783);
        assert_eq!(MDBX_BAD_TXN, -30782);
        assert_eq!(MDBX_BAD_VALSIZE, -30781);
        assert_eq!(MDBX_BAD_DBI, -30780);
        assert_eq!(MDBX_PROBLEM, -30779);
    }

    #[test]
    fn test_added_error_codes() {
        assert_eq!(MDBX_BUSY, -30778);
        assert_eq!(MDBX_EMULTIVAL, -30421);
        assert_eq!(MDBX_EBADSIGN, -30420);
        assert_eq!(MDBX_WANNA_RECOVERY, -30419);
        assert_eq!(MDBX_EKEYMISMATCH, -30418);
        assert_eq!(MDBX_TOO_LARGE, -30417);
        assert_eq!(MDBX_THREAD_MISMATCH, -30416);
        assert_eq!(MDBX_TXN_OVERLAPPING, -30415);
        assert_eq!(MDBX_BACKLOG_DEPLETED, -30414);
        assert_eq!(MDBX_DUPLICATED_CLK, -30413);
        assert_eq!(MDBX_DANGLING_DBI, -30412);
        assert_eq!(MDBX_OUSTED, -30411);
        assert_eq!(MDBX_MVCC_RETARDED, -30410);
    }

    #[test]
    fn test_errcode_bounds() {
        assert_eq!(MDBX_FIRST_LMDB_ERRCODE, MDBX_KEYEXIST);
        assert_eq!(MDBX_LAST_LMDB_ERRCODE, MDBX_PROBLEM);
        assert_eq!(MDBX_FIRST_ADDED_ERRCODE, MDBX_BUSY);
        assert_eq!(MDBX_LAST_ADDED_ERRCODE, MDBX_MVCC_RETARDED);
    }

    #[test]
    fn test_env_flags() {
        assert_eq!(MDBX_ENV_DEFAULTS, 0);
        assert_eq!(MDBX_NOSUBDIR, 0x4000);
        assert_eq!(MDBX_RDONLY, 0x20000);
        assert_eq!(MDBX_EXCLUSIVE, 0x400000);
        assert_eq!(MDBX_WRITEMAP, 0x80000);
        assert_eq!(MDBX_NOSTICKYTHREADS, 0x200000);
        assert_eq!(MDBX_NOTLS, MDBX_NOSTICKYTHREADS);
        assert_eq!(MDBX_NORDAHEAD, 0x800000);
        assert_eq!(MDBX_NOMEMINIT, 0x1000000);
        assert_eq!(MDBX_LIFORECLAIM, 0x4000000);
    }

    #[test]
    fn test_sync_flags() {
        assert_eq!(MDBX_SYNC_DURABLE, 0);
        assert_eq!(MDBX_NOMETASYNC, 0x40000);
        assert_eq!(MDBX_SAFE_NOSYNC, 0x10000);
        assert_eq!(MDBX_MAPASYNC, MDBX_SAFE_NOSYNC);
        assert_eq!(MDBX_UTTERLY_NOSYNC, MDBX_SAFE_NOSYNC | MDBX_NOMETASYNC);
    }

    #[test]
    fn test_txn_flags() {
        assert_eq!(MDBX_TXN_READWRITE, 0);
        assert_eq!(MDBX_TXN_RDONLY, MDBX_RDONLY);
        assert_eq!(MDBX_TXN_TRY, 0x10000000);
        assert_eq!(MDBX_TXN_INVALID, i32::MIN);
        assert_eq!(MDBX_TXN_FINISHED, 0x01);
        assert_eq!(MDBX_TXN_ERROR, 0x02);
        assert_eq!(MDBX_TXN_DIRTY, 0x04);
        assert_eq!(MDBX_TXN_SPILLS, 0x08);
        assert_eq!(MDBX_TXN_HAS_CHILD, 0x10);
        assert_eq!(MDBX_TXN_PARKED, 0x20);
        assert_eq!(
            MDBX_TXN_BLOCKED,
            MDBX_TXN_FINISHED | MDBX_TXN_ERROR | MDBX_TXN_HAS_CHILD | MDBX_TXN_PARKED
        );
    }

    #[test]
    fn test_db_flags() {
        assert_eq!(MDBX_DB_DEFAULTS, 0);
        assert_eq!(MDBX_REVERSEKEY, 0x02);
        assert_eq!(MDBX_DUPSORT, 0x04);
        assert_eq!(MDBX_INTEGERKEY, 0x08);
        assert_eq!(MDBX_DUPFIXED, 0x10);
        assert_eq!(MDBX_INTEGERDUP, 0x20);
        assert_eq!(MDBX_REVERSEDUP, 0x40);
        assert_eq!(MDBX_CREATE, 0x40000);
    }

    #[test]
    fn test_put_flags() {
        assert_eq!(MDBX_UPSERT, 0);
        assert_eq!(MDBX_NOOVERWRITE, 0x10);
        assert_eq!(MDBX_NODUPDATA, 0x20);
        assert_eq!(MDBX_CURRENT, 0x40);
        assert_eq!(MDBX_ALLDUPS, 0x80);
        assert_eq!(MDBX_RESERVE, 0x10000);
        assert_eq!(MDBX_APPEND, 0x20000);
        assert_eq!(MDBX_APPENDDUP, 0x40000);
        assert_eq!(MDBX_MULTIPLE, 0x80000);
    }

    #[test]
    fn test_copy_flags() {
        assert_eq!(MDBX_CP_DEFAULTS, 0);
        assert_eq!(MDBX_CP_COMPACT, 1);
        assert_eq!(MDBX_CP_FORCE_DYNAMIC_SIZE, 2);
        assert_eq!(MDBX_CP_DONT_FLUSH, 4);
        assert_eq!(MDBX_CP_THROTTLE_MVCC, 8);
        assert_eq!(MDBX_CP_DISPOSE_TXN, 16);
        assert_eq!(MDBX_CP_RENEW_TXN, 32);
        assert_eq!(MDBX_CP_OVERWRITE, 64);
    }

    #[test]
    fn test_cursor_op_values() {
        assert_eq!(MDBX_cursor_op::MDBX_FIRST as i32, 0);
        assert_eq!(MDBX_cursor_op::MDBX_LAST as i32, 6);
        assert_eq!(MDBX_cursor_op::MDBX_NEXT as i32, 8);
        assert_eq!(MDBX_cursor_op::MDBX_PREV as i32, 12);
        assert_eq!(MDBX_cursor_op::MDBX_SET as i32, 15);
        assert_eq!(MDBX_cursor_op::MDBX_SET_RANGE as i32, 17);
        assert_eq!(MDBX_cursor_op::MDBX_SEEK_AND_GET_MULTIPLE as i32, 36);
    }

    #[test]
    fn test_log_levels() {
        assert_eq!(MDBX_LOG_FATAL, 0);
        assert_eq!(MDBX_LOG_ERROR, 1);
        assert_eq!(MDBX_LOG_WARN, 2);
        assert_eq!(MDBX_LOG_NOTICE, 3);
        assert_eq!(MDBX_LOG_VERBOSE, 4);
        assert_eq!(MDBX_LOG_DEBUG, 5);
        assert_eq!(MDBX_LOG_TRACE, 6);
        assert_eq!(MDBX_LOG_EXTRA, 7);
        assert_eq!(MDBX_LOG_DONTCHANGE, -1);
    }

    #[test]
    fn test_debug_flags() {
        assert_eq!(MDBX_DBG_NONE, 0);
        assert_eq!(MDBX_DBG_ASSERT, 1);
        assert_eq!(MDBX_DBG_AUDIT, 2);
        assert_eq!(MDBX_DBG_JITTER, 4);
        assert_eq!(MDBX_DBG_DUMP, 8);
        assert_eq!(MDBX_DBG_LEGACY_MULTIOPEN, 16);
        assert_eq!(MDBX_DBG_LEGACY_OVERLAP, 32);
        assert_eq!(MDBX_DBG_DONT_UPGRADE, 64);
        assert_eq!(MDBX_DBG_DONTCHANGE, -1);
    }

    #[test]
    fn test_dbi_state_flags() {
        assert_eq!(MDBX_DBI_DIRTY, 0x01);
        assert_eq!(MDBX_DBI_STALE, 0x02);
        assert_eq!(MDBX_DBI_FRESH, 0x04);
        assert_eq!(MDBX_DBI_CREAT, 0x08);
    }

    #[test]
    fn test_limits() {
        assert_eq!(MDBX_MAX_DBI, 32765);
        assert_eq!(MDBX_MAXDATASIZE, 0x7fff0000);
        assert_eq!(MDBX_MIN_PAGESIZE, 256);
        assert_eq!(MDBX_MAX_PAGESIZE, 65536);
    }

    #[test]
    fn test_env_delete_flags() {
        assert_eq!(MDBX_ENV_JUST_DELETE, 0);
        assert_eq!(MDBX_ENV_ENSURE_UNUSED, 1);
        assert_eq!(MDBX_ENV_WAIT_FOR_UNUSED, 2);
    }

    #[test]
    fn test_warmup_flags() {
        assert_eq!(MDBX_WARMUP_DEFAULT, 0);
        assert_eq!(MDBX_WARMUP_FORCE, 1);
        assert_eq!(MDBX_WARMUP_OOMSAFE, 2);
        assert_eq!(MDBX_WARMUP_LOCK, 4);
        assert_eq!(MDBX_WARMUP_TOUCHLIMIT, 8);
        assert_eq!(MDBX_WARMUP_RELEASE, 16);
    }

    #[test]
    fn test_option_constants() {
        assert_eq!(MDBX_opt_max_db, 0);
        assert_eq!(MDBX_opt_max_readers, 1);
        assert_eq!(MDBX_opt_sync_bytes, 2);
        assert_eq!(MDBX_opt_gc_time_limit, 15);
        assert_eq!(MDBX_opt_subpage_reserve_limit, 20);
    }

    #[test]
    fn test_cursor_op_debug_clone() {
        let op = MDBX_cursor_op::MDBX_FIRST;
        let cloned = op;
        assert_eq!(op, cloned);
        assert!(format!("{:?}", op).contains("FIRST"));
    }
}
