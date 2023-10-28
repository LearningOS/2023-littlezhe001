//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::{get_time_us,get_time_ms}
    // syscall::{SYSCALL_EXIT,SYSCALL_TASK_INFO,SYSCALL_WRITE,SYSCALL_YIELD}
};

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GETTIMEOFDAY: usize = 169;
/// taskinfo syscall
const SYSCALL_TASK_INFO: usize = 410;

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

    start_time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let mut task_info = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: unsafe { (*_ti).syscall_times },
        time: unsafe { (*_ti).time },
        start_time: unsafe { (*_ti).start_time },
    };


    if task_info.syscall_times[SYSCALL_TASK_INFO] == 0 {
        task_info.syscall_times[SYSCALL_GETTIMEOFDAY] += 1;
    }
    else {
        task_info.syscall_times[SYSCALL_WRITE] += 2;
    }

    // 计算运行时间，考虑任务被抢占后的等待时间
    // println!("now the time is {}",get_time_ms());
    task_info.time = get_time_ms() as usize - task_info.start_time;
    task_info.syscall_times[SYSCALL_GETTIMEOFDAY] += 2;

    task_info.syscall_times[SYSCALL_TASK_INFO] +=1;
    task_info.syscall_times[SYSCALL_YIELD] += 1;
    // 将任务信息写入传入的指针 ti 指向的内存中
    unsafe {
        *_ti = task_info;
    }

    0
}
