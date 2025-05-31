/**
 * Rust基础数据结构教程 - 数组、切片和元组
 * 
 * 包含内容：
 * - 数组 (Array): 固定大小的同类型元素集合
 * - 切片 (Slice): 对数组或向量一部分的引用
 * - 元组 (Tuple): 固定大小的不同类型元素集合
 */

fn main() {
    println!("=== Rust 数组、切片和元组教程 ===\n");

    // ========== 数组 (Array) ==========
    println!("【第一部分：数组 (Array)】");
    
    // 1. 创建数组
    println!("\n1. 创建数组：");
    
    // 方式1：直接初始化
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("   整数数组: {:?}", numbers);
    
    // 方式2：相同值初始化
    let zeros = [0; 10];  // 创建10个0的数组
    println!("   10个0的数组: {:?}", zeros);
    
    // 方式3：不同类型的数组
    let fruits = ["苹果", "香蕉", "橙子"];
    println!("   水果数组: {:?}", fruits);
    
    let mixed_chars = ['A', '中', '😀'];
    println!("   字符数组: {:?}", mixed_chars);

    // 2. 数组访问
    println!("\n2. 数组访问：");
    println!("   第一个元素: {}", numbers[0]);
    println!("   最后一个元素: {}", numbers[4]);
    println!("   数组长度: {}", numbers.len());
    
    // 安全访问
    match numbers.get(2) {
        Some(value) => println!("   安全访问索引2: {}", value),
        None => println!("   索引2不存在"),
    }
    
    match numbers.get(10) {
        Some(value) => println!("   索引10: {}", value),
        None => println!("   索引10不存在（预期的）"),
    }

    // 3. 数组遍历
    println!("\n3. 数组遍历：");
    
    // 遍历值
    print!("   遍历值: ");
    for value in numbers.iter() {
        print!("{} ", value);
    }
    println!();
    
    // 遍历索引和值
    println!("   遍历索引和值:");
    for (index, value) in numbers.iter().enumerate() {
        println!("     索引{}: 值{}", index, value);
    }
    
    // 可变数组遍历
    let mut mutable_array = [1, 2, 3, 4, 5];
    println!("   修改前: {:?}", mutable_array);
    for item in mutable_array.iter_mut() {
        *item *= 2;  // 每个元素乘以2
    }
    println!("   修改后: {:?}", mutable_array);

    // 4. 数组方法
    println!("\n4. 数组方法：");
    let demo_array = [5, 2, 8, 1, 9];
    println!("   原数组: {:?}", demo_array);
    
    // first和last
    println!("   第一个元素: {:?}", demo_array.first());
    println!("   最后一个元素: {:?}", demo_array.last());
    
    // contains
    println!("   是否包含5: {}", demo_array.contains(&5));
    println!("   是否包含10: {}", demo_array.contains(&10));

    // ========== 切片 (Slice) ==========
    println!("\n【第二部分：切片 (Slice)】");
    
    // 1. 创建切片
    println!("\n1. 创建切片：");
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("   原数组: {:?}", array);
    
    let slice1 = &array[2..5];      // 索引2到4（不包含5）
    let slice2 = &array[..3];       // 从开始到索引2
    let slice3 = &array[5..];       // 从索引5到结尾
    let slice4 = &array[..];        // 整个数组的切片
    
    println!("   切片[2..5]: {:?}", slice1);
    println!("   切片[..3]: {:?}", slice2);
    println!("   切片[5..]: {:?}", slice3);
    println!("   切片[..]: {:?}", slice4);

    // 2. 切片操作
    println!("\n2. 切片操作：");
    let numbers_slice = &[10, 20, 30, 40, 50];
    println!("   切片: {:?}", numbers_slice);
    println!("   长度: {}", numbers_slice.len());
    println!("   是否为空: {}", numbers_slice.is_empty());
    
    // 切片分割
    let (left, right) = numbers_slice.split_at(2);
    println!("   在索引2分割:");
    println!("     左半部分: {:?}", left);
    println!("     右半部分: {:?}", right);

    // 3. 切片遍历
    println!("\n3. 切片遍历：");
    let slice = &array[1..6];
    println!("   遍历切片 {:?}:", slice);
    for (i, &value) in slice.iter().enumerate() {
        println!("     索引{}: 值{}", i, value);
    }

    // 4. 可变切片
    println!("\n4. 可变切片：");
    let mut mutable_array = [1, 2, 3, 4, 5];
    println!("   修改前: {:?}", mutable_array);
    
    let mutable_slice = &mut mutable_array[1..4];
    for item in mutable_slice.iter_mut() {
        *item += 10;
    }
    println!("   修改切片[1..4]后: {:?}", mutable_array);

    // ========== 元组 (Tuple) ==========
    println!("\n【第三部分：元组 (Tuple)】");
    
    // 1. 创建元组
    println!("\n1. 创建元组：");
    
    // 不同类型的元组
    let person: (String, i32, bool) = ("张三".to_string(), 25, true);
    let coordinates = (3.14, 2.71);
    let mixed = (42, "hello", 'c', true, [1, 2, 3]);
    
    println!("   个人信息: {:?}", person);
    println!("   坐标: {:?}", coordinates);
    println!("   混合类型: {:?}", mixed);
    
    // 空元组（单元类型）
    let unit = ();
    println!("   空元组: {:?}", unit);

    // 2. 元组访问
    println!("\n2. 元组访问：");
    let student = ("李四", 20, 85.5, true);
    println!("   完整元组: {:?}", student);
    println!("   姓名: {}", student.0);
    println!("   年龄: {}", student.1);
    println!("   分数: {}", student.2);
    println!("   是否及格: {}", student.3);

    // 3. 元组解构
    println!("\n3. 元组解构：");
    let point = (100, 200);
    let (x, y) = point;  // 解构赋值
    println!("   坐标点: {:?}", point);
    println!("   x坐标: {}, y坐标: {}", x, y);
    
    // 部分解构
    let data = ("Alice", 30, 95.0, "Engineer");
    let (name, age, _, job) = data;  // 忽略分数
    println!("   姓名: {}, 年龄: {}, 职业: {}", name, age, job);
    
    // 嵌套解构
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("   嵌套元组: {:?}", nested);
    println!("   解构结果: a={}, b={}, c={}, d={}", a, b, c, d);

    // 4. 元组作为函数参数和返回值
    println!("\n4. 元组作为函数参数和返回值：");
    let result = calculate(10, 5);
    println!("   计算结果: {:?}", result);
    
    let (sum, diff, prod, quot) = result;
    println!("   和: {}, 差: {}, 积: {}, 商: {}", sum, diff, prod, quot);
    
    // 使用元组交换变量
    let mut a = 10;
    let mut b = 20;
    println!("   交换前: a={}, b={}", a, b);
    (a, b) = (b, a);  // 交换
    println!("   交换后: a={}, b={}", a, b);

    // 5. 元组数组
    println!("\n5. 元组数组：");
    let points = [(0, 0), (1, 2), (3, 4), (5, 6)];
    println!("   坐标点数组: {:?}", points);
    
    println!("   遍历坐标点:");
    for (i, (x, y)) in points.iter().enumerate() {
        println!("     点{}: ({}, {})", i, x, y);
    }

    // 6. 复杂元组示例
    println!("\n6. 复杂元组示例：");
    
    // 存储学生信息的元组
    type StudentInfo = (String, u8, Vec<f64>, bool);
    
    let students: Vec<StudentInfo> = vec![
        ("张三".to_string(), 20, vec![85.0, 92.0, 78.0], true),
        ("李四".to_string(), 19, vec![90.0, 88.0, 95.0], true),
        ("王五".to_string(), 21, vec![76.0, 82.0, 80.0], false),
    ];
    
    println!("   学生信息:");
    for (name, age, scores, is_active) in &students {
        let average = scores.iter().sum::<f64>() / scores.len() as f64;
        println!("     姓名: {}, 年龄: {}, 平均分: {:.1}, 活跃: {}", 
                name, age, average, is_active);
    }

    // 7. 元组方法
    println!("\n7. 元组与模式匹配：");
    let status = ("Success", 200, true);
    
    match status {
        ("Success", code, true) => println!("   成功状态，代码: {}", code),
        ("Error", code, false) => println!("   错误状态，代码: {}", code),
        (msg, code, active) => println!("   其他状态: {}, 代码: {}, 活跃: {}", msg, code, active),
    }
    
    // 使用if let
    if let ("Success", code, _) = status {
        println!("   通过if let匹配成功状态，代码: {}", code);
    }

    println!("\n=== 数组、切片和元组教程结束 ===");
}

// 返回多个值的函数
fn calculate(a: i32, b: i32) -> (i32, i32, i32, i32) {
    (a + b, a - b, a * b, a / b)
} 