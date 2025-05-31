/*
 * Rust智能指针教程 - 例子5: Weak<T>
 * 
 * Weak<T> 是弱引用智能指针，不会影响引用计数
 * 主要特点：
 * 1. 不拥有数据，不影响数据的生命周期
 * 2. 可以检查数据是否仍然存在
 * 3. 主要用于避免循环引用
 * 4. 需要通过upgrade()方法转换为强引用才能访问数据
 */

use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 定义一个树节点，演示父子关系中的循环引用问题
#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
    parent: RefCell<Weak<TreeNode>>, // 使用Weak避免循环引用
}

impl TreeNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }
    
    fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
        // 设置子节点的父节点引用
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        // 将子节点添加到父节点的子节点列表中
        parent.children.borrow_mut().push(child);
    }
    
    fn get_parent(&self) -> Option<Rc<TreeNode>> {
        self.parent.borrow().upgrade()
    }
    
    fn print_tree(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}节点值: {}", indent, self.value);
        
        for child in self.children.borrow().iter() {
            child.print_tree(depth + 1);
        }
    }
    
    fn count_descendants(&self) -> usize {
        let mut count = 0;
        for child in self.children.borrow().iter() {
            count += 1 + child.count_descendants();
        }
        count
    }
}

// 定义一个观察者模式的例子
#[derive(Debug)]
struct Subject {
    value: RefCell<i32>,
    observers: RefCell<Vec<Weak<Observer>>>,
}

#[derive(Debug)]
struct Observer {
    id: u32,
    subject: RefCell<Weak<Subject>>,
}

impl Subject {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Subject {
            value: RefCell::new(value),
            observers: RefCell::new(Vec::new()),
        })
    }
    
    fn add_observer(self: &Rc<Self>, observer: &Rc<Observer>) {
        // 观察者持有主题的弱引用
        *observer.subject.borrow_mut() = Rc::downgrade(self);
        // 主题持有观察者的弱引用
        self.observers.borrow_mut().push(Rc::downgrade(observer));
    }
    
    fn set_value(&self, new_value: i32) {
        *self.value.borrow_mut() = new_value;
        self.notify_observers();
    }
    
    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
    
    fn notify_observers(&self) {
        let mut observers = self.observers.borrow_mut();
        // 清理已经被释放的观察者
        observers.retain(|weak_observer| {
            if let Some(observer) = weak_observer.upgrade() {
                observer.on_notify(self.get_value());
                true
            } else {
                false // 观察者已被释放，从列表中移除
            }
        });
    }
    
    fn observer_count(&self) -> usize {
        self.observers.borrow().len()
    }
}

impl Observer {
    fn new(id: u32) -> Rc<Self> {
        Rc::new(Observer {
            id,
            subject: RefCell::new(Weak::new()),
        })
    }
    
    fn on_notify(&self, value: i32) {
        println!("观察者 {} 收到通知，新值: {}", self.id, value);
    }
    
    fn get_subject_value(&self) -> Option<i32> {
        self.subject.borrow().upgrade().map(|s| s.get_value())
    }
}

// 定义一个缓存系统，使用弱引用避免内存泄漏
#[derive(Debug)]
struct CacheEntry {
    key: String,
    value: String,
    cache: RefCell<Weak<Cache>>,
}

#[derive(Debug)]
struct Cache {
    entries: RefCell<Vec<Rc<CacheEntry>>>,
    max_size: usize,
}

impl Cache {
    fn new(max_size: usize) -> Rc<Self> {
        Rc::new(Cache {
            entries: RefCell::new(Vec::new()),
            max_size,
        })
    }
    
    fn insert(self: &Rc<Self>, key: String, value: String) -> Rc<CacheEntry> {
        let entry = Rc::new(CacheEntry {
            key,
            value,
            cache: RefCell::new(Rc::downgrade(self)),
        });
        
        let mut entries = self.entries.borrow_mut();
        entries.push(Rc::clone(&entry));
        
        // 如果超过最大大小，移除最旧的条目
        if entries.len() > self.max_size {
            entries.remove(0);
        }
        
        entry
    }
    
    fn get(&self, key: &str) -> Option<Rc<CacheEntry>> {
        self.entries.borrow()
            .iter()
            .find(|entry| entry.key == key)
            .cloned()
    }
    
