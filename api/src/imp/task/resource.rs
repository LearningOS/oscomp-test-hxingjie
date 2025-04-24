use core::ffi::c_int;
use arceos_posix_api;
use axerrno::{LinuxResult, LinuxError};

pub fn sys_getrlimit(resource: c_int, rlimits: *mut arceos_posix_api::ctypes::rlimit)-> LinuxResult<isize> {
    let res = unsafe {arceos_posix_api::sys_getrlimit(resource, rlimits)}; 
    if res == 0 {
        Ok(res as isize)
    } else {
        Err(LinuxError::EINVAL)
    }
}

pub fn sys_setrlimit(resource: c_int, rlimits: *mut arceos_posix_api::ctypes::rlimit)-> LinuxResult<isize> {
    let res = unsafe {arceos_posix_api::sys_setrlimit(resource, rlimits)}; 
    if res == 0 {
        Ok(res as isize)
    } else {
        Err(LinuxError::EINVAL)
    }
}