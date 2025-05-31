/*
 * Rust智能指针教程 - 例子3: Arc<T> (Atomically Reference Counted)
 * 
 * Arc<T> 是原子引用计数智能指针，是Rc<T>的线程安全版本
 * 主要特点：
 * 1. 可以在多线程间安全共享数据
 * 2. 使用原子操作来管理引用计数
 * 3. 性能略低于Rc，但提供线程安全
 * 4. 数据同样是不可变的（除非配合Mutex等同步原语）
 */

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

// 定义一个共享的配置结构
#[derive(Debug, Clone)]
struct Config {
    app_name: String,
    version: String,
    max_connections: usize,
    timeout_seconds: u64,
}

impl Config {
    fn new() -> Self {
        Config {
            app_name: "智能指针演示应用".to_string(),
            version: "1.0.0".to_string(),
            max_connections: 100,
            timeout_seconds: 30,
        }
    }
}

// 定义一个工作任务
#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
    data: Vec<u8>,
}

impl Task {
    fn new(id: u32, name: String, size: usize) -> Self {
        Task {
            id,
            name,
            data: vec![0; size],
        }
    }
    
    fn execute(&self) {
        println!("线程 {:?} 正在执行任务 {}: {}", 
                 thread::current().id(), self.id, self.name);
        thread::sleep(Duration::from_millis(100)); // 模拟工作
    }
}

// 定义一个简单的线程安全计数器
#[derive(Debug)]
struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn new() -> Arc<Self> {
        Arc::new(Counter {
            value: Mutex::new(0),
        })
    }
    
    fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
        println!("计数器增加到: {}", *val);
    }
    
    fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

fn main() {
    println!("=== Rust智能指针教程 - Arc<T> ===\n");
    
    // 1. 基本的Arc使用
    println!("1. 基本Arc使用:");
    let data = Arc::new("Hello, Arc!".to_string());
    println!("初始引用计数: {}", Arc::strong_count(&data));
    
    let data_clone = Arc::clone(&data);
    println!("克隆后引用计数: {}", Arc::strong_count(&data));
    println!("数据内容: {}\n", data);
    
    // 2. 多线程共享不可变数据
    println!("2. 多线程共享不可变数据:");
    let config = Arc::new(Config::new());
    let mut handles = vec![];
    
    // 创建5个线程，每个都使用共享的配置
    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            println!("线程 {} 使用配置: {} v{}", 
                     i, config_clone.app_name, config_clone.version);
            println!("线程 {} 看到的最大连接数: {}", 
                     i, config_clone.max_connections);
            
            // 模拟一些工作
            thread::sleep(Duration::from_millis(50));
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("所有线程完成后，配置引用计数: {}\n", Arc::strong_count(&config));
    
    // 3. 共享任务队列
    println!("3. 共享任务队列:");
    let tasks = Arc::new(vec![
        Task::new(1, "数据处理".to_string(), 1000),
        Task::new(2, "文件上传".to_string(), 2000),
        Task::new(3, "邮件发送".to_string(), 500),
        Task::new(4, "日志清理".to_string(), 1500),
    ]);
    
    println!("任务队列引用计数: {}", Arc::strong_count(&tasks));
    
    let mut worker_handles = vec![];
    
    // 创建4个工作线程处理任务
    for worker_id in 0..4 {
        let tasks_clone = Arc::clone(&tasks);
        let handle = thread::spawn(move || {
            if let Some(task) = tasks_clone.get(worker_id) {
                task.execute();
            }
        });
        worker_handles.push(handle);
    }
    
    // 等待所有工作线程完成
    for handle in worker_handles {
        handle.join().unwrap();
    }
    
    println!("所有工作完成后，任务队列引用计数: {}\n", Arc::strong_count(&tasks));
    
    // 4. Arc配合Mutex实现线程安全的可变数据
    println!("4. Arc + Mutex 实现线程安全的可变数据:");
    let counter = Counter::new();
    println!("初始计数器引用计数: {}", Arc::strong_count(&counter));
    
    let mut counter_handles = vec![];
    
    // 创建10个线程，每个都增加计数器
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("线程 {} 准备增加计数器", i);
            counter_clone.increment();
        });
        counter_handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in counter_handles {
        handle.join().unwrap();
    }
    
    println!("最终计数器值: {}", counter.get_value());
    println!("计数器引用计数: {}\n", Arc::strong_count(&counter));
    
    // 5. Arc的内存效率演示
    println!("5. Arc的内存效率:");
    let large_data = Arc::new(vec![42u8; 10000]); // 10KB数据
    println!("大数据初始引用计数: {}", Arc::strong_count(&large_data));
    
    let mut data_handles = vec![];
    
    // 创建20个线程共享大数据
    for i in 0..20 {
        let data_clone = Arc::clone(&large_data);
        let handle = thread::spawn(move || {
            // 每个线程都可以访问相同的数据
            let sum: u64 = data_clone.iter().map(|&x| x as u64).sum();
            println!("线程 {} 计算的数据总和: {}", i, sum);
            thread::sleep(Duration::from_millis(10));
        });
        data_handles.push(handle);
    }
    
    // 在主线程中也可以访问数据
    println!("主线程中数据长度: {}", large_data.len());
    println!("创建所有线程后引用计数: {}", Arc::strong_count(&large_data));
    
    // 等待所有线程完成
    for handle in data_handles {
        handle.join().unwrap();
    }
    
    println!("所有线程完成后引用计数: {}\n", Arc::strong_count(&large_data));
    
    // 6. Arc::try_unwrap 在多线程环境中的使用
    println!("6. Arc::try_unwrap 在多线程环境:");
    let single_threaded_data = Arc::new("单线程数据".to_string());
    
    // 只有一个引用时可以成功解包
    match Arc::try_unwrap(single_threaded_data) {
        Ok(data) => println!("成功解包: {}", data),
        Err(arc) => println!("解包失败，引用计数: {}", Arc::strong_count(&arc)),
    }
    
    // 7. 演示Arc vs Rc的区别
    println!("\n7. Arc vs Rc 的区别:");
    println!("Arc 特点:");
    println!("- 线程安全，可以在多线程间共享");
    println!("- 使用原子操作，性能略低于Rc");
    println!("- 适用于多线程场景");
    println!();
    println!("Rc 特点:");
    println!("- 非线程安全，只能在单线程中使用");
    println!("- 性能更好，因为不需要原子操作");
    println!("- 适用于单线程场景");
    
    println!("\n=== Arc教程完成 ===");
}

