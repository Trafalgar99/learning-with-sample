// 示例8: 异步流处理
// 这个示例展示如何使用Streams处理异步数据流

use std::time::Duration;
use tokio::time::sleep;
use futures::{Stream, StreamExt, TryStreamExt, stream};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

// 基本的流操作
async fn basic_stream_operations() {
    println!("=== 基本流操作 ===\n");
    
    // 从向量创建流
    println!("1. 从向量创建流:");
    let numbers = vec![1, 2, 3, 4, 5];
    let mut stream = stream::iter(numbers);
    
    while let Some(number) = stream.next().await {
        println!("  处理数字: {}", number);
    }
    
    // 使用map转换流中的元素
    println!("\n2. 使用map转换流:");
    let numbers = vec![1, 2, 3, 4, 5];
    let mut doubled_stream = stream::iter(numbers)
        .map(|x| x * 2);
    
    while let Some(doubled) = doubled_stream.next().await {
        println!("  双倍数字: {}", doubled);
    }
    
    // 使用filter过滤流
    println!("\n3. 使用filter过滤流:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut even_stream = Box::pin(stream::iter(numbers)
        .filter(|&x| async move { x % 2 == 0 }));
    
    while let Some(even) = even_stream.next().await {
        println!("  偶数: {}", even);
    }
    
    println!();
}

// 异步流生成器
async fn async_stream_generators() {
    println!("=== 异步流生成器 ===\n");
    
    // 创建一个定时器流
    fn timer_stream(count: u32, interval_ms: u64) -> impl Stream<Item = u32> {
        stream::unfold((0, count, interval_ms), |(current, max, delay)| async move {
            if current >= max {
                None
            } else {
                sleep(Duration::from_millis(delay)).await;
                Some((current, (current + 1, max, delay)))
            }
        })
    }
    
    println!("1. 定时器流 (每500ms产生一个数字):");
    let mut timer = Box::pin(timer_stream(5, 500));
    while let Some(tick) = timer.next().await {
        println!("  定时器tick: {}", tick);
    }
    
    // 创建一个异步数据生成流
    fn data_generator_stream() -> impl Stream<Item = String> {
        stream::unfold(0, |state| async move {
            if state >= 3 {
                None
            } else {
                // 模拟异步数据获取
                sleep(Duration::from_millis(200)).await;
                let data = format!("数据项_{}", state);
                Some((data, state + 1))
            }
        })
    }
    
    println!("\n2. 异步数据生成流:");
    let mut data_stream = Box::pin(data_generator_stream());
    while let Some(data) = data_stream.next().await {
        println!("  生成的数据: {}", data);
    }
    
    println!();
}

// 自定义流实现
async fn custom_stream_implementation() {
    println!("=== 自定义流实现 ===\n");
    
    // 实现一个简单的计数器流
    struct CounterStream {
        current: u32,
        max: u32,
        delay: Duration,
    }
    
    impl CounterStream {
        fn new(max: u32, delay: Duration) -> Self {
            CounterStream {
                current: 0,
                max,
                delay,
            }
        }
    }
    
    impl Stream for CounterStream {
        type Item = u32;
        
        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.current >= self.max {
                return Poll::Ready(None);
            }
            
            // 在实际实现中，这里应该使用适当的异步机制
            // 这里为了简化，直接返回当前值
            let current = self.current;
            self.current += 1;
            
            // 模拟异步延迟（在实际实现中应该使用Timer）
            Poll::Ready(Some(current))
        }
    }
    
    println!("自定义计数器流:");
    let mut counter = CounterStream::new(5, Duration::from_millis(100));
    while let Some(count) = counter.next().await {
        println!("  计数: {}", count);
        // 手动添加延迟，因为我们的简化实现没有内置延迟
        sleep(Duration::from_millis(100)).await;
    }
    
    println!();
}

