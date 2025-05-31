/**
 * Rust基础数据结构教程 - HashMap（哈希映射）
 * 
 * HashMap是一种键值对数据结构，类似于其他语言中的字典或关联数组
 * 特点：
 * - 键必须实现Hash和Eq trait
 * - 平均O(1)的查找、插入和删除时间复杂度
 * - 键值对无序存储
 * - 所有键必须是同一类型，所有值必须是同一类型
 */

use std::collections::HashMap;

fn main() {
    println!("=== Rust HashMap教程 ===\n");

    // 1. 创建HashMap
    println!("1. 创建HashMap：");
    
    // 方式1：使用new()创建空HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    println!("   空HashMap: {:?}", scores);
    
    // 方式2：使用collect()从元组向量创建
    let teams = vec![
        ("Blue".to_string(), 10),
        ("Yellow".to_string(), 50),
        ("Red".to_string(), 30),
    ];
    let scores_from_vec: HashMap<String, i32> = teams.into_iter().collect();
    println!("   从向量创建: {:?}", scores_from_vec);
    
    // 方式3：使用宏（需要额外的crate）
    // 这里手动插入数据来演示
    let mut manual_map = HashMap::new();
    manual_map.insert("Alice".to_string(), 25);
    manual_map.insert("Bob".to_string(), 30);
    manual_map.insert("Charlie".to_string(), 35);
    println!("   手动创建: {:?}", manual_map);

    // 2. 插入和更新
    println!("\n2. 插入和更新：");
    let mut scores = HashMap::new();
    
    // 插入新键值对
    scores.insert("Alice".to_string(), 90);
    scores.insert("Bob".to_string(), 85);
    println!("   插入后: {:?}", scores);
    
    // 更新已存在的键
    scores.insert("Alice".to_string(), 95);  // 会覆盖原值
    println!("   更新Alice的分数后: {:?}", scores);
    
    // 只在键不存在时插入
    scores.entry("Charlie".to_string()).or_insert(88);
    scores.entry("Alice".to_string()).or_insert(100);  // 不会覆盖，因为Alice已存在
    println!("   使用entry后: {:?}", scores);

    // 3. 访问值
    println!("\n3. 访问值：");
    
    // 使用get方法（返回Option）
    match scores.get("Alice") {
        Some(score) => println!("   Alice的分数: {}", score),
        None => println!("   Alice不存在"),
    }
    
    match scores.get("David") {
        Some(score) => println!("   David的分数: {}", score),
        None => println!("   David不存在"),
    }
    
    // 使用get的简化写法
    if let Some(score) = scores.get("Bob") {
        println!("   Bob的分数: {}", score);
    }
    
    // 使用[]操作符（会panic如果键不存在）
    println!("   Charlie的分数: {}", scores["Charlie"]);
    // println!("{}", scores["David"]);  // 这会panic！

    // 4. 检查键是否存在
    println!("\n4. 检查键是否存在：");
    println!("   Alice存在吗？ {}", scores.contains_key("Alice"));
    println!("   David存在吗？ {}", scores.contains_key("David"));

    // 5. 遍历HashMap
    println!("\n5. 遍历HashMap：");
    
    // 遍历键值对
    println!("   遍历所有键值对:");
    for (name, score) in &scores {
        println!("     {}: {}", name, score);
    }
    
    // 只遍历键
    println!("   遍历所有键:");
    for name in scores.keys() {
        println!("     {}", name);
    }
    
    // 只遍历值
    println!("   遍历所有值:");
    for score in scores.values() {
        println!("     {}", score);
    }
    
    // 可变遍历值
    println!("   给所有分数加10分:");
    for score in scores.values_mut() {
        *score += 10;
    }
    println!("   更新后: {:?}", scores);

    // 6. 删除元素
    println!("\n6. 删除元素：");
    println!("   删除前: {:?}", scores);
    
    // 删除指定键
    if let Some(removed_score) = scores.remove("Bob") {
        println!("   删除了Bob，分数为: {}", removed_score);
    }
    println!("   删除Bob后: {:?}", scores);
    
    // 尝试删除不存在的键
    match scores.remove("David") {
        Some(score) => println!("   删除了David，分数为: {}", score),
        None => println!("   David不存在，无法删除"),
    }

    // 7. HashMap的属性
    println!("\n7. HashMap的属性：");
    println!("   元素数量: {}", scores.len());
    println!("   是否为空: {}", scores.is_empty());
    
    // 清空HashMap
    scores.clear();
    println!("   清空后的长度: {}", scores.len());
    println!("   清空后是否为空: {}", scores.is_empty());

    // 8. 高级操作 - entry API
    println!("\n8. Entry API高级操作：");
    let mut player_stats: HashMap<String, (i32, i32)> = HashMap::new(); // (得分, 游戏次数)
    
    let players = vec!["Alice", "Bob", "Alice", "Charlie", "Bob", "Alice"];
    let scores = vec![100, 80, 120, 90, 95, 110];
    
    for (player, score) in players.iter().zip(scores.iter()) {
        player_stats.entry(player.to_string())
            .and_modify(|(total_score, games)| {
                *total_score += score;
                *games += 1;
            })
            .or_insert((*score, 1));
    }
    
    println!("   玩家统计:");
    for (player, (total_score, games)) in &player_stats {
        let average = *total_score as f64 / *games as f64;
        println!("     {}: 总分{}, {}局游戏, 平均{:.1}分", player, total_score, games, average);
    }

    // 9. 不同类型的HashMap
    println!("\n9. 不同类型的HashMap：");
    
    // 字符串到向量的映射
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    groups.insert("fruits".to_string(), vec!["apple".to_string(), "banana".to_string()]);
    groups.insert("colors".to_string(), vec!["red".to_string(), "blue".to_string()]);
    println!("   分组数据: {:?}", groups);
    
    // 数字到布尔值的映射
    let mut is_even: HashMap<i32, bool> = HashMap::new();
    for i in 1..=10 {
        is_even.insert(i, i % 2 == 0);
    }
    println!("   奇偶性映射: {:?}", is_even);
    
    // 字符到计数的映射
    let text = "hello world";
    let mut char_count: HashMap<char, i32> = HashMap::new();
    for ch in text.chars() {
        if ch != ' ' {  // 忽略空格
            *char_count.entry(ch).or_insert(0) += 1;
        }
    }
    println!("   字符计数: {:?}", char_count);

    // 10. HashMap与结构体
    println!("\n10. HashMap与结构体：");
    
    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        age: u32,
        grade: f64,
    }
    
    let mut students: HashMap<u32, Student> = HashMap::new();  // 学号 -> 学生信息
    
    students.insert(1001, Student {
        name: "张三".to_string(),
        age: 20,
        grade: 85.5,
    });
    
    students.insert(1002, Student {
        name: "李四".to_string(),
        age: 19,
        grade: 92.0,
    });
    
    students.insert(1003, Student {
        name: "王五".to_string(),
        age: 21,
        grade: 78.5,
    });
    
    println!("   学生信息:");
    for (id, student) in &students {
        println!("     学号{}: {:?}", id, student);
    }
    
    // 查找特定学生
    if let Some(student) = students.get(&1002) {
        println!("   学号1002的学生: {}, 年龄: {}, 成绩: {}", 
               student.name, student.age, student.grade);
    }

    // 11. HashMap的性能考虑
    println!("\n11. HashMap的性能考虑：");
    
    // 预分配容量
    let mut large_map: HashMap<i32, String> = HashMap::with_capacity(1000);
    println!("   预分配容量的HashMap容量: {}", large_map.capacity());
    
    // 插入大量数据
    for i in 0..100 {
        large_map.insert(i, format!("value_{}", i));
    }
    println!("   插入100个元素后，长度: {}, 容量: {}", large_map.len(), large_map.capacity());

    // 12. 复杂的嵌套HashMap
    println!("\n12. 复杂的嵌套HashMap：");
    
    // 表示学校 -> 班级 -> 学生的层次结构
    let mut school: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    
    // 初始化学校数据
    let mut computer_science = HashMap::new();
    computer_science.insert("CS101".to_string(), vec!["Alice".to_string(), "Bob".to_string()]);
    computer_science.insert("CS102".to_string(), vec!["Charlie".to_string(), "David".to_string()]);
    
    let mut mathematics = HashMap::new();
    mathematics.insert("MATH101".to_string(), vec!["Eve".to_string(), "Frank".to_string()]);
    
    school.insert("计算机科学".to_string(), computer_science);
    school.insert("数学".to_string(), mathematics);
    
    println!("   学校结构:");
    for (department, classes) in &school {
        println!("     系别: {}", department);
        for (class, students) in classes {
            println!("       班级: {}", class);
            for student in students {
                println!("         学生: {}", student);
            }
        }
    }

    // 13. 使用HashMap实现缓存
    println!("\n13. 使用HashMap实现简单缓存：");
    
    struct SimpleCache {
        data: HashMap<String, String>,
        max_size: usize,
    }
    
    impl SimpleCache {
        fn new(max_size: usize) -> Self {
            SimpleCache {
                data: HashMap::with_capacity(max_size),
                max_size,
            }
        }
        
        fn get(&self, key: &str) -> Option<&String> {
            self.data.get(key)
        }
        
        fn put(&mut self, key: String, value: String) {
            if self.data.len() >= self.max_size && !self.data.contains_key(&key) {
                // 简单策略：删除第一个找到的元素
                if let Some(first_key) = self.data.keys().next().cloned() {
                    self.data.remove(&first_key);
                }
            }
            self.data.insert(key, value);
        }
        
        fn size(&self) -> usize {
            self.data.len()
        }
    }
    
    let mut cache = SimpleCache::new(3);
    cache.put("key1".to_string(), "value1".to_string());
    cache.put("key2".to_string(), "value2".to_string());
    cache.put("key3".to_string(), "value3".to_string());
    
    println!("   缓存大小: {}", cache.size());
    println!("   获取key1: {:?}", cache.get("key1"));
    
    // 添加第四个元素，应该会删除一个旧元素
    cache.put("key4".to_string(), "value4".to_string());
    println!("   添加key4后缓存大小: {}", cache.size());

    println!("\n=== HashMap教程结束 ===");
} 