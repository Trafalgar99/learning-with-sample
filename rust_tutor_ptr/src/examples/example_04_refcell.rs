/*
 * Rust智能指针教程 - 例子4: RefCell<T>
 * 
 * RefCell<T> 提供内部可变性，允许在不可变引用存在时修改数据
 * 主要特点：
 * 1. 在运行时而不是编译时检查借用规则
 * 2. 只能用于单线程场景
 * 3. 允许在拥有不可变引用的情况下修改数据
 * 4. 如果违反借用规则会在运行时panic
 */

use std::cell::RefCell;
use std::rc::Rc;

// 定义一个可变的计数器，即使在不可变上下文中也能修改
#[derive(Debug)]
struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }
    
    // 即使self是不可变引用，也能修改内部值
    fn increment(&self) {
        let mut val = self.value.borrow_mut();
        *val += 1;
    }
    
    fn decrement(&self) {
        let mut val = self.value.borrow_mut();
        *val -= 1;
    }
    
    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
    
    // 演示多个不可变借用
    fn get_value_twice(&self) -> (i32, i32) {
        let borrow1 = self.value.borrow();
        let borrow2 = self.value.borrow(); // 多个不可变借用是允许的
        (*borrow1, *borrow2)
    }
}

// 定义一个模拟的消息系统
#[derive(Debug)]
struct Message {
    id: u32,
    content: String,
    read: RefCell<bool>,
    read_count: RefCell<u32>,
}

impl Message {
    fn new(id: u32, content: String) -> Self {
        Message {
            id,
            content,
            read: RefCell::new(false),
            read_count: RefCell::new(0),
        }
    }
    
    // 标记消息为已读（即使在不可变上下文中）
    fn mark_as_read(&self) {
        *self.read.borrow_mut() = true;
        *self.read_count.borrow_mut() += 1;
    }
    
    fn is_read(&self) -> bool {
        *self.read.borrow()
    }
    
    fn get_read_count(&self) -> u32 {
        *self.read_count.borrow()
    }
}

// 定义一个可变的链表节点
#[derive(Debug)]
struct ListNode {
    value: i32,
    next: RefCell<Option<Rc<ListNode>>>,
}

impl ListNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(ListNode {
            value,
            next: RefCell::new(None),
        })
    }
    
    fn append(self: &Rc<Self>, value: i32) -> Rc<ListNode> {
        let new_node = ListNode::new(value);
        *self.next.borrow_mut() = Some(Rc::clone(&new_node));
        new_node
    }
    
    fn print_list(&self) {
        print!("{}", self.value);
        if let Some(ref next) = *self.next.borrow() {
            print!(" -> ");
            next.print_list();
        } else {
            println!(" -> None");
        }
    }
}

// 定义一个缓存系统
#[derive(Debug)]
struct Cache {
    data: RefCell<std::collections::HashMap<String, String>>,
    hit_count: RefCell<u32>,
    miss_count: RefCell<u32>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: RefCell::new(std::collections::HashMap::new()),
            hit_count: RefCell::new(0),
            miss_count: RefCell::new(0),
        }
    }
    
    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.borrow();
        match data.get(key) {
            Some(value) => {
                *self.hit_count.borrow_mut() += 1;
                Some(value.clone())
            },
            None => {
                *self.miss_count.borrow_mut() += 1;
                None
            }
        }
    }
    
    fn set(&self, key: String, value: String) {
        self.data.borrow_mut().insert(key, value);
    }
    
    fn get_stats(&self) -> (u32, u32) {
        (*self.hit_count.borrow(), *self.miss_count.borrow())
    }
}

