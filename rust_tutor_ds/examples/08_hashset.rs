/**
 * Rust基础数据结构教程 - HashSet（哈希集合）
 * 
 * HashSet是一种存储唯一值的集合数据结构
 * 特点：
 * - 每个值只能出现一次（唯一性）
 * - 元素必须实现Hash和Eq trait
 * - 平均O(1)的查找、插入和删除时间复杂度
 * - 元素无序存储
 * - 基于HashMap实现
 */

use std::collections::HashSet;

fn main() {
    println!("=== Rust HashSet教程 ===\n");

    // 1. 创建HashSet
    println!("1. 创建HashSet：");
    
    // 方式1：创建空集合
    let mut numbers: HashSet<i32> = HashSet::new();
    println!("   空集合: {:?}", numbers);
    
    // 方式2：从向量创建
    let vec_data = vec![1, 2, 3, 2, 1, 4, 5, 4];
    let numbers_from_vec: HashSet<i32> = vec_data.into_iter().collect();
    println!("   从向量创建（自动去重）: {:?}", numbers_from_vec);
    
    // 方式3：从数组创建
    let array_data = [10, 20, 30, 20, 10];
    let numbers_from_array: HashSet<i32> = array_data.into_iter().collect();
    println!("   从数组创建: {:?}", numbers_from_array);
    
    // 方式4：手动插入
    let mut fruits = HashSet::new();
    fruits.insert("apple".to_string());
    fruits.insert("banana".to_string());
    fruits.insert("orange".to_string());
    println!("   手动插入的水果集合: {:?}", fruits);

    // 2. 插入元素
    println!("\n2. 插入元素：");
    let mut colors = HashSet::new();
    
    // insert返回bool，表示是否成功插入（即元素是否原本不存在）
    println!("   插入red: {}", colors.insert("red".to_string()));
    println!("   插入blue: {}", colors.insert("blue".to_string()));
    println!("   再次插入red: {}", colors.insert("red".to_string()));  // 应该返回false
    
    println!("   当前集合: {:?}", colors);

    // 3. 检查元素是否存在
    println!("\n3. 检查元素是否存在：");
    println!("   包含red? {}", colors.contains("red"));
    println!("   包含green? {}", colors.contains("green"));

    // 4. 删除元素
    println!("\n4. 删除元素：");
    println!("   删除前: {:?}", colors);
    
    // remove返回bool，表示元素是否存在并被删除
    println!("   删除blue: {}", colors.remove("blue"));
    println!("   删除green: {}", colors.remove("green"));  // 应该返回false
    
    println!("   删除后: {:?}", colors);

    // 5. 集合的属性
    println!("\n5. 集合的属性：");
    let demo_set: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    println!("   集合: {:?}", demo_set);
    println!("   长度: {}", demo_set.len());
    println!("   是否为空: {}", demo_set.is_empty());

    // 6. 遍历集合
    println!("\n6. 遍历集合：");
    let words: HashSet<&str> = ["hello", "world", "rust", "programming"].into_iter().collect();
    
    println!("   遍历所有元素:");
    for word in &words {
        println!("     {}", word);
    }
    
    // 收集到向量中进行排序显示
    let mut sorted_words: Vec<&str> = words.iter().cloned().collect();
    sorted_words.sort();
    println!("   排序后的元素: {:?}", sorted_words);

    // 7. 集合运算
    println!("\n7. 集合运算：");
    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].into_iter().collect();
    
    println!("   集合A: {:?}", set_a);
    println!("   集合B: {:?}", set_b);
    
    // 交集
    let intersection: HashSet<i32> = set_a.intersection(&set_b).cloned().collect();
    println!("   交集 (A ∩ B): {:?}", intersection);
    
    // 并集
    let union: HashSet<i32> = set_a.union(&set_b).cloned().collect();
    println!("   并集 (A ∪ B): {:?}", union);
    
    // 差集
    let difference: HashSet<i32> = set_a.difference(&set_b).cloned().collect();
    println!("   差集 (A - B): {:?}", difference);
    
    // 对称差集
    let symmetric_difference: HashSet<i32> = set_a.symmetric_difference(&set_b).cloned().collect();
    println!("   对称差集 (A △ B): {:?}", symmetric_difference);

    // 8. 集合关系判断
    println!("\n8. 集合关系判断：");
    let small_set: HashSet<i32> = [2, 3].into_iter().collect();
    let large_set: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let other_set: HashSet<i32> = [6, 7, 8].into_iter().collect();
    
    println!("   小集合: {:?}", small_set);
    println!("   大集合: {:?}", large_set);
    println!("   其他集合: {:?}", other_set);
    
    // 子集判断
    println!("   小集合是大集合的子集? {}", small_set.is_subset(&large_set));
    println!("   大集合是小集合的子集? {}", large_set.is_subset(&small_set));
    
    // 超集判断
    println!("   大集合是小集合的超集? {}", large_set.is_superset(&small_set));
    
    // 不相交判断
    println!("   小集合与其他集合不相交? {}", small_set.is_disjoint(&other_set));
    println!("   小集合与大集合不相交? {}", small_set.is_disjoint(&large_set));

    // 9. 实际应用示例：去重
    println!("\n9. 实际应用示例：去重");
    
    // 去除重复的用户ID
    let user_actions = vec![
        "user123", "user456", "user789", "user123", "user456", 
        "user999", "user123", "user888", "user456"
    ];
    
    println!("   原始用户行为记录: {:?}", user_actions);
    
    let unique_users: HashSet<&str> = user_actions.into_iter().collect();
    println!("   独特用户数量: {}", unique_users.len());
    println!("   独特用户列表: {:?}", unique_users);

    // 10. 实际应用示例：标签系统
    println!("\n10. 实际应用示例：标签系统");
    
    #[derive(Debug)]
    struct Article {
        title: String,
        tags: HashSet<String>,
    }
    
    impl Article {
        fn new(title: &str) -> Self {
            Article {
                title: title.to_string(),
                tags: HashSet::new(),
            }
        }
        
        fn add_tag(&mut self, tag: &str) {
            self.tags.insert(tag.to_string());
        }
        
        fn remove_tag(&mut self, tag: &str) -> bool {
            self.tags.remove(tag)
        }
        
        fn has_tag(&self, tag: &str) -> bool {
            self.tags.contains(tag)
        }
        
        fn common_tags(&self, other: &Article) -> HashSet<String> {
            self.tags.intersection(&other.tags).cloned().collect()
        }
    }
    
    let mut article1 = Article::new("Rust编程入门");
    article1.add_tag("编程");
    article1.add_tag("Rust");
    article1.add_tag("初学者");
    article1.add_tag("教程");
    
    let mut article2 = Article::new("高级Rust特性");
    article2.add_tag("编程");
    article2.add_tag("Rust");
    article2.add_tag("高级");
    article2.add_tag("特性");
    
    println!("   文章1: {:?}", article1);
    println!("   文章2: {:?}", article2);
    
    let common = article1.common_tags(&article2);
    println!("   共同标签: {:?}", common);

    // 11. 性能考虑
    println!("\n11. 性能考虑：");
    
    // 预分配容量
    let mut large_set: HashSet<i32> = HashSet::with_capacity(1000);
    println!("   预分配容量: {}", large_set.capacity());
    
    // 批量插入
    for i in 0..100 {
        large_set.insert(i);
    }
    println!("   插入100个元素后，长度: {}", large_set.len());

    // 12. 自定义类型的HashSet
    println!("\n12. 自定义类型的HashSet：");
    
    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }
    
    let mut points = HashSet::new();
    points.insert(Point::new(0, 0));
    points.insert(Point::new(1, 1));
    points.insert(Point::new(0, 0));  // 重复点，不会被插入
    points.insert(Point::new(2, 2));
    
    println!("   点集合: {:?}", points);
    println!("   点的数量: {}", points.len());

    // 13. 复杂的实际应用：权限系统
    println!("\n13. 复杂应用：权限系统");
    
    #[derive(Debug)]
    struct User {
        name: String,
        permissions: HashSet<String>,
    }
    
    impl User {
        fn new(name: &str) -> Self {
            User {
                name: name.to_string(),
                permissions: HashSet::new(),
            }
        }
        
        fn grant_permission(&mut self, permission: &str) {
            self.permissions.insert(permission.to_string());
        }
        
        fn revoke_permission(&mut self, permission: &str) -> bool {
            self.permissions.remove(permission)
        }
        
        fn has_permission(&self, permission: &str) -> bool {
            self.permissions.contains(permission)
        }
        
        fn has_all_permissions(&self, required: &HashSet<String>) -> bool {
            required.is_subset(&self.permissions)
        }
        
        fn common_permissions(&self, other: &User) -> HashSet<String> {
            self.permissions.intersection(&other.permissions).cloned().collect()
        }
    }
    
    let mut admin = User::new("admin");
    admin.grant_permission("read");
    admin.grant_permission("write");
    admin.grant_permission("delete");
    admin.grant_permission("execute");
    
    let mut user = User::new("regular_user");
    user.grant_permission("read");
    user.grant_permission("write");
    
    println!("   管理员权限: {:?}", admin.permissions);
    println!("   普通用户权限: {:?}", user.permissions);
    
    let required_permissions: HashSet<String> = ["read", "write"].iter()
        .map(|s| s.to_string()).collect();
    
    println!("   管理员有读写权限? {}", admin.has_all_permissions(&required_permissions));
    println!("   普通用户有读写权限? {}", user.has_all_permissions(&required_permissions));
    
    let common_perms = admin.common_permissions(&user);
    println!("   共同权限: {:?}", common_perms);

    // 14. 集合的过滤和转换
    println!("\n14. 集合的过滤和转换：");
    let numbers: HashSet<i32> = (1..=20).collect();
    
    // 过滤偶数
    let even_numbers: HashSet<i32> = numbers.iter()
        .filter(|&&n| n % 2 == 0)
        .cloned()
        .collect();
    println!("   原集合中的偶数: {:?}", even_numbers);
    
    // 转换为字符串集合
    let number_strings: HashSet<String> = numbers.iter()
        .map(|n| format!("num_{}", n))
        .collect();
    println!("   转换为字符串集合的前5个: {:?}", 
           number_strings.iter().take(5).collect::<Vec<_>>());

    // 15. 清空和重置
    println!("\n15. 清空和重置：");
    let mut test_set: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    println!("   清空前: {:?}", test_set);
    
    test_set.clear();
    println!("   清空后: {:?}", test_set);
    println!("   是否为空: {}", test_set.is_empty());

    println!("\n=== HashSet教程结束 ===");
} 