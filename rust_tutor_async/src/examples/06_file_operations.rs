// 示例6: 异步文件操作
// 这个示例展示如何使用tokio进行异步文件读写操作

use std::path::Path;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};
use std::time::Instant;
use serde::{Serialize, Deserialize};

// 用于演示的数据结构
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

// 基本的文件写入
async fn basic_file_write() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 基本文件写入 ===\n");
    
    let content = "这是一个异步写入的文件内容\n这是第二行\n这是第三行\n";
    
    println!("写入内容到文件: basic_output.txt");
    
    // 创建文件并写入内容
    let mut file = File::create("basic_output.txt").await?;
    file.write_all(content.as_bytes()).await?;
    
    // 确保数据被写入磁盘
    file.sync_all().await?;
    
    println!("文件写入完成");
    println!("写入内容:\n{}", content);
    
    Ok(())
}

// 基本的文件读取
async fn basic_file_read() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 基本文件读取 ===\n");
    
    println!("读取文件: basic_output.txt");
    
    // 读取整个文件内容
    let content = tokio::fs::read_to_string("basic_output.txt").await?;
    
    println!("文件内容:");
    println!("{}", content);
    
    // 也可以读取为字节
    let bytes = tokio::fs::read("basic_output.txt").await?;
    println!("文件大小: {} 字节\n", bytes.len());
    
    Ok(())
}

// 逐行读取文件
async fn line_by_line_read() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 逐行读取文件 ===\n");
    
    // 首先创建一个多行文件
    let lines = vec![
        "第一行内容",
        "第二行内容", 
        "第三行内容",
        "第四行内容",
        "第五行内容",
    ];
    
    let content = lines.join("\n");
    tokio::fs::write("lines.txt", content).await?;
    
    println!("逐行读取文件: lines.txt");
    
    // 使用BufReader逐行读取
    let file = File::open("lines.txt").await?;
    let reader = BufReader::new(file);
    let mut lines_read = reader.lines();
    
    let mut line_number = 1;
    while let Some(line) = lines_read.next_line().await? {
        println!("第{}行: {}", line_number, line);
        line_number += 1;
    }
    
    println!();
    
    Ok(())
}

// 缓冲写入
async fn buffered_write() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 缓冲写入 ===\n");
    
    println!("使用缓冲写入大量数据");
    
    let file = File::create("buffered_output.txt").await?;
    let mut writer = BufWriter::new(file);
    
    // 写入大量数据
    for i in 1..=1000 {
        let line = format!("这是第{}行数据\n", i);
        writer.write_all(line.as_bytes()).await?;
        
        if i % 100 == 0 {
            println!("已写入{}行", i);
        }
    }
    
    // 刷新缓冲区
    writer.flush().await?;
    
    println!("缓冲写入完成\n");
    
    Ok(())
}

// 并发文件操作
async fn concurrent_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 并发文件操作 ===\n");
    
    let start = Instant::now();
    
    // 创建多个文件写入任务
    let write_tasks = (1..=5).map(|i| {
        tokio::spawn(async move {
            let filename = format!("concurrent_file_{}.txt", i);
            let content = format!("这是并发文件{}的内容\n", i);
            
            println!("开始写入文件: {}", filename);
            
            // 模拟一些处理时间
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            
            match tokio::fs::write(&filename, content).await {
                Ok(_) => {
                    println!("文件{}写入完成", filename);
                    Ok(filename)
                }
                Err(e) => {
                    println!("文件{}写入失败: {}", filename, e);
                    Err(e)
                }
            }
        })
    });
    
    // 等待所有写入任务完成
    let write_results = futures::future::join_all(write_tasks).await;
    
    // 收集成功写入的文件名
    let mut successful_files = Vec::new();
    for result in write_results {
        match result {
            Ok(Ok(filename)) => successful_files.push(filename),
            Ok(Err(e)) => println!("写入任务失败: {}", e),
            Err(e) => println!("任务执行失败: {}", e),
        }
    }
    
    println!("所有写入任务完成，成功写入{}个文件", successful_files.len());
    
    // 并发读取所有文件
    let read_tasks = successful_files.into_iter().map(|filename| {
        tokio::spawn(async move {
            println!("开始读取文件: {}", filename);
            match tokio::fs::read_to_string(&filename).await {
                Ok(content) => {
                    println!("文件{}读取完成，内容长度: {}", filename, content.len());
                    Ok(content)
                }
                Err(e) => {
                    println!("文件{}读取失败: {}", filename, e);
                    Err(e)
                }
            }
        })
    });
    
    let read_results = futures::future::join_all(read_tasks).await;
    
    let mut successful_reads = 0;
    for result in read_results {
        match result {
            Ok(Ok(_)) => successful_reads += 1,
            Ok(Err(e)) => println!("读取任务失败: {}", e),
            Err(e) => println!("任务执行失败: {}", e),
        }
    }
    
    let elapsed = start.elapsed();
    println!("并发操作完成，成功读取{}个文件，总耗时: {:?}\n", successful_reads, elapsed);
    
    Ok(())
}

