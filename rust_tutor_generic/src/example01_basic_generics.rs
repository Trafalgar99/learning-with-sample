// 例子1: 基础泛型概念
// 这个例子将介绍什么是泛型，为什么需要泛型，以及最基本的泛型语法

pub fn run() {
    println!("\n🎯 例子1: 基础泛型概念");
    println!("========================");
    
    // 1. 为什么需要泛型？
    println!("\n📖 1. 为什么需要泛型？");
    println!("想象一下，如果我们要为不同类型实现相同的逻辑...");
    
    // 没有泛型的情况 - 需要为每种类型写重复代码
    fn find_largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    fn find_largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    
    println!("最大的数字: {}", find_largest_i32(&numbers));
    println!("最大的字符: {}", find_largest_char(&chars));
    println!("❌ 问题：代码重复，维护困难！");
    
    // 2. 使用泛型解决问题
    println!("\n📖 2. 使用泛型解决问题");
    
    // 泛型函数 - 一个函数处理多种类型
    fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    println!("使用泛型函数:");
    println!("最大的数字: {}", find_largest(&numbers));
    println!("最大的字符: {}", find_largest(&chars));
    println!("✅ 优势：一个函数，多种类型！");
    
    // 3. 泛型语法解释
    println!("\n📖 3. 泛型语法解释");
    println!("fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T");
    println!("                ↑                      ↑        ↑");
    println!("                |                      |        |");
    println!("            泛型参数T              参数类型    返回类型");
    println!("        (必须实现PartialOrd和Copy)");
    
    // 4. 多个泛型参数
    println!("\n📖 4. 多个泛型参数");
    
    // 可以有多个泛型参数
    fn display_pair<T, U>(x: T, y: U) 
    where 
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("配对显示: {} 和 {}", x, y);
    }
    
    display_pair(42, "hello");
    display_pair(3.14, 'A');
    display_pair("world", 100);
    
    // 5. 泛型的编译时特性
    println!("\n📖 5. 泛型的编译时特性");
    println!("🔍 重要概念：单态化(Monomorphization)");
    println!("编译器会为每种具体类型生成专门的代码");
    println!("例如：find_largest::<i32> 和 find_largest::<char>");
    println!("这意味着泛型在运行时没有性能开销！");
    
    // 6. 常见的泛型约束
    println!("\n📖 6. 常见的泛型约束");
    
    // Clone约束
    fn duplicate<T: Clone>(x: T) -> (T, T) {
        (x.clone(), x.clone())
    }
    
    let original = String::from("Hello");
    let (copy1, copy2) = duplicate(original);
    println!("克隆结果: '{}' 和 '{}'", copy1, copy2);
    
    // Debug约束
    fn debug_print<T: std::fmt::Debug>(x: T) {
        println!("调试输出: {:?}", x);
    }
    
    debug_print(vec![1, 2, 3]);
    debug_print("调试字符串");
    
    // 7. 实际应用示例
    println!("\n📖 7. 实际应用示例");
    
    // 泛型容器
    struct Container<T> {
        value: T,
    }
    
    impl<T> Container<T> {
        fn new(value: T) -> Self {
            Container { value }
        }
        
        fn get(&self) -> &T {
            &self.value
        }
    }
    
    let int_container = Container::new(42);
    let string_container = Container::new(String::from("泛型容器"));
    
    println!("整数容器: {}", int_container.get());
    println!("字符串容器: {}", string_container.get());
    
    println!("\n🎉 基础泛型概念学习完成！");
    println!("💡 关键要点：");
    println!("   • 泛型让代码更加通用和可重用");
    println!("   • 使用 <T> 语法定义泛型参数");
    println!("   • 特征约束确保类型具有所需功能");
    println!("   • 编译时单态化保证运行时性能");
} 