// 演示Arc在异步环境中的使用
async fn async_worker(id: usize, data: Arc<Vec<i32>>) {
    println!("异步工作者 {} 开始处理数据", id);
    
    // 模拟异步工作
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let sum: i32 = data.iter().sum();
    println!("异步工作者 {} 计算结果: {}", id, sum);
}

// 辅助函数：创建共享的工作队列
fn create_shared_work_queue() -> Arc<Mutex<Vec<String>>> {
    Arc::new(Mutex::new(vec![
        "任务1".to_string(),
        "任务2".to_string(),
        "任务3".to_string(),
        "任务4".to_string(),
        "任务5".to_string(),
    ]))
}

// 工作者函数：从共享队列中取任务
fn worker(id: usize, queue: Arc<Mutex<Vec<String>>>) {
    loop {
        let task = {
            let mut q = queue.lock().unwrap();
            q.pop()
        };
        
        match task {
            Some(task) => {
                println!("工作者 {} 处理任务: {}", id, task);
                thread::sleep(Duration::from_millis(100));
            },
            None => {
                println!("工作者 {} 没有更多任务，退出", id);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arc_reference_counting() {
        let data = Arc::new(42);
        assert_eq!(Arc::strong_count(&data), 1);
        
        let data_clone = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 2);
        
        drop(data_clone);
        assert_eq!(Arc::strong_count(&data), 1);
    }
    
    #[test]
    fn test_arc_thread_safety() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut val = counter_clone.lock().unwrap();
                *val += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*counter.lock().unwrap(), 10);
    }
    
    #[test]
    fn test_shared_data_immutability() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data_clone = Arc::clone(&data);
        
        // 验证两个Arc指向同一数据
        assert!(std::ptr::eq(Arc::as_ptr(&data), Arc::as_ptr(&data_clone)));
        
        // 验证数据内容相同
        assert_eq!(*data, *data_clone);
    }
} 