// JSON文件操作
async fn json_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON文件操作 ===\n");
    
    // 创建一些示例数据
    let people = vec![
        Person {
            name: "张三".to_string(),
            age: 25,
            email: "zhangsan@example.com".to_string(),
        },
        Person {
            name: "李四".to_string(),
            age: 30,
            email: "lisi@example.com".to_string(),
        },
        Person {
            name: "王五".to_string(),
            age: 28,
            email: "wangwu@example.com".to_string(),
        },
    ];
    
    println!("写入JSON数据到文件");
    
    // 序列化为JSON并写入文件
    let json_content = serde_json::to_string_pretty(&people)?;
    tokio::fs::write("people.json", json_content).await?;
    
    println!("JSON文件写入完成");
    
    // 从文件读取JSON数据
    println!("从文件读取JSON数据");
    let json_content = tokio::fs::read_to_string("people.json").await?;
    let loaded_people: Vec<Person> = serde_json::from_str(&json_content)?;
    
    println!("成功读取{}个人员记录:", loaded_people.len());
    for (i, person) in loaded_people.iter().enumerate() {
        println!("  {}: {} ({}岁) - {}", i + 1, person.name, person.age, person.email);
    }
    
    println!();
    
    Ok(())
}

// 文件追加操作
async fn file_append_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 文件追加操作 ===\n");
    
    let log_file = "app.log";
    
    // 创建初始日志文件
    tokio::fs::write(log_file, "应用程序启动\n").await?;
    
    println!("创建日志文件: {}", log_file);
    
    // 追加多条日志
    let log_entries = vec![
        "用户登录: user123",
        "执行操作: 创建文档",
        "执行操作: 保存文档", 
        "用户登出: user123",
        "应用程序关闭",
    ];
    
    for entry in log_entries {
        // 以追加模式打开文件
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .await?;
        
        let log_line = format!("[{}] {}\n", 
                              chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), 
                              entry);
        
        file.write_all(log_line.as_bytes()).await?;
        println!("追加日志: {}", entry);
        
        // 模拟一些时间间隔
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    
    // 读取完整的日志文件
    println!("\n完整的日志文件内容:");
    let log_content = tokio::fs::read_to_string(log_file).await?;
    println!("{}", log_content);
    
    Ok(())
}

// 文件元数据和目录操作
async fn file_metadata_and_directory() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 文件元数据和目录操作 ===\n");
    
    // 创建目录
    let dir_path = "test_directory";
    tokio::fs::create_dir_all(dir_path).await?;
    println!("创建目录: {}", dir_path);
    
    // 在目录中创建文件
    let file_path = format!("{}/test_file.txt", dir_path);
    tokio::fs::write(&file_path, "测试文件内容").await?;
    println!("创建文件: {}", file_path);
    
    // 获取文件元数据
    let metadata = tokio::fs::metadata(&file_path).await?;
    println!("文件元数据:");
    println!("  文件大小: {} 字节", metadata.len());
    println!("  是否为文件: {}", metadata.is_file());
    println!("  是否为目录: {}", metadata.is_dir());
    
    if let Ok(modified) = metadata.modified() {
        println!("  修改时间: {:?}", modified);
    }
    
    // 列出目录内容
    println!("\n目录内容:");
    let mut entries = tokio::fs::read_dir(dir_path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let metadata = entry.metadata().await?;
        
        let file_type = if metadata.is_file() { "文件" } else { "目录" };
        println!("  {} - {} ({} 字节)", 
                path.file_name().unwrap().to_string_lossy(),
                file_type,
                metadata.len());
    }
    
    // 检查文件是否存在
    let exists = Path::new(&file_path).exists();
    println!("\n文件是否存在: {}", exists);
    
    println!();
    
    Ok(())
}

