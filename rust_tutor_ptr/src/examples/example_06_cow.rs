/*
 * Rust智能指针教程 - 例子6: Cow<T> (Clone on Write)
 * 
 * Cow<T> 是写时克隆智能指针，提供延迟克隆的功能
 * 主要特点：
 * 1. 可以持有借用的数据或拥有的数据
 * 2. 只有在需要修改时才进行克隆
 * 3. 优化内存使用，避免不必要的克隆
 * 4. 常用于字符串处理和数据转换
 */

use std::borrow::Cow;
use std::collections::HashMap;

// 定义一个配置管理器，演示Cow在配置处理中的应用
#[derive(Debug)]
struct ConfigManager {
    defaults: HashMap<String, String>,
    overrides: HashMap<String, String>,
}

impl ConfigManager {
    fn new() -> Self {
        let mut defaults = HashMap::new();
        defaults.insert("host".to_string(), "localhost".to_string());
        defaults.insert("port".to_string(), "8080".to_string());
        defaults.insert("timeout".to_string(), "30".to_string());
        defaults.insert("debug".to_string(), "false".to_string());
        
        ConfigManager {
            defaults,
            overrides: HashMap::new(),
        }
    }
    
    fn set_override(&mut self, key: String, value: String) {
        self.overrides.insert(key, value);
    }
    
    // 使用Cow避免不必要的字符串克隆
    fn get_config(&self, key: &str) -> Cow<str> {
        if let Some(override_value) = self.overrides.get(key) {
            // 如果有覆盖值，返回借用
            Cow::Borrowed(override_value)
        } else if let Some(default_value) = self.defaults.get(key) {
            // 如果有默认值，返回借用
            Cow::Borrowed(default_value)
        } else {
            // 如果没有找到，返回拥有的默认值
            Cow::Owned(format!("unknown_{}", key))
        }
    }
    
    // 获取格式化的配置值
    fn get_formatted_config(&self, key: &str, prefix: &str) -> Cow<str> {
        let value = self.get_config(key);
        if prefix.is_empty() {
            value // 不需要修改，直接返回
        } else {
            // 需要修改，转换为拥有的数据
            Cow::Owned(format!("{}{}", prefix, value))
        }
    }
}

// 定义一个文本处理器，演示Cow在字符串处理中的应用
struct TextProcessor;

impl TextProcessor {
    // 清理文本，只有在需要时才克隆
    fn clean_text(input: &str) -> Cow<str> {
        let needs_cleaning = input.chars().any(|c| c.is_whitespace() && c != ' ');
        
        if needs_cleaning {
            // 需要清理，创建新的字符串
            let cleaned: String = input
                .chars()
                .map(|c| if c.is_whitespace() { ' ' } else { c })
                .collect();
            Cow::Owned(cleaned)
        } else {
            // 不需要清理，直接借用
            Cow::Borrowed(input)
        }
    }
    
    // 标准化文本格式
    fn normalize_text(input: &str) -> Cow<str> {
        let trimmed = input.trim();
        
        if trimmed.len() == input.len() {
            // 没有需要修剪的空白，直接借用
            Cow::Borrowed(input)
        } else {
            // 需要修剪，创建新字符串
            Cow::Owned(trimmed.to_string())
        }
    }
    
    // 添加前缀，演示条件性修改
    fn add_prefix_if_needed<'a>(input: &'a str, prefix: &'a str) -> Cow<'a, str> {
        if input.starts_with(prefix) {
            // 已经有前缀，不需要修改
            Cow::Borrowed(input)
        } else {
            // 需要添加前缀
            Cow::Owned(format!("{}{}", prefix, input))
        }
    }
}

// 定义一个路径处理器
struct PathProcessor;

impl PathProcessor {
    // 标准化路径分隔符
    fn normalize_path(path: &str) -> Cow<str> {
        if cfg!(windows) {
            if path.contains('/') {
                // Windows上需要将/替换为\
                Cow::Owned(path.replace('/', "\\"))
            } else {
                Cow::Borrowed(path)
            }
        } else {
            if path.contains('\\') {
                // Unix上需要将\替换为/
                Cow::Owned(path.replace('\\', "/"))
            } else {
                Cow::Borrowed(path)
            }
        }
    }
    
