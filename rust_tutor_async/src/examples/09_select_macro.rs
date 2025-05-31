// 示例9: select!宏的使用
// 这个示例展示如何使用tokio::select!宏来处理多个异步操作

use std::time::Duration;
use tokio::time::{sleep, timeout, interval};
use tokio::sync::{mpsc, oneshot};
use futures::FutureExt;

// 基本的select!使用
async fn basic_select_usage() {
    println!("=== 基本select!使用 ===\n");
    
    // 创建两个不同速度的异步操作
    async fn fast_operation() -> &'static str {
        sleep(Duration::from_millis(100)).await;
        "快速操作完成"
    }
    
    async fn slow_operation() -> &'static str {
        sleep(Duration::from_millis(500)).await;
        "慢速操作完成"
    }
    
    println!("1. 等待第一个完成的操作:");
    
    // select!会等待第一个完成的分支
    tokio::select! {
        result = fast_operation() => {
            println!("  快速操作先完成: {}", result);
        }
        result = slow_operation() => {
            println!("  慢速操作先完成: {}", result);
        }
    }
    
    println!("\n2. 带超时的操作:");
    
    // 使用select!实现超时
    tokio::select! {
        result = slow_operation() => {
            println!("  操作完成: {}", result);
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("  操作超时");
        }
    }
    
    println!();
}

// 通道与select!的结合
async fn channels_with_select() {
    println!("=== 通道与select!的结合 ===\n");
    
    let (tx1, mut rx1) = mpsc::channel::<String>(10);
    let (tx2, mut rx2) = mpsc::channel::<i32>(10);
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
    
    // 生产者任务
    let producer1 = tokio::spawn(async move {
        for i in 1..=5 {
            let message = format!("消息{}", i);
            println!("  发送字符串: {}", message);
            if tx1.send(message).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(300)).await;
        }
    });
    
    let producer2 = tokio::spawn(async move {
        for i in 1..=3 {
            println!("  发送数字: {}", i * 10);
            if tx2.send(i * 10).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
    
    // 在2秒后发送关闭信号
    tokio::spawn(async move {
        sleep(Duration::from_millis(2000)).await;
        println!("  发送关闭信号");
        let _ = shutdown_tx.send(());
    });
    
    // 消费者使用select!处理多个通道
    let consumer = tokio::spawn(async move {
        loop {
            tokio::select! {
                // 接收字符串消息
                msg = rx1.recv() => {
                    match msg {
                        Some(message) => println!("  接收到字符串: {}", message),
                        None => {
                            println!("  字符串通道关闭");
                            break;
                        }
                    }
                }
                
                // 接收数字消息
                num = rx2.recv() => {
                    match num {
                        Some(number) => println!("  接收到数字: {}", number),
                        None => {
                            println!("  数字通道关闭");
                            break;
                        }
                    }
                }
                
                // 接收关闭信号
                _ = &mut shutdown_rx => {
                    println!("  收到关闭信号，退出消费者");
                    break;
                }
            }
        }
    });
    
    // 等待所有任务完成
    let _ = tokio::join!(producer1, producer2, consumer);
    
    println!();
}

// 带条件的select!分支
async fn conditional_select_branches() {
    println!("=== 带条件的select!分支 ===\n");
    
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    
    // 发送一些数据
    tokio::spawn(async move {
        for i in 1..=10 {
            if tx.send(i).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(200)).await;
        }
    });
    
    let mut count = 0;
    let mut should_timeout = false;
    
    loop {
        tokio::select! {
            // 只有在count < 5时才接收消息
            msg = rx.recv(), if count < 5 => {
                match msg {
                    Some(value) => {
                        count += 1;
                        println!("  接收到消息: {} (计数: {})", value, count);
                        
                        if count >= 5 {
                            should_timeout = true;
                            println!("  达到限制，启用超时分支");
                        }
                    }
                    None => break,
                }
            }
            
            // 只有在should_timeout为true时才启用超时
            _ = sleep(Duration::from_millis(500)), if should_timeout => {
                println!("  超时，停止接收");
                break;
            }
        }
    }
    
    println!();
}

// 使用select!实现竞争条件处理
async fn race_condition_handling() {
    println!("=== 竞争条件处理 ===\n");
    
    async fn database_query(id: u32) -> Result<String, &'static str> {
        // 模拟数据库查询，有时会很慢
        let delay = if id % 3 == 0 { 800 } else { 200 };
        sleep(Duration::from_millis(delay)).await;
        
        if id == 6 {
            Err("数据库错误")
        } else {
            Ok(format!("用户数据_{}", id))
        }
    }
    
    async fn cache_query(id: u32) -> Option<String> {
        // 模拟缓存查询，通常很快但可能没有数据
        sleep(Duration::from_millis(50)).await;
        
        if id % 2 == 0 {
            Some(format!("缓存数据_{}", id))
        } else {
            None
        }
    }
    
    // 实现缓存优先的数据获取策略
    async fn get_user_data(id: u32) -> String {
        println!("  查询用户ID: {}", id);
        
        tokio::select! {
            // 尝试从缓存获取
            cache_result = cache_query(id) => {
                match cache_result {
                    Some(data) => {
                        println!("    从缓存获取: {}", data);
                        data
                    }
                    None => {
                        println!("    缓存未命中，查询数据库");
                        // 缓存未命中，查询数据库
                        match database_query(id).await {
                            Ok(data) => {
                                println!("    从数据库获取: {}", data);
                                data
                            }
                            Err(e) => {
                                println!("    数据库查询失败: {}", e);
                                format!("默认数据_{}", id)
                            }
                        }
                    }
                }
            }
            
            // 同时查询数据库（作为备选）
            db_result = database_query(id) => {
                match db_result {
                    Ok(data) => {
                        println!("    从数据库获取: {}", data);
                        data
                    }
                    Err(e) => {
                        println!("    数据库查询失败: {}", e);
                        format!("默认数据_{}", id)
                    }
                }
            }
        }
    }
    
    // 测试不同的用户ID
    for id in [1, 2, 3, 6] {
        let data = get_user_data(id).await;
        println!("  最终结果: {}\n", data);
    }
    
    println!();
}

