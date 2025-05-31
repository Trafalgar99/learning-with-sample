// 示例10: 高级异步编程模式
// 这个示例展示复杂的异步编程模式和最佳实践

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::time::{sleep, interval};
use tokio::sync::{mpsc, Semaphore, RwLock, Notify};

// 异步资源池模式
async fn async_resource_pool_pattern() {
    println!("=== 异步资源池模式 ===\n");
    
    // 模拟数据库连接
    #[derive(Debug)]
    struct DatabaseConnection {
        id: u32,
        in_use: bool,
    }
    
    impl DatabaseConnection {
        fn new(id: u32) -> Self {
            Self { id, in_use: false }
        }
        
        async fn execute_query(&mut self, query: &str) -> String {
            println!("    连接{}执行查询: {}", self.id, query);
            sleep(Duration::from_millis(100)).await;
            format!("查询结果_{}", self.id)
        }
    }
    
    // 简化的异步连接池
    struct AsyncConnectionPool {
        connections: Arc<tokio::sync::Mutex<Vec<DatabaseConnection>>>,
        semaphore: Arc<Semaphore>,
    }
    
    impl AsyncConnectionPool {
        fn new(max_connections: usize) -> Self {
            let mut connections = Vec::new();
            for i in 0..max_connections {
                connections.push(DatabaseConnection::new(i as u32));
            }
            
            Self {
                connections: Arc::new(tokio::sync::Mutex::new(connections)),
                semaphore: Arc::new(Semaphore::new(max_connections)),
            }
        }
        
        async fn execute_query(&self, query: &str) -> Option<String> {
            // 等待可用连接
            let _permit = self.semaphore.acquire().await.ok()?;
            
            // 获取连接并执行查询
            let mut connections = self.connections.lock().await;
            for conn in connections.iter_mut() {
                if !conn.in_use {
                    conn.in_use = true;
                    let result = conn.execute_query(query).await;
                    conn.in_use = false;
                    return Some(result);
                }
            }
            None
        }
    }
    
    // 测试连接池
    let pool = Arc::new(AsyncConnectionPool::new(3));
    
    println!("创建连接池，最大连接数: 3");
    
    // 并发使用连接池
    let tasks: Vec<_> = (0..6).map(|i| {
        let pool = pool.clone();
        tokio::spawn(async move {
            println!("  任务{}请求连接", i);
            let query = format!("SELECT * FROM table_{}", i);
            if let Some(result) = pool.execute_query(&query).await {
                println!("  任务{}完成: {}", i, result);
            } else {
                println!("  任务{}无法获取连接", i);
            }
            sleep(Duration::from_millis(200)).await;
        })
    }).collect();
    
    futures::future::join_all(tasks).await;
    
    println!();
}