    // 确保路径以分隔符结尾
    fn ensure_trailing_separator(path: &str) -> Cow<str> {
        let separator = if cfg!(windows) { "\\" } else { "/" };
        
        if path.ends_with(separator) {
            Cow::Borrowed(path)
        } else {
            Cow::Owned(format!("{}{}", path, separator))
        }
    }
}

// 定义一个数据转换器，演示Cow在数据处理中的应用
struct DataConverter;

impl DataConverter {
    // 转换数字列表为字符串，只有在需要时才分配新内存
    fn numbers_to_string(numbers: &[i32]) -> Cow<str> {
        // 检查是否所有数字都是单位数
        if numbers.iter().all(|&n| n >= 0 && n <= 9) {
            // 可以高效地转换为字符串
            let result: String = numbers.iter().map(|&n| char::from(b'0' + n as u8)).collect();
            Cow::Owned(result)
        } else {
            // 需要更复杂的格式化
            let result = numbers.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",");
            Cow::Owned(result)
        }
    }
    
    // 格式化用户名，只有在需要时才修改
    fn format_username(username: &str) -> Cow<str> {
        if username.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            // 用户名已经是标准格式
            Cow::Borrowed(username)
        } else {
            // 需要标准化
            let formatted: String = username
                .chars()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        c.to_ascii_lowercase()
                    } else if c.is_ascii_digit() {
                        c
                    } else {
                        '_'
                    }
                })
                .collect();
            Cow::Owned(formatted)
        }
    }
}

