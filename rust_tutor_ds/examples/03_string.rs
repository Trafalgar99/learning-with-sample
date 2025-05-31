/**
 * Rust基础数据结构教程 - 字符串类型
 * 
 * Rust中有两种主要的字符串类型：
 * - &str：字符串切片，不可变，通常用于字符串字面量
 * - String：可变的字符串类型，分配在堆上
 */

fn main() {
    println!("=== Rust 字符串类型教程 ===\n");

    // 1. 字符串字面量 &str
    println!("1. 字符串字面量 (&str)：");
    let string_literal: &str = "Hello, World!";
    let chinese_string: &str = "你好，世界！";
    let multiline_string: &str = "这是一个
多行
字符串";
    
    println!("   英文字符串: {}", string_literal);
    println!("   中文字符串: {}", chinese_string);
    println!("   多行字符串: {}", multiline_string);
    
    // 原始字符串字面量（不需要转义）
    let raw_string = r"这是一个原始字符串，可以包含 \ 和 " 而不需要转义";
    let raw_multiline = r#"
        这是一个带有"引号"的
        多行原始字符串
    "#;
    println!("   原始字符串: {}", raw_string);
    println!("   原始多行字符串: {}", raw_multiline);

    // 2. String类型
    println!("\n2. String类型：");
    
    // 创建String的方式
    let mut owned_string = String::new();  // 创建空字符串
    let from_literal = String::from("从字面量创建");
    let with_to_string = "使用to_string方法".to_string();
    let with_capacity = String::with_capacity(50);  // 预分配容量
    
    println!("   空字符串: '{}'", owned_string);
    println!("   从字面量创建: {}", from_literal);
    println!("   使用to_string: {}", with_to_string);
    println!("   预分配容量的字符串: '{}' (容量: {})", with_capacity, with_capacity.capacity());

    // 3. 字符串操作
    println!("\n3. 字符串操作：");
    
    // 添加内容
    owned_string.push_str("Hello");  // 添加字符串切片
    owned_string.push(' ');          // 添加单个字符
    owned_string.push_str("Rust");
    println!("   添加内容后: {}", owned_string);
    
    // 字符串连接
    let hello = String::from("Hello");
    let world = String::from(" World");
    let combined = hello + &world;  // hello被移动，world被借用
    println!("   字符串连接: {}", combined);
    // println!("{}", hello);  // 这行会报错，因为hello已被移动
    
    // 使用format!宏连接字符串
    let name = "Alice";
    let age = 30;
    let formatted = format!("姓名: {}, 年龄: {}", name, age);
    println!("   格式化字符串: {}", formatted);

    // 4. 字符串长度和容量
    println!("\n4. 字符串长度和容量：");
    let sample = String::from("Hello, 世界!");
    println!("   字符串: {}", sample);
    println!("   字符数量: {}", sample.chars().count());  // Unicode字符数
    println!("   字节长度: {}", sample.len());           // UTF-8字节数
    println!("   容量: {}", sample.capacity());

    // 5. 字符串遍历
    println!("\n5. 字符串遍历：");
    let text = "Hello你好";
    
    // 按字符遍历
    print!("   按字符遍历: ");
    for char in text.chars() {
        print!("'{}' ", char);
    }
    println!();
    
    // 按字节遍历
    print!("   按字节遍历: ");
    for byte in text.bytes() {
        print!("{} ", byte);
    }
    println!();
    
    // 按字符获取索引
    println!("   字符索引:");
    for (i, char) in text.char_indices() {
        println!("     索引 {}: '{}'", i, char);
    }

    // 6. 字符串切片
    println!("\n6. 字符串切片：");
    let s = String::from("Hello World");
    let hello = &s[0..5];    // 获取前5个字节
    let world = &s[6..];     // 从索引6到结尾
    println!("   原字符串: {}", s);
    println!("   前5个字节: {}", hello);
    println!("   从索引6开始: {}", world);
    
    // 注意：对于包含非ASCII字符的字符串，字节索引可能不安全
    let chinese = "你好世界";
    // let bad_slice = &chinese[0..1];  // 这会panic！
    // 安全的方式是使用chars().take()等方法

    // 7. 字符串方法
    println!("\n7. 字符串方法：");
    let demo_text = "  Hello, Rust World!  ";
    println!("   原字符串: '{}'", demo_text);
    
    // 大小写转换
    println!("   转大写: {}", demo_text.to_uppercase());
    println!("   转小写: {}", demo_text.to_lowercase());
    
    // 去除空白
    println!("   去除首尾空白: '{}'", demo_text.trim());
    println!("   去除开头空白: '{}'", demo_text.trim_start());
    println!("   去除结尾空白: '{}'", demo_text.trim_end());
    
    // 查找和替换
    println!("   是否包含'Rust': {}", demo_text.contains("Rust"));
    println!("   是否以'  Hello'开头: {}", demo_text.starts_with("  Hello"));
    println!("   是否以'!  '结尾: {}", demo_text.ends_with("!  "));
    println!("   替换'World'为'Universe': {}", demo_text.replace("World", "Universe"));

    // 8. 字符串分割
    println!("\n8. 字符串分割：");
    let data = "apple,banana,orange,grape";
    println!("   原字符串: {}", data);
    
    // split方法返回迭代器
    let fruits: Vec<&str> = data.split(',').collect();
    println!("   按逗号分割: {:?}", fruits);
    
    // split_whitespace分割空白字符
    let sentence = "Hello   Rust    World";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("   按空白分割: {:?}", words);
    
    // lines分割行
    let multiline_text = "第一行\n第二行\n第三行";
    let lines: Vec<&str> = multiline_text.lines().collect();
    println!("   按行分割: {:?}", lines);

    // 9. 字符串和数字转换
    println!("\n9. 字符串和数字转换：");
    
    // 数字转字符串
    let number = 42;
    let number_str = number.to_string();
    let formatted_number = format!("数字是: {}", number);
    println!("   数字转字符串: {}", number_str);
    println!("   格式化数字: {}", formatted_number);
    
    // 字符串转数字
    let str_number = "123";
    match str_number.parse::<i32>() {
        Ok(num) => println!("   字符串转数字: {} -> {}", str_number, num),
        Err(e) => println!("   转换失败: {}", e),
    }
    
    let invalid_str = "abc";
    match invalid_str.parse::<i32>() {
        Ok(num) => println!("   意外成功: {}", num),
        Err(e) => println!("   转换失败（预期的）: '{}' -> {}", invalid_str, e),
    }

    // 10. 字符串的所有权
    println!("\n10. 字符串的所有权：");
    
    // &str -> String
    let string_slice: &str = "这是字符串切片";
    let owned_string: String = string_slice.to_string();
    println!("   &str -> String: {}", owned_string);
    
    // String -> &str (借用)
    let string_ref: &str = &owned_string;
    println!("   String -> &str: {}", string_ref);
    
    // 函数参数
    print_str_slice("传递字符串字面量");
    print_str_slice(&owned_string);  // 传递String的引用
    
    print_string(owned_string.clone());  // 克隆String
    println!("   原String仍然有效: {}", owned_string);

    // 11. 字符串插值和格式化
    println!("\n11. 字符串插值和格式化：");
    let name = "张三";
    let score = 95.5;
    
    // 位置参数
    println!("   位置参数: {} 的分数是 {}", name, score);
    
    // 命名参数
    println!("   命名参数: {student} 的分数是 {grade}", student=name, grade=score);
    
    // 格式说明符
    println!("   格式说明符:");
    println!("     十进制: {:d}", 42);
    println!("     十六进制: {:x}", 42);
    println!("     八进制: {:o}", 42);
    println!("     二进制: {:b}", 42);
    println!("     浮点数: {:.2}", 3.14159);
    println!("     填充对齐: '{:>10}'", "右对齐");
    println!("     填充对齐: '{:<10}'", "左对齐");
    println!("     填充对齐: '{:^10}'", "居中");

    println!("\n=== 字符串类型教程结束 ===");
}

// 接受字符串切片的函数
fn print_str_slice(s: &str) {
    println!("   函数接收&str: {}", s);
}

// 接受String的函数
fn print_string(s: String) {
    println!("   函数接收String: {}", s);
} 