// 异步缓存模式
async fn async_cache_pattern() {
    println!("=== 异步缓存模式 ===\n");
    
    // 异步缓存实现
    struct AsyncCache<K, V> {
        data: Arc<RwLock<HashMap<K, V>>>,
        loading: Arc<RwLock<HashMap<K, Arc<Notify>>>>,
    }
    
    impl<K, V> AsyncCache<K, V> 
    where 
        K: Clone + Eq + std::hash::Hash + std::fmt::Debug,
        V: Clone + std::fmt::Debug,
    {
        fn new() -> Self {
            Self {
                data: Arc::new(RwLock::new(HashMap::new())),
                loading: Arc::new(RwLock::new(HashMap::new())),
            }
        }
        
        async fn get_or_load<F, Fut>(&self, key: K, loader: F) -> V
        where
            F: FnOnce(K) -> Fut,
            Fut: std::future::Future<Output = V>,
        {
            // 首先检查缓存
            {
                let data = self.data.read().await;
                if let Some(value) = data.get(&key) {
                    println!("    缓存命中: {:?}", key);
                    return value.clone();
                }
            }
            
            // 检查是否正在加载
            let notify = {
                let mut loading = self.loading.write().await;
                if let Some(notify) = loading.get(&key) {
                    println!("    等待加载完成: {:?}", key);
                    notify.clone()
                } else {
                    let notify = Arc::new(Notify::new());
                    loading.insert(key.clone(), notify.clone());
                    notify
                }
            };
            
            // 如果已经在加载，等待完成
            {
                let data = self.data.read().await;
                if data.contains_key(&key) {
                    notify.notified().await;
                    return data.get(&key).unwrap().clone();
                }
            }
            
            // 执行加载
            println!("    开始加载: {:?}", key);
            let value = loader(key.clone()).await;
            
            // 存储到缓存
            {
                let mut data = self.data.write().await;
                data.insert(key.clone(), value.clone());
            }
            
            // 清理加载状态并通知等待者
            {
                let mut loading = self.loading.write().await;
                loading.remove(&key);
            }
            notify.notify_waiters();
            
            println!("    加载完成: {:?} -> {:?}", key, value);
            value
        }
    }
    
    // 模拟数据加载函数
    async fn load_user_data(user_id: u32) -> String {
        println!("      从数据库加载用户{}", user_id);
        sleep(Duration::from_millis(500)).await;
        format!("用户{}的数据", user_id)
    }
    
    let cache = Arc::new(AsyncCache::new());
    
    // 并发访问相同的key
    let tasks: Vec<_> = (0..5).map(|i| {
        let cache = cache.clone();
        tokio::spawn(async move {
            let user_id = if i < 3 { 1 } else { 2 }; // 前3个任务访问用户1，后2个访问用户2
            println!("  任务{}请求用户{}", i, user_id);
            let data = cache.get_or_load(user_id, load_user_data).await;
            println!("  任务{}获得: {}", i, data);
        })
    }).collect();
    
    futures::future::join_all(tasks).await;
    
    println!();
}

// 异步工作队列模式
async fn async_work_queue_pattern() {
    println!("=== 异步工作队列模式 ===\n");
    
    // 工作项定义
    #[derive(Debug, Clone)]
    struct WorkItem {
        id: u32,
        data: String,
        priority: u8, // 0-255，数字越小优先级越高
    }
    
    // 异步工作队列
    struct AsyncWorkQueue {
        workers: Vec<tokio::task::JoinHandle<()>>,
        work_sender: mpsc::Sender<WorkItem>,
    }
    
    impl AsyncWorkQueue {
        fn new(worker_count: usize) -> Self {
            let (work_sender, work_receiver) = mpsc::channel::<WorkItem>(100);
            let work_receiver = Arc::new(tokio::sync::Mutex::new(work_receiver));
            
            let mut workers = Vec::new();
            
            for worker_id in 0..worker_count {
                let receiver = work_receiver.clone();
                let worker = tokio::spawn(async move {
                    println!("    工作者{}启动", worker_id);
                    
                    loop {
                        let work_item = {
                            let mut rx = receiver.lock().await;
                            rx.recv().await
                        };
                        
                        match work_item {
                            Some(item) => {
                                println!("    工作者{}处理任务{}: {}", 
                                        worker_id, item.id, item.data);
                                
                                // 模拟工作处理时间
                                let work_time = Duration::from_millis(200 + (item.priority as u64 * 10));
                                sleep(work_time).await;
                                
                                println!("    工作者{}完成任务{}", worker_id, item.id);
                            }
                            None => {
                                println!("    工作者{}退出", worker_id);
                                break;
                            }
                        }
                    }
                });
                
                workers.push(worker);
            }
            
            Self {
                workers,
                work_sender,
            }
        }
        
        async fn submit_work(&self, work_item: WorkItem) -> Result<(), mpsc::error::SendError<WorkItem>> {
            println!("  提交工作: {:?}", work_item);
            self.work_sender.send(work_item).await
        }
        
        async fn shutdown(self) {
            drop(self.work_sender); // 关闭发送端
            
            for worker in self.workers {
                let _ = worker.await;
            }
            println!("  所有工作者已关闭");
        }
    }
    
    // 创建工作队列
    let queue = AsyncWorkQueue::new(3);
    
    // 提交一些工作
    for i in 1..=10 {
        let work_item = WorkItem {
            id: i,
            data: format!("任务数据_{}", i),
            priority: (i % 3) as u8, // 不同优先级
        };
        
        if let Err(e) = queue.submit_work(work_item).await {
            println!("  提交工作失败: {}", e);
        }
        
        sleep(Duration::from_millis(100)).await;
    }
    
    // 等待一段时间让工作完成
    sleep(Duration::from_millis(2000)).await;
    
    // 关闭队列
    queue.shutdown().await;
    
    println!();
}

