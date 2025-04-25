use core::ffi::c_void;

use axerrno::{LinuxResult, LinuxError};
use axtask::{TaskExtRef, current};

use crate::ptr::{UserConstPtr, UserPtr};

pub fn sys_rt_sigprocmask(
    _how: i32,
    _set: usize,
    _oldset: usize,
    _sigsetsize: usize,
) -> LinuxResult<isize> {
    let set: Option<*const usize> = if _set == 0 {
        None
    } else {
        Some(_set as *const usize)
    };
    let mut oldset: Option<*mut usize> = if _oldset == 0 {
        None
    } else {
        Some(_oldset as *mut usize)
    };

    let task = current();
    if let Some(set) = set {
        match _how {
            0 => { // BLOCK
                let maskset = task.task_ext().process_data().read_maskset();
                if let Some(oldset) = oldset {
                    unsafe { *oldset = maskset; }
                }
                unsafe { task.task_ext().process_data().update_maskset(maskset | *set); }
                let maskset = task.task_ext().process_data().read_maskset();
            },
            1 => { // UNBLOCK
                let maskset = task.task_ext().process_data().read_maskset();
                if let Some(oldset) = oldset {
                    unsafe { *oldset = maskset; }
                }
                unsafe { task.task_ext().process_data().update_maskset(maskset & !(*set)); }
                let maskset = task.task_ext().process_data().read_maskset();
            },
            2 => { // SIG_SETMASK
                let maskset = task.task_ext().process_data().read_maskset();
                if let Some(oldset) = oldset {
                    unsafe { *oldset = maskset; }
                }
                unsafe { task.task_ext().process_data().update_maskset(*set); }
                let maskset = task.task_ext().process_data().read_maskset();
            },
            _ => { return Err(LinuxError::EINVAL) },
        }
    } else {
        if let Some(oldset) = oldset {
            unsafe { *oldset = task.task_ext().process_data().read_maskset(); }
        }
        return Ok(0);
    }

    //warn!("sys_rt_sigprocmask: not implemented");
    Ok(0)
}

pub fn sys_rt_sigaction(
    _signum: i32,
    _act: UserConstPtr<c_void>,
    _oldact: UserPtr<c_void>,
    _sigsetsize: usize,
) -> LinuxResult<isize> {
    warn!("sys_rt_sigaction: not implemented");
    Ok(0)
}

// my code
pub fn sys_rt_sigtimedwait(
    _signum: i32,
    _act: UserConstPtr<c_void>,
    _oldact: UserPtr<c_void>,
    _sigsetsize: usize,
) -> LinuxResult<isize> {
    warn!("sys_rt_sigtimedwait: not implemented");
    Ok(0)
}
