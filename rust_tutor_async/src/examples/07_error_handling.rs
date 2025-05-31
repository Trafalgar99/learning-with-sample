// 示例7: 异步错误处理
// 这个示例展示在异步代码中处理错误的各种模式和最佳实践

use std::time::Duration;
use tokio::time::sleep;
use std::fmt;
use std::error::Error as StdError;

// 自定义错误类型
#[derive(Debug)]
enum CustomError {
    NetworkError(String),
    ValidationError(String),
    TimeoutError,
    DatabaseError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            CustomError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            CustomError::TimeoutError => write!(f, "超时错误"),
            CustomError::DatabaseError(msg) => write!(f, "数据库错误: {}", msg),
        }
    }
}

impl StdError for CustomError {}

// 基本的错误处理
async fn basic_error_handling() {
    println!("=== 基本错误处理 ===\n");
    
    // 可能失败的异步函数
    async fn risky_operation(should_fail: bool) -> Result<String, CustomError> {
        println!("执行风险操作...");
        sleep(Duration::from_millis(100)).await;
        
        if should_fail {
            Err(CustomError::NetworkError("连接失败".to_string()))
        } else {
            Ok("操作成功".to_string())
        }
    }
    
    // 处理成功的情况
    println!("1. 处理成功的操作:");
    match risky_operation(false).await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("失败: {}", e),
    }
    
    // 处理失败的情况
    println!("\n2. 处理失败的操作:");
    match risky_operation(true).await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("失败: {}", e),
    }
    
    println!();
}

// 使用?操作符进行错误传播
async fn error_propagation() -> Result<(), CustomError> {
    println!("=== 错误传播 ===\n");
    
    async fn step1() -> Result<String, CustomError> {
        println!("执行步骤1");
        sleep(Duration::from_millis(50)).await;
        Ok("步骤1完成".to_string())
    }
    
    async fn step2() -> Result<String, CustomError> {
        println!("执行步骤2");
        sleep(Duration::from_millis(50)).await;
        Err(CustomError::ValidationError("数据验证失败".to_string()))
    }
    
    async fn step3() -> Result<String, CustomError> {
        println!("执行步骤3");
        sleep(Duration::from_millis(50)).await;
        Ok("步骤3完成".to_string())
    }
    
    // 使用?操作符，任何步骤失败都会立即返回错误
    let result1 = step1().await?;
    println!("步骤1结果: {}", result1);
    
    let result2 = step2().await?; // 这里会失败并返回错误
    println!("步骤2结果: {}", result2);
    
    let result3 = step3().await?; // 这行不会执行
    println!("步骤3结果: {}", result3);
    
    Ok(())
}

// 并发操作的错误处理
async fn concurrent_error_handling() {
    println!("=== 并发操作错误处理 ===\n");
    
    async fn task(id: u32, should_fail: bool) -> Result<String, CustomError> {
        println!("任务{}开始", id);
        sleep(Duration::from_millis(100 * id as u64)).await;
        
        if should_fail {
            Err(CustomError::NetworkError(format!("任务{}失败", id)))
        } else {
            Ok(format!("任务{}成功", id))
        }
    }
    
    // 使用join_all处理多个可能失败的任务
    println!("1. 使用join_all处理并发任务:");
    let tasks = vec![
        task(1, false),
        task(2, true),  // 这个会失败
        task(3, false),
    ];
    
    let results = futures::future::join_all(tasks).await;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(value) => println!("  任务{}: 成功 - {}", i + 1, value),
            Err(e) => println!("  任务{}: 失败 - {}", i + 1, e),
        }
    }
    
    // 使用try_join!，任何一个失败就全部失败
    println!("\n2. 使用try_join!，一个失败全部失败:");
    match tokio::try_join!(
        task(4, false),
        task(5, true),  // 这个会失败
        task(6, false)
    ) {
        Ok((r1, r2, r3)) => {
            println!("  全部成功: {}, {}, {}", r1, r2, r3);
        }
        Err(e) => {
            println!("  有任务失败: {}", e);
        }
    }
    
    println!();
}