// 流的组合和链式操作
async fn stream_composition_and_chaining() {
    println!("=== 流的组合和链式操作 ===\n");
    
    // 复杂的流处理管道
    println!("1. 复杂的流处理管道:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let result: Vec<String> = stream::iter(numbers)
        .filter(|&x| async move { x % 2 == 0 })  // 过滤偶数
        .map(|x| async move {                     // 异步转换
            sleep(Duration::from_millis(50)).await;
            x * x
        })
        .buffer_unordered(3)                      // 并发处理，最多3个
        .filter(|&x| async move { x > 10 })       // 过滤大于10的数
        .map(|x| format!("结果: {}", x))          // 转换为字符串
        .collect()                                // 收集所有结果
        .await;
    
    for item in result {
        println!("  {}", item);
    }
    
    // 流的合并
    println!("\n2. 流的合并:");
    let stream1 = stream::iter(vec!["A", "B", "C"]);
    let stream2 = stream::iter(vec!["1", "2", "3"]);
    
    let mut merged = stream1.chain(stream2);
    while let Some(item) = merged.next().await {
        println!("  合并的项: {}", item);
    }
    
    println!();
}

// 并发流处理
async fn concurrent_stream_processing() {
    println!("=== 并发流处理 ===\n");
    
    // 模拟异步处理函数
    async fn process_item(item: i32) -> String {
        let delay = (item % 3 + 1) * 100; // 不同的延迟
        sleep(Duration::from_millis(delay as u64)).await;
        format!("处理完成: {}", item)
    }
    
    println!("1. 顺序处理 vs 并发处理:");
    
    // 顺序处理
    let start = std::time::Instant::now();
    let items = vec![1, 2, 3, 4, 5];
    
    println!("  顺序处理:");
    let mut sequential_results = Vec::new();
    for item in items.clone() {
        let result = process_item(item).await;
        sequential_results.push(result);
        println!("    {}", sequential_results.last().unwrap());
    }
    let sequential_time = start.elapsed();
    
    // 并发处理
    let start = std::time::Instant::now();
    println!("  并发处理:");
    
    let concurrent_results: Vec<String> = stream::iter(items)
        .map(|item| process_item(item))
        .buffer_unordered(3)  // 最多3个并发任务
        .collect()
        .await;
    
    for result in concurrent_results {
        println!("    {}", result);
    }
    let concurrent_time = start.elapsed();
    
    println!("  顺序处理耗时: {:?}", sequential_time);
    println!("  并发处理耗时: {:?}", concurrent_time);
    println!("  性能提升: {:.2}x", 
             sequential_time.as_millis() as f64 / concurrent_time.as_millis() as f64);
    
    println!();
}

// 流与通道的结合
async fn streams_with_channels() {
    println!("=== 流与通道的结合 ===\n");
    
    let (tx, rx) = mpsc::channel::<i32>(10);
    
    // 生产者任务
    let producer = tokio::spawn(async move {
        for i in 1..=10 {
            println!("  发送: {}", i);
            if let Err(e) = tx.send(i).await {
                println!("  发送失败: {}", e);
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
        println!("  生产者完成");
    });
    
    // 将接收器转换为流并处理
    let stream_processor = tokio::spawn(async move {
        let mut stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        
        while let Some(value) = stream.next().await {
            println!("  从流接收: {}", value);
            
            // 模拟处理时间
            sleep(Duration::from_millis(50)).await;
            
            println!("  处理完成: {}", value * 2);
        }
        println!("  流处理器完成");
    });
    
    // 等待两个任务完成
    let _ = tokio::join!(producer, stream_processor);
    
    println!();
}

// 流的错误处理
async fn stream_error_handling() {
    println!("=== 流的错误处理 ===\n");
    
    // 创建一个可能产生错误的流
    fn fallible_stream() -> impl Stream<Item = Result<i32, String>> {
        stream::iter(vec![
            Ok(1),
            Ok(2),
            Err("处理错误".to_string()),
            Ok(4),
            Ok(5),
        ])
    }
    
    println!("1. 处理流中的错误:");
    let mut error_stream = fallible_stream();
    
    while let Some(result) = error_stream.next().await {
        match result {
            Ok(value) => println!("  成功: {}", value),
            Err(e) => println!("  错误: {}", e),
        }
    }
    
    // 使用try_collect收集成功的结果
    println!("\n2. 使用try_collect (遇到错误会停止):");
    let success_stream = stream::iter(vec![
        Ok::<i32, String>(1),
        Ok::<i32, String>(2),
        Ok::<i32, String>(3),
    ]);
    
    match success_stream.try_collect::<Vec<i32>>().await {
        Ok(results) => println!("  收集的结果: {:?}", results),
        Err(e) => println!("  收集失败: {}", e),
    }
    
    println!();
}

// 实时数据流模拟
async fn real_time_data_stream() {
    println!("=== 实时数据流模拟 ===\n");
    
    // 模拟传感器数据流 - 简化版本
    fn sensor_data_stream() -> impl Stream<Item = f64> {
        stream::unfold(0, |counter| async move {
            if counter >= 10 {
                None
            } else {
                // 模拟200ms延迟
                sleep(Duration::from_millis(200)).await;
                
                // 模拟传感器读数（带一些随机性）
                let base_value = 20.0;
                let variation = (counter as f64 * 0.5).sin() * 5.0;
                let reading = base_value + variation;
                
                Some((reading, counter + 1))
            }
        })
    }
    
    println!("模拟传感器数据流 (每200ms一个读数):");
    let mut sensor_stream = Box::pin(sensor_data_stream());
    
    // 处理传感器数据
    let mut readings = Vec::new();
    while let Some(reading) = sensor_stream.next().await {
        readings.push(reading);
        println!("  传感器读数: {:.2}°C", reading);
        
        // 简单的异常检测
        if reading > 25.0 {
            println!("    ⚠️  高温警告!");
        } else if reading < 15.0 {
            println!("    ❄️  低温警告!");
        }
    }
    
    // 计算统计信息
    let avg = readings.iter().sum::<f64>() / readings.len() as f64;
    let max = readings.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min = readings.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    
    println!("\n传感器数据统计:");
    println!("  平均值: {:.2}°C", avg);
    println!("  最高值: {:.2}°C", max);
    println!("  最低值: {:.2}°C", min);
    
    println!();
}

#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例8: 流处理 ===\n");
    
    // 1. 基本流操作
    basic_stream_operations().await;
    
    // 2. 异步流生成器
    async_stream_generators().await;
    
    // 3. 自定义流实现
    custom_stream_implementation().await;
    
    // 4. 流的组合和链式操作
    stream_composition_and_chaining().await;
    
    // 5. 并发流处理
    concurrent_stream_processing().await;
    
    // 6. 流与通道的结合
    streams_with_channels().await;
    
    // 7. 流的错误处理
    stream_error_handling().await;
    
    // 8. 实时数据流模拟
    real_time_data_stream().await;
    
    println!("=== 示例完成 ===");
}

/*
运行这个示例：
cargo run --bin example_08_streams

关键学习点：
1. Stream trait - 异步迭代器的抽象
2. 流的创建方法 - iter, unfold, 自定义实现
3. 流的转换操作 - map, filter, chain
4. 并发流处理 - buffer_unordered提高性能
5. 流与通道的结合使用
6. 流中的错误处理模式
7. 实时数据流的处理

流操作方法：
- next(): 获取下一个元素
- map(): 转换流中的元素
- filter(): 过滤流中的元素
- collect(): 收集所有元素到集合
- buffer_unordered(): 并发处理多个元素
- chain(): 连接多个流
- try_collect(): 收集Result流的成功元素

性能优化：
- 使用buffer_unordered进行并发处理
- 合理设置并发度避免资源耗尽
- 流式处理避免内存积累
- 惰性求值减少不必要的计算

应用场景：
- 实时数据处理
- 网络数据流
- 文件流处理
- 事件流处理
- 传感器数据采集

注意事项：
- 流是惰性的，只有在被消费时才会执行
- 注意流的生命周期和所有权
- 合理处理流中的错误
- 考虑背压(backpressure)问题
*/ 