// 异步状态机模式
async fn async_state_machine_pattern() {
    println!("=== 异步状态机模式 ===\n");
    
    // 状态定义
    #[derive(Debug, Clone)]
    enum ConnectionState {
        Disconnected,
        Connecting,
        Connected,
        Reconnecting,
        Failed(String),
    }
    
    // 事件定义
    #[derive(Debug)]
    enum ConnectionEvent {
        Connect,
        Connected,
        Disconnect,
        ConnectionLost,
        Retry,
        Error(String),
    }
    
    // 异步状态机
    struct AsyncStateMachine {
        state: ConnectionState,
        retry_count: u32,
        max_retries: u32,
    }
    
    impl AsyncStateMachine {
        fn new(max_retries: u32) -> Self {
            Self {
                state: ConnectionState::Disconnected,
                retry_count: 0,
                max_retries,
            }
        }
        
        async fn handle_event(&mut self, event: ConnectionEvent) -> Option<ConnectionState> {
            println!("    状态: {:?}, 事件: {:?}", self.state, event);
            
            let new_state = match (&self.state, event) {
                (ConnectionState::Disconnected, ConnectionEvent::Connect) => {
                    println!("      开始连接...");
                    sleep(Duration::from_millis(100)).await;
                    ConnectionState::Connecting
                }
                
                (ConnectionState::Connecting, ConnectionEvent::Connected) => {
                    println!("      连接成功!");
                    self.retry_count = 0;
                    ConnectionState::Connected
                }
                
                (ConnectionState::Connecting, ConnectionEvent::Error(msg)) => {
                    println!("      连接失败: {}", msg);
                    ConnectionState::Failed(msg)
                }
                
                (ConnectionState::Connected, ConnectionEvent::Disconnect) => {
                    println!("      主动断开连接");
                    ConnectionState::Disconnected
                }
                
                (ConnectionState::Connected, ConnectionEvent::ConnectionLost) => {
                    println!("      连接丢失，准备重连");
                    ConnectionState::Reconnecting
                }
                
                (ConnectionState::Reconnecting, ConnectionEvent::Retry) => {
                    if self.retry_count < self.max_retries {
                        self.retry_count += 1;
                        println!("      重连尝试 {}/{}", self.retry_count, self.max_retries);
                        sleep(Duration::from_millis(200)).await;
                        ConnectionState::Connecting
                    } else {
                        println!("      重连次数超限");
                        ConnectionState::Failed("重连失败".to_string())
                    }
                }
                
                (ConnectionState::Failed(_), ConnectionEvent::Connect) => {
                    println!("      从失败状态重新连接");
                    self.retry_count = 0;
                    ConnectionState::Connecting
                }
                
                _ => {
                    println!("      无效的状态转换");
                    return None;
                }
            };
            
            self.state = new_state.clone();
            println!("      新状态: {:?}", self.state);
            Some(new_state)
        }
        
        fn current_state(&self) -> &ConnectionState {
            &self.state
        }
    }
    
    // 测试状态机
    let mut state_machine = AsyncStateMachine::new(3);
    
    let events = vec![
        ConnectionEvent::Connect,
        ConnectionEvent::Connected,
        ConnectionEvent::ConnectionLost,
        ConnectionEvent::Retry,
        ConnectionEvent::Error("网络错误".to_string()),
        ConnectionEvent::Retry,
        ConnectionEvent::Connected,
        ConnectionEvent::Disconnect,
    ];
    
    for event in events {
        state_machine.handle_event(event).await;
        sleep(Duration::from_millis(100)).await;
    }
    
    println!("  最终状态: {:?}", state_machine.current_state());
    
    println!();
}

