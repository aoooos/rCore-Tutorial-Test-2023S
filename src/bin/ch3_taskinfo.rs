#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{
    get_time, print, println, sleep, task_info, TaskInfo, TaskStatus, SYSCALL_EXIT,
    SYSCALL_GETTIMEOFDAY, SYSCALL_TASK_INFO, SYSCALL_WRITE, SYSCALL_YIELD,
};

#[no_mangle]
pub fn main() -> usize {
    let t1 = get_time() as usize;
    let info = TaskInfo::new();
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    // 注意本次 task info 调用也计入
    assert_eq!(0, task_info(&info));
    let t3 = get_time() as usize;
    assert!(3 <= info.syscall_times[SYSCALL_GETTIMEOFDAY]);
    assert_eq!(1, info.syscall_times[SYSCALL_TASK_INFO]);
    assert_eq!(0, info.syscall_times[SYSCALL_WRITE]);
    assert!(0 < info.syscall_times[SYSCALL_YIELD]);
    assert_eq!(0, info.syscall_times[SYSCALL_EXIT]);
    assert!(t2 - t1 <= info.time + 1);
    assert!(info.time < t3 - t1 + 100);
    assert!(info.status == TaskStatus::Running);

    // 想想为什么 write 调用是两次
    /*
        在使用 println 的时候，调用了 /user/src/console.rs 中的 print 函数，print 函数调用了 ConsoleBuffer 所拥有的 Write trait 中的 write_fmt 方法，write_fmt 又调用 core::fmt 中的 write 函数，write 函数又调用了 ConsoleBuffer 实现的 Write trait 中的 write_str 方法，当 write_str 方法被调用时，如果缓冲区已满`或`遇到换行符，则会调用 flush 方法将缓冲区中的数据写入标准输出():
                if (*c == b'\n' || self.0.len() == CONSOLE_BUFFER_SIZE) && -1 == self.flush() {...}
        这里出现2次的原因是因为 println! 自带一个'\n'，同时字符串结尾又有一个'\n'，所以 write 调用是两次。
    */
    println!("string from task info test\n");
    //print!("string from task info test\n");
    let t4 = get_time() as usize;
    assert_eq!(0, task_info(&info));
    let t5 = get_time() as usize;
    assert!(5 <= info.syscall_times[SYSCALL_GETTIMEOFDAY]);
    assert_eq!(2, info.syscall_times[SYSCALL_TASK_INFO]);
    assert_eq!(2, info.syscall_times[SYSCALL_WRITE]);
    assert!(0 < info.syscall_times[SYSCALL_YIELD]);
    assert_eq!(0, info.syscall_times[SYSCALL_EXIT]);
    assert!(t4 - t1 <= info.time + 1);
    assert!(info.time < t5 - t1 + 100);
    assert!(info.status == TaskStatus::Running);

    println!("Test task info OK!");
    0
}
