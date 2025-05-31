// 例子5: 泛型特征(Traits)
// 这个例子将详细讲解如何定义和使用泛型特征

pub fn run() {
    println!("\n🎯 例子5: 泛型特征(Traits)");
    println!("==========================");
    
    // 1. 基本泛型特征
    println!("\n📖 1. 基本泛型特征");
    
    // 定义一个泛型特征
    trait Container<T> {
        fn new() -> Self;
        fn add(&mut self, item: T);
        fn get(&self, index: usize) -> Option<&T>;
        fn len(&self) -> usize;
    }
    
    // 为Vec实现Container特征
    impl<T> Container<T> for Vec<T> {
        fn new() -> Self {
            Vec::new()
        }
        
        fn add(&mut self, item: T) {
            self.push(item);
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            self.as_slice().get(index)
        }
        
        fn len(&self) -> usize {
            Vec::len(self)
        }
    }
    
    let mut container: Vec<i32> = Container::new();
    container.add(1);
    container.add(2);
    container.add(3);
    
    println!("容器长度: {}", container.len());
    println!("第一个元素: {:?}", container.get(0));
    
    // 2. 带关联类型的泛型特征
    println!("\n📖 2. 带关联类型的泛型特征");
    
    trait Iterator<T> {
        type Item;
        
        fn next(&mut self) -> Option<Self::Item>;
        fn collect(self) -> Vec<Self::Item> where Self: Sized;
    }
    
    struct NumberIterator {
        current: i32,
        max: i32,
    }
    
    impl NumberIterator {
        fn new(max: i32) -> Self {
            NumberIterator { current: 0, max }
        }
    }
    
    impl Iterator<i32> for NumberIterator {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
        
        fn collect(mut self) -> Vec<Self::Item> {
            let mut result = Vec::new();
            while let Some(item) = self.next() {
                result.push(item);
            }
            result
        }
    }
    
    let mut iter = NumberIterator::new(5);
    println!("迭代器输出:");
    while let Some(num) = iter.next() {
        println!("  {}", num);
    }
    
    let collected = NumberIterator::new(3).collect();
    println!("收集结果: {:?}", collected);
    
    // 3. 多个泛型参数的特征
    println!("\n📖 3. 多个泛型参数的特征");
    
    trait Converter<From, To> {
        fn convert(from: From) -> To;
    }
    
    struct StringToNumber;
    
    impl Converter<String, i32> for StringToNumber {
        fn convert(from: String) -> i32 {
            from.parse().unwrap_or(0)
        }
    }
    
    impl Converter<i32, String> for StringToNumber {
        fn convert(from: i32) -> String {
            from.to_string()
        }
    }
    
    let number = StringToNumber::convert(String::from("42"));
    let text = StringToNumber::convert(123);
    
    println!("字符串转数字: {}", number);
    println!("数字转字符串: {}", text);
    
    // 4. 泛型特征的默认实现
    println!("\n📖 4. 泛型特征的默认实现");
    
    trait Printable<T> {
        fn print(&self, item: &T);
        
        // 默认实现
        fn print_multiple(&self, items: &[T]) {
            for item in items {
                self.print(item);
            }
        }
    }
    
    struct SimplePrinter;
    
    impl Printable<i32> for SimplePrinter {
        fn print(&self, item: &i32) {
            println!("数字: {}", item);
        }
    }
    
    impl Printable<String> for SimplePrinter {
        fn print(&self, item: &String) {
            println!("字符串: {}", item);
        }
    }
    
    let printer = SimplePrinter;
    let numbers = vec![1, 2, 3];
    let strings = vec![String::from("a"), String::from("b")];
    
    println!("打印多个数字:");
    printer.print_multiple(&numbers);
    
    println!("打印多个字符串:");
    printer.print_multiple(&strings);
    
    // 5. 泛型特征约束
    println!("\n📖 5. 泛型特征约束");
    
    trait Comparable<T> {
        fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
    }
    
    trait Sortable<T> 
    where 
        T: Clone,
        Self: Comparable<T>,
    {
        fn sort(&self, items: &mut [T]) {
            items.sort_by(|a, b| self.compare(a, b));
        }
    }
    
    struct NumberComparator;
    
    impl Comparable<i32> for NumberComparator {
        fn compare(&self, a: &i32, b: &i32) -> std::cmp::Ordering {
            a.cmp(b)
        }
    }
    
    impl Sortable<i32> for NumberComparator {}
    
    let comparator = NumberComparator;
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("排序前: {:?}", numbers);
    
    comparator.sort(&mut numbers);
    println!("排序后: {:?}", numbers);
    
    // 6. 泛型特征对象
    println!("\n📖 6. 泛型特征对象");
    
    trait Drawable {
        fn draw(&self);
    }
    
    struct Circle<T> {
        radius: T,
    }
    
    struct Rectangle<T> {
        width: T,
        height: T,
    }
    
    impl<T: std::fmt::Display> Drawable for Circle<T> {
        fn draw(&self) {
            println!("绘制圆形，半径: {}", self.radius);
        }
    }
    
    impl<T: std::fmt::Display> Drawable for Rectangle<T> {
        fn draw(&self) {
            println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
        }
    }
    
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10, height: 20 }),
        Box::new(Circle { radius: 3 }),
    ];
    
    println!("绘制所有形状:");
    for shape in shapes {
        shape.draw();
    }
    
    // 7. 高阶特征
    println!("\n📖 7. 高阶特征");
    
    trait Mapper<T, U> {
        fn map<F>(&self, items: Vec<T>, f: F) -> Vec<U> 
        where 
            F: Fn(T) -> U;
    }
    
    struct SimpleMapper;
    
    impl<T, U> Mapper<T, U> for SimpleMapper {
        fn map<F>(&self, items: Vec<T>, f: F) -> Vec<U> 
        where 
            F: Fn(T) -> U,
        {
            items.into_iter().map(f).collect()
        }
    }
    
    let mapper = SimpleMapper;
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled = mapper.map(numbers.clone(), |x| x * 2);
    let strings = mapper.map(numbers, |x| format!("数字{}", x));
    
    println!("翻倍结果: {:?}", doubled);
    println!("字符串结果: {:?}", strings);
    
    // 8. 泛型特征的条件实现
    println!("\n📖 8. 泛型特征的条件实现");
    
    trait Display<T> {
        fn display(&self, item: &T);
    }
    
    struct Displayer;
    
    // 只为实现了std::fmt::Display的类型实现Display特征
    impl<T> Display<T> for Displayer 
    where 
        T: std::fmt::Display,
    {
        fn display(&self, item: &T) {
            println!("显示: {}", item);
        }
    }
    
    // 注意：这里我们移除了冲突的Debug实现，因为一个类型可能同时实现Display和Debug
    // 在实际应用中，你需要使用不同的特征或者更具体的约束来避免冲突
    
    let displayer = Displayer;
    displayer.display(&42);
    displayer.display(&"Hello");
    
    // 9. 泛型特征与生命周期
    println!("\n📖 9. 泛型特征与生命周期");
    
    trait Borrower<T> {
        fn borrow(&self) -> &T;
    }
    
    struct Holder<T> {
        value: T,
    }
    
    impl<T> Holder<T> {
        fn new(value: T) -> Self {
            Holder { value }
        }
    }
    
    impl<T> Borrower<T> for Holder<T> {
        fn borrow(&self) -> &T {
            &self.value
        }
    }
    
    let holder = Holder::new(42);
    
    println!("借用的值: {}", holder.borrow());
    
    // 10. 实际应用：序列化特征
    println!("\n📖 10. 实际应用：序列化特征");
    
    trait Serializer<T> {
        type Output;
        
        fn serialize(&self, item: &T) -> Self::Output;
        fn deserialize(&self, data: Self::Output) -> Option<T>;
    }
    
    struct JsonSerializer;
    
    impl Serializer<i32> for JsonSerializer {
        type Output = String;
        
        fn serialize(&self, item: &i32) -> Self::Output {
            item.to_string()
        }
        
        fn deserialize(&self, data: Self::Output) -> Option<i32> {
            data.parse().ok()
        }
    }
    
    impl Serializer<String> for JsonSerializer {
        type Output = String;
        
        fn serialize(&self, item: &String) -> Self::Output {
            format!("\"{}\"", item)
        }
        
        fn deserialize(&self, data: Self::Output) -> Option<String> {
            if data.starts_with('"') && data.ends_with('"') {
                Some(data[1..data.len()-1].to_string())
            } else {
                None
            }
        }
    }
    
    let serializer = JsonSerializer;
    
    let number = 42;
    let serialized_number = serializer.serialize(&number);
    let deserialized_number: Option<i32> = serializer.deserialize(serialized_number);
    println!("数字序列化/反序列化: {} -> {:?}", number, deserialized_number);
    
    let text = String::from("Hello World");
    let serialized_text = serializer.serialize(&text);
    let deserialized_text: Option<String> = serializer.deserialize(serialized_text);
    println!("字符串序列化/反序列化: {} -> {:?}", text, deserialized_text);
    
    println!("\n🎉 泛型特征学习完成！");
    println!("💡 关键要点：");
    println!("   • 泛型特征提供类型安全的多态性");
    println!("   • 关联类型让特征更加灵活");
    println!("   • 可以为泛型特征提供默认实现");
    println!("   • 特征约束确保类型具有所需功能");
    println!("   • 特征对象支持动态分发");
    println!("   • 条件实现允许为特定类型提供专门实现");
} 