    fn size(&self) -> usize {
        self.entries.borrow().len()
    }
}

impl CacheEntry {
    fn get_cache(&self) -> Option<Rc<Cache>> {
        self.cache.borrow().upgrade()
    }
}

fn main() {
    println!("=== Rust智能指针教程 - Weak<T> ===\n");
    
    // 1. 基本的Weak使用
    println!("1. 基本Weak使用:");
    let strong_ref = Rc::new("Hello, Weak!".to_string());
    println!("强引用计数: {}", Rc::strong_count(&strong_ref));
    
    let weak_ref = Rc::downgrade(&strong_ref);
    println!("弱引用计数: {}", Rc::weak_count(&strong_ref));
    println!("强引用计数: {}", Rc::strong_count(&strong_ref));
    
    // 通过弱引用访问数据
    match weak_ref.upgrade() {
        Some(strong) => println!("通过弱引用访问: {}", strong),
        None => println!("数据已被释放"),
    }
    
    // 释放强引用
    drop(strong_ref);
    
    // 尝试再次通过弱引用访问
    match weak_ref.upgrade() {
        Some(strong) => println!("通过弱引用访问: {}", strong),
        None => println!("数据已被释放"),
    }
    println!();
    
    // 2. 树结构避免循环引用
    println!("2. 树结构避免循环引用:");
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    let grandchild = TreeNode::new(4);
    
    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());
    TreeNode::add_child(&child1, grandchild.clone());
    
    println!("树结构:");
    root.print_tree(0);
    
    println!("根节点强引用计数: {}", Rc::strong_count(&root));
    println!("子节点1强引用计数: {}", Rc::strong_count(&child1));
    println!("孙子节点强引用计数: {}", Rc::strong_count(&grandchild));
    
    // 通过子节点访问父节点
    if let Some(parent) = grandchild.get_parent() {
        println!("孙子节点的父节点值: {}", parent.value);
    }
    
    println!("根节点的后代数量: {}\n", root.count_descendants());
    
    // 3. 观察者模式
    println!("3. 观察者模式:");
    let subject = Subject::new(42);
    let observer1 = Observer::new(1);
    let observer2 = Observer::new(2);
    let observer3 = Observer::new(3);
    
    // 添加观察者
    subject.add_observer(&observer1);
    subject.add_observer(&observer2);
    subject.add_observer(&observer3);
    
    println!("观察者数量: {}", subject.observer_count());
    
    // 更新主题值，通知所有观察者
    println!("更新主题值为 100:");
    subject.set_value(100);
    
    // 释放一个观察者
    drop(observer2);
    
    println!("\n释放观察者2后，更新主题值为 200:");
    subject.set_value(200);
    
    println!("剩余观察者数量: {}", subject.observer_count());
    
    // 观察者访问主题
    if let Some(value) = observer1.get_subject_value() {
        println!("观察者1看到的主题值: {}", value);
    }
    println!();
    
    // 4. 缓存系统
    println!("4. 缓存系统:");
    let cache = Cache::new(3);
    
    let entry1 = cache.insert("key1".to_string(), "value1".to_string());
    let entry2 = cache.insert("key2".to_string(), "value2".to_string());
    let entry3 = cache.insert("key3".to_string(), "value3".to_string());
    
    println!("缓存大小: {}", cache.size());
    
    // 缓存条目可以访问其所属的缓存
    if let Some(cache_ref) = entry1.get_cache() {
        println!("条目1所属缓存大小: {}", cache_ref.size());
    }
    
    // 添加第四个条目，会移除最旧的条目
    let entry4 = cache.insert("key4".to_string(), "value4".to_string());
    println!("添加第四个条目后缓存大小: {}", cache.size());
    
    // 尝试获取已被移除的条目
    match cache.get("key1") {
        Some(entry) => println!("找到条目: {} = {}", entry.key, entry.value),
        None => println!("条目key1已被移除"),
    }
    println!();
    
    // 5. 演示Weak的生命周期
    println!("5. Weak的生命周期:");
    let weak_refs = {
        let data = Rc::new(vec![1, 2, 3, 4, 5]);
        let weak1 = Rc::downgrade(&data);
        let weak2 = Rc::downgrade(&data);
        
        println!("作用域内 - 强引用计数: {}", Rc::strong_count(&data));
        println!("作用域内 - 弱引用计数: {}", Rc::weak_count(&data));
        
        vec![weak1, weak2]
    }; // data在这里被释放
    
    println!("作用域外 - 尝试升级弱引用:");
    for (i, weak_ref) in weak_refs.iter().enumerate() {
        match weak_ref.upgrade() {
            Some(data) => println!("弱引用 {} 升级成功: {:?}", i, data),
            None => println!("弱引用 {} 升级失败，数据已释放", i),
        }
    }
    
    // 6. Weak::new() 创建空的弱引用
    println!("\n6. 空的弱引用:");
    let empty_weak: Weak<String> = Weak::new();
    match empty_weak.upgrade() {
        Some(data) => println!("空弱引用升级成功: {}", data),
        None => println!("空弱引用升级失败"),
    }
    
    println!("\n=== Weak教程完成 ===");
    println!("Weak引用的主要用途：");
    println!("1. 避免循环引用导致的内存泄漏");
    println!("2. 实现观察者模式等设计模式");
    println!("3. 缓存系统中的反向引用");
    println!("4. 父子关系中的父引用");
}

