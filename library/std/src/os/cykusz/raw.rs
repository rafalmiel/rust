//! Cykusz-specific raw type definitions

#![stable(feature = "raw_ext", since = "1.1.0")]
#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
            the standard library, the `libc` crate on \
            crates.io should be used instead for the correct \
            definitions"
)]
#![allow(deprecated)]

#[stable(feature = "pthread_t", since = "1.8.0")]
pub type pthread_t = usize; // TODO: This is completely wrong tbh

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type dev_t = libc::dev_t;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type ino_t = libc::ino_t;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type mode_t = libc::mode_t;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type nlink_t = libc::nlink_t;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type off_t = libc::off_t;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type time_t = libc::time_t;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blkcnt_t = libc::blkcnt_t;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blksize_t = libc::blksize_t;

#[repr(C)]
#[derive(Clone)]
#[stable(feature = "raw_ext", since = "1.1.0")]
pub struct stat {
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_dev: libc::dev_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ino: libc::ino_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mode: libc::mode_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_nlink: libc::nlink_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_uid: libc::uid_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_gid: libc::gid_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_rdev: libc::dev_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_size: libc::off_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_atime: libc::time_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_atime_nsec: libc::c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mtime: libc::time_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mtime_nsec: libc::c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ctime: libc::time_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ctime_nsec: libc::c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_blksize: libc::blksize_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_blocks: libc::blkcnt_t,
}
