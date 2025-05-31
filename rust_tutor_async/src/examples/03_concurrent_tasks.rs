// 示例3: 并发任务
// 这个示例展示如何使用tokio::join!、tokio::spawn等工具实现真正的并发执行

use std::time::{Duration, Instant};
use tokio::time::sleep;
use tokio::task;

// 模拟不同类型的异步工作
async fn download_file(filename: &str, size_mb: u32) -> String {
    println!("开始下载文件: {} ({}MB)", filename, size_mb);
    
    // 模拟下载时间，大文件需要更长时间
    let download_time = size_mb * 100; // 每MB需要100ms
    sleep(Duration::from_millis(download_time as u64)).await;
    
    println!("文件下载完成: {}", filename);
    format!("已下载: {}", filename)
}

async fn process_data(data_name: &str, complexity: u32) -> String {
    println!("开始处理数据: {} (复杂度: {})", data_name, complexity);
    
    // 模拟数据处理时间
    let process_time = complexity * 50;
    sleep(Duration::from_millis(process_time as u64)).await;
    
    println!("数据处理完成: {}", data_name);
    format!("已处理: {}", data_name)
}

async fn send_notification(message: &str, delay: u64) -> String {
    println!("准备发送通知: {}", message);
    
    sleep(Duration::from_millis(delay)).await;
    
    println!("通知发送完成: {}", message);
    format!("已发送: {}", message)
}

// 演示顺序执行 vs 并发执行的性能差异
async fn sequential_vs_concurrent() {
    println!("=== 顺序执行 vs 并发执行 ===\n");
    
    // 顺序执行
    println!("1. 顺序执行三个任务:");
    let start = Instant::now();
    
    let result1 = download_file("文档.pdf", 5).await;
    let result2 = process_data("用户数据", 8).await;
    let result3 = send_notification("任务完成", 200).await;
    
    let sequential_time = start.elapsed();
    println!("顺序执行结果: {}, {}, {}", result1, result2, result3);
    println!("顺序执行总耗时: {:?}\n", sequential_time);
    
    // 并发执行 - 使用join!宏
    println!("2. 使用join!并发执行三个任务:");
    let start = Instant::now();
    
    let (result1, result2, result3) = tokio::join!(
        download_file("图片.jpg", 3),
        process_data("日志数据", 6),
        send_notification("处理完成", 150)
    );
    
    let concurrent_time = start.elapsed();
    println!("并发执行结果: {}, {}, {}", result1, result2, result3);
    println!("并发执行总耗时: {:?}", concurrent_time);
    println!("性能提升: {:.2}x\n", 
             sequential_time.as_millis() as f64 / concurrent_time.as_millis() as f64);
}

// 演示使用tokio::spawn创建独立任务
async fn demonstrate_spawn() {
    println!("=== 使用tokio::spawn创建独立任务 ===\n");
    
    // spawn创建的任务会在后台运行，即使当前函数结束也会继续执行
    let handle1 = tokio::spawn(async {
        println!("任务1开始执行");
        sleep(Duration::from_millis(300)).await;
        println!("任务1完成");
        "任务1的结果"
    });
    
    let handle2 = tokio::spawn(async {
        println!("任务2开始执行");
        sleep(Duration::from_millis(200)).await;
        println!("任务2完成");
        "任务2的结果"
    });
    
    let handle3 = tokio::spawn(async {
        println!("任务3开始执行");
        sleep(Duration::from_millis(400)).await;
        println!("任务3完成");
        "任务3的结果"
    });
    
    println!("所有任务已启动，等待完成...");
    
    // 等待所有任务完成
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    let result3 = handle3.await.unwrap();
    
    println!("所有spawn任务完成: {}, {}, {}\n", result1, result2, result3);
}