// 超时错误处理
async fn timeout_error_handling() {
    println!("=== 超时错误处理 ===\n");
    
    async fn slow_operation(delay_ms: u64) -> Result<String, CustomError> {
        println!("开始慢操作 ({}ms)", delay_ms);
        sleep(Duration::from_millis(delay_ms)).await;
        Ok(format!("慢操作完成 ({}ms)", delay_ms))
    }
    
    // 使用tokio::time::timeout处理超时
    println!("1. 正常完成的操作:");
    match tokio::time::timeout(Duration::from_millis(500), slow_operation(200)).await {
        Ok(Ok(result)) => println!("  成功: {}", result),
        Ok(Err(e)) => println!("  操作失败: {}", e),
        Err(_) => println!("  超时"),
    }
    
    println!("\n2. 超时的操作:");
    match tokio::time::timeout(Duration::from_millis(300), slow_operation(500)).await {
        Ok(Ok(result)) => println!("  成功: {}", result),
        Ok(Err(e)) => println!("  操作失败: {}", e),
        Err(_) => println!("  超时"),
    }
    
    println!();
}

// 重试机制
async fn retry_mechanism() {
    println!("=== 重试机制 ===\n");
    
    async fn unreliable_operation(attempt: u32) -> Result<String, CustomError> {
        println!("  尝试第{}次", attempt);
        sleep(Duration::from_millis(100)).await;
        
        // 前两次尝试失败，第三次成功
        if attempt < 3 {
            Err(CustomError::NetworkError(format!("第{}次尝试失败", attempt)))
        } else {
            Ok("操作最终成功".to_string())
        }
    }
    
    async fn retry_with_backoff<F, Fut, T, E>(
        mut operation: F,
        max_retries: u32,
        initial_delay: Duration,
    ) -> Result<T, E>
    where
        F: FnMut(u32) -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: fmt::Display,
    {
        let mut delay = initial_delay;
        
        for attempt in 1..=max_retries {
            match operation(attempt).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    println!("  第{}次尝试失败: {}", attempt, e);
                    
                    if attempt < max_retries {
                        println!("  等待{:?}后重试", delay);
                        sleep(delay).await;
                        delay *= 2; // 指数退避
                    }
                }
            }
        }
        
        // 所有重试都失败了，执行最后一次尝试并返回错误
        operation(max_retries + 1).await
    }
    
    println!("开始重试操作 (最多3次):");
    match retry_with_backoff(
        unreliable_operation,
        3,
        Duration::from_millis(100),
    ).await {
        Ok(result) => println!("最终成功: {}", result),
        Err(e) => println!("最终失败: {}", e),
    }
    
    println!();
}

// 错误恢复和降级
async fn error_recovery_and_fallback() {
    println!("=== 错误恢复和降级 ===\n");
    
    async fn primary_service() -> Result<String, CustomError> {
        println!("尝试主服务");
        sleep(Duration::from_millis(100)).await;
        Err(CustomError::NetworkError("主服务不可用".to_string()))
    }
    
    async fn backup_service() -> Result<String, CustomError> {
        println!("尝试备用服务");
        sleep(Duration::from_millis(150)).await;
        Ok("备用服务响应".to_string())
    }
    
    async fn cache_service() -> Result<String, CustomError> {
        println!("尝试缓存服务");
        sleep(Duration::from_millis(50)).await;
        Ok("缓存数据".to_string())
    }
    
    // 实现降级策略
    async fn get_data_with_fallback() -> Result<String, CustomError> {
        // 首先尝试主服务
        match primary_service().await {
            Ok(data) => return Ok(data),
            Err(e) => println!("主服务失败: {}", e),
        }
        
        // 主服务失败，尝试备用服务
        match backup_service().await {
            Ok(data) => return Ok(data),
            Err(e) => println!("备用服务失败: {}", e),
        }
        
        // 备用服务也失败，使用缓存
        match cache_service().await {
            Ok(data) => {
                println!("使用缓存数据");
                Ok(data)
            }
            Err(e) => {
                println!("缓存服务也失败: {}", e);
                Err(CustomError::NetworkError("所有服务都不可用".to_string()))
            }
        }
    }
    
    match get_data_with_fallback().await {
        Ok(data) => println!("获取数据成功: {}", data),
        Err(e) => println!("获取数据失败: {}", e),
    }
    
    println!();
}

