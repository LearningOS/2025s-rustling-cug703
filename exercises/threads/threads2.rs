// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus {
        jobs_completed: Mutex::new(0 ),
    });
    let mut handles = vec![];
    for _ in 0..10 {
        let  status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
           let mut jobs= status_shared.jobs_completed.lock().unwrap();
            *jobs+=1;
        });
        handles.push(handle);
    }
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    
    struct JobStatus {
        jobs_completed: Mutex<u32>, // 使用 Mutex 保护 jobs_completed
    }
    
    fn main() {
        // 创建共享的 JobStatus 实例，初始化 jobs_completed 为 0
        let status = Arc::new(JobStatus {
            jobs_completed: Mutex::new(0),
        });
        let mut handles = vec![];
    
        // 启动 10 个线程
        for _ in 0..10 {
            let status_shared = Arc::clone(&status); // 为每个线程克隆 Arc
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(250)); // 模拟工作
                // 获取锁并更新 jobs_completed
                let mut jobs = status_shared.jobs_completed.lock().unwrap();
                *jobs += 1; // 安全地增加值
            });
            handles.push(handle);
        }
    
        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();    
        }  //确保进程执行完
    
    assert_eq!(10,*status.jobs_completed.lock().unwrap());
    }
}