// 使用select!实现心跳和工作循环
async fn heartbeat_and_work_loop() {
    println!("=== 心跳和工作循环 ===\n");
    
    let (work_tx, mut work_rx) = mpsc::channel::<String>(10);
    let (stop_tx, mut stop_rx) = oneshot::channel::<()>();
    
    // 工作任务生产者
    tokio::spawn(async move {
        for i in 1..=8 {
            let task = format!("任务_{}", i);
            println!("  添加工作任务: {}", task);
            if work_tx.send(task).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(700)).await;
        }
    });
    
    // 5秒后发送停止信号
    tokio::spawn(async move {
        sleep(Duration::from_millis(5000)).await;
        println!("  发送停止信号");
        let _ = stop_tx.send(());
    });
    
    // 主工作循环
    let mut heartbeat_interval = interval(Duration::from_millis(1000));
    let mut task_count = 0;
    
    loop {
        tokio::select! {
            // 处理工作任务
            work = work_rx.recv() => {
                match work {
                    Some(task) => {
                        task_count += 1;
                        println!("  执行工作: {} (总计: {})", task, task_count);
                        // 模拟工作处理时间
                        sleep(Duration::from_millis(200)).await;
                    }
                    None => {
                        println!("  工作通道关闭");
                        break;
                    }
                }
            }
            
            // 定期心跳
            _ = heartbeat_interval.tick() => {
                println!("  💓 心跳 - 系统运行正常 (已处理{}个任务)", task_count);
            }
            
            // 停止信号
            _ = &mut stop_rx => {
                println!("  收到停止信号，正在关闭...");
                break;
            }
        }
    }
    
    println!("  工作循环结束\n");
}

// 复杂的select!模式：多路复用服务器模拟
async fn multiplexed_server_simulation() {
    println!("=== 多路复用服务器模拟 ===\n");
    
    let (client_tx, mut client_rx) = mpsc::channel::<String>(10);
    let (admin_tx, mut admin_rx) = mpsc::channel::<String>(10);
    let (health_tx, mut health_rx) = mpsc::channel::<()>(10);
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
    
    // 模拟客户端请求
    let client_simulator = tokio::spawn(async move {
        for i in 1..=5 {
            let request = format!("客户端请求_{}", i);
            println!("  📨 {}", request);
            if client_tx.send(request).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(800)).await;
        }
    });
    
    // 模拟管理员命令
    let admin_simulator = tokio::spawn(async move {
        sleep(Duration::from_millis(1500)).await;
        for cmd in ["status", "reload", "backup"] {
            let command = format!("管理员命令: {}", cmd);
            println!("  🔧 {}", command);
            if admin_tx.send(command).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(1200)).await;
        }
    });
    
    // 模拟健康检查
    let health_checker = tokio::spawn(async move {
        for _ in 0..3 {
            sleep(Duration::from_millis(2000)).await;
            println!("  🏥 健康检查请求");
            if health_tx.send(()).await.is_err() {
                break;
            }
        }
    });
    
    // 6秒后关闭服务器
    tokio::spawn(async move {
        sleep(Duration::from_millis(6000)).await;
        println!("  🛑 发送关闭信号");
        let _ = shutdown_tx.send(());
    });
    
    // 服务器主循环
    let mut stats = (0, 0, 0); // (client_requests, admin_commands, health_checks)
    
    loop {
        tokio::select! {
            // 处理客户端请求（高优先级）
            request = client_rx.recv() => {
                match request {
                    Some(req) => {
                        stats.0 += 1;
                        println!("  ✅ 处理: {} (客户端请求: {})", req, stats.0);
                        sleep(Duration::from_millis(100)).await;
                    }
                    None => println!("  客户端通道关闭"),
                }
            }
            
            // 处理管理员命令（中优先级）
            command = admin_rx.recv() => {
                match command {
                    Some(cmd) => {
                        stats.1 += 1;
                        println!("  🔧 执行: {} (管理员命令: {})", cmd, stats.1);
                        sleep(Duration::from_millis(200)).await;
                    }
                    None => println!("  管理员通道关闭"),
                }
            }
            
            // 处理健康检查（低优先级）
            _ = health_rx.recv() => {
                stats.2 += 1;
                println!("  💚 健康检查通过 (健康检查: {})", stats.2);
            }
            
            // 关闭信号（最高优先级）
            _ = &mut shutdown_rx => {
                println!("  🛑 服务器正在关闭...");
                break;
            }
        }
    }
    
    println!("  📊 服务器统计:");
    println!("    客户端请求: {}", stats.0);
    println!("    管理员命令: {}", stats.1);
    println!("    健康检查: {}", stats.2);
    
    // 等待模拟器完成
    let _ = tokio::join!(client_simulator, admin_simulator, health_checker);
    
    println!();
}