// 异步流水线模式
async fn async_pipeline_pattern() {
    println!("=== 异步流水线模式 ===\n");
    
    // 流水线阶段定义
    async fn stage1_fetch(id: u32) -> String {
        println!("    阶段1: 获取数据 {}", id);
        sleep(Duration::from_millis(100)).await;
        format!("原始数据_{}", id)
    }
    
    async fn stage2_transform(data: String) -> String {
        println!("    阶段2: 转换数据 {}", data);
        sleep(Duration::from_millis(150)).await;
        format!("转换后_{}", data)
    }
    
    async fn stage3_validate(data: String) -> Result<String, String> {
        println!("    阶段3: 验证数据 {}", data);
        sleep(Duration::from_millis(80)).await;
        
        // 模拟验证失败
        if data.contains("3") {
            Err(format!("验证失败: {}", data))
        } else {
            Ok(format!("验证通过_{}", data))
        }
    }
    
    async fn stage4_save(data: String) -> String {
        println!("    阶段4: 保存数据 {}", data);
        sleep(Duration::from_millis(120)).await;
        format!("已保存_{}", data)
    }
    
    // 流水线处理函数
    async fn process_pipeline(id: u32) -> Result<String, String> {
        let data = stage1_fetch(id).await;
        let transformed = stage2_transform(data).await;
        let validated = stage3_validate(transformed).await?;
        let saved = stage4_save(validated).await;
        Ok(saved)
    }
    
    // 并发处理多个项目
    println!("开始流水线处理:");
    
    let start = Instant::now();
    
    // 顺序处理
    println!("  顺序处理:");
    for id in 1..=4 {
        match process_pipeline(id).await {
            Ok(result) => println!("    成功: {}", result),
            Err(e) => println!("    失败: {}", e),
        }
    }
    let sequential_time = start.elapsed();
    
    // 并发处理
    println!("  并发处理:");
    let start = Instant::now();
    
    let tasks: Vec<_> = (5..=8).map(|id| {
        tokio::spawn(async move {
            process_pipeline(id).await
        })
    }).collect();
    
    let results = futures::future::join_all(tasks).await;
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(data)) => println!("    任务{}成功: {}", i + 5, data),
            Ok(Err(e)) => println!("    任务{}失败: {}", i + 5, e),
            Err(e) => println!("    任务{}执行错误: {}", i + 5, e),
        }
    }
    
    let concurrent_time = start.elapsed();
    
    println!("  性能对比:");
    println!("    顺序处理: {:?}", sequential_time);
    println!("    并发处理: {:?}", concurrent_time);
    println!("    性能提升: {:.2}x", 
             sequential_time.as_millis() as f64 / concurrent_time.as_millis() as f64);
    
    println!();
}

// 异步监控和指标收集
async fn async_monitoring_pattern() {
    println!("=== 异步监控和指标收集 ===\n");
    
    // 指标收集器
    #[derive(Debug, Clone)]
    struct Metrics {
        requests_total: Arc<Mutex<u64>>,
        requests_success: Arc<Mutex<u64>>,
        requests_error: Arc<Mutex<u64>>,
        response_times: Arc<Mutex<Vec<Duration>>>,
    }
    
    impl Metrics {
        fn new() -> Self {
            Self {
                requests_total: Arc::new(Mutex::new(0)),
                requests_success: Arc::new(Mutex::new(0)),
                requests_error: Arc::new(Mutex::new(0)),
                response_times: Arc::new(Mutex::new(Vec::new())),
            }
        }
        
        fn record_request(&self, duration: Duration, success: bool) {
            {
                let mut total = self.requests_total.lock().unwrap();
                *total += 1;
            }
            
            if success {
                let mut success_count = self.requests_success.lock().unwrap();
                *success_count += 1;
            } else {
                let mut error_count = self.requests_error.lock().unwrap();
                *error_count += 1;
            }
            
            {
                let mut times = self.response_times.lock().unwrap();
                times.push(duration);
                // 保持最近100个记录
                if times.len() > 100 {
                    times.remove(0);
                }
            }
        }
        
        fn get_stats(&self) -> (u64, u64, u64, f64) {
            let total = *self.requests_total.lock().unwrap();
            let success = *self.requests_success.lock().unwrap();
            let error = *self.requests_error.lock().unwrap();
            
            let times = self.response_times.lock().unwrap();
            let avg_time = if times.is_empty() {
                0.0
            } else {
                times.iter().map(|d| d.as_millis() as f64).sum::<f64>() / times.len() as f64
            };
            
            (total, success, error, avg_time)
        }
    }
    
    // 模拟服务
    async fn simulate_service(id: u32, metrics: Arc<Metrics>) {
        let start = Instant::now();
        
        // 模拟不同的处理时间和成功率
        let delay = 50 + (id % 5) * 30;
        sleep(Duration::from_millis(delay as u64)).await;
        
        let success = id % 7 != 0; // 大约85%的成功率
        let duration = start.elapsed();
        
        metrics.record_request(duration, success);
        
        if success {
            println!("    请求{}成功 (耗时: {:?})", id, duration);
        } else {
            println!("    请求{}失败 (耗时: {:?})", id, duration);
        }
    }
    
    let metrics = Arc::new(Metrics::new());
    
    // 启动监控任务
    let monitor_metrics = metrics.clone();
    let monitor_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_millis(1000));
        
        for _ in 0..5 {
            interval.tick().await;
            let (total, success, error, avg_time) = monitor_metrics.get_stats();
            let success_rate = if total > 0 { 
                (success as f64 / total as f64) * 100.0 
            } else { 
                0.0 
            };
            
            println!("  📊 监控报告:");
            println!("    总请求: {}, 成功: {}, 失败: {}", total, success, error);
            println!("    成功率: {:.1}%, 平均响应时间: {:.1}ms", success_rate, avg_time);
        }
    });
    
    // 模拟并发请求
    let request_tasks: Vec<_> = (1..=20).map(|id| {
        let metrics = metrics.clone();
        tokio::spawn(async move {
            simulate_service(id, metrics).await;
        })
    }).collect();
    
    // 等待所有任务完成
    let _ = tokio::join!(
        futures::future::join_all(request_tasks),
        monitor_task
    );
    
    // 最终统计
    let (total, success, error, avg_time) = metrics.get_stats();
    println!("  📈 最终统计:");
    println!("    总请求: {}, 成功: {}, 失败: {}", total, success, error);
    println!("    成功率: {:.1}%, 平均响应时间: {:.1}ms", 
             (success as f64 / total as f64) * 100.0, avg_time);
    
    println!();
}

