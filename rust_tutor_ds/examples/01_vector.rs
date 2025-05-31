/**
 * Rust基础数据结构教程 - Vector（动态数组）
 * 
 * Vector是Rust中最常用的集合类型之一，类似于其他语言中的动态数组
 * 特点：
 * - 存储相同类型的元素
 * - 大小可变
 * - 元素在内存中连续存储
 * - 支持随机访问
 */

fn main() {
    println!("=== Rust Vector（动态数组）教程 ===\n");

    // 1. 创建Vector的几种方式
    println!("1. 创建Vector：");
    
    // 方式1：使用Vec::new()创建空vector
    let mut numbers: Vec<i32> = Vec::new();
    println!("   空的numbers vector: {:?}", numbers);
    
    // 方式2：使用vec!宏创建并初始化
    let fruits = vec!["苹果", "香蕉", "橙子"];
    println!("   水果vector: {:?}", fruits);
    
    // 方式3：使用with_capacity指定初始容量
    let scores: Vec<i32> = Vec::with_capacity(10);
    println!("   预分配容量的scores vector: {:?}", scores);

    // 2. 添加元素
    println!("\n2. 添加元素：");
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("   添加元素后的numbers: {:?}", numbers);
    
    // 使用extend添加多个元素
    numbers.extend(vec![40, 50]);
    println!("   扩展后的numbers: {:?}", numbers);

    // 3. 访问元素
    println!("\n3. 访问元素：");
    
    // 通过索引访问（可能panic）
    println!("   第一个元素: {}", numbers[0]);
    println!("   第三个元素: {}", numbers[2]);
    
    // 安全访问（返回Option）
    match numbers.get(1) {
        Some(value) => println!("   安全访问第二个元素: {}", value),
        None => println!("   索引越界"),
    }
    
    // 访问不存在的索引
    match numbers.get(10) {
        Some(value) => println!("   第11个元素: {}", value),
        None => println!("   索引10不存在"),
    }

    // 4. 遍历Vector
    println!("\n4. 遍历Vector：");
    
    // 遍历值
    print!("   遍历值: ");
    for number in &numbers {
        print!("{} ", number);
    }
    println!();
    
    // 遍历索引和值
    println!("   遍历索引和值:");
    for (index, value) in numbers.iter().enumerate() {
        println!("     索引{}: 值{}", index, value);
    }

    // 5. 修改元素
    println!("\n5. 修改元素：");
    println!("   修改前: {:?}", numbers);
    numbers[0] = 100;  // 直接修改
    println!("   修改第一个元素后: {:?}", numbers);
    
    // 可变引用遍历并修改
    for number in &mut numbers {
        *number *= 2;  // 每个元素乘以2
    }
    println!("   所有元素乘以2后: {:?}", numbers);

    // 6. 删除元素
    println!("\n6. 删除元素：");
    
    // pop删除最后一个元素
    if let Some(last) = numbers.pop() {
        println!("   删除的最后一个元素: {}", last);
    }
    println!("   pop后的numbers: {:?}", numbers);
    
    // remove删除指定索引的元素
    let removed = numbers.remove(1);  // 删除索引1的元素
    println!("   删除索引1的元素: {}", removed);
    println!("   remove后的numbers: {:?}", numbers);

    // 7. Vector的属性
    println!("\n7. Vector的属性：");
    println!("   长度: {}", numbers.len());
    println!("   容量: {}", numbers.capacity());
    println!("   是否为空: {}", numbers.is_empty());

    // 8. 清空Vector
    println!("\n8. 清空Vector：");
    numbers.clear();
    println!("   清空后的numbers: {:?}", numbers);
    println!("   是否为空: {}", numbers.is_empty());

    // 9. Vector的常用方法
    println!("\n9. Vector的常用方法：");
    let mut demo_vec = vec![1, 2, 3, 4, 5];
    println!("   原始vector: {:?}", demo_vec);
    
    // first和last
    println!("   第一个元素: {:?}", demo_vec.first());
    println!("   最后一个元素: {:?}", demo_vec.last());
    
    // contains检查是否包含元素
    println!("   是否包含3: {}", demo_vec.contains(&3));
    println!("   是否包含10: {}", demo_vec.contains(&10));
    
    // 插入元素
    demo_vec.insert(2, 99);  // 在索引2插入99
    println!("   插入99后: {:?}", demo_vec);
    
    // 反转
    demo_vec.reverse();
    println!("   反转后: {:?}", demo_vec);
    
    // 排序
    demo_vec.sort();
    println!("   排序后: {:?}", demo_vec);

    // 10. Vector与slice
    println!("\n10. Vector与slice：");
    let slice = &demo_vec[1..4];  // 获取slice
    println!("   slice [1..4]: {:?}", slice);
    
    // to_vec将slice转换为vector
    let new_vec = slice.to_vec();
    println!("   slice转换的新vector: {:?}", new_vec);

    println!("\n=== Vector教程结束 ===");
} 