// select!的高级用法：biased选择
async fn biased_select_usage() {
    println!("=== Biased Select用法 ===\n");
    
    let (high_priority_tx, mut high_priority_rx) = mpsc::channel::<String>(10);
    let (low_priority_tx, mut low_priority_rx) = mpsc::channel::<String>(10);
    
    // 发送高优先级消息
    tokio::spawn(async move {
        for i in 1..=3 {
            let msg = format!("高优先级_{}", i);
            println!("  发送高优先级: {}", msg);
            if high_priority_tx.send(msg).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
    
    // 发送低优先级消息
    tokio::spawn(async move {
        for i in 1..=6 {
            let msg = format!("低优先级_{}", i);
            println!("  发送低优先级: {}", msg);
            if low_priority_tx.send(msg).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(200)).await;
        }
    });
    
    let mut processed = 0;
    
    // 使用biased select确保高优先级消息优先处理
    while processed < 9 {
        tokio::select! {
            biased;  // 启用biased模式，按分支顺序检查
            
            // 高优先级分支（会被优先检查）
            msg = high_priority_rx.recv() => {
                match msg {
                    Some(message) => {
                        processed += 1;
                        println!("  🔴 处理高优先级: {}", message);
                    }
                    None => break,
                }
            }
            
            // 低优先级分支（只有在高优先级没有消息时才处理）
            msg = low_priority_rx.recv() => {
                match msg {
                    Some(message) => {
                        processed += 1;
                        println!("  🔵 处理低优先级: {}", message);
                    }
                    None => break,
                }
            }
        }
        
        sleep(Duration::from_millis(100)).await;
    }
    
    println!();
}

#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例9: select!宏 ===\n");
    
    // 1. 基本select!使用
    basic_select_usage().await;
    
    // 2. 通道与select!的结合
    channels_with_select().await;
    
    // 3. 带条件的select!分支
    conditional_select_branches().await;
    
    // 4. 竞争条件处理
    race_condition_handling().await;
    
    // 5. 心跳和工作循环
    heartbeat_and_work_loop().await;
    
    // 6. 多路复用服务器模拟
    multiplexed_server_simulation().await;
    
    // 7. Biased select用法
    biased_select_usage().await;
    
    println!("=== 示例完成 ===");
}

/*
运行这个示例：
cargo run --bin example_09_select_macro

关键学习点：
1. tokio::select! - 等待多个异步操作中的第一个完成
2. 分支条件 - 使用if条件控制分支是否启用
3. 通道多路复用 - 同时监听多个通道
4. 超时处理 - 结合sleep实现超时机制
5. 优先级处理 - 使用biased模式控制分支优先级
6. 竞争条件 - 让多个操作竞争，使用最快的结果
7. 服务器模拟 - 处理多种类型的请求

select!宏特性：
- 随机选择：默认情况下随机选择就绪的分支
- biased模式：按分支顺序检查，实现优先级
- 条件分支：使用if条件动态启用/禁用分支
- 引用语义：使用&mut避免移动所有权
- 取消安全：未选中的分支会被取消

应用场景：
- 网络服务器：处理多种类型的连接
- 事件循环：响应不同类型的事件
- 超时处理：为操作添加时间限制
- 优雅关闭：监听关闭信号
- 负载均衡：在多个服务间选择
- 心跳机制：定期发送状态更新

最佳实践：
- 合理使用biased模式避免饥饿
- 正确处理通道关闭情况
- 使用条件分支实现动态行为
- 注意分支中的异步操作取消
- 避免在select!中进行阻塞操作
*/ 