fn main() {
    println!("=== Rust智能指针教程 - Cow<T> ===\n");
    
    // 1. 基本的Cow使用
    println!("1. 基本Cow使用:");
    
    // 创建借用的Cow
    let borrowed_cow: Cow<str> = Cow::Borrowed("Hello, Cow!");
    println!("借用的Cow: {}", borrowed_cow);
    println!("是否拥有数据: {}", matches!(borrowed_cow, Cow::Owned(_)));
    
    // 创建拥有的Cow
    let owned_cow: Cow<str> = Cow::Owned("Hello, Owned Cow!".to_string());
    println!("拥有的Cow: {}", owned_cow);
    println!("是否拥有数据: {}", matches!(owned_cow, Cow::Owned(_)));
    
    // 转换为拥有的数据
    let mut mutable_cow = borrowed_cow.clone();
    mutable_cow.to_mut().push_str(" Modified!");
    println!("修改后的Cow: {}", mutable_cow);
    println!("修改后是否拥有数据: {}", matches!(mutable_cow, Cow::Owned(_)));
    println!();
    
    // 2. 配置管理器示例
    println!("2. 配置管理器示例:");
    let mut config = ConfigManager::new();
    
    // 获取默认配置（借用）
    let host = config.get_config("host");
    println!("主机配置: {} (借用: {})", host, matches!(host, Cow::Borrowed(_)));
    
    // 设置覆盖值
    config.set_override("host".to_string(), "production.example.com".to_string());
    let host_override = config.get_config("host");
    println!("覆盖后的主机配置: {} (借用: {})", host_override, matches!(host_override, Cow::Borrowed(_)));
    
    // 获取不存在的配置（拥有）
    let unknown = config.get_config("unknown_key");
    println!("未知配置: {} (借用: {})", unknown, matches!(unknown, Cow::Borrowed(_)));
    
    // 格式化配置
    let formatted = config.get_formatted_config("port", "tcp://");
    println!("格式化配置: {} (借用: {})", formatted, matches!(formatted, Cow::Borrowed(_)));
    
    let no_prefix = config.get_formatted_config("port", "");
    println!("无前缀配置: {} (借用: {})", no_prefix, matches!(no_prefix, Cow::Borrowed(_)));
    println!();
    
    // 3. 文本处理示例
    println!("3. 文本处理示例:");
    
    let clean_text = "Hello World";
    let dirty_text = "Hello\tWorld\nTest";
    
    let result1 = TextProcessor::clean_text(clean_text);
    println!("清理 '{}': {} (借用: {})", clean_text, result1, matches!(result1, Cow::Borrowed(_)));
    
    let result2 = TextProcessor::clean_text(dirty_text);
    println!("清理 '{}': {} (借用: {})", dirty_text, result2, matches!(result2, Cow::Borrowed(_)));
    
    // 文本标准化
    let normal_text = "Hello";
    let padded_text = "  Hello  ";
    
    let norm1 = TextProcessor::normalize_text(normal_text);
    println!("标准化 '{}': '{}' (借用: {})", normal_text, norm1, matches!(norm1, Cow::Borrowed(_)));
    
    let norm2 = TextProcessor::normalize_text(padded_text);
    println!("标准化 '{}': '{}' (借用: {})", padded_text, norm2, matches!(norm2, Cow::Borrowed(_)));
    
    // 添加前缀
    let with_prefix = "Mr. John";
    let without_prefix = "John";
    
    let prefix1 = TextProcessor::add_prefix_if_needed(with_prefix, "Mr. ");
    println!("添加前缀 '{}': '{}' (借用: {})", with_prefix, prefix1, matches!(prefix1, Cow::Borrowed(_)));
    
    let prefix2 = TextProcessor::add_prefix_if_needed(without_prefix, "Mr. ");
    println!("添加前缀 '{}': '{}' (借用: {})", without_prefix, prefix2, matches!(prefix2, Cow::Borrowed(_)));
    println!();
    
    // 4. 路径处理示例
    println!("4. 路径处理示例:");
    
    let unix_path = "/home/user/documents";
    let windows_path = "C:\\Users\\User\\Documents";
    let mixed_path = "/home/user\\documents";
    
    let norm_unix = PathProcessor::normalize_path(unix_path);
    println!("标准化路径 '{}': '{}' (借用: {})", unix_path, norm_unix, matches!(norm_unix, Cow::Borrowed(_)));
    
    let norm_mixed = PathProcessor::normalize_path(mixed_path);
    println!("标准化路径 '{}': '{}' (借用: {})", mixed_path, norm_mixed, matches!(norm_mixed, Cow::Borrowed(_)));
    
    // 确保尾部分隔符
    let path_with_sep = "/home/user/";
    let path_without_sep = "/home/user";
    
    let sep1 = PathProcessor::ensure_trailing_separator(path_with_sep);
    println!("确保分隔符 '{}': '{}' (借用: {})", path_with_sep, sep1, matches!(sep1, Cow::Borrowed(_)));
    
    let sep2 = PathProcessor::ensure_trailing_separator(path_without_sep);
    println!("确保分隔符 '{}': '{}' (借用: {})", path_without_sep, sep2, matches!(sep2, Cow::Borrowed(_)));
    println!();
    
    // 5. 数据转换示例
    println!("5. 数据转换示例:");
    
    let single_digits = vec![1, 2, 3, 4, 5];
    let mixed_numbers = vec![10, 20, 30];
    
    let str1 = DataConverter::numbers_to_string(&single_digits);
    println!("单位数转换 {:?}: '{}' (拥有: {})", single_digits, str1, matches!(str1, Cow::Owned(_)));
    
    let str2 = DataConverter::numbers_to_string(&mixed_numbers);
    println!("混合数字转换 {:?}: '{}' (拥有: {})", mixed_numbers, str2, matches!(str2, Cow::Owned(_)));
    
    // 用户名格式化
    let good_username = "john_doe123";
    let bad_username = "John Doe!";
    
    let user1 = DataConverter::format_username(good_username);
    println!("格式化用户名 '{}': '{}' (借用: {})", good_username, user1, matches!(user1, Cow::Borrowed(_)));
    
    let user2 = DataConverter::format_username(bad_username);
    println!("格式化用户名 '{}': '{}' (借用: {})", bad_username, user2, matches!(user2, Cow::Borrowed(_)));
    println!();
    
    // 6. Cow的方法演示
    println!("6. Cow的方法演示:");
    
    let mut cow = Cow::Borrowed("original");
    println!("原始Cow: {} (借用: {})", cow, matches!(cow, Cow::Borrowed(_)));
    
    // into_owned() 转换为拥有的数据
    let owned = cow.clone().into_owned();
    println!("转换为拥有: {} (类型: String)", owned);
    
    // to_mut() 获取可变引用（会触发克隆如果是借用的）
    let mutable_ref = cow.to_mut();
    mutable_ref.push_str(" modified");
    println!("修改后的Cow: {} (借用: {})", cow, matches!(cow, Cow::Borrowed(_)));
    
    // 7. 性能对比演示
    println!("\n7. 性能对比演示:");
    demonstrate_cow_performance();
    
    println!("\n=== Cow教程完成 ===");
    println!("Cow的主要优势：");
    println!("1. 避免不必要的克隆，提高性能");
    println!("2. 统一处理借用和拥有的数据");
    println!("3. 延迟克隆，只在需要修改时才分配内存");
    println!("4. 在API设计中提供灵活性");
}