#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程示例10: 高级模式 ===\n");
    
    // 1. 异步资源池模式
    async_resource_pool_pattern().await;
    
    // 2. 异步缓存模式
    async_cache_pattern().await;
    
    // 3. 异步工作队列模式
    async_work_queue_pattern().await;
    
    // 4. 异步状态机模式
    async_state_machine_pattern().await;
    
    // 5. 异步流水线模式
    async_pipeline_pattern().await;
    
    // 6. 异步监控和指标收集
    async_monitoring_pattern().await;
    
    println!("=== 示例完成 ===");
    println!("\n🎉 恭喜！你已经完成了Rust异步编程的全部教程！");
    println!("现在你已经掌握了:");
    println!("  ✅ 异步编程基础概念");
    println!("  ✅ 并发任务处理");
    println!("  ✅ 通道通信");
    println!("  ✅ HTTP客户端");
    println!("  ✅ 文件操作");
    println!("  ✅ 错误处理");
    println!("  ✅ 流处理");
    println!("  ✅ select!宏");
    println!("  ✅ 高级异步模式");
    println!("\n继续探索Rust异步编程的更多可能性吧！");
}

/*
运行这个示例：
cargo run --bin example_10_advanced_patterns

关键学习点：
1. 资源池模式 - 管理有限的异步资源
2. 缓存模式 - 避免重复的异步计算
3. 工作队列模式 - 异步任务调度和处理
4. 状态机模式 - 管理复杂的异步状态转换
5. 流水线模式 - 异步数据处理管道
6. 监控模式 - 异步系统的指标收集

高级模式特点：
- 资源管理：合理分配和回收异步资源
- 性能优化：通过缓存和池化提高效率
- 可扩展性：支持动态调整和负载均衡
- 可观测性：提供监控和调试能力
- 容错性：优雅处理错误和异常情况

实际应用：
- Web服务器：连接池、请求处理、监控
- 数据处理：ETL管道、流处理、批处理
- 微服务：服务发现、负载均衡、熔断
- 实时系统：事件处理、状态管理、通知

最佳实践：
- 合理设计异步接口
- 注意资源生命周期管理
- 实现适当的背压机制
- 提供可观测性和调试能力
- 考虑错误恢复和降级策略
- 进行性能测试和优化

这些模式可以组合使用，构建复杂而高效的异步系统。
*/ 