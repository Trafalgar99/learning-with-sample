// 示例5: HTTP客户端
// 这个示例展示如何使用reqwest进行异步HTTP请求

use std::time::{Duration, Instant};
use tokio::time::sleep;
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 定义一些用于演示的数据结构
#[derive(Debug, Deserialize)]
struct JsonPlaceholderPost {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[derive(Debug, Serialize)]
struct NewPost {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    url: String,
    headers: HashMap<String, String>,
    origin: String,
}

// 基本的GET请求
async fn basic_get_request() -> Result<(), ReqwestError> {
    println!("=== 基本GET请求 ===\n");
    
    let client = Client::new();
    
    // 发送GET请求到httpbin.org
    println!("发送GET请求到 httpbin.org/get");
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;
    
    println!("响应状态: {}", response.status());
    println!("响应头:");
    for (key, value) in response.headers() {
        println!("  {}: {:?}", key, value);
    }
    
    // 获取响应体
    let body = response.text().await?;
    println!("响应体长度: {} 字符", body.len());
    println!("响应体前200字符: {}\n", 
             if body.len() > 200 { &body[..200] } else { &body });
    
    Ok(())
}

// JSON数据的GET请求
async fn json_get_request() -> Result<(), ReqwestError> {
    println!("=== JSON GET请求 ===\n");
    
    let client = Client::new();
    
    // 从JSONPlaceholder API获取帖子
    println!("从JSONPlaceholder获取帖子数据");
    let posts: Vec<JsonPlaceholderPost> = client
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await?
        .json()
        .await?;
    
    println!("获取到 {} 个帖子", posts.len());
    
    // 显示前3个帖子
    for (i, post) in posts.iter().take(3).enumerate() {
        println!("帖子 {}: {}", i + 1, post.title);
        println!("  用户ID: {}, 帖子ID: {}", post.user_id, post.id);
        println!("  内容: {}\n", 
                 if post.body.len() > 50 { 
                     format!("{}...", &post.body[..50]) 
                 } else { 
                     post.body.clone() 
                 });
    }
    
    Ok(())
}

// POST请求发送JSON数据
async fn json_post_request() -> Result<(), ReqwestError> {
    println!("=== JSON POST请求 ===\n");
    
    let client = Client::new();
    
    let new_post = NewPost {
        title: "我的新帖子".to_string(),
        body: "这是一个通过Rust异步HTTP客户端创建的帖子".to_string(),
        user_id: 1,
    };
    
    println!("发送POST请求创建新帖子");
    println!("请求数据: {:?}", new_post);
    
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;
    
    println!("响应状态: {}", response.status());
    
    // 解析响应
    let created_post: JsonPlaceholderPost = response.json().await?;
    println!("创建的帖子: {:?}\n", created_post);
    
    Ok(())
}

// 带有自定义头部的请求
async fn request_with_headers() -> Result<(), ReqwestError> {
    println!("=== 带自定义头部的请求 ===\n");
    
    let client = Client::new();
    
    println!("发送带自定义头部的请求");
    let response = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "Rust-Async-Tutorial/1.0")
        .header("X-Custom-Header", "Hello from Rust!")
        .header("Accept", "application/json")
        .send()
        .await?;
    
    let headers_response: HttpBinResponse = response.json().await?;
    println!("服务器看到的头部:");
    for (key, value) in &headers_response.headers {
        println!("  {}: {}", key, value);
    }
    println!();
    
    Ok(())
}

// 并发HTTP请求
async fn concurrent_requests() -> Result<(), ReqwestError> {
    println!("=== 并发HTTP请求 ===\n");
    
    let client = Client::new();
    
    // 准备多个URL
    let urls = vec![
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2", 
        "https://httpbin.org/delay/1",
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
    ];
    
    println!("发送 {} 个并发请求", urls.len());
    let start = Instant::now();
    
    // 创建所有请求的Future
    let requests: Vec<_> = urls.iter()
        .enumerate()
        .map(|(i, url)| {
            let client = client.clone();
            let url = url.to_string();
            async move {
                println!("  请求 {} 开始: {}", i + 1, url);
                let result = client.get(&url).send().await;
                match result {
                    Ok(response) => {
                        println!("  请求 {} 完成: {} (状态: {})", 
                                i + 1, url, response.status());
                        Ok(response.status().as_u16())
                    }
                    Err(e) => {
                        println!("  请求 {} 失败: {} (错误: {})", i + 1, url, e);
                        Err(e)
                    }
                }
            }
        })
        .collect();
    
    // 等待所有请求完成
    let results = futures::future::join_all(requests).await;
    
    let elapsed = start.elapsed();
    println!("所有请求完成，耗时: {:?}", elapsed);
    
    // 统计结果
    let mut success_count = 0;
    let mut error_count = 0;
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(status) => {
                success_count += 1;
                println!("  结果 {}: 成功 (状态码: {})", i + 1, status);
            }
            Err(e) => {
                error_count += 1;
                println!("  结果 {}: 失败 ({})", i + 1, e);
            }
        }
    }
    
    println!("成功: {}, 失败: {}\n", success_count, error_count);
    
    Ok(())
}

