// 例子7: Where子句
// 这个例子将详细讲解Where子句的各种用法和优势

pub fn run() {
    println!("\n🎯 例子7: Where子句");
    println!("===================");
    
    // 1. 基本Where子句语法
    println!("\n📖 1. 基本Where子句语法");
    
    // 传统的特征约束写法
    fn old_style<T: std::fmt::Display + Clone + std::fmt::Debug>(item: T) {
        println!("传统写法: {}", item);
    }
    
    // 使用where子句的写法
    fn new_style<T>(item: T) 
    where 
        T: std::fmt::Display + Clone + std::fmt::Debug,
    {
        println!("Where子句写法: {}", item);
    }
    
    old_style(42);
    new_style(String::from("Hello"));
    
    // 2. 复杂约束的可读性提升
    println!("\n📖 2. 复杂约束的可读性提升");
    
    // 难以阅读的传统写法
    fn complex_old<T: std::fmt::Display + Clone, U: std::fmt::Debug + PartialEq<U>, V: Iterator<Item = T>>(
        _t: T, _u: U, _v: V
    ) -> String {
        String::from("复杂函数(旧语法)")
    }
    
    // 清晰的where子句写法
    fn complex_new<T, U, V>(_t: T, _u: U, _v: V) -> String 
    where 
        T: std::fmt::Display + Clone,
        U: std::fmt::Debug + PartialEq<U>,
        V: Iterator<Item = T>,
    {
        String::from("清晰复杂函数")
    }
    
    let vec = vec![1, 2, 3];
    let result = complex_new(42, "test", vec.into_iter());
    println!("{}", result);
    
    // 3. 条件实现中的Where子句
    println!("\n📖 3. 条件实现中的Where子句");
    
    struct Wrapper<T> {
        value: T,
    }
    
    impl<T> Wrapper<T> {
        fn new(value: T) -> Self {
            Wrapper { value }
        }
    }
    
    // 只有当T实现了Display时才提供display方法
    impl<T> Wrapper<T> 
    where 
        T: std::fmt::Display,
    {
        fn display(&self) {
            println!("包装的值: {}", self.value);
        }
    }
    
    // 只有当T实现了Clone时才提供clone_value方法
    impl<T> Wrapper<T> 
    where 
        T: Clone,
    {
        fn clone_value(&self) -> T {
            self.value.clone()
        }
    }
    
    let wrapper = Wrapper::new(42);
    wrapper.display();
    let cloned = wrapper.clone_value();
    println!("克隆的值: {}", cloned);
    
    // 4. 关联类型的Where约束
    println!("\n📖 4. 关联类型的Where约束");
    
    trait MyIterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    trait Collect<T> {
        fn collect<I>(iter: I) -> Self 
        where 
            I: MyIterator<Item = T>;
    }
    
    impl<T> Collect<T> for Vec<T> {
        fn collect<I>(mut iter: I) -> Self 
        where 
            I: MyIterator<Item = T>,
        {
            let mut result = Vec::new();
            while let Some(item) = iter.next() {
                result.push(item);
            }
            result
        }
    }
    
    struct Counter {
        current: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Self {
            Counter { current: 0, max }
        }
    }
    
    impl MyIterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }
    
    let counter = Counter::new(5);
    let collected: Vec<usize> = Collect::collect(counter);
    println!("收集的计数: {:?}", collected);
    
    // 5. 生命周期参数的Where约束
    println!("\n📖 5. 生命周期参数的Where约束");
    
    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
    where 
        'b: 'a,  // 'b必须比'a活得更久
    {
        if x.len() > y.len() { x } else { y }
    }
    
    let string1 = String::from("long string");
    let string2 = String::from("short");
    let result = longest(&string1, &string2);
    println!("最长的字符串: {}", result);
    
    // 6. 高阶特征约束(HRTB)
    println!("\n📖 6. 高阶特征约束(HRTB)");
    
    fn apply_to_all<F>(f: F) 
    where 
        F: for<'a> Fn(&'a str) -> usize,
    {
        let strings = vec!["hello", "world", "rust"];
        for s in strings {
            println!("字符串 '{}' 的长度: {}", s, f(s));
        }
    }
    
    apply_to_all(|s| s.len());
    
    // 7. 复杂的Where子句组合
    println!("\n📖 7. 复杂的Where子句组合");
    
    trait Processor {
        type Input;
        type Output;
        fn process(&self, input: Self::Input) -> Self::Output;
    }
    
    fn chain_processors<P1, P2>(p1: P1, p2: P2, input: P1::Input) -> P2::Output 
    where 
        P1: Processor,
        P2: Processor<Input = P1::Output>,
        P1::Input: Clone,
        P1::Output: std::fmt::Debug,
        P2::Output: std::fmt::Display,
    {
        let intermediate = p1.process(input);
        println!("中间结果: {:?}", intermediate);
        p2.process(intermediate)
    }
    
    struct Doubler;
    struct Stringifier;
    
    impl Processor for Doubler {
        type Input = i32;
        type Output = i32;
        
        fn process(&self, input: Self::Input) -> Self::Output {
            input * 2
        }
    }
    
    impl Processor for Stringifier {
        type Input = i32;
        type Output = String;
        
        fn process(&self, input: Self::Input) -> Self::Output {
            format!("数字: {}", input)
        }
    }
    
    let result = chain_processors(Doubler, Stringifier, 21);
    println!("链式处理结果: {}", result);
    
    // 8. Where子句与泛型结构体
    println!("\n📖 8. Where子句与泛型结构体");
    
    struct Container<T, U> 
    where 
        T: Clone + std::fmt::Debug,
        U: std::fmt::Display,
    {
        items: Vec<T>,
        metadata: U,
    }
    
    impl<T, U> Container<T, U> 
    where 
        T: Clone + std::fmt::Debug + PartialEq,
        U: std::fmt::Display + Clone,
    {
        fn new(metadata: U) -> Self {
            Container {
                items: Vec::new(),
                metadata,
            }
        }
        
        fn add(&mut self, item: T) {
            self.items.push(item);
        }
        
        fn contains(&self, item: &T) -> bool {
            self.items.contains(item)
        }
        
        fn info(&self) {
            println!("容器信息: {}", self.metadata);
            println!("项目: {:?}", self.items);
        }
    }
    
    let mut container = Container::new("数字容器");
    container.add(1);
    container.add(2);
    container.add(3);
    container.info();
    println!("包含2: {}", container.contains(&2));
    
    // 9. Where子句的性能优化
    println!("\n📖 9. Where子句的性能优化");
    
    trait FastOperation<T> {
        fn fast_op(&self, items: &mut [T]);
    }
    
    struct Optimizer;
    
    // 为Copy类型提供优化实现
    impl<T> FastOperation<T> for Optimizer 
    where 
        T: Copy + Ord,
    {
        fn fast_op(&self, items: &mut [T]) {
            items.sort_unstable();
            println!("使用快速不稳定排序(Copy类型)");
        }
    }
    
    // 注意：在实际应用中，你需要使用不同的特征或更具体的约束来避免冲突
    // 这里我们只保留一个实现作为示例
    
    let optimizer = Optimizer;
    let mut numbers = vec![3, 1, 4, 1, 5];
    optimizer.fast_op(&mut numbers);
    println!("排序结果: {:?}", numbers);
    
    // 10. 实际应用：数据库查询构建器
    println!("\n📖 10. 实际应用：数据库查询构建器");
    
    trait Query {
        type Output;
        fn execute(&self) -> Self::Output;
    }
    
    trait Filterable<T> {
        fn filter<F>(self, predicate: F) -> Self 
        where 
            F: Fn(&T) -> bool + 'static;
    }
    
    struct QueryBuilder<T> 
    where 
        T: Clone + std::fmt::Debug,
    {
        data: Vec<T>,
        filters: Vec<Box<dyn Fn(&T) -> bool>>,
    }
    
    impl<T> QueryBuilder<T> 
    where 
        T: Clone + std::fmt::Debug + 'static,
    {
        fn new(data: Vec<T>) -> Self {
            QueryBuilder {
                data,
                filters: Vec::new(),
            }
        }
    }
    
    impl<T> Filterable<T> for QueryBuilder<T> 
    where 
        T: Clone + std::fmt::Debug + 'static,
    {
        fn filter<F>(mut self, predicate: F) -> Self 
        where 
            F: Fn(&T) -> bool + 'static,
        {
            self.filters.push(Box::new(predicate));
            self
        }
    }
    
    impl<T> Query for QueryBuilder<T> 
    where 
        T: Clone + std::fmt::Debug,
    {
        type Output = Vec<T>;
        
        fn execute(&self) -> Self::Output {
            let mut result = self.data.clone();
            for filter in &self.filters {
                result.retain(|item| filter(item));
            }
            result
        }
    }
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let query = QueryBuilder::new(data)
        .filter(|&x| x > 5)
        .filter(|&x| x % 2 == 0);
    
    let result = query.execute();
    println!("查询结果: {:?}", result);
    
    println!("\n🎉 Where子句学习完成！");
    println!("💡 关键要点：");
    println!("   • Where子句提供更清晰的约束语法");
    println!("   • 适用于复杂的泛型约束场景");
    println!("   • 支持关联类型和生命周期约束");
    println!("   • 可以实现条件编译和性能优化");
    println!("   • 是构建复杂泛型系统的重要工具");
} 