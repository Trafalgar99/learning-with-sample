/*
 * Rust智能指针教程 - 例子2: Rc<T> (Reference Counted)
 * 
 * Rc<T> 是引用计数智能指针，允许多个所有者共享同一份数据
 * 主要特点：
 * 1. 只能用于单线程场景
 * 2. 通过引用计数来管理内存
 * 3. 当引用计数为0时，数据会被自动清理
 * 4. 数据是不可变的（除非配合RefCell使用）
 */

use std::rc::Rc;

// 定义一个图节点，演示多个父节点共享子节点的场景
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            children: Vec::new(),
        })
    }
    
    // 注意：由于Rc<T>是不可变的，我们不能直接修改children
    // 这里只是为了演示，实际应用中可能需要配合RefCell
    fn with_children(value: i32, children: Vec<Rc<Node>>) -> Rc<Self> {
        Rc::new(Node {
            value,
            children,
        })
    }
}

// 定义一个简单的链表，演示Rc的基本使用
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn new() -> Rc<Self> {
        Rc::new(List::Nil)
    }
    
    fn prepend(list: Rc<List>, elem: i32) -> Rc<List> {
        Rc::new(List::Cons(elem, list))
    }
    
    fn head(&self) -> Option<&i32> {
        match self {
            List::Cons(ref head, _) => Some(head),
            List::Nil => None,
        }
    }
    
    fn tail(&self) -> Option<&Rc<List>> {
        match self {
            List::Cons(_, ref tail) => Some(tail),
            List::Nil => None,
        }
    }
}

// 演示Rc在复杂数据结构中的应用
#[derive(Debug)]
struct Database {
    name: String,
    tables: Vec<Rc<Table>>,
}

#[derive(Debug)]
struct Table {
    name: String,
    columns: Vec<String>,
}

impl Table {
    fn new(name: String, columns: Vec<String>) -> Rc<Self> {
        Rc::new(Table { name, columns })
    }
}

impl Database {
    fn new(name: String) -> Self {
        Database {
            name,
            tables: Vec::new(),
        }
    }
    
    fn add_table(&mut self, table: Rc<Table>) {
        self.tables.push(table);
    }
}

