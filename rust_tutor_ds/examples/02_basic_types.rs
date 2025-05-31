/**
 * Rust基础数据结构教程 - 基本数据类型
 * 
 * Rust的基本数据类型包括：
 * - 整数类型：i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
 * - 浮点类型：f32, f64
 * - 布尔类型：bool
 * - 字符类型：char
 */

fn main() {
    println!("=== Rust 基本数据类型教程 ===\n");

    // 1. 整数类型
    println!("1. 整数类型：");
    
    // 有符号整数
    let small_int: i8 = -128;          // 8位有符号整数，范围：-128 到 127
    let medium_int: i16 = -32768;      // 16位有符号整数
    let normal_int: i32 = -2147483648; // 32位有符号整数（默认）
    let big_int: i64 = -9223372036854775808; // 64位有符号整数
    let huge_int: i128 = -170141183460469231731687303715884105728; // 128位有符号整数
    let pointer_size: isize = -100;    // 指针大小的有符号整数（取决于架构）
    
    println!("   有符号整数：");
    println!("     i8:    {} (范围: {} 到 {})", small_int, i8::MIN, i8::MAX);
    println!("     i16:   {} (范围: {} 到 {})", medium_int, i16::MIN, i16::MAX);
    println!("     i32:   {} (范围: {} 到 {})", normal_int, i32::MIN, i32::MAX);
    println!("     i64:   {} (范围: {} 到 {})", big_int, i64::MIN, i64::MAX);
    println!("     i128:  {} (范围: {} 到 {})", huge_int, i128::MIN, i128::MAX);
    println!("     isize: {} (当前架构的指针大小)", pointer_size);
    
    // 无符号整数
    let u_small: u8 = 255;
    let u_medium: u16 = 65535;
    let u_normal: u32 = 4294967295;
    let u_big: u64 = 18446744073709551615;
    let u_huge: u128 = 340282366920938463463374607431768211455;
    let u_pointer: usize = 100;
    
    println!("\n   无符号整数：");
    println!("     u8:    {} (范围: {} 到 {})", u_small, u8::MIN, u8::MAX);
    println!("     u16:   {} (范围: {} 到 {})", u_medium, u16::MIN, u16::MAX);
    println!("     u32:   {} (范围: {} 到 {})", u_normal, u32::MIN, u32::MAX);
    println!("     u64:   {} (范围: {} 到 {})", u_big, u64::MIN, u64::MAX);
    println!("     u128:  {} (范围: {} 到 {})", u_huge, u128::MIN, u128::MAX);
    println!("     usize: {} (当前架构的指针大小)", u_pointer);

    // 2. 整数字面量的不同表示法
    println!("\n2. 整数字面量的表示法：");
    let decimal = 98_222;      // 十进制，可用下划线分隔
    let hex = 0xff;            // 十六进制
    let octal = 0o77;          // 八进制
    let binary = 0b1111_0000;  // 二进制
    let byte = b'A';           // 字节（仅限u8）
    
    println!("   十进制: {}", decimal);
    println!("   十六进制0xff: {}", hex);
    println!("   八进制0o77: {}", octal);
    println!("   二进制0b1111_0000: {}", binary);
    println!("   字节b'A': {}", byte);

    // 3. 浮点类型
    println!("\n3. 浮点类型：");
    let single_precision: f32 = 3.14159;  // 32位浮点数
    let double_precision: f64 = 2.718281828; // 64位浮点数（默认）
    
    println!("   f32: {} (单精度浮点数)", single_precision);
    println!("   f64: {} (双精度浮点数)", double_precision);
    
    // 浮点数运算
    let sum = single_precision + double_precision as f32;
    let product = single_precision * 2.0;
    println!("   运算示例: {} + {} = {}", single_precision, double_precision as f32, sum);
    println!("   运算示例: {} * 2.0 = {}", single_precision, product);

    // 4. 布尔类型
    println!("\n4. 布尔类型：");
    let is_rust_awesome: bool = true;
    let is_learning_hard: bool = false;
    
    println!("   Rust很棒吗？ {}", is_rust_awesome);
    println!("   学习困难吗？ {}", is_learning_hard);
    
    // 布尔运算
    let logical_and = is_rust_awesome && !is_learning_hard;
    let logical_or = is_rust_awesome || is_learning_hard;
    println!("   逻辑与: {} && {} = {}", is_rust_awesome, !is_learning_hard, logical_and);
    println!("   逻辑或: {} || {} = {}", is_rust_awesome, is_learning_hard, logical_or);

    // 5. 字符类型
    println!("\n5. 字符类型：");
    let english_char: char = 'A';
    let chinese_char: char = '中';
    let emoji_char: char = '😀';
    let unicode_char: char = '\u{1F600}';  // Unicode码点
    
    println!("   英文字符: '{}'", english_char);
    println!("   中文字符: '{}'", chinese_char);
    println!("   表情符号: '{}'", emoji_char);
    println!("   Unicode字符: '{}'", unicode_char);
    
    // 字符的大小
    println!("   char类型大小: {} 字节", std::mem::size_of::<char>());

    // 6. 类型转换
    println!("\n6. 类型转换：");
    let integer = 65;
    let float_num = 3.14;
    let character = 'A';
    
    // 显式类型转换
    let int_to_float = integer as f64;
    let float_to_int = float_num as i32;
    let char_to_int = character as u8;
    
    println!("   整数转浮点: {} -> {}", integer, int_to_float);
    println!("   浮点转整数: {} -> {}", float_num, float_to_int);
    println!("   字符转整数: '{}' -> {}", character, char_to_int);

    // 7. 数值运算
    println!("\n7. 数值运算：");
    let a = 10;
    let b = 3;
    
    println!("   加法: {} + {} = {}", a, b, a + b);
    println!("   减法: {} - {} = {}", a, b, a - b);
    println!("   乘法: {} * {} = {}", a, b, a * b);
    println!("   除法: {} / {} = {}", a, b, a / b);
    println!("   取余: {} % {} = {}", a, b, a % b);
    
    // 浮点数运算
    let x = 10.0;
    let y = 3.0;
    println!("   浮点除法: {} / {} = {}", x, y, x / y);

    // 8. 常量和静态变量
    println!("\n8. 常量和静态变量：");
    const MAX_POINTS: u32 = 100_000;  // 常量，必须注明类型
    static LANGUAGE: &str = "Rust";   // 静态变量
    
    println!("   常量MAX_POINTS: {}", MAX_POINTS);
    println!("   静态变量LANGUAGE: {}", LANGUAGE);

    // 9. 变量可变性
    println!("\n9. 变量可变性：");
    let immutable_var = 5;
    let mut mutable_var = 5;
    
    println!("   不可变变量: {}", immutable_var);
    println!("   可变变量（修改前）: {}", mutable_var);
    
    mutable_var = 10;
    println!("   可变变量（修改后）: {}", mutable_var);
    
    // 变量遮蔽（shadowing）
    let shadowed_var = 5;
    println!("   遮蔽变量（第一次）: {}", shadowed_var);
    
    let shadowed_var = shadowed_var * 2;
    println!("   遮蔽变量（第二次）: {}", shadowed_var);
    
    let shadowed_var = "现在是字符串了";
    println!("   遮蔽变量（第三次）: {}", shadowed_var);

    println!("\n=== 基本数据类型教程结束 ===");
} 