// 请求超时处理
async fn request_with_timeout() -> Result<(), ReqwestError> {
    println!("=== 请求超时处理 ===\n");
    
    // 创建带超时的客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;
    
    println!("发送会超时的请求 (超时设置: 3秒)");
    
    // 尝试请求一个会延迟5秒的端点
    match client.get("https://httpbin.org/delay/5").send().await {
        Ok(response) => {
            println!("请求成功: {}", response.status());
        }
        Err(e) => {
            if e.is_timeout() {
                println!("请求超时: {}", e);
            } else {
                println!("请求失败: {}", e);
            }
        }
    }
    
    println!("发送正常请求 (应该成功)");
    match client.get("https://httpbin.org/delay/1").send().await {
        Ok(response) => {
            println!("请求成功: {}", response.status());
        }
        Err(e) => {
            println!("请求失败: {}", e);
        }
    }
    
    println!();
    
    Ok(())
}

// 错误处理和重试机制
async fn error_handling_and_retry() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 错误处理和重试机制 ===\n");
    
    let client = Client::new();
    
    // 模拟重试逻辑
    async fn fetch_with_retry(
        client: &Client, 
        url: &str, 
        max_retries: u32
    ) -> Result<reqwest::Response, ReqwestError> {
        let mut attempts = 0;
        
        loop {
            attempts += 1;
            println!("  尝试 {}/{}: {}", attempts, max_retries + 1, url);
            
            match client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("  成功!");
                        return Ok(response);
                    } else {
                        println!("  HTTP错误: {}", response.status());
                        if attempts > max_retries {
                            return Ok(response);
                        }
                    }
                }
                Err(e) => {
                    println!("  网络错误: {}", e);
                    if attempts > max_retries {
                        return Err(e);
                    }
                }
            }
            
            // 等待后重试
            let delay = Duration::from_millis(1000 * attempts as u64);
            println!("  等待 {:?} 后重试", delay);
            sleep(delay).await;
        }
    }
    
    // 测试重试机制
    println!("测试对不存在URL的重试:");
    match fetch_with_retry(&client, "https://httpbin.org/status/500", 2).await {
        Ok(response) => println!("最终结果: {}", response.status()),
        Err(e) => println!("最终失败: {}", e),
    }
    
    println!("\n测试对正常URL的请求:");
    match fetch_with_retry(&client, "https://httpbin.org/get", 2).await {
        Ok(response) => println!("最终结果: {}", response.status()),
        Err(e) => println!("最终失败: {}", e),
    }
    
    println!();
    
    Ok(())
}

// 流式下载
async fn streaming_download() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 流式下载演示 ===\n");
    
    let client = Client::new();
    
    println!("开始流式下载...");
    let response = client
        .get("https://httpbin.org/bytes/1024")  // 下载1KB数据
        .send()
        .await?;
    
    println!("响应状态: {}", response.status());
    
    if let Some(content_length) = response.content_length() {
        println!("内容长度: {} 字节", content_length);
    }
    
    // 流式读取响应体 - 使用正确的方法
    let mut downloaded = 0;
    let bytes = response.bytes().await?;
    
    // 模拟分块处理
    let chunk_size = 256;
    for (i, chunk) in bytes.chunks(chunk_size).enumerate() {
        downloaded += chunk.len();
        println!("处理块 {}: {} 字节，总计 {} 字节", i + 1, chunk.len(), downloaded);
        
        // 模拟处理延迟
        sleep(Duration::from_millis(50)).await;
    }
    
    println!("下载完成，总计 {} 字节\n", downloaded);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust 异步编程示例5: HTTP客户端 ===\n");
    
    // 1. 基本GET请求
    if let Err(e) = basic_get_request().await {
        println!("基本GET请求失败: {}\n", e);
    }
    
    // 2. JSON GET请求
    if let Err(e) = json_get_request().await {
        println!("JSON GET请求失败: {}\n", e);
    }
    
    // 3. JSON POST请求
    if let Err(e) = json_post_request().await {
        println!("JSON POST请求失败: {}\n", e);
    }
    
    // 4. 带自定义头部的请求
    if let Err(e) = request_with_headers().await {
        println!("自定义头部请求失败: {}\n", e);
    }
    
    // 5. 并发请求
    if let Err(e) = concurrent_requests().await {
        println!("并发请求失败: {}\n", e);
    }
    
    // 6. 超时处理
    if let Err(e) = request_with_timeout().await {
        println!("超时处理失败: {}\n", e);
    }
    
    // 7. 错误处理和重试
    if let Err(e) = error_handling_and_retry().await {
        println!("错误处理和重试失败: {}\n", e);
    }
    
    // 8. 流式下载
    if let Err(e) = streaming_download().await {
        println!("流式下载失败: {}\n", e);
    }
    
    println!("=== 示例完成 ===");
    
    Ok(())
}

/*
运行这个示例：
cargo run --bin example_05_http_client

关键学习点：
1. reqwest::Client - 异步HTTP客户端
2. GET/POST请求的发送和响应处理
3. JSON序列化和反序列化
4. 自定义请求头部
5. 并发HTTP请求提高性能
6. 超时处理避免无限等待
7. 错误处理和重试机制
8. 流式下载处理大文件

最佳实践：
- 重用Client实例以获得连接池的好处
- 设置合理的超时时间
- 实现重试机制处理临时网络问题
- 使用流式处理大响应体
- 正确处理各种HTTP状态码和网络错误
- 使用并发请求提高性能，但注意不要过载服务器

注意事项：
- 网络请求可能失败，总是要处理错误
- 某些示例需要网络连接才能正常运行
- 在生产环境中要考虑速率限制和服务器负载
*/ 