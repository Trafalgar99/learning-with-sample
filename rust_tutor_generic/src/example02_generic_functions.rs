// 例子2: 泛型函数
// 这个例子将深入探讨泛型函数的各种用法和高级技巧

pub fn run() {
    println!("\n🎯 例子2: 泛型函数");
    println!("===================");
    
    // 1. 基本泛型函数
    println!("\n📖 1. 基本泛型函数");
    
    // 简单的泛型函数
    fn identity<T>(x: T) -> T {
        x  // 直接返回输入值
    }
    
    let num = identity(42);
    let text = identity("Hello");
    let flag = identity(true);
    
    println!("恒等函数测试:");
    println!("数字: {}, 文本: {}, 布尔: {}", num, text, flag);
    
    // 2. 带约束的泛型函数
    println!("\n📖 2. 带约束的泛型函数");
    
    // 需要实现Display特征的泛型函数
    fn print_twice<T: std::fmt::Display>(x: T) {
        println!("第一次: {}", x);
        println!("第二次: {}", x);
    }
    
    print_twice("泛型函数");
    print_twice(123);
    
    // 需要实现Clone特征的泛型函数
    fn make_pair<T: Clone>(x: T) -> (T, T) {
        (x.clone(), x.clone())
    }
    
    let pair = make_pair(String::from("复制我"));
    println!("克隆对: ({}, {})", pair.0, pair.1);
    
    // 3. 多个泛型参数
    println!("\n📖 3. 多个泛型参数");
    
    // 两个不同类型的泛型参数
    fn combine<T, U>(first: T, second: U) -> String 
    where 
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        format!("{} + {} = 组合", first, second)
    }
    
    println!("{}", combine(42, "世界"));
    println!("{}", combine("Hello", 3.14));
    println!("{}", combine(true, 'A'));
    
    // 4. 返回泛型类型
    println!("\n📖 4. 返回泛型类型");
    
    // 返回第一个元素
    fn first<T: Clone>(list: &[T]) -> Option<T> {
        if list.is_empty() {
            None
        } else {
            Some(list[0].clone())
        }
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let words = vec!["apple", "banana", "cherry"];
    
    match first(&numbers) {
        Some(n) => println!("第一个数字: {}", n),
        None => println!("列表为空"),
    }
    
    match first(&words) {
        Some(w) => println!("第一个单词: {}", w),
        None => println!("列表为空"),
    }
    
    // 5. 泛型函数与闭包
    println!("\n📖 5. 泛型函数与闭包");
    
    // 接受闭包作为参数的泛型函数
    fn apply_operation<T, F>(x: T, y: T, op: F) -> T 
    where 
        F: Fn(T, T) -> T,
    {
        op(x, y)
    }
    
    let add = |a, b| a + b;
    let multiply = |a, b| a * b;
    
    println!("5 + 3 = {}", apply_operation(5, 3, add));
    println!("5 * 3 = {}", apply_operation(5, 3, multiply));
    
    // 字符串连接
    let concat = |a: String, b: String| format!("{}{}", a, b);
    println!("字符串连接: {}", 
        apply_operation(String::from("Hello"), String::from("World"), concat));
    
    // 6. 条件泛型实现
    println!("\n📖 6. 条件泛型实现");
    
    // 只有当T实现了PartialEq时才能比较
    fn are_equal<T: PartialEq>(a: T, b: T) -> bool {
        a == b
    }
    
    println!("42 == 42: {}", are_equal(42, 42));
    println!("'a' == 'b': {}", are_equal('a', 'b'));
    println!("\"hello\" == \"hello\": {}", are_equal("hello", "hello"));
    
    // 7. 泛型函数的类型推断
    println!("\n📖 7. 泛型函数的类型推断");
    
    fn create_vector<T>() -> Vec<T> {
        Vec::new()
    }
    
    // 编译器可以从使用方式推断类型
    let mut int_vec: Vec<i32> = create_vector();
    int_vec.push(1);
    int_vec.push(2);
    
    // 或者显式指定类型
    let mut string_vec = create_vector::<String>();
    string_vec.push(String::from("第一个"));
    string_vec.push(String::from("第二个"));
    
    println!("整数向量: {:?}", int_vec);
    println!("字符串向量: {:?}", string_vec);
    
    // 8. 高级泛型函数示例
    println!("\n📖 8. 高级泛型函数示例");
    
    // 泛型映射函数
    fn map_vector<T, U, F>(vec: Vec<T>, f: F) -> Vec<U> 
    where 
        F: Fn(T) -> U,
    {
        vec.into_iter().map(f).collect()
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = map_vector(numbers, |x| x * 2);
    let strings = map_vector(doubled, |x| format!("数字{}", x));
    
    println!("映射结果: {:?}", strings);
    
    // 泛型过滤函数
    fn filter_vector<T, F>(vec: Vec<T>, predicate: F) -> Vec<T> 
    where 
        F: Fn(&T) -> bool,
    {
        vec.into_iter().filter(predicate).collect()
    }
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = filter_vector(numbers, |&x| x % 2 == 0);
    println!("偶数: {:?}", evens);
    
    // 9. 泛型函数的性能考虑
    println!("\n📖 9. 泛型函数的性能考虑");
    println!("🚀 零成本抽象：泛型函数在编译时会被单态化");
    println!("   例如：identity::<i32> 和 identity::<String> 是不同的函数");
    println!("   运行时性能等同于手写的具体类型函数！");
    
    // 10. 常见错误和解决方案
    println!("\n📖 10. 常见错误和解决方案");
    
    // 错误示例（注释掉，因为不能编译）
    // fn bad_function<T>(x: T) -> T {
    //     x + 1  // 错误：T可能不支持+运算
    // }
    
    // 正确的做法：添加适当的约束
    fn good_function<T>(x: T) -> T 
    where 
        T: std::ops::Add<Output = T> + From<i32>,
    {
        x + T::from(1)
    }
    
    println!("正确的泛型函数: {}", good_function(41));
    println!("正确的泛型函数: {}", good_function(3.14));
    
    println!("\n🎉 泛型函数学习完成！");
    println!("💡 关键要点：");
    println!("   • 泛型函数提供类型安全的代码重用");
    println!("   • 使用特征约束确保类型具有所需功能");
    println!("   • 编译器会进行类型推断，减少冗余");
    println!("   • 单态化确保零运行时开销");
    println!("   • 合理使用约束避免编译错误");
} 