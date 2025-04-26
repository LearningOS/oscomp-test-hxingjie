use axerrno::LinuxResult;
use axtask::{TaskExtRef, current};
use macro_rules_attribute::apply;
use num_enum::TryFromPrimitive;

use crate::{ptr::PtrWrapper, syscall_instrument};

#[apply(syscall_instrument)]
pub fn sys_getpid() -> LinuxResult<isize> {
    Ok(axtask::current().task_ext().thread.process().pid() as _)
}

#[apply(syscall_instrument)]
pub fn sys_getppid() -> LinuxResult<isize> {
    Ok(axtask::current()
        .task_ext()
        .thread
        .process()
        .parent()
        .unwrap()
        .pid() as _)
}

#[apply(syscall_instrument)]
pub fn sys_gettid() -> LinuxResult<isize> {
    Ok(axtask::current().id().as_u64() as _)
}

/// ARCH_PRCTL codes
///
/// It is only avaliable on x86_64, and is not convenient
/// to generate automatically via c_to_rust binding.
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
enum ArchPrctlCode {
    /// Set the GS segment base
    SetGs = 0x1001,
    /// Set the FS segment base
    SetFs = 0x1002,
    /// Get the FS segment base
    GetFs = 0x1003,
    /// Get the GS segment base
    GetGs = 0x1004,
    /// The setting of the flag manipulated by ARCH_SET_CPUID
    GetCpuid = 0x1011,
    /// Enable (addr != 0) or disable (addr == 0) the cpuid instruction for the calling thread.
    SetCpuid = 0x1012,
}

/// To set the clear_child_tid field in the task extended data.
///
/// The set_tid_address() always succeeds
#[apply(syscall_instrument)]
pub fn sys_set_tid_address(clear_child_tid: usize) -> LinuxResult<isize> {
    let curr = current();
    curr.task_ext()
        .thread_data()
        .set_clear_child_tid(clear_child_tid);
    Ok(curr.id().as_u64() as isize)
}

#[cfg(target_arch = "x86_64")]
#[apply(syscall_instrument)]
pub fn sys_arch_prctl(code: i32, addr: crate::ptr::UserPtr<u64>) -> LinuxResult<isize> {
    use crate::ptr::PtrWrapper;
    match ArchPrctlCode::try_from(code).map_err(|_| axerrno::LinuxError::EINVAL)? {
        // According to Linux implementation, SetFs & SetGs does not return
        // error at all
        ArchPrctlCode::SetFs => {
            unsafe {
                axhal::arch::write_thread_pointer(addr.address().as_usize());
            }
            Ok(0)
        }
        ArchPrctlCode::SetGs => {
            unsafe {
                x86::msr::wrmsr(x86::msr::IA32_KERNEL_GSBASE, addr.address().as_usize() as _);
            }
            Ok(0)
        }
        ArchPrctlCode::GetFs => {
            unsafe {
                *addr.get()? = axhal::arch::read_thread_pointer() as u64;
            }
            Ok(0)
        }

        ArchPrctlCode::GetGs => {
            unsafe {
                *addr.get()? = x86::msr::rdmsr(x86::msr::IA32_KERNEL_GSBASE);
            }
            Ok(0)
        }
        ArchPrctlCode::GetCpuid => Ok(0),
        ArchPrctlCode::SetCpuid => Err(axerrno::LinuxError::ENODEV),
    }
}

// FUTEX_WAIT=0 和 FUTEX_WAKE=1
pub fn sys_futex(uaddr: crate::ptr::UserPtr<u32>, futex_op: i32, val: u32, 
    timeout: crate::ptr::UserConstPtr<arceos_posix_api::ctypes::timespec>) -> LinuxResult<isize> {
    let mut op = futex_op;
    if op & (1 << 7) != 0 {
        op &= !(1 << 7);
        if op == 0 {
            let uaddr = uaddr.get_as_bytes(4).unwrap();
            unsafe {
                if *uaddr != val {
                    ax_println!("sys_futex todo");
                    panic!()
                } else {
                    ax_println!("sys_futex sleep");
                    arceos_posix_api::sys_nanosleep(timeout.get()?, core::ptr::null_mut());
                    ax_println!("sys_futex wake");
                    return Ok(-1)
                }
            }
        } else {
            ax_println!("sys_futex todo");
            panic!()
        }
    } else {
        ax_println!("sys_futex todo");
        panic!()    
    }
    Ok(0)
}