// 结构化错误处理
async fn structured_error_handling() {
    println!("=== 结构化错误处理 ===\n");
    
    #[derive(Debug)]
    struct ErrorContext {
        operation: String,
        timestamp: std::time::SystemTime,
        retry_count: u32,
    }
    
    #[derive(Debug)]
    struct DetailedError {
        error: CustomError,
        context: ErrorContext,
    }
    
    impl fmt::Display for DetailedError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} (操作: {}, 重试: {}次)", 
                   self.error, self.context.operation, self.context.retry_count)
        }
    }
    
    async fn complex_operation() -> Result<String, DetailedError> {
        let context = ErrorContext {
            operation: "复杂业务操作".to_string(),
            timestamp: std::time::SystemTime::now(),
            retry_count: 1,
        };
        
        sleep(Duration::from_millis(100)).await;
        
        // 模拟失败
        Err(DetailedError {
            error: CustomError::DatabaseError("连接超时".to_string()),
            context,
        })
    }
    
    match complex_operation().await {
        Ok(result) => println!("操作成功: {}", result),
        Err(e) => {
            println!("操作失败: {}", e);
            println!("错误详情: {:?}", e);
        }
    }
    
    println!();
}

// 异步闭包中的错误处理
async fn async_closure_error_handling() {
    println!("=== 异步闭包错误处理 ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 使用map和collect处理可能失败的异步操作
    async fn process_number(n: i32) -> Result<i32, CustomError> {
        sleep(Duration::from_millis(50)).await;
        
        if n == 3 {
            Err(CustomError::ValidationError(format!("数字{}无效", n)))
        } else {
            Ok(n * 2)
        }
    }
    
    println!("处理数字列表:");
    let tasks: Vec<_> = numbers.into_iter()
        .map(|n| async move {
            println!("  处理数字: {}", n);
            process_number(n).await
        })
        .collect();
    
    let results = futures::future::join_all(tasks).await;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(value) => println!("  结果{}: {}", i + 1, value),
            Err(e) => println!("  错误{}: {}", i + 1, e),
        }
    }
    
    println!();
}

#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例7: 错误处理 ===\n");
    
    // 1. 基本错误处理
    basic_error_handling().await;
    
    // 2. 错误传播
    if let Err(e) = error_propagation().await {
        println!("错误传播示例失败: {}\n", e);
    }
    
    // 3. 并发操作错误处理
    concurrent_error_handling().await;
    
    // 4. 超时错误处理
    timeout_error_handling().await;
    
    // 5. 重试机制
    retry_mechanism().await;
    
    // 6. 错误恢复和降级
    error_recovery_and_fallback().await;
    
    // 7. 结构化错误处理
    structured_error_handling().await;
    
    // 8. 异步闭包错误处理
    async_closure_error_handling().await;
    
    println!("=== 示例完成 ===");
}

/*
运行这个示例：
cargo run --bin example_07_error_handling

关键学习点：
1. 自定义错误类型和Error trait实现
2. 使用?操作符进行错误传播
3. 并发操作中的错误处理策略
4. 超时处理避免无限等待
5. 重试机制和指数退避
6. 错误恢复和服务降级
7. 结构化错误信息
8. 异步闭包中的错误处理

错误处理模式：
- Result<T, E>: 标准的错误处理类型
- match表达式: 显式处理成功和失败情况
- ?操作符: 简化错误传播
- try_join!: 任一失败则全部失败
- join_all: 收集所有结果，包括错误

最佳实践：
- 定义清晰的错误类型
- 提供有用的错误信息
- 实现适当的重试策略
- 考虑降级和恢复机制
- 记录错误上下文信息
- 避免忽略错误

性能考虑：
- 错误处理不应该成为性能瓶颈
- 合理使用重试，避免雪崩效应
- 超时设置要平衡响应性和成功率
*/ 