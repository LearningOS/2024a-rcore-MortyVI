//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE},
    task::{
        change_program_brk, exit_current_and_run_next, 
        suspend_current_and_run_next, current_user_token, 
        get_start_time, get_syscall_times, mmap, munmap,
        TaskStatus,
    },
    mm::{KERNEL_SPACE, PageTable, VirtAddr, VirtPageNum},
    timer::{get_time_us, get_time_ms}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let size = core::mem::size_of::<TimeVal>();
    let tv = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let bytes = unsafe {core::slice::from_raw_parts(&tv as *const TimeVal as *const u8, size)};
    copy_bytes_to_use_space(bytes, _ts as usize);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    let size = core::mem::size_of::<TaskInfo>();
    let task_info = TaskInfo {
        status: TaskStatus::Running,
        time: get_time_ms() - get_start_time(),
        syscall_times: get_syscall_times()
    };
    let bytes = unsafe {core::slice::from_raw_parts(&task_info as *const TaskInfo as *const u8, size)};
    copy_bytes_to_use_space(bytes, _ti as usize);
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    // check port
    if _port == 0 || _port >= 8 {
        return -1;
    }
    // check start
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    if _len == 0 {
        return 0;
    }
    let result = mmap(_start, _len, _port);
    if result {
        return 0;
    } else {
        return -1;
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    let result = munmap(_start, _len);
    if result {
        return 0;
    } else {
        return -1;
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

fn copy_bytes_to_use_space(bytes: &[u8], va: usize) {
    let mut va = va;
    let token = current_user_token();
    for byte in bytes {
        let vpn = VirtPageNum::from(VirtAddr::from(va));
        let pte = PageTable::from_token(token).translate(vpn).unwrap();
        let ppn = pte.ppn();
        let pa = (ppn.0 << 12) | (va & 0xfff);
        KERNEL_SPACE.exclusive_access().page_table.identical_map(pa);
        unsafe {
            *(pa as *mut u8) = byte.clone();
        }
        va += 1;
    }
    KERNEL_SPACE.exclusive_access().page_table.identical_unmap(va);
    if va / PAGE_SIZE != (va - bytes.len()) / PAGE_SIZE {
        KERNEL_SPACE.exclusive_access().page_table.identical_unmap(va - bytes.len());
    }
}