// 演示try_join!处理可能失败的并发任务
async fn demonstrate_try_join() {
    println!("=== 使用try_join!处理可能失败的任务 ===\n");
    
    // 模拟可能失败的异步操作
    async fn risky_operation(name: &str, should_fail: bool) -> Result<String, String> {
        println!("执行风险操作: {}", name);
        sleep(Duration::from_millis(200)).await;
        
        if should_fail {
            Err(format!("{} 失败了", name))
        } else {
            Ok(format!("{} 成功了", name))
        }
    }
    
    // 所有任务都成功的情况
    println!("1. 所有任务都成功:");
    match tokio::try_join!(
        risky_operation("任务A", false),
        risky_operation("任务B", false),
        risky_operation("任务C", false)
    ) {
        Ok((a, b, c)) => println!("全部成功: {}, {}, {}", a, b, c),
        Err(e) => println!("有任务失败: {}", e),
    }
    
    println!();
    
    // 有任务失败的情况
    println!("2. 有任务失败:");
    match tokio::try_join!(
        risky_operation("任务D", false),
        risky_operation("任务E", true),  // 这个会失败
        risky_operation("任务F", false)
    ) {
        Ok((d, e, f)) => println!("全部成功: {}, {}, {}", d, e, f),
        Err(e) => println!("有任务失败: {}", e),
    }
    
    println!();
}

// 演示使用JoinSet管理多个任务
async fn demonstrate_join_set() {
    println!("=== 使用JoinSet管理多个任务 ===\n");
    
    let mut join_set = task::JoinSet::new();
    
    // 添加多个任务到JoinSet
    for i in 1..=5 {
        join_set.spawn(async move {
            let delay = (i * 100) as u64;
            println!("任务{}开始 (延迟{}ms)", i, delay);
            sleep(Duration::from_millis(delay)).await;
            println!("任务{}完成", i);
            format!("任务{}的结果", i)
        });
    }
    
    println!("已启动5个任务，等待完成...");
    
    // 等待所有任务完成
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(value) => println!("收到结果: {}", value),
            Err(e) => println!("任务出错: {}", e),
        }
    }
    
    println!("所有JoinSet任务完成\n");
}

// 演示任务取消
async fn demonstrate_task_cancellation() {
    println!("=== 任务取消演示 ===\n");
    
    let handle = tokio::spawn(async {
        println!("长时间运行的任务开始");
        for i in 1..=10 {
            println!("任务进度: {}/10", i);
            sleep(Duration::from_millis(200)).await;
        }
        println!("长时间任务完成");
        "任务完成"
    });
    
    // 让任务运行一段时间
    sleep(Duration::from_millis(600)).await;
    
    // 取消任务
    handle.abort();
    
    // 尝试获取结果
    match handle.await {
        Ok(result) => println!("任务正常完成: {}", result),
        Err(e) if e.is_cancelled() => println!("任务被取消"),
        Err(e) => println!("任务出错: {}", e),
    }
    
    println!();
}

#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例3: 并发任务 ===\n");
    
    // 1. 顺序执行 vs 并发执行
    sequential_vs_concurrent().await;
    
    // 2. 使用spawn创建独立任务
    demonstrate_spawn().await;
    
    // 3. 使用try_join!处理可能失败的任务
    demonstrate_try_join().await;
    
    // 4. 使用JoinSet管理多个任务
    demonstrate_join_set().await;
    
    // 5. 任务取消
    demonstrate_task_cancellation().await;
    
    println!("=== 示例完成 ===");
}

/*
运行这个示例：
cargo run --bin example_03_concurrent_tasks

关键学习点：
1. tokio::join! - 等待多个Future并发完成
2. tokio::try_join! - 处理可能失败的并发任务
3. tokio::spawn - 创建独立的异步任务
4. JoinSet - 管理动态数量的任务
5. 任务取消 - 使用abort()取消正在运行的任务

性能对比：
- 顺序执行：任务一个接一个执行，总时间是所有任务时间的总和
- 并发执行：任务同时执行，总时间接近最长任务的时间

重要概念：
- 并发 vs 并行：并发是逻辑上的同时执行，并行是物理上的同时执行
- 任务调度：tokio运行时负责在线程间调度异步任务
- 任务生命周期：spawn的任务有独立的生命周期
*/ 