// 演示Cow的性能优势
fn demonstrate_cow_performance() {
    let data = vec!["hello", "world", "rust", "programming"];
    
    // 使用Cow的版本 - 只有在需要修改时才克隆
    fn process_with_cow(input: &str) -> Cow<str> {
        if input.len() > 5 {
            Cow::Owned(input.to_uppercase())
        } else {
            Cow::Borrowed(input)
        }
    }
    
    // 不使用Cow的版本 - 总是克隆
    fn process_without_cow(input: &str) -> String {
        if input.len() > 5 {
            input.to_uppercase()
        } else {
            input.to_string() // 不必要的克隆
        }
    }
    
    println!("使用Cow处理数据:");
    for item in &data {
        let result = process_with_cow(item);
        println!("  '{}' -> '{}' (借用: {})", item, result, matches!(result, Cow::Borrowed(_)));
    }
    
    println!("不使用Cow处理数据 (总是分配新内存):");
    for item in &data {
        let result = process_without_cow(item);
        println!("  '{}' -> '{}' (总是拥有)", item, result);
    }
}

// 辅助函数：演示Cow在函数参数中的使用
fn print_message(message: Cow<str>) {
    println!("消息: {}", message);
    // 函数可以接受借用或拥有的字符串
}

// 演示Cow在结构体中的使用
#[derive(Debug)]
struct Document<'a> {
    title: Cow<'a, str>,
    content: Cow<'a, str>,
}

impl<'a> Document<'a> {
    fn new(title: &'a str, content: &'a str) -> Self {
        Document {
            title: Cow::Borrowed(title),
            content: Cow::Borrowed(content),
        }
    }
    
    fn set_title(&mut self, title: String) {
        self.title = Cow::Owned(title);
    }
    
    fn append_content(&mut self, additional: &str) {
        let current = self.content.to_mut();
        current.push_str(additional);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cow_basic_operations() {
        let borrowed = Cow::Borrowed("test");
        let owned: Cow<str> = Cow::Owned("test".to_string());
        
        assert_eq!(borrowed, owned);
        assert!(matches!(borrowed, Cow::Borrowed(_)));
        assert!(matches!(owned, Cow::Owned(_)));
    }
    
    #[test]
    fn test_cow_to_mut() {
        let mut cow = Cow::Borrowed("test");
        assert!(matches!(cow, Cow::Borrowed(_)));
        
        cow.to_mut().push_str("ing");
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(cow, "testing");
    }
    
    #[test]
    fn test_text_processor() {
        let clean = TextProcessor::clean_text("hello world");
        assert!(matches!(clean, Cow::Borrowed(_)));
        
        let dirty = TextProcessor::clean_text("hello\tworld");
        assert!(matches!(dirty, Cow::Owned(_)));
    }
    
    #[test]
    fn test_config_manager() {
        let config = ConfigManager::new();
        let host = config.get_config("host");
        assert!(matches!(host, Cow::Borrowed(_)));
        
        let unknown = config.get_config("unknown");
        assert!(matches!(unknown, Cow::Owned(_)));
    }
    
    #[test]
    fn test_document() {
        let mut doc = Document::new("Title", "Content");
        assert!(matches!(doc.title, Cow::Borrowed(_)));
        assert!(matches!(doc.content, Cow::Borrowed(_)));
        
        doc.set_title("New Title".to_string());
        assert!(matches!(doc.title, Cow::Owned(_)));
        
        doc.append_content(" More");
        assert!(matches!(doc.content, Cow::Owned(_)));
    }
} 