// 演示循环引用问题（如果不使用Weak会导致内存泄漏）
#[derive(Debug)]
struct BadNode {
    value: i32,
    children: RefCell<Vec<Rc<BadNode>>>,
    parent: RefCell<Option<Rc<BadNode>>>, // 这会导致循环引用！
}

impl BadNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(BadNode {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        })
    }
    
    // 这个方法会创建循环引用，导致内存泄漏
    fn add_child_bad(parent: &Rc<Self>, child: Rc<Self>) {
        *child.parent.borrow_mut() = Some(Rc::clone(parent));
        parent.children.borrow_mut().push(child);
    }
}

// 辅助函数：演示弱引用在回调中的使用
fn setup_callback_with_weak() {
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let weak_data = Rc::downgrade(&data);
    
    // 模拟一个回调函数，使用弱引用避免循环引用
    let callback = move || {
        if let Some(strong_data) = weak_data.upgrade() {
            println!("回调中访问数据: {:?}", strong_data.borrow());
        } else {
            println!("回调中数据已释放");
        }
    };
    
    callback(); // 数据仍然存在
    drop(data); // 释放数据
    callback(); // 数据已被释放
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_weak_basic_operations() {
        let strong = Rc::new(42);
        let weak = Rc::downgrade(&strong);
        
        assert_eq!(Rc::strong_count(&strong), 1);
        assert_eq!(Rc::weak_count(&strong), 1);
        
        assert_eq!(weak.upgrade().unwrap(), strong);
        
        drop(strong);
        assert!(weak.upgrade().is_none());
    }
    
    #[test]
    fn test_tree_structure() {
        let root = TreeNode::new(1);
        let child = TreeNode::new(2);
        
        TreeNode::add_child(&root, child.clone());
        
        assert_eq!(root.children.borrow().len(), 1);
        assert!(child.get_parent().is_some());
        assert_eq!(child.get_parent().unwrap().value, 1);
    }
    
    #[test]
    fn test_observer_pattern() {
        let subject = Subject::new(0);
        let observer = Observer::new(1);
        
        subject.add_observer(&observer);
        assert_eq!(subject.observer_count(), 1);
        
        subject.set_value(42);
        assert_eq!(observer.get_subject_value().unwrap(), 42);
        
        drop(observer);
        subject.set_value(100); // 这会清理已释放的观察者
        assert_eq!(subject.observer_count(), 0);
    }
    
    #[test]
    fn test_cache_system() {
        let cache = Cache::new(2);
        
        let entry1 = cache.insert("key1".to_string(), "value1".to_string());
        let entry2 = cache.insert("key2".to_string(), "value2".to_string());
        
        assert_eq!(cache.size(), 2);
        assert!(entry1.get_cache().is_some());
        
        // 添加第三个条目会移除第一个
        cache.insert("key3".to_string(), "value3".to_string());
        assert_eq!(cache.size(), 2);
        assert!(cache.get("key1").is_none());
    }
    
    #[test]
    fn test_weak_lifecycle() {
        let weak = {
            let strong = Rc::new("test".to_string());
            Rc::downgrade(&strong)
        }; // strong在这里被释放
        
        assert!(weak.upgrade().is_none());
    }
} 