fn main() {
    println!("=== Rust智能指针教程 - Rc<T> ===\n");
    
    // 1. 基本的Rc使用和引用计数
    println!("1. 基本Rc使用和引用计数:");
    let data = Rc::new("Hello, Rc!".to_string());
    println!("初始引用计数: {}", Rc::strong_count(&data));
    
    {
        let data_clone1 = Rc::clone(&data);  // 增加引用计数
        let data_clone2 = data.clone();      // 另一种克隆方式
        
        println!("克隆后引用计数: {}", Rc::strong_count(&data));
        println!("所有引用指向同一数据: {}", 
                 std::ptr::eq(Rc::as_ptr(&data), Rc::as_ptr(&data_clone1)));
        
        // 所有的Rc都指向同一份数据
        println!("原始数据: {}", data);
        println!("克隆1: {}", data_clone1);
        println!("克隆2: {}", data_clone2);
    } // data_clone1 和 data_clone2 在这里被销毁
    
    println!("作用域结束后引用计数: {}\n", Rc::strong_count(&data));
    
    // 2. 共享链表的例子
    println!("2. 共享链表:");
    let shared_tail = List::new();
    let shared_tail = List::prepend(shared_tail, 1);
    let shared_tail = List::prepend(shared_tail, 2);
    
    println!("共享尾部引用计数: {}", Rc::strong_count(&shared_tail));
    
    // 创建两个不同的链表，但它们共享相同的尾部
    let list_a = List::prepend(Rc::clone(&shared_tail), 3);
    let list_b = List::prepend(Rc::clone(&shared_tail), 4);
    
    println!("添加两个头部后，共享尾部引用计数: {}", Rc::strong_count(&shared_tail));
    
    println!("链表A的头部: {:?}", list_a.head());
    println!("链表B的头部: {:?}", list_b.head());
    
    // 验证它们确实共享尾部
    if let (Some(tail_a), Some(tail_b)) = (list_a.tail(), list_b.tail()) {
        println!("两个链表共享尾部: {}", std::ptr::eq(Rc::as_ptr(tail_a), Rc::as_ptr(tail_b)));
    }
    println!();
    
    // 3. 图结构中的共享节点
    println!("3. 图结构中的共享节点:");
    let leaf = Node::new(3);
    println!("叶子节点引用计数: {}", Rc::strong_count(&leaf));
    
    let branch1 = Node::with_children(1, vec![Rc::clone(&leaf)]);
    let branch2 = Node::with_children(2, vec![Rc::clone(&leaf)]);
    
    println!("两个分支共享叶子节点后，叶子节点引用计数: {}", Rc::strong_count(&leaf));
    
    let root = Node::with_children(0, vec![branch1, branch2]);
    
    println!("根节点值: {}", root.value);
    println!("根节点有 {} 个子节点", root.children.len());
    
    // 叶子节点仍然被两个分支共享
    println!("叶子节点最终引用计数: {}\n", Rc::strong_count(&leaf));
    
    // 4. 数据库表的共享
    println!("4. 数据库表的共享:");
    let users_table = Table::new(
        "users".to_string(),
        vec!["id".to_string(), "name".to_string(), "email".to_string()]
    );
    
    println!("用户表初始引用计数: {}", Rc::strong_count(&users_table));
    
    let mut db1 = Database::new("主数据库".to_string());
    let mut db2 = Database::new("备份数据库".to_string());
    
    // 两个数据库共享同一个表定义
    db1.add_table(Rc::clone(&users_table));
    db2.add_table(Rc::clone(&users_table));
    
    println!("两个数据库共享表后，用户表引用计数: {}", Rc::strong_count(&users_table));
    println!("数据库1有 {} 个表", db1.tables.len());
    println!("数据库2有 {} 个表", db2.tables.len());
    println!();
    
    // 5. Rc的内存效率演示
    println!("5. Rc的内存效率:");
    let large_data = Rc::new(vec![0u8; 1000]); // 1KB数据
    println!("大数据引用计数: {}", Rc::strong_count(&large_data));
    
    // 创建100个"拷贝"，但实际上只是增加引用计数
    let mut copies = Vec::new();
    for i in 0..100 {
        copies.push(Rc::clone(&large_data));
        if i % 20 == 0 {
            println!("创建{}个拷贝后，引用计数: {}", i + 1, Rc::strong_count(&large_data));
        }
    }
    
    println!("所有拷贝都指向同一内存地址: {}", 
             copies.iter().all(|rc| std::ptr::eq(Rc::as_ptr(rc), Rc::as_ptr(&large_data))));
    
    // 6. Rc::try_unwrap 和 Rc::get_mut
    println!("\n6. Rc::try_unwrap 示例:");
    let single_owner = Rc::new("唯一所有者".to_string());
    println!("尝试解包前引用计数: {}", Rc::strong_count(&single_owner));
    
    match Rc::try_unwrap(single_owner) {
        Ok(data) => println!("成功解包: {}", data),
        Err(rc) => println!("解包失败，引用计数: {}", Rc::strong_count(&rc)),
    }
    
    // 7. 演示Rc的限制 - 不能修改数据
    println!("\n7. Rc的限制 - 数据不可变:");
    let shared_vec = Rc::new(vec![1, 2, 3]);
    let shared_vec_clone = Rc::clone(&shared_vec);
    
    // shared_vec.push(4); // 这行会编译错误！Rc<T>中的数据是不可变的
    println!("共享向量: {:?}", shared_vec);
    println!("克隆的向量: {:?}", shared_vec_clone);
    
    println!("\n=== Rc教程完成 ===");
    println!("注意：Rc只能在单线程中使用，多线程请使用Arc！");
}

// 辅助函数：演示Rc在函数间的传递
fn process_shared_data(data: Rc<String>) -> Rc<String> {
    println!("处理共享数据: {}", data);
    println!("函数内引用计数: {}", Rc::strong_count(&data));
    data // 返回相同的Rc，不增加引用计数
}

fn create_shared_list() -> (Rc<List>, Rc<List>) {
    let shared_tail = List::new();
    let shared_tail = List::prepend(shared_tail, 10);
    
    let list1 = List::prepend(Rc::clone(&shared_tail), 20);
    let list2 = List::prepend(Rc::clone(&shared_tail), 30);
    
    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rc_reference_counting() {
        let data = Rc::new(42);
        assert_eq!(Rc::strong_count(&data), 1);
        
        let data_clone = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(Rc::strong_count(&data_clone), 2);
        
        drop(data_clone);
        assert_eq!(Rc::strong_count(&data), 1);
    }
    
    #[test]
    fn test_shared_data() {
        let shared = Rc::new("test".to_string());
        let clone1 = Rc::clone(&shared);
        let clone2 = Rc::clone(&shared);
        
        assert!(std::ptr::eq(Rc::as_ptr(&shared), Rc::as_ptr(&clone1)));
        assert!(std::ptr::eq(Rc::as_ptr(&shared), Rc::as_ptr(&clone2)));
    }
    
    #[test]
    fn test_try_unwrap() {
        let single = Rc::new(100);
        let unwrapped = Rc::try_unwrap(single).unwrap();
        assert_eq!(unwrapped, 100);
    }
} 