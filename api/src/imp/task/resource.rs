use core::ffi::c_int;
use arceos_posix_api::{self, ctypes::rlimit};
use axerrno::{LinuxResult, LinuxError};

use crate::ptr::{PtrWrapper, UserPtr};

pub fn sys_getrlimit(resource: c_int, rlimits: *mut rlimit)-> LinuxResult<isize> {
    let res = unsafe {arceos_posix_api::sys_getrlimit(resource, rlimits)}; 
    if res == 0 {
        Ok(res as isize)
    } else {
        Err(LinuxError::EINVAL)
    }
}

pub fn sys_setrlimit(resource: c_int, rlimits: *mut rlimit)-> LinuxResult<isize> {
    let res = unsafe {arceos_posix_api::sys_setrlimit(resource, rlimits)}; 
    if res == 0 {
        Ok(res as isize)
    } else {
        Err(LinuxError::EINVAL)
    }
}

// *const rlimit *mut rlimit
pub fn sys_prlimit64(pid: u32, resource: c_int, 
    new_limit: usize, old_limit: usize)-> LinuxResult<isize> {
    if pid != 0 {
        // todo
        return Err(LinuxError::EINVAL);
    }

    unsafe {arceos_posix_api::sys_getrlimit(resource, old_limit as *mut rlimit)};
    unsafe {arceos_posix_api::sys_setrlimit(resource, new_limit as *mut rlimit)};

    Ok(0)
}