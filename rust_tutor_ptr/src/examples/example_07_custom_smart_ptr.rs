/*
 * Rust智能指针教程 - 例子7: 自定义智能指针
 * 
 * 学习如何创建自定义的智能指针类型
 * 主要内容：
 * 1. 实现Deref和DerefMut trait
 * 2. 实现Drop trait进行资源清理
 * 3. 创建带有额外功能的智能指针
 * 4. RAII模式的应用
 */

use std::ops::{Deref, DerefMut};
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

// 1. 简单的Box替代品
#[derive(Debug)]
struct MyBox<T> {
    data: T,
}

impl<T> MyBox<T> {
    fn new(data: T) -> Self {
        MyBox { data }
    }
    
    fn into_inner(self) -> T {
        self.data
    }
}

// 实现Deref trait，使MyBox可以像引用一样使用
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// 实现DerefMut trait，允许可变解引用
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// 2. 带有引用计数和访问统计的智能指针
#[derive(Debug)]
struct CountedPtr<T> {
    data: Rc<RefCell<T>>,
    access_count: Rc<RefCell<usize>>,
}

impl<T> CountedPtr<T> {
    fn new(data: T) -> Self {
        CountedPtr {
            data: Rc::new(RefCell::new(data)),
            access_count: Rc::new(RefCell::new(0)),
        }
    }
    
    fn access_count(&self) -> usize {
        *self.access_count.borrow()
    }
    
    fn strong_count(&self) -> usize {
        Rc::strong_count(&self.data)
    }
}

impl<T> Clone for CountedPtr<T> {
    fn clone(&self) -> Self {
        CountedPtr {
            data: Rc::clone(&self.data),
            access_count: Rc::clone(&self.access_count),
        }
    }
}

impl<T> Deref for CountedPtr<T> {
    type Target = RefCell<T>;
    
    fn deref(&self) -> &Self::Target {
        // 每次解引用都增加访问计数
        *self.access_count.borrow_mut() += 1;
        &self.data
    }
}

// 3. 带有自动清理功能的资源管理器
struct ResourceManager<T> {
    resource: Option<T>,
    cleanup_fn: Option<Box<dyn FnOnce(&mut T)>>,
}

impl<T> ResourceManager<T> {
    fn new(resource: T) -> Self {
        ResourceManager {
            resource: Some(resource),
            cleanup_fn: None,
        }
    }
    
    fn with_cleanup<F>(mut self, cleanup: F) -> Self 
    where 
        F: FnOnce(&mut T) + 'static,
    {
        self.cleanup_fn = Some(Box::new(cleanup));
        self
    }
    
    fn take(mut self) -> Option<T> {
        self.resource.take()
    }
}

impl<T> Deref for ResourceManager<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.resource.as_ref().expect("Resource has been taken")
    }
}

impl<T> DerefMut for ResourceManager<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.resource.as_mut().expect("Resource has been taken")
    }
}

impl<T> Drop for ResourceManager<T> {
    fn drop(&mut self) {
        if let (Some(ref mut resource), Some(cleanup)) = 
            (self.resource.as_mut(), self.cleanup_fn.take()) {
            println!("执行资源清理...");
            cleanup(resource);
        }
    }
}

// 4. 带有生命周期跟踪的智能指针
struct TrackedPtr<T> {
    data: Box<T>,
    id: usize,
    created_at: std::time::Instant,
}

impl<T> TrackedPtr<T> {
    fn new(data: T) -> Self {
        static mut COUNTER: usize = 0;
        let id = unsafe {
            COUNTER += 1;
            COUNTER
        };
        
        println!("创建TrackedPtr #{}", id);
        
        TrackedPtr {
            data: Box::new(data),
            id,
            created_at: std::time::Instant::now(),
        }
    }
    
    fn id(&self) -> usize {
        self.id
    }
    
    fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }
}

impl<T> Deref for TrackedPtr<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for TrackedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Drop for TrackedPtr<T> {
    fn drop(&mut self) {
        println!("销毁TrackedPtr #{}, 存活时间: {:?}", self.id, self.age());
    }
}

impl<T: fmt::Debug> fmt::Debug for TrackedPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrackedPtr")
            .field("id", &self.id)
            .field("data", &self.data)
            .field("age", &self.age())
            .finish()
    }
}

// 5. 带有延迟初始化的智能指针
struct LazyPtr<T, F> 
where 
    F: FnOnce() -> T,
{
    data: RefCell<Option<T>>,
    init_fn: RefCell<Option<F>>,
}

impl<T, F> LazyPtr<T, F> 
where 
    F: FnOnce() -> T,
{
    fn new(init_fn: F) -> Self {
        LazyPtr {
            data: RefCell::new(None),
            init_fn: RefCell::new(Some(init_fn)),
        }
    }
    
    fn get(&self) -> std::cell::Ref<T> {
        // 如果数据还没有初始化，先初始化
        if self.data.borrow().is_none() {
            if let Some(init_fn) = self.init_fn.borrow_mut().take() {
                println!("延迟初始化数据...");
                *self.data.borrow_mut() = Some(init_fn());
            }
        }
        
        std::cell::Ref::map(self.data.borrow(), |opt| opt.as_ref().unwrap())
    }
    
    fn get_mut(&self) -> std::cell::RefMut<T> {
        // 确保数据已初始化
        self.get();
        
        std::cell::RefMut::map(self.data.borrow_mut(), |opt| opt.as_mut().unwrap())
    }
}

