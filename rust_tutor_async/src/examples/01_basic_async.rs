// 示例1: 基础异步函数
// 这个示例介绍了Rust异步编程的基本概念：async函数和await关键字

use std::time::Duration;
use tokio::time::sleep;

// 这是一个异步函数，使用async关键字声明
// 异步函数返回一个Future，需要被执行器(executor)运行
async fn say_hello() {
    println!("开始执行异步函数...");
    
    // sleep是一个异步操作，它不会阻塞线程
    // await关键字用于等待异步操作完成
    sleep(Duration::from_secs(1)).await;
    
    println!("Hello, 异步世界!");
}

// 另一个异步函数，演示异步函数可以调用其他异步函数
async fn greet_user(name: &str) {
    println!("准备问候用户: {}", name);
    
    // 模拟一些异步工作，比如从数据库获取用户信息
    sleep(Duration::from_millis(500)).await;
    
    println!("你好, {}! 欢迎来到异步编程的世界!", name);
}

// 演示返回值的异步函数
async fn calculate_async(x: i32, y: i32) -> i32 {
    println!("开始异步计算: {} + {}", x, y);
    
    // 模拟一个耗时的计算过程
    sleep(Duration::from_millis(300)).await;
    
    let result = x + y;
    println!("计算完成，结果是: {}", result);
    result
}

// main函数也可以是异步的，但需要使用tokio::main宏
// tokio::main宏会创建一个异步运行时来执行我们的异步代码
#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例1: 基础异步函数 ===\n");
    
    // 调用异步函数需要使用.await
    // 这里会等待say_hello函数完成后再继续
    say_hello().await;
    
    println!(); // 空行分隔
    
    // 调用带参数的异步函数
    greet_user("张三").await;
    
    println!(); // 空行分隔
    
    // 调用有返回值的异步函数
    let result = calculate_async(10, 20).await;
    println!("从异步函数获得的结果: {}", result);
    
    println!("\n=== 示例完成 ===");
    
    // 重要概念总结：
    // 1. async关键字用于声明异步函数
    // 2. 异步函数返回Future，需要被执行器运行
    // 3. await关键字用于等待异步操作完成
    // 4. tokio::main宏提供异步运行时
    // 5. 异步函数可以调用其他异步函数
}

/*
运行这个示例：
cargo run --bin example_01_basic_async

预期输出：
=== Rust 异步编程示例1: 基础异步函数 ===

开始执行异步函数...
Hello, 异步世界!

准备问候用户: 张三
你好, 张三! 欢迎来到异步编程的世界!

开始异步计算: 10 + 20
计算完成，结果是: 30
从异步函数获得的结果: 30

=== 示例完成 ===

关键学习点：
- 异步函数使用async关键字声明
- 使用await等待异步操作完成
- 异步操作不会阻塞线程，允许其他任务并发执行
- tokio提供了异步运行时环境
*/ 