// 示例4: 通道通信
// 这个示例展示如何使用各种类型的channels在异步任务间传递数据

use std::time::Duration;
use tokio::time::{sleep, interval};
use tokio::sync::{mpsc, oneshot, broadcast, watch};
use tokio::task;

// 演示基本的mpsc (multiple producer, single consumer) 通道
async fn demonstrate_mpsc() {
    println!("=== MPSC通道演示 ===\n");
    
    // 创建一个容量为10的有界通道
    let (tx, mut rx) = mpsc::channel::<String>(10);
    
    // 创建多个生产者任务
    for i in 1..=3 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            for j in 1..=3 {
                let message = format!("生产者{}发送消息{}", i, j);
                println!("发送: {}", message);
                
                // 发送消息到通道
                if let Err(e) = tx_clone.send(message).await {
                    println!("发送失败: {}", e);
                }
                
                sleep(Duration::from_millis(100)).await;
            }
            println!("生产者{}完成", i);
        });
    }
    
    // 关闭原始发送端，这样当所有克隆的发送端都关闭时，接收端会知道
    drop(tx);
    
    // 消费者任务
    let consumer = tokio::spawn(async move {
        let mut count = 0;
        while let Some(message) = rx.recv().await {
            count += 1;
            println!("接收到: {} (第{}条消息)", message, count);
            sleep(Duration::from_millis(50)).await;
        }
        println!("消费者完成，总共接收{}条消息", count);
    });
    
    // 等待消费者完成
    consumer.await.unwrap();
    println!();
}

// 演示oneshot通道（一次性通道）
async fn demonstrate_oneshot() {
    println!("=== Oneshot通道演示 ===\n");
    
    // oneshot通道只能发送一次消息
    let (tx, rx) = oneshot::channel::<String>();
    
    // 启动一个任务来执行某些工作并返回结果
    let worker = tokio::spawn(async move {
        println!("工作任务开始执行...");
        
        // 模拟一些异步工作
        sleep(Duration::from_millis(500)).await;
        
        let result = "工作完成的结果".to_string();
        println!("工作完成，发送结果");
        
        // 发送结果
        if let Err(_) = tx.send(result) {
            println!("发送结果失败，接收端可能已关闭");
        }
    });
    
    // 等待结果
    println!("等待工作结果...");
    match rx.await {
        Ok(result) => println!("收到结果: {}", result),
        Err(e) => println!("接收结果失败: {}", e),
    }
    
    worker.await.unwrap();
    println!();
}

// 演示broadcast通道（广播通道）
async fn demonstrate_broadcast() {
    println!("=== Broadcast通道演示 ===\n");
    
    // 创建一个容量为16的广播通道
    let (tx, _rx) = broadcast::channel::<String>(16);
    
    // 创建多个订阅者
    for i in 1..=3 {
        let mut subscriber = tx.subscribe();
        tokio::spawn(async move {
            println!("订阅者{}开始监听", i);
            while let Ok(message) = subscriber.recv().await {
                println!("订阅者{}收到: {}", i, message);
                sleep(Duration::from_millis(100)).await;
            }
            println!("订阅者{}结束", i);
        });
    }
    
    // 发布者发送消息
    let publisher = tokio::spawn(async move {
        for i in 1..=5 {
            let message = format!("广播消息{}", i);
            println!("发布: {}", message);
            
            match tx.send(message) {
                Ok(subscriber_count) => println!("  -> 发送给{}个订阅者", subscriber_count),
                Err(e) => println!("  -> 发送失败: {}", e),
            }
            
            sleep(Duration::from_millis(200)).await;
        }
        println!("发布者完成");
    });
    
    publisher.await.unwrap();
    
    // 等待一段时间让订阅者处理完消息
    sleep(Duration::from_millis(500)).await;
    println!();
}

// 演示watch通道（状态监视通道）
async fn demonstrate_watch() {
    println!("=== Watch通道演示 ===\n");
    
    // watch通道用于共享状态，接收者总是能看到最新的值
    let (tx, rx) = watch::channel::<i32>(0);
    
    // 创建多个观察者
    for i in 1..=3 {
        let mut watcher = rx.clone();
        tokio::spawn(async move {
            println!("观察者{}开始监视", i);
            
            // 获取当前值
            println!("观察者{}看到初始值: {}", i, *watcher.borrow());
            
            // 监听值的变化
            while watcher.changed().await.is_ok() {
                let value = *watcher.borrow();
                println!("观察者{}看到新值: {}", i, value);
                sleep(Duration::from_millis(50)).await;
            }
            println!("观察者{}结束", i);
        });
    }
    
    // 状态更新者
    let updater = tokio::spawn(async move {
        for i in 1..=5 {
            println!("更新状态为: {}", i);
            if let Err(e) = tx.send(i) {
                println!("更新失败: {}", e);
                break;
            }
            sleep(Duration::from_millis(300)).await;
        }
        println!("状态更新者完成");
    });
    
    updater.await.unwrap();
    
    // 等待观察者处理完
    sleep(Duration::from_millis(500)).await;
    println!();
}

