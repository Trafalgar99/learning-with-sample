// 例子6: 特征约束(Trait Bounds)
// 这个例子将详细讲解特征约束的各种用法和高级技巧

pub fn run() {
    println!("\n🎯 例子6: 特征约束(Trait Bounds)");
    println!("=================================");
    
    // 1. 基本特征约束
    println!("\n📖 1. 基本特征约束");
    
    // 单个特征约束
    fn print_debug<T: std::fmt::Debug>(item: T) {
        println!("调试输出: {:?}", item);
    }
    
    // 多个特征约束
    fn print_and_clone<T: std::fmt::Display + Clone>(item: T) -> T {
        println!("显示: {}", item);
        item.clone()
    }
    
    print_debug(42);
    print_debug("Hello");
    print_debug(vec![1, 2, 3]);
    
    let cloned = print_and_clone(String::from("测试"));
    println!("克隆结果: {}", cloned);
    
    // 2. 使用where子句的特征约束
    println!("\n📖 2. 使用where子句的特征约束");
    
    fn complex_function<T, U>(t: T, u: U) -> String 
    where 
        T: std::fmt::Display + Clone,
        U: std::fmt::Debug + PartialEq<U>,
    {
        format!("T: {}, U: {:?}", t, u)
    }
    
    let result = complex_function("Hello", 42);
    println!("复杂函数结果: {}", result);
    
    // 3. 结构体的特征约束
    println!("\n📖 3. 结构体的特征约束");
    
    #[derive(Debug)]
    struct Pair<T> 
    where 
        T: std::fmt::Display + PartialOrd,
    {
        first: T,
        second: T,
    }
    
    impl<T> Pair<T> 
    where 
        T: std::fmt::Display + PartialOrd + Copy,
    {
        fn new(first: T, second: T) -> Self {
            Pair { first, second }
        }
        
        fn larger(&self) -> T {
            if self.first > self.second {
                self.first
            } else {
                self.second
            }
        }
        
        fn display(&self) {
            println!("对: ({}, {})", self.first, self.second);
        }
    }
    
    let pair = Pair::new(10, 20);
    pair.display();
    println!("较大值: {}", pair.larger());
    
    println!("\n🎉 特征约束基础学习完成！");
    println!("💡 关键要点：");
    println!("   • 特征约束确保泛型类型具有所需功能");
    println!("   • where子句提供更清晰的约束语法");
    println!("   • 特征约束是Rust零成本抽象的核心");
} 