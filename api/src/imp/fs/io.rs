use core::ffi::{c_char, c_void, c_int};

use arceos_posix_api::{self as api, ctypes::mode_t};
use axerrno::LinuxResult;

use crate::ptr::{PtrWrapper, UserConstPtr, UserPtr};

pub fn sys_read(fd: i32, buf: UserPtr<c_void>, count: usize) -> LinuxResult<isize> {
    let buf = buf.get_as_bytes(count)?;
    Ok(api::sys_read(fd, buf, count))
}

pub fn sys_write(fd: i32, buf: UserConstPtr<c_void>, count: usize) -> LinuxResult<isize> {
    let buf = buf.get_as_bytes(count)?;
    Ok(api::sys_write(fd, buf, count))
}

pub fn sys_writev(
    fd: i32,
    iov: UserConstPtr<api::ctypes::iovec>,
    iocnt: i32,
) -> LinuxResult<isize> {
    let iov = iov.get_as_bytes(iocnt as _)?;
    unsafe { Ok(api::sys_writev(fd, iov, iocnt)) }
}

pub fn sys_openat(
    dirfd: i32,
    path: UserConstPtr<c_char>,
    flags: i32,
    modes: mode_t,
) -> LinuxResult<isize> {
    let path = path.get_as_null_terminated()?;
    Ok(api::sys_openat(dirfd, path.as_ptr(), flags, modes) as _)
}

pub fn sys_open(path: UserConstPtr<c_char>, flags: i32, modes: mode_t) -> LinuxResult<isize> {
    use arceos_posix_api::AT_FDCWD;
    sys_openat(AT_FDCWD as _, path, flags, modes)
}

// 成功：返回新的文件偏移量（从文件头开始的字节数）。
// 失败：返回 (off_t)-1，并设置 errno（如 EBADF 无效文件描述符）。
pub fn sys_lseek(fd: c_int, offset: api::ctypes::off_t, whence: c_int) -> LinuxResult<isize> {
    let off: api::ctypes::off_t = api::sys_lseek(fd, offset, whence);
    Ok(off as isize)
}