// 演示生产者-消费者模式
async fn demonstrate_producer_consumer() {
    println!("=== 生产者-消费者模式演示 ===\n");
    
    #[derive(Debug, Clone)]
    struct Task {
        id: u32,
        data: String,
    }
    
    let (task_tx, mut task_rx) = mpsc::channel::<Task>(5);
    let (result_tx, mut result_rx) = mpsc::channel::<String>(5);
    
    // 生产者：生成任务
    let producer = tokio::spawn(async move {
        for i in 1..=10 {
            let task = Task {
                id: i,
                data: format!("任务数据{}", i),
            };
            
            println!("生产任务: {:?}", task);
            if let Err(e) = task_tx.send(task).await {
                println!("发送任务失败: {}", e);
                break;
            }
            
            sleep(Duration::from_millis(100)).await;
        }
        println!("生产者完成");
    });
    
    // 消费者：处理任务
    let consumer = tokio::spawn(async move {
        while let Some(task) = task_rx.recv().await {
            println!("处理任务: {:?}", task);
            
            // 模拟任务处理
            sleep(Duration::from_millis(200)).await;
            
            let result = format!("任务{}处理完成", task.id);
            if let Err(e) = result_tx.send(result).await {
                println!("发送结果失败: {}", e);
                break;
            }
        }
        println!("消费者完成");
    });
    
    // 结果收集器
    let collector = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(result) = result_rx.recv().await {
            println!("收集结果: {}", result);
            results.push(result);
        }
        println!("收集器完成，总共收集{}个结果", results.len());
        results
    });
    
    // 等待所有任务完成
    producer.await.unwrap();
    consumer.await.unwrap();
    let results = collector.await.unwrap();
    
    println!("所有结果: {:?}", results);
    println!();
}

// 演示通道的错误处理和优雅关闭
async fn demonstrate_error_handling() {
    println!("=== 通道错误处理演示 ===\n");
    
    let (tx, mut rx) = mpsc::channel::<i32>(3);
    
    // 发送者任务
    let sender = tokio::spawn(async move {
        for i in 1..=5 {
            match tx.send(i).await {
                Ok(_) => println!("成功发送: {}", i),
                Err(e) => {
                    println!("发送失败: {} (错误: {})", i, e);
                    break;
                }
            }
            sleep(Duration::from_millis(100)).await;
        }
        println!("发送者结束");
    });
    
    // 接收者任务（提前退出）
    let receiver = tokio::spawn(async move {
        let mut count = 0;
        while let Some(value) = rx.recv().await {
            count += 1;
            println!("接收到: {}", value);
            
            // 模拟接收者在处理3个消息后退出
            if count >= 3 {
                println!("接收者提前退出");
                break;
            }
            
            sleep(Duration::from_millis(150)).await;
        }
    });
    
    // 等待任务完成
    let (sender_result, receiver_result) = tokio::join!(sender, receiver);
    
    match sender_result {
        Ok(_) => println!("发送者正常完成"),
        Err(e) => println!("发送者出错: {}", e),
    }
    
    match receiver_result {
        Ok(_) => println!("接收者正常完成"),
        Err(e) => println!("接收者出错: {}", e),
    }
    
    println!();
}

#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例4: 通道通信 ===\n");
    
    // 1. MPSC通道
    demonstrate_mpsc().await;
    
    // 2. Oneshot通道
    demonstrate_oneshot().await;
    
    // 3. Broadcast通道
    demonstrate_broadcast().await;
    
    // 4. Watch通道
    demonstrate_watch().await;
    
    // 5. 生产者-消费者模式
    demonstrate_producer_consumer().await;
    
    // 6. 错误处理
    demonstrate_error_handling().await;
    
    println!("=== 示例完成 ===");
}

/*
运行这个示例：
cargo run --bin example_04_channels

关键学习点：
1. mpsc::channel - 多生产者单消费者通道，用于任务间传递数据
2. oneshot::channel - 一次性通道，用于获取单个异步操作的结果
3. broadcast::channel - 广播通道，一个发送者对多个接收者
4. watch::channel - 状态监视通道，接收者总是能看到最新状态

通道类型选择：
- mpsc: 多个生产者向一个消费者发送数据
- oneshot: 获取异步操作的单个结果
- broadcast: 向多个订阅者广播消息
- watch: 共享状态，多个观察者监视状态变化

最佳实践：
- 合理设置通道容量，避免内存泄漏
- 正确处理通道关闭和错误情况
- 使用适当的通道类型来匹配通信模式
- 注意通道的生命周期管理
*/ 