// 清理测试文件
async fn cleanup_test_files() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 清理测试文件 ===\n");
    
    let files_to_remove = vec![
        "basic_output.txt",
        "lines.txt", 
        "buffered_output.txt",
        "people.json",
        "app.log",
    ];
    
    for file in files_to_remove {
        if Path::new(file).exists() {
            match tokio::fs::remove_file(file).await {
                Ok(_) => println!("删除文件: {}", file),
                Err(e) => println!("删除文件{}失败: {}", file, e),
            }
        }
    }
    
    // 删除并发创建的文件
    for i in 1..=5 {
        let filename = format!("concurrent_file_{}.txt", i);
        if Path::new(&filename).exists() {
            match tokio::fs::remove_file(&filename).await {
                Ok(_) => println!("删除文件: {}", filename),
                Err(e) => println!("删除文件{}失败: {}", filename, e),
            }
        }
    }
    
    // 删除测试目录
    if Path::new("test_directory").exists() {
        match tokio::fs::remove_dir_all("test_directory").await {
            Ok(_) => println!("删除目录: test_directory"),
            Err(e) => println!("删除目录失败: {}", e),
        }
    }
    
    println!("清理完成\n");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust 异步编程示例6: 文件操作 ===\n");
    
    // 1. 基本文件写入
    if let Err(e) = basic_file_write().await {
        println!("基本文件写入失败: {}\n", e);
    }
    
    // 2. 基本文件读取
    if let Err(e) = basic_file_read().await {
        println!("基本文件读取失败: {}\n", e);
    }
    
    // 3. 逐行读取
    if let Err(e) = line_by_line_read().await {
        println!("逐行读取失败: {}\n", e);
    }
    
    // 4. 缓冲写入
    if let Err(e) = buffered_write().await {
        println!("缓冲写入失败: {}\n", e);
    }
    
    // 5. 并发文件操作
    if let Err(e) = concurrent_file_operations().await {
        println!("并发文件操作失败: {}\n", e);
    }
    
    // 6. JSON文件操作
    if let Err(e) = json_file_operations().await {
        println!("JSON文件操作失败: {}\n", e);
    }
    
    // 7. 文件追加操作
    if let Err(e) = file_append_operations().await {
        println!("文件追加操作失败: {}\n", e);
    }
    
    // 8. 文件元数据和目录操作
    if let Err(e) = file_metadata_and_directory().await {
        println!("文件元数据和目录操作失败: {}\n", e);
    }
    
    // 9. 清理测试文件
    if let Err(e) = cleanup_test_files().await {
        println!("清理测试文件失败: {}\n", e);
    }
    
    println!("=== 示例完成 ===");
    
    Ok(())
}

/*
运行这个示例：
cargo run --bin example_06_file_operations

关键学习点：
1. tokio::fs - 异步文件系统操作
2. AsyncReadExt/AsyncWriteExt - 异步读写trait
3. BufReader/BufWriter - 缓冲读写提高性能
4. 并发文件操作提高I/O效率
5. JSON序列化/反序列化与文件操作结合
6. 文件追加和日志记录
7. 文件元数据和目录操作
8. 错误处理和资源清理

最佳实践：
- 使用缓冲读写处理大文件
- 并发处理多个文件操作
- 正确处理文件I/O错误
- 及时关闭文件句柄
- 使用适当的文件打开模式
- 考虑文件锁定和并发访问

性能提示：
- 对于大文件使用流式处理
- 批量操作比单个操作更高效
- 缓冲I/O减少系统调用
- 并发操作可以提高整体吞吐量

注意事项：
- 异步文件操作不会阻塞线程
- 错误处理对文件操作尤其重要
- 考虑文件权限和磁盘空间
- 在程序结束前清理临时文件
*/ 