// 6. 带有访问权限控制的智能指针
#[derive(Debug, Clone, Copy, PartialEq)]
enum AccessLevel {
    ReadOnly,
    ReadWrite,
    Admin,
}

struct SecurePtr<T> {
    data: Rc<RefCell<T>>,
    access_level: AccessLevel,
}

impl<T> SecurePtr<T> {
    fn new(data: T, access_level: AccessLevel) -> Self {
        SecurePtr {
            data: Rc::new(RefCell::new(data)),
            access_level,
        }
    }
    
    fn read(&self) -> std::cell::Ref<T> {
        match self.access_level {
            AccessLevel::ReadOnly | AccessLevel::ReadWrite | AccessLevel::Admin => {
                self.data.borrow()
            }
        }
    }
    
    fn write(&self) -> Result<std::cell::RefMut<T>, &'static str> {
        match self.access_level {
            AccessLevel::ReadOnly => Err("没有写入权限"),
            AccessLevel::ReadWrite | AccessLevel::Admin => Ok(self.data.borrow_mut()),
        }
    }
    
    fn admin_access(&self) -> Result<std::cell::RefMut<T>, &'static str> {
        match self.access_level {
            AccessLevel::ReadOnly | AccessLevel::ReadWrite => Err("需要管理员权限"),
            AccessLevel::Admin => Ok(self.data.borrow_mut()),
        }
    }
    
    fn clone_with_access(&self, new_access: AccessLevel) -> Self {
        SecurePtr {
            data: Rc::clone(&self.data),
            access_level: new_access,
        }
    }
}

