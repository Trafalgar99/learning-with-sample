// 示例2: 深入理解async/await
// 这个示例深入探讨async/await的工作机制，Future的概念，以及异步代码的执行顺序

use std::time::{Duration, Instant};
use tokio::time::sleep;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 演示异步函数实际上返回的是Future
async fn simple_async_function() -> String {
    sleep(Duration::from_millis(100)).await;
    "异步函数完成".to_string()
}

// 手动实现一个简单的Future来理解其工作原理
struct SimpleFuture {
    completed: bool,
}

impl SimpleFuture {
    fn new() -> Self {
        SimpleFuture { completed: false }
    }
}

impl Future for SimpleFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready("手动实现的Future完成".to_string())
        } else {
            self.completed = true;
            // 在真实场景中，这里会注册waker来在准备好时唤醒任务
            Poll::Ready("手动实现的Future完成".to_string())
        }
    }
}

// 演示异步代码的执行顺序
async fn demonstrate_execution_order() {
    println!("1. 开始执行异步函数");
    
    // 这里创建了Future，但还没有开始执行
    let future1 = async {
        println!("3. Future1 开始执行");
        sleep(Duration::from_millis(200)).await;
        println!("5. Future1 完成");
        "Future1结果"
    };
    
    let future2 = async {
        println!("4. Future2 开始执行");
        sleep(Duration::from_millis(100)).await;
        println!("6. Future2 完成");
        "Future2结果"
    };
    
    println!("2. 创建了两个Future，但还没有执行");
    
    // 只有当我们await时，Future才开始执行
    // 注意：这里是顺序执行，不是并发
    let result1 = future1.await;
    let result2 = future2.await;
    
    println!("7. 两个Future都完成了: {}, {}", result1, result2);
}

// 演示不同的await模式
async fn different_await_patterns() {
    println!("\n=== 不同的await模式 ===");
    
    // 模式1: 立即await（顺序执行）
    println!("模式1: 顺序执行");
    let start = Instant::now();
    
    let result1 = slow_operation("任务A", 200).await;
    let result2 = slow_operation("任务B", 300).await;
    
    println!("顺序执行结果: {}, {} (耗时: {:?})", 
             result1, result2, start.elapsed());
    
    // 模式2: 先创建Future，再await（仍然是顺序执行）
    println!("\n模式2: 先创建Future再await（仍然顺序）");
    let start = Instant::now();
    
    let future_a = slow_operation("任务C", 200);
    let future_b = slow_operation("任务D", 300);
    
    let result_a = future_a.await;
    let result_b = future_b.await;
    
    println!("结果: {}, {} (耗时: {:?})", 
             result_a, result_b, start.elapsed());
}

// 模拟一个耗时的异步操作
async fn slow_operation(name: &str, delay_ms: u64) -> String {
    println!("  {} 开始执行", name);
    sleep(Duration::from_millis(delay_ms)).await;
    println!("  {} 完成", name);
    format!("{} 的结果", name)
}

// 演示Future的惰性特性
async fn demonstrate_lazy_futures() {
    println!("\n=== Future的惰性特性 ===");
    
    // 创建Future但不执行
    let lazy_future = async {
        println!("这条消息只有在Future被await时才会打印");
        sleep(Duration::from_millis(100)).await;
        "惰性Future的结果"
    };
    
    println!("Future已创建，但还没有执行");
    sleep(Duration::from_millis(200)).await;
    println!("等待了200ms，但Future仍然没有执行");
    
    // 现在执行Future
    let result = lazy_future.await;
    println!("Future执行完成: {}", result);
}

// 演示async块
async fn demonstrate_async_blocks() {
    println!("\n=== async块的使用 ===");
    
    // async块创建一个匿名的异步函数
    let async_block = async {
        println!("在async块中执行");
        sleep(Duration::from_millis(100)).await;
        42
    };
    
    let result = async_block.await;
    println!("async块的结果: {}", result);
    
    // 可以在async块中捕获变量
    let message = "来自外部的消息";
    let capturing_block = async {
        println!("捕获的变量: {}", message);
        sleep(Duration::from_millis(50)).await;
        message.len()
    };
    
    let length = capturing_block.await;
    println!("消息长度: {}", length);
}

#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例2: 深入理解async/await ===\n");
    
    // 1. 基本的异步函数调用
    let result = simple_async_function().await;
    println!("异步函数结果: {}", result);
    
    // 2. 手动实现的Future
    let manual_future = SimpleFuture::new();
    let manual_result = manual_future.await;
    println!("手动Future结果: {}", manual_result);
    
    // 3. 演示执行顺序
    demonstrate_execution_order().await;
    
    // 4. 不同的await模式
    different_await_patterns().await;
    
    // 5. Future的惰性特性
    demonstrate_lazy_futures().await;
    
    // 6. async块的使用
    demonstrate_async_blocks().await;
    
    println!("\n=== 示例完成 ===");
}

/*
运行这个示例：
cargo run --bin example_02_async_await

关键学习点：
1. async函数返回Future，Future是惰性的（lazy）
2. 只有当Future被await时才开始执行
3. await会暂停当前函数的执行，等待Future完成
4. Future可以手动实现，但通常使用async/await更简单
5. async块可以创建匿名的异步函数
6. 顺序await会导致顺序执行，不是并发执行

重要概念：
- Future: 代表一个可能还没有完成的异步计算
- Poll: Future的执行状态（Ready或Pending）
- Executor: 负责运行Future的执行器
- Waker: 用于唤醒等待中的Future
*/ 