fn main() {
    println!("=== Rust智能指针教程 - RefCell<T> ===\n");
    
    // 1. 基本的RefCell使用
    println!("1. 基本RefCell使用:");
    let data = RefCell::new(5);
    
    println!("初始值: {}", *data.borrow());
    
    // 修改值
    *data.borrow_mut() = 10;
    println!("修改后的值: {}", *data.borrow());
    
    // 多个不可变借用
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        println!("多个不可变借用: {} 和 {}", *borrow1, *borrow2);
    } // 借用在这里结束
    
    println!();
    
    // 2. 在不可变上下文中修改数据
    println!("2. 在不可变上下文中修改数据:");
    let counter = Counter::new();
    println!("初始计数器值: {}", counter.get_value());
    
    // 即使counter是不可变的，我们仍然可以修改它
    counter.increment();
    counter.increment();
    println!("增加后的值: {}", counter.get_value());
    
    counter.decrement();
    println!("减少后的值: {}", counter.get_value());
    
    // 演示多个不可变借用
    let (val1, val2) = counter.get_value_twice();
    println!("两次获取的值: {} 和 {}\n", val1, val2);
    
    // 3. RefCell与Rc结合使用
    println!("3. RefCell与Rc结合使用:");
    let shared_counter = Rc::new(Counter::new());
    
    let counter1 = Rc::clone(&shared_counter);
    let counter2 = Rc::clone(&shared_counter);
    let counter3 = Rc::clone(&shared_counter);
    
    // 多个所有者都可以修改同一个数据
    counter1.increment();
    counter2.increment();
    counter3.increment();
    
    println!("共享计数器最终值: {}", shared_counter.get_value());
    println!("引用计数: {}\n", Rc::strong_count(&shared_counter));
    
    // 4. 消息系统示例
    println!("4. 消息系统示例:");
    let messages = vec![
        Message::new(1, "欢迎使用系统".to_string()),
        Message::new(2, "您有新的通知".to_string()),
        Message::new(3, "系统维护通知".to_string()),
    ];
    
    // 即使messages是不可变的，我们仍然可以标记消息为已读
    for message in &messages {
        println!("消息 {}: {}", message.id, message.content);
        println!("已读状态: {}", message.is_read());
        
        message.mark_as_read(); // 修改内部状态
        println!("标记为已读后: {}", message.is_read());
        println!("读取次数: {}", message.get_read_count());
        println!();
    }
    
    // 5. 可变链表
    println!("5. 可变链表:");
    let head = ListNode::new(1);
    let second = head.append(2);
    let third = second.append(3);
    third.append(4);
    
    println!("链表内容:");
    head.print_list();
    println!();
    
    // 6. 缓存系统
    println!("6. 缓存系统:");
    let cache = Cache::new();
    
    // 设置一些缓存数据
    cache.set("user:1".to_string(), "Alice".to_string());
    cache.set("user:2".to_string(), "Bob".to_string());
    
    // 访问缓存
    println!("查找 user:1: {:?}", cache.get("user:1"));
    println!("查找 user:2: {:?}", cache.get("user:2"));
    println!("查找 user:3: {:?}", cache.get("user:3")); // 缓存未命中
    
    let (hits, misses) = cache.get_stats();
    println!("缓存统计 - 命中: {}, 未命中: {}\n", hits, misses);
    
    // 7. 演示RefCell的运行时借用检查
    println!("7. RefCell的运行时借用检查:");
    let cell = RefCell::new(42);
    
    // 正常的借用
    {
        let borrow1 = cell.borrow();
        let borrow2 = cell.borrow(); // 多个不可变借用OK
        println!("两个不可变借用: {} 和 {}", *borrow1, *borrow2);
    }
    
    // 可变借用
    {
        let mut borrow_mut = cell.borrow_mut();
        *borrow_mut = 100;
        println!("可变借用修改后: {}", *borrow_mut);
    }
    
    // 演示借用检查失败的情况（注释掉以避免panic）
    /*
    {
        let _borrow = cell.borrow();
        let _borrow_mut = cell.borrow_mut(); // 这会panic！
    }
    */
    
    println!("RefCell在运行时检查借用规则，违反规则会panic");
    
    // 8. try_borrow 和 try_borrow_mut
    println!("\n8. 安全的借用尝试:");
    let cell2 = RefCell::new("Hello".to_string());
    
    match cell2.try_borrow() {
        Ok(borrow) => println!("成功借用: {}", *borrow),
        Err(e) => println!("借用失败: {:?}", e),
    }
    
    match cell2.try_borrow_mut() {
        Ok(mut borrow) => {
            *borrow = "World".to_string();
            println!("成功可变借用并修改: {}", *borrow);
        },
        Err(e) => println!("可变借用失败: {:?}", e),
    }
    
    println!("\n=== RefCell教程完成 ===");
    println!("注意：RefCell只能在单线程中使用，多线程请使用Mutex！");
}

// 演示RefCell在函数参数中的使用
fn process_shared_data(data: &RefCell<Vec<i32>>) {
    println!("处理前数据长度: {}", data.borrow().len());
    
    // 添加一些数据
    data.borrow_mut().push(42);
    data.borrow_mut().push(84);
    
    println!("处理后数据长度: {}", data.borrow().len());
}

// 演示RefCell与trait对象的结合
trait Drawable {
    fn draw(&self);
    fn set_position(&self, x: i32, y: i32);
    fn get_position(&self) -> (i32, i32);
}

struct Circle {
    radius: f64,
    position: RefCell<(i32, i32)>,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle {
            radius,
            position: RefCell::new((0, 0)),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        let (x, y) = *self.position.borrow();
        println!("绘制半径为 {} 的圆，位置: ({}, {})", self.radius, x, y);
    }
    
    fn set_position(&self, x: i32, y: i32) {
        *self.position.borrow_mut() = (x, y);
    }
    
    fn get_position(&self) -> (i32, i32) {
        *self.position.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_refcell_basic_operations() {
        let cell = RefCell::new(42);
        
        assert_eq!(*cell.borrow(), 42);
        
        *cell.borrow_mut() = 100;
        assert_eq!(*cell.borrow(), 100);
    }
    
    #[test]
    fn test_multiple_immutable_borrows() {
        let cell = RefCell::new(42);
        
        let borrow1 = cell.borrow();
        let borrow2 = cell.borrow();
        
        assert_eq!(*borrow1, 42);
        assert_eq!(*borrow2, 42);
    }
    
    #[test]
    fn test_counter_operations() {
        let counter = Counter::new();
        
        assert_eq!(counter.get_value(), 0);
        
        counter.increment();
        counter.increment();
        assert_eq!(counter.get_value(), 2);
        
        counter.decrement();
        assert_eq!(counter.get_value(), 1);
    }
    
    #[test]
    fn test_try_borrow() {
        let cell = RefCell::new(42);
        
        // 成功的不可变借用
        assert!(cell.try_borrow().is_ok());
        
        // 成功的可变借用
        assert!(cell.try_borrow_mut().is_ok());
    }
    
    #[test]
    #[should_panic]
    fn test_borrow_conflict() {
        let cell = RefCell::new(42);
        
        let _borrow = cell.borrow();
        let _borrow_mut = cell.borrow_mut(); // 这应该panic
    }
} 