fn main() {
    println!("=== Rust智能指针教程 - 自定义智能指针 ===\n");
    
    // 1. 简单的MyBox使用
    println!("1. 简单的MyBox使用:");
    let mut my_box = MyBox::new("Hello, MyBox!".to_string());
    
    // 通过Deref trait自动解引用
    println!("MyBox内容: {}", *my_box);
    println!("字符串长度: {}", my_box.len());
    
    // 通过DerefMut trait可变解引用
    my_box.push_str(" 修改后");
    println!("修改后内容: {}", *my_box);
    
    let inner = my_box.into_inner();
    println!("提取的内容: {}\n", inner);
    
    // 2. 带有访问统计的CountedPtr
    println!("2. 带有访问统计的CountedPtr:");
    let counted = CountedPtr::new(vec![1, 2, 3, 4, 5]);
    
    println!("初始访问次数: {}", counted.access_count());
    println!("强引用计数: {}", counted.strong_count());
    
    // 每次访问都会增加计数
    {
        let data = counted.borrow();
        println!("数据: {:?}", *data);
    }
    println!("访问后计数: {}", counted.access_count());
    
    // 克隆指针
    let counted_clone = counted.clone();
    println!("克隆后强引用计数: {}", counted.strong_count());
    
    {
        let mut data = counted_clone.borrow_mut();
        data.push(6);
    }
    println!("修改后访问计数: {}", counted.access_count());
    
    {
        let data = counted.borrow();
        println!("修改后数据: {:?}", *data);
    }
    println!("最终访问计数: {}\n", counted.access_count());
    
    // 3. 资源管理器
    println!("3. 资源管理器:");
    {
        let mut resource = ResourceManager::new(vec![1, 2, 3])
            .with_cleanup(|data| {
                println!("清理资源，数据长度: {}", data.len());
                data.clear();
            });
        
        resource.push(4);
        resource.push(5);
        println!("资源内容: {:?}", *resource);
    } // 在这里自动调用清理函数
    
    println!("资源已被清理\n");
    
    // 4. 生命周期跟踪
    println!("4. 生命周期跟踪:");
    {
        let tracked1 = TrackedPtr::new("第一个跟踪指针".to_string());
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        let tracked2 = TrackedPtr::new(42);
        
        println!("tracked1 ID: {}, 内容: {}", tracked1.id(), *tracked1);
        println!("tracked2 ID: {}, 内容: {}", tracked2.id(), *tracked2);
        
        std::thread::sleep(std::time::Duration::from_millis(20));
        println!("tracked1 存活时间: {:?}", tracked1.age());
    } // 在这里自动销毁
    
    println!("跟踪指针已销毁\n");
    
    // 5. 延迟初始化
    println!("5. 延迟初始化:");
    let lazy = LazyPtr::new(|| {
        println!("执行昂贵的初始化操作...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        vec![1, 2, 3, 4, 5]
    });
    
    println!("LazyPtr已创建，但数据尚未初始化");
    
    // 第一次访问时才初始化
    {
        let data = lazy.get();
        println!("第一次访问数据: {:?}", *data);
    }
    
    // 第二次访问不会重新初始化
    {
        let data = lazy.get();
        println!("第二次访问数据: {:?}", *data);
    }
    
    // 可变访问
    {
        let mut data = lazy.get_mut();
        data.push(6);
        println!("修改后数据: {:?}", *data);
    }
    println!();
    
    // 6. 访问权限控制
    println!("6. 访问权限控制:");
    let admin_data = SecurePtr::new("敏感数据".to_string(), AccessLevel::Admin);
    let readonly_data = admin_data.clone_with_access(AccessLevel::ReadOnly);
    let readwrite_data = admin_data.clone_with_access(AccessLevel::ReadWrite);
    
    // 所有权限都可以读取
    println!("管理员读取: {}", *admin_data.read());
    println!("只读用户读取: {}", *readonly_data.read());
    println!("读写用户读取: {}", *readwrite_data.read());
    
    // 只有读写和管理员权限可以写入
    match readonly_data.write() {
        Ok(_) => println!("只读用户写入成功"),
        Err(e) => println!("只读用户写入失败: {}", e),
    }
    
    match readwrite_data.write() {
        Ok(mut data) => {
            *data = "读写用户修改的数据".to_string();
            println!("读写用户写入成功");
        },
        Err(e) => println!("读写用户写入失败: {}", e),
    }
    
    // 只有管理员权限可以进行管理员操作
    match readwrite_data.admin_access() {
        Ok(_) => println!("读写用户管理员操作成功"),
        Err(e) => println!("读写用户管理员操作失败: {}", e),
    }
    
    match admin_data.admin_access() {
        Ok(mut data) => {
            *data = "管理员修改的数据".to_string();
            println!("管理员操作成功");
        },
        Err(e) => println!("管理员操作失败: {}", e),
    }
    
    println!("最终数据: {}", *admin_data.read());
    
    println!("\n=== 自定义智能指针教程完成 ===");
    println!("关键要点：");
    println!("1. Deref trait 使类型可以像引用一样使用");
    println!("2. DerefMut trait 允许可变解引用");
    println!("3. Drop trait 提供自动资源清理");
    println!("4. 可以组合多种功能创建强大的智能指针");
}

// 演示智能指针的组合使用
fn demonstrate_composition() {
    println!("\n演示智能指针组合:");
    
    // 组合使用多个智能指针特性
    let tracked_resource = TrackedPtr::new(
        ResourceManager::new(vec![1, 2, 3])
            .with_cleanup(|data| {
                println!("清理向量，长度: {}", data.len());
            })
    );
    
    println!("组合智能指针 ID: {}", tracked_resource.id());
    println!("数据: {:?}", **tracked_resource);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_box() {
        let mut my_box = MyBox::new(42);
        assert_eq!(*my_box, 42);
        
        *my_box = 100;
        assert_eq!(*my_box, 100);
        
        let value = my_box.into_inner();
        assert_eq!(value, 100);
    }
    
    #[test]
    fn test_counted_ptr() {
        let counted = CountedPtr::new(vec![1, 2, 3]);
        assert_eq!(counted.access_count(), 0);
        
        {
            let _data = counted.borrow();
        }
        assert_eq!(counted.access_count(), 1);
        
        let counted_clone = counted.clone();
        assert_eq!(counted.strong_count(), 2);
        
        {
            let _data = counted_clone.borrow();
        }
        assert_eq!(counted.access_count(), 2);
    }
    
    #[test]
    fn test_resource_manager() {
        let mut cleanup_called = false;
        {
            let _resource = ResourceManager::new(vec![1, 2, 3])
                .with_cleanup(|_| {
                    // 在实际测试中，我们无法直接修改外部变量
                    // 这里只是演示清理函数会被调用
                });
        }
        // 清理函数在这里被调用
    }
    
    #[test]
    fn test_lazy_ptr() {
        let mut init_called = false;
        let lazy = LazyPtr::new(|| {
            init_called = true;
            vec![1, 2, 3]
        });
        
        // 数据还没有初始化
        // 注意：在实际测试中，我们无法检查闭包内的变量
        
        {
            let data = lazy.get();
            assert_eq!(*data, vec![1, 2, 3]);
        }
        
        // 第二次访问不会重新初始化
        {
            let data = lazy.get();
            assert_eq!(*data, vec![1, 2, 3]);
        }
    }
    
    #[test]
    fn test_secure_ptr() {
        let admin_ptr = SecurePtr::new("test".to_string(), AccessLevel::Admin);
        let readonly_ptr = admin_ptr.clone_with_access(AccessLevel::ReadOnly);
        
        // 都可以读取
        assert_eq!(*admin_ptr.read(), "test");
        assert_eq!(*readonly_ptr.read(), "test");
        
        // 只有管理员可以写入
        assert!(admin_ptr.write().is_ok());
        assert!(readonly_ptr.write().is_err());
        
        // 只有管理员可以进行管理员操作
        assert!(admin_ptr.admin_access().is_ok());
        assert!(readonly_ptr.admin_access().is_err());
    }
    
    #[test]
    fn test_tracked_ptr() {
        let tracked = TrackedPtr::new(42);
        assert_eq!(*tracked, 42);
        assert!(tracked.id() > 0);
        assert!(tracked.age().as_nanos() > 0);
    }
} 