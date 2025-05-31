/*
 * Rust智能指针教程 - 例子1: Box<T>
 * 
 * Box<T> 是最简单的智能指针，用于在堆上分配数据
 * 主要用途：
 * 1. 当你有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用它的值时
 * 2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
 * 3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
 */

use std::fmt::Display;

// 定义一个递归数据结构 - 链表
// 如果不使用Box，这个定义会导致编译错误，因为Rust无法确定List的大小
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),  // Box让我们可以创建递归类型
    Nil,
}

impl List {
    // 创建一个新的链表
    fn new() -> List {
        List::Nil
    }
    
    // 在链表前面添加元素
    fn prepend(self, elem: i32) -> List {
        List::Cons(elem, Box::new(self))
    }
    
    // 计算链表长度
    fn len(&self) -> usize {
        match self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
    
    // 将链表转换为字符串表示
    fn stringify(&self) -> String {
        match self {
            List::Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil")
            },
        }
    }
}

// 演示Box用于trait对象
trait Animal {
    fn make_sound(&self) -> &str;
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "汪汪!"
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "喵喵!"
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// 大型数据结构示例
#[derive(Debug)]
struct LargeData {
    data: [u8; 1000],  // 1KB的数据
    id: u32,
}

impl LargeData {
    fn new(id: u32) -> Self {
        LargeData {
            data: [0; 1000],
            id,
        }
    }
}

fn main() {
    println!("=== Rust智能指针教程 - Box<T> ===\n");
    
    // 1. 基本的Box使用
    println!("1. 基本Box使用:");
    let x = 5;
    let y = Box::new(x);  // 将x的值放在堆上
    
    println!("栈上的值: {}", x);
    println!("堆上的值: {}", y);
    println!("Box解引用: {}", *y);
    println!("Box和原值相等: {}\n", x == *y);
    
    // 2. 递归数据结构
    println!("2. 递归数据结构 - 链表:");
    let list = List::new()
        .prepend(1)
        .prepend(2)
        .prepend(3);
    
    println!("链表内容: {}", list.stringify());
    println!("链表长度: {}\n", list.len());
    
    // 3. Box用于trait对象
    println!("3. Box用于trait对象:");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "旺财".to_string() }),
        Box::new(Cat { name: "咪咪".to_string() }),
        Box::new(Dog { name: "大黄".to_string() }),
    ];
    
    for animal in &animals {
        println!("{} 说: {}", animal.name(), animal.make_sound());
    }
    println!();
    
    // 4. 避免大数据的栈拷贝
    println!("4. 避免大数据的栈拷贝:");
    
    // 不使用Box - 会在栈上拷贝大量数据
    let large_data1 = LargeData::new(1);
    let large_data2 = large_data1;  // 这里发生了1KB数据的拷贝
    println!("栈上大数据ID: {}", large_data2.id);
    
    // 使用Box - 只拷贝指针，数据保留在堆上
    let boxed_large_data1 = Box::new(LargeData::new(2));
    let boxed_large_data2 = boxed_large_data1;  // 只拷贝了指针
    println!("堆上大数据ID: {}", boxed_large_data2.id);
    println!();
    
    // 5. Box的所有权转移
    println!("5. Box的所有权转移:");
    let original_box = Box::new("Hello, Box!".to_string());
    println!("原始Box: {}", original_box);
    
    let moved_box = original_box;  // 所有权转移
    println!("转移后的Box: {}", moved_box);
    // println!("{}", original_box);  // 这行会编译错误，因为所有权已转移
    
    // 6. Box的解构
    println!("\n6. Box的解构:");
    let boxed_tuple = Box::new((1, 2, 3));
    let (a, b, c) = *boxed_tuple;  // 解引用并解构
    println!("解构后的值: a={}, b={}, c={}", a, b, c);
    
    // 7. Box::leak - 将Box转换为静态引用
    println!("\n7. Box::leak 示例:");
    let boxed_string = Box::new("这是一个泄漏的字符串".to_string());
    let static_str: &'static str = Box::leak(boxed_string);
    println!("泄漏的静态字符串: {}", static_str);
    // 注意：这会导致内存泄漏，除非你有特殊需求，否则不要使用
    
    println!("\n=== Box教程完成 ===");
}

// 辅助函数：演示Box在函数间的传递
fn process_boxed_data(data: Box<i32>) -> Box<i32> {
    println!("处理Box数据: {}", data);
    Box::new(*data * 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_list_operations() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        assert_eq!(list.len(), 3);
    }
    
    #[test]
    fn test_box_deref() {
        let boxed_value = Box::new(42);
        assert_eq!(*boxed_value, 42);
    }
    
    #[test]
    fn test_trait_objects() {
        let dog: Box<dyn Animal> = Box::new(Dog { name: "测试狗".to_string() });
        assert_eq!(dog.make_sound(), "汪汪!");
    }
} 