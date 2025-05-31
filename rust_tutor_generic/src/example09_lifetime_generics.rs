// 例子9: 生命周期与泛型
// 这个例子将详细讲解生命周期与泛型的结合使用

pub fn run() {
    println!("\n🎯 例子9: 生命周期与泛型");
    println!("==========================");
    
    // 1. 基本生命周期泛型
    println!("\n📖 1. 基本生命周期泛型");
    
    // 泛型结构体包含引用
    #[derive(Debug)]
    struct Holder<'a, T> {
        value: &'a T,
        name: &'a str,
    }
    
    impl<'a, T> Holder<'a, T> {
        fn new(value: &'a T, name: &'a str) -> Self {
            Holder { value, name }
        }
        
        fn get_value(&self) -> &T {
            self.value
        }
        
        fn get_name(&self) -> &str {
            self.name
        }
    }
    
    let number = 42;
    let name = "数字容器";
    let holder = Holder::new(&number, name);
    
    println!("容器: {:?}", holder);
    println!("值: {}, 名称: {}", holder.get_value(), holder.get_name());
    
    // 2. 生命周期约束的泛型函数
    println!("\n📖 2. 生命周期约束的泛型函数");
    
    // 函数返回较长生命周期的引用
    fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T 
    where 
        T: PartialOrd,
    {
        if x > y { x } else { y }
    }
    
    let num1 = 10;
    let num2 = 20;
    let larger = longest(&num1, &num2);
    println!("较大的数字: {}", larger);
    
    let str1 = "hello";
    let str2 = "world";
    let longer = longest(&str1, &str2);
    println!("较大的字符串: {}", longer);
    
    // 3. 复杂的生命周期泛型结构体
    println!("\n📖 3. 复杂的生命周期泛型结构体");
    
    struct Parser<'a, T> 
    where 
        T: 'a,  // T必须至少活得和'a一样久
    {
        input: &'a str,
        position: usize,
        _phantom: std::marker::PhantomData<T>,
    }
    
    impl<'a, T> Parser<'a, T> 
    where 
        T: std::str::FromStr + 'a,
        T::Err: std::fmt::Debug,
    {
        fn new(input: &'a str) -> Self {
            Parser {
                input,
                position: 0,
                _phantom: std::marker::PhantomData,
            }
        }
        
        fn parse_next(&mut self) -> Option<T> {
            let remaining = &self.input[self.position..];
            if let Some(space_pos) = remaining.find(' ') {
                let token = &remaining[..space_pos];
                self.position += space_pos + 1;
                token.parse().ok()
            } else if !remaining.is_empty() {
                let token = remaining;
                self.position = self.input.len();
                token.parse().ok()
            } else {
                None
            }
        }
    }
    
    let input = "42 3.14 100";
    let mut int_parser: Parser<i32> = Parser::new(input);
    
    println!("解析整数:");
    while let Some(num) = int_parser.parse_next() {
        println!("  解析到: {}", num);
    }
    
    // 4. 生命周期子类型化
    println!("\n📖 4. 生命周期子类型化");
    
    fn process_data<'a, 'b, T>(long_lived: &'a T, short_lived: &'b T) -> &'a T 
    where 
        'b: 'a,  // 'b必须比'a活得更久
        T: std::fmt::Display,
    {
        println!("处理长期数据: {}", long_lived);
        println!("处理短期数据: {}", short_lived);
        long_lived
    }
    
    let long_data = String::from("长期数据");
    let short_data = String::from("短期数据");
    let result = process_data(&long_data, &short_data);
    println!("返回的数据: {}", result);
    
    // 5. 高阶生命周期约束(HRTB)
    println!("\n📖 5. 高阶生命周期约束(HRTB)");
    
    fn apply_closure<F, T>(f: F, data: &T) -> String 
    where 
        F: for<'a> Fn(&'a T) -> String,
        T: ?Sized,
    {
        f(data)
    }
    
    let closure = |s: &str| format!("处理: {}", s);
    let text = "测试文本";
    let result = apply_closure(closure, text);
    println!("闭包结果: {}", result);
    
    // 6. 生命周期与特征对象
    println!("\n📖 6. 生命周期与特征对象");
    
    trait Processor<'a> {
        type Output;
        fn process(&self, input: &'a str) -> Self::Output;
    }
    
    struct UpperCaseProcessor;
    struct LengthProcessor;
    
    impl<'a> Processor<'a> for UpperCaseProcessor {
        type Output = String;
        
        fn process(&self, input: &'a str) -> Self::Output {
            input.to_uppercase()
        }
    }
    
    impl<'a> Processor<'a> for LengthProcessor {
        type Output = usize;
        
        fn process(&self, input: &'a str) -> Self::Output {
            input.len()
        }
    }
    
    fn process_with<'a, P>(processor: &P, input: &'a str) -> P::Output 
    where 
        P: Processor<'a>,
    {
        processor.process(input)
    }
    
    let text = "hello world";
    let upper_processor = UpperCaseProcessor;
    let length_processor = LengthProcessor;
    
    let upper_result = process_with(&upper_processor, text);
    let length_result = process_with(&length_processor, text);
    
    println!("大写结果: {}", upper_result);
    println!("长度结果: {}", length_result);
    
    // 7. 自引用结构体
    println!("\n📖 7. 自引用结构体");
    
    struct SelfReferential<'a, T> {
        data: T,
        reference: Option<&'a T>,
    }
    
    impl<'a, T> SelfReferential<'a, T> {
        fn new(data: T) -> Self {
            SelfReferential {
                data,
                reference: None,
            }
        }
        
        // 注意：这种模式在实际中很难使用，通常需要Pin或其他技术
        fn get_data(&self) -> &T {
            &self.data
        }
    }
    
    let self_ref = SelfReferential::new(String::from("自引用数据"));
    println!("自引用结构体数据: {}", self_ref.get_data());
    
    // 8. 生命周期与迭代器
    println!("\n📖 8. 生命周期与迭代器");
    
    struct WindowIterator<'a, T> {
        data: &'a [T],
        window_size: usize,
        position: usize,
    }
    
    impl<'a, T> WindowIterator<'a, T> {
        fn new(data: &'a [T], window_size: usize) -> Self {
            WindowIterator {
                data,
                window_size,
                position: 0,
            }
        }
    }
    
    impl<'a, T> Iterator for WindowIterator<'a, T> {
        type Item = &'a [T];
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.position + self.window_size <= self.data.len() {
                let window = &self.data[self.position..self.position + self.window_size];
                self.position += 1;
                Some(window)
            } else {
                None
            }
        }
    }
    
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let mut window_iter = WindowIterator::new(&numbers, 3);
    
    println!("滑动窗口迭代:");
    while let Some(window) = window_iter.next() {
        println!("  窗口: {:?}", window);
    }
    
    // 9. 生命周期与异步编程模拟
    println!("\n📖 9. 生命周期与异步编程模拟");
    
    struct Future<'a, T> {
        data: &'a T,
        ready: bool,
    }
    
    impl<'a, T> Future<'a, T> 
    where 
        T: Clone,
    {
        fn new(data: &'a T) -> Self {
            Future { data, ready: false }
        }
        
        fn poll(&mut self) -> Option<T> {
            if !self.ready {
                self.ready = true;
                None  // 模拟未就绪
            } else {
                Some(self.data.clone())  // 模拟就绪
            }
        }
    }
    
    let data = String::from("异步数据");
    let mut future = Future::new(&data);
    
    println!("模拟异步轮询:");
    loop {
        match future.poll() {
            Some(result) => {
                println!("  异步结果: {}", result);
                break;
            }
            None => {
                println!("  等待中...");
            }
        }
    }
    
    // 10. 实际应用：缓存系统
    println!("\n📖 10. 实际应用：缓存系统");
    
    use std::collections::HashMap;
    
    struct Cache<'a, K, V> 
    where 
        K: std::hash::Hash + Eq + 'a,
        V: 'a,
    {
        data: HashMap<&'a K, &'a V>,
        default_value: &'a V,
    }
    
    impl<'a, K, V> Cache<'a, K, V> 
    where 
        K: std::hash::Hash + Eq + 'a,
        V: 'a,
    {
        fn new(default_value: &'a V) -> Self {
            Cache {
                data: HashMap::new(),
                default_value,
            }
        }
        
        fn insert(&mut self, key: &'a K, value: &'a V) {
            self.data.insert(key, value);
        }
        
        fn get(&self, key: &K) -> &'a V {
            self.data.get(key).unwrap_or(&self.default_value)
        }
        
        fn contains_key(&self, key: &K) -> bool {
            self.data.contains_key(key)
        }
    }
    
    let default_name = String::from("未知");
    let mut cache = Cache::new(&default_name);
    
    let key1 = String::from("user1");
    let value1 = String::from("Alice");
    let key2 = String::from("user2");
    let value2 = String::from("Bob");
    
    cache.insert(&key1, &value1);
    cache.insert(&key2, &value2);
    
    println!("缓存查询:");
    println!("  user1: {}", cache.get(&key1));
    println!("  user2: {}", cache.get(&key2));
    
    let unknown_key = String::from("user3");
    println!("  user3: {}", cache.get(&unknown_key));
    println!("  包含user1: {}", cache.contains_key(&key1));
    
    println!("\n🎉 生命周期与泛型学习完成！");
    println!("💡 关键要点：");
    println!("   • 生命周期参数确保引用的有效性");
    println!("   • 生命周期约束控制类型参数的生命周期");
    println!("   • HRTB允许处理任意生命周期的闭包");
    println!("   • 生命周期子类型化提供灵活性");
    println!("   • 自引用结构体需要特殊处理");
    println!("   • 生命周期与泛型结合提供强大的抽象能力");
} 