// 例子3: 泛型结构体
// 这个例子将详细讲解如何定义和使用泛型结构体

pub fn run() {
    println!("\n🎯 例子3: 泛型结构体");
    println!("=====================");
    
    // 1. 基本泛型结构体
    println!("\n📖 1. 基本泛型结构体");
    
    // 定义一个简单的泛型结构体
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // 创建不同类型的Point实例
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let char_point = Point { x: 'A', y: 'B' };
    
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    println!("字符点: {:?}", char_point);
    
    // 2. 多个泛型参数的结构体
    println!("\n📖 2. 多个泛型参数的结构体");
    
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }
    
    let mixed_pair = Pair {
        first: "Hello",
        second: 42,
    };
    
    let number_pair = Pair {
        first: 3.14,
        second: true,
    };
    
    println!("混合对: {:?}", mixed_pair);
    println!("数字布尔对: {:?}", number_pair);
    
    // 3. 泛型结构体的方法实现
    println!("\n📖 3. 泛型结构体的方法实现");
    
    impl<T> Point<T> {
        // 构造函数
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
        
        // 获取x坐标的引用
        fn x(&self) -> &T {
            &self.x
        }
        
        // 获取y坐标的引用
        fn y(&self) -> &T {
            &self.y
        }
    }
    
    let point = Point::new(10, 20);
    println!("使用构造函数创建的点: x={}, y={}", point.x(), point.y());
    
    // 4. 带约束的泛型结构体方法
    println!("\n📖 4. 带约束的泛型结构体方法");
    
    impl<T> Point<T> 
    where 
        T: std::fmt::Display + Copy,
    {
        fn display(&self) {
            println!("点坐标: ({}, {})", self.x, self.y);
        }
    }
    
    // 只有实现了Display和Copy的类型才能调用display方法
    let displayable_point = Point::new(5, 10);
    displayable_point.display();
    
    // 5. 条件实现 - 只为特定类型实现方法
    println!("\n📖 5. 条件实现 - 只为特定类型实现方法");
    
    impl Point<f64> {
        // 只为f64类型的Point实现距离计算
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }
    
    let float_point = Point::new(3.0, 4.0);
    println!("到原点的距离: {:.2}", float_point.distance_from_origin());
    
    // 6. 复杂的泛型结构体示例
    println!("\n📖 6. 复杂的泛型结构体示例");
    
    // 泛型容器结构体
    #[derive(Debug)]
    struct Container<T> {
        items: Vec<T>,
    }
    
    impl<T> Container<T> {
        fn new() -> Self {
            Container {
                items: Vec::new(),
            }
        }
        
        fn add(&mut self, item: T) {
            self.items.push(item);
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }
    
    impl<T: Clone> Container<T> {
        fn get(&self, index: usize) -> Option<T> {
            self.items.get(index).cloned()
        }
        
        fn get_all(&self) -> Vec<T> {
            self.items.clone()
        }
    }
    
    let mut string_container = Container::new();
    string_container.add(String::from("第一个"));
    string_container.add(String::from("第二个"));
    string_container.add(String::from("第三个"));
    
    println!("容器长度: {}", string_container.len());
    println!("第一个元素: {:?}", string_container.get(0));
    println!("所有元素: {:?}", string_container.get_all());
    
    // 7. 泛型结构体与生命周期
    println!("\n📖 7. 泛型结构体与生命周期");
    
    #[derive(Debug)]
    struct Wrapper<'a, T> {
        value: &'a T,
        name: &'a str,
    }
    
    impl<'a, T> Wrapper<'a, T> 
    where 
        T: std::fmt::Display,
    {
        fn new(value: &'a T, name: &'a str) -> Self {
            Wrapper { value, name }
        }
        
        fn display(&self) {
            println!("{}: {}", self.name, self.value);
        }
    }
    
    let number = 42;
    let wrapper = Wrapper::new(&number, "我的数字");
    wrapper.display();
    
    // 8. 嵌套泛型结构体
    println!("\n📖 8. 嵌套泛型结构体");
    
    #[derive(Debug)]
    struct Node<T> {
        value: T,
        children: Vec<Node<T>>,
    }
    
    impl<T> Node<T> {
        fn new(value: T) -> Self {
            Node {
                value,
                children: Vec::new(),
            }
        }
        
        fn add_child(&mut self, child: Node<T>) {
            self.children.push(child);
        }
        
        fn count_nodes(&self) -> usize {
            1 + self.children.iter().map(|child| child.count_nodes()).sum::<usize>()
        }
    }
    
    let mut root = Node::new("根节点");
    let child1 = Node::new("子节点1");
    let child2 = Node::new("子节点2");
    
    root.add_child(child1);
    root.add_child(child2);
    
    println!("树结构: {:?}", root);
    println!("节点总数: {}", root.count_nodes());
    
    // 9. 泛型结构体的关联函数
    println!("\n📖 9. 泛型结构体的关联函数");
    
    #[derive(Debug)]
    struct Rectangle<T> {
        width: T,
        height: T,
    }
    
    impl<T> Rectangle<T> 
    where 
        T: Copy + std::ops::Mul<Output = T>,
    {
        fn square(size: T) -> Self {
            Rectangle {
                width: size,
                height: size,
            }
        }
        
        fn area(&self) -> T {
            self.width * self.height
        }
    }
    
    let square = Rectangle::square(5);
    let rectangle = Rectangle { width: 10, height: 20 };
    
    println!("正方形: {:?}, 面积: {}", square, square.area());
    println!("矩形: {:?}, 面积: {}", rectangle, rectangle.area());
    
    // 10. 实际应用：泛型缓存结构体
    println!("\n📖 10. 实际应用：泛型缓存结构体");
    
    use std::collections::HashMap;
    use std::hash::Hash;
    
    #[derive(Debug)]
    struct Cache<K, V> {
        data: HashMap<K, V>,
        max_size: usize,
    }
    
    impl<K, V> Cache<K, V> 
    where 
        K: Eq + Hash + Clone,
        V: Clone,
    {
        fn new(max_size: usize) -> Self {
            Cache {
                data: HashMap::new(),
                max_size,
            }
        }
        
        fn insert(&mut self, key: K, value: V) {
            if self.data.len() >= self.max_size {
                // 简单的清理策略：清空缓存
                self.data.clear();
            }
            self.data.insert(key, value);
        }
        
        fn get(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }
        
        fn size(&self) -> usize {
            self.data.len()
        }
    }
    
    let mut cache = Cache::new(3);
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    cache.insert("key3", "value3");
    
    println!("缓存大小: {}", cache.size());
    println!("获取key1: {:?}", cache.get(&"key1"));
    
    // 触发缓存清理
    cache.insert("key4", "value4");
    println!("插入key4后缓存大小: {}", cache.size());
    
    println!("\n🎉 泛型结构体学习完成！");
    println!("💡 关键要点：");
    println!("   • 泛型结构体让数据结构更加灵活");
    println!("   • 可以有多个泛型参数");
    println!("   • 方法实现可以添加特征约束");
    println!("   • 条件实现允许为特定类型添加专门方法");
    println!("   • 泛型结构体支持生命周期参数");
    println!("   • 嵌套泛型结构体可以构建复杂数据结构");
} 