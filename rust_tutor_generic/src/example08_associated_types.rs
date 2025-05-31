// 例子8: 关联类型
// 这个例子将详细讲解关联类型的概念、用法和优势

pub fn run() {
    println!("\n🎯 例子8: 关联类型");
    println!("===================");
    
    // 1. 基本关联类型概念
    println!("\n📖 1. 基本关联类型概念");
    
    // 使用泛型参数的方式
    trait IteratorGeneric<T> {
        fn next(&mut self) -> Option<T>;
    }
    
    // 使用关联类型的方式
    trait Iterator {
        type Item;  // 关联类型
        fn next(&mut self) -> Option<Self::Item>;
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
    
    impl Iterator for Counter {
        type Item = usize;  // 指定关联类型
        
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
    
    let mut counter = Counter::new(3);
    println!("计数器输出:");
    while let Some(num) = counter.next() {
        println!("  {}", num);
    }
    
    // 2. 关联类型 vs 泛型参数
    println!("\n📖 2. 关联类型 vs 泛型参数");
    
    // 泛型参数版本 - 可以为同一类型实现多次
    trait ConvertGeneric<T, U> {
        fn convert(&self, input: T) -> U;
    }
    
    // 关联类型版本 - 每个类型只能有一个实现
    trait Convert {
        type Input;
        type Output;
        fn convert(&self, input: Self::Input) -> Self::Output;
    }
    
    struct StringProcessor;
    
    impl Convert for StringProcessor {
        type Input = String;
        type Output = usize;
        
        fn convert(&self, input: Self::Input) -> Self::Output {
            input.len()
        }
    }
    
    let processor = StringProcessor;
    let length = processor.convert(String::from("Hello World"));
    println!("字符串长度: {}", length);
    
    // 3. 复杂的关联类型示例
    println!("\n📖 3. 复杂的关联类型示例");
    
    trait Graph {
        type Node;
        type Edge;
        
        fn nodes(&self) -> Vec<Self::Node>;
        fn edges(&self) -> Vec<Self::Edge>;
        fn add_node(&mut self, node: Self::Node);
        fn add_edge(&mut self, edge: Self::Edge);
    }
    
    #[derive(Debug, Clone)]
    struct SimpleNode {
        id: usize,
        name: String,
    }
    
    #[derive(Debug, Clone)]
    struct SimpleEdge {
        from: usize,
        to: usize,
        weight: f64,
    }
    
    struct SimpleGraph {
        nodes: Vec<SimpleNode>,
        edges: Vec<SimpleEdge>,
    }
    
    impl SimpleGraph {
        fn new() -> Self {
            SimpleGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            }
        }
    }
    
    impl Graph for SimpleGraph {
        type Node = SimpleNode;
        type Edge = SimpleEdge;
        
        fn nodes(&self) -> Vec<Self::Node> {
            self.nodes.clone()
        }
        
        fn edges(&self) -> Vec<Self::Edge> {
            self.edges.clone()
        }
        
        fn add_node(&mut self, node: Self::Node) {
            self.nodes.push(node);
        }
        
        fn add_edge(&mut self, edge: Self::Edge) {
            self.edges.push(edge);
        }
    }
    
    let mut graph = SimpleGraph::new();
    graph.add_node(SimpleNode { id: 1, name: "节点1".to_string() });
    graph.add_node(SimpleNode { id: 2, name: "节点2".to_string() });
    graph.add_edge(SimpleEdge { from: 1, to: 2, weight: 1.5 });
    
    println!("图的节点: {:?}", graph.nodes());
    println!("图的边: {:?}", graph.edges());
    
    // 4. 关联类型的约束
    println!("\n📖 4. 关联类型的约束");
    
    trait Collect {
        type Item;
        type Collection;
        
        fn collect<I>(iter: I) -> Self::Collection 
        where 
            I: Iterator<Item = Self::Item>,
            Self::Item: Clone;
    }
    
    struct VecCollector;
    
    impl Collect for VecCollector {
        type Item = i32;
        type Collection = Vec<i32>;
        
        fn collect<I>(mut iter: I) -> Self::Collection 
        where 
            I: Iterator<Item = Self::Item>,
            Self::Item: Clone,
        {
            let mut result = Vec::new();
            while let Some(item) = iter.next() {
                result.push(item);
            }
            result
        }
    }
    
    let _counter = Counter::new(5);
    // 注意：这里需要类型转换，因为Counter::Item是usize，而VecCollector::Item是i32
    println!("收集器示例需要类型匹配");
    
    // 5. 关联类型与生命周期
    println!("\n📖 5. 关联类型与生命周期");
    
    trait Borrower<'a> {
        type Borrowed;
        fn borrow(&'a self) -> Self::Borrowed;
    }
    
    struct Container<T> {
        items: Vec<T>,
    }
    
    impl<T> Container<T> {
        fn new() -> Self {
            Container { items: Vec::new() }
        }
        
        fn add(&mut self, item: T) {
            self.items.push(item);
        }
    }
    
    impl<'a, T: 'a> Borrower<'a> for Container<T> {
        type Borrowed = &'a [T];
        
        fn borrow(&'a self) -> Self::Borrowed {
            &self.items
        }
    }
    
    let mut container = Container::new();
    container.add(1);
    container.add(2);
    container.add(3);
    
    let borrowed = container.borrow();
    println!("借用的切片: {:?}", borrowed);
    
    // 6. 关联类型的默认值
    println!("\n📖 6. 关联类型的默认值");
    
    trait DefaultAssociated {
        type Output; // 移除默认值，因为它是不稳定特性
        
        fn process(&self) -> Self::Output;
    }
    
    struct Processor1;
    struct Processor2;
    
    // 为Processor1指定String类型
    impl DefaultAssociated for Processor1 {
        type Output = String;
        
        fn process(&self) -> Self::Output {
            "使用String类型".to_string()
        }
    }
    
    // 为Processor2指定i32类型
    impl DefaultAssociated for Processor2 {
        type Output = i32;
        
        fn process(&self) -> Self::Output {
            42
        }
    }
    
    let p1 = Processor1;
    let p2 = Processor2;
    
    println!("处理器1结果: {}", p1.process());
    println!("处理器2结果: {}", p2.process());
    
    // 7. 关联类型与泛型函数
    println!("\n📖 7. 关联类型与泛型函数");
    
    fn process_iterator<I>(mut iter: I) 
    where 
        I: Iterator,
        I::Item: std::fmt::Display,
    {
        println!("处理迭代器:");
        while let Some(item) = iter.next() {
            println!("  项目: {}", item);
        }
    }
    
    let counter = Counter::new(3);
    process_iterator(counter);
    
    // 8. 复杂的关联类型约束
    println!("\n📖 8. 复杂的关联类型约束");
    
    trait Parser {
        type Input;
        type Output;
        type Error;
        
        fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
    }
    
    trait Combinator: Parser {
        fn map<F, U>(self, f: F) -> Map<Self, F> 
        where 
            Self: Sized,
            F: Fn(Self::Output) -> U,
        {
            Map { parser: self, func: f }
        }
    }
    
    struct Map<P, F> {
        parser: P,
        func: F,
    }
    
    impl<P, F, U> Parser for Map<P, F> 
    where 
        P: Parser,
        F: Fn(P::Output) -> U,
    {
        type Input = P::Input;
        type Output = U;
        type Error = P::Error;
        
        fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            match self.parser.parse(input) {
                Ok(output) => Ok((self.func)(output)),
                Err(error) => Err(error),
            }
        }
    }
    
    impl<P: Parser> Combinator for P {}
    
    struct NumberParser;
    
    impl Parser for NumberParser {
        type Input = String;
        type Output = i32;
        type Error = String;
        
        fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            input.parse().map_err(|_| format!("无法解析数字: {}", input))
        }
    }
    
    let parser = NumberParser;
    let mapped_parser = parser.map(|x| x * 2);
    
    match mapped_parser.parse("42".to_string()) {
        Ok(result) => println!("解析并映射结果: {}", result),
        Err(error) => println!("解析错误: {}", error),
    }
    
    // 9. 关联类型与特征对象
    println!("\n📖 9. 关联类型与特征对象");
    
    trait Drawable {
        type Canvas;
        fn draw(&self, canvas: &mut Self::Canvas);
    }
    
    struct Circle {
        radius: f64,
    }
    
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    type Canvas = Vec<String>;
    
    impl Drawable for Circle {
        type Canvas = Canvas;
        
        fn draw(&self, canvas: &mut Self::Canvas) {
            canvas.push(format!("绘制圆形，半径: {}", self.radius));
        }
    }
    
    impl Drawable for Rectangle {
        type Canvas = Canvas;
        
        fn draw(&self, canvas: &mut Self::Canvas) {
            canvas.push(format!("绘制矩形，宽: {}, 高: {}", self.width, self.height));
        }
    }
    
    fn draw_shape<D: Drawable<Canvas = Canvas>>(shape: &D, canvas: &mut Canvas) {
        shape.draw(canvas);
    }
    
    let mut canvas = Vec::new();
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 20.0 };
    
    draw_shape(&circle, &mut canvas);
    draw_shape(&rectangle, &mut canvas);
    
    println!("画布内容:");
    for line in canvas {
        println!("  {}", line);
    }
    
    // 10. 实际应用：序列化框架
    println!("\n📖 10. 实际应用：序列化框架");
    
    trait Serializer {
        type Output;
        type Error;
        
        fn serialize<T>(&self, value: &T) -> Result<Self::Output, Self::Error> 
        where 
            T: Serialize<Self>,
            Self: Sized;
    }
    
    trait Serialize<S: Serializer + ?Sized> {
        fn serialize(&self, serializer: &S) -> Result<S::Output, S::Error>;
    }
    
    struct JsonSerializer;
    
    impl Serializer for JsonSerializer {
        type Output = String;
        type Error = String;
        
        fn serialize<T>(&self, value: &T) -> Result<Self::Output, Self::Error> 
        where 
            T: Serialize<Self>,
        {
            value.serialize(self)
        }
    }
    
    impl Serialize<JsonSerializer> for i32 {
        fn serialize(&self, _serializer: &JsonSerializer) -> Result<String, String> {
            Ok(self.to_string())
        }
    }
    
    impl Serialize<JsonSerializer> for String {
        fn serialize(&self, _serializer: &JsonSerializer) -> Result<String, String> {
            Ok(format!("\"{}\"", self))
        }
    }
    
    let serializer = JsonSerializer;
    
    match serializer.serialize(&42) {
        Ok(json) => println!("序列化数字: {}", json),
        Err(error) => println!("序列化错误: {}", error),
    }
    
    match serializer.serialize(&"Hello".to_string()) {
        Ok(json) => println!("序列化字符串: {}", json),
        Err(error) => println!("序列化错误: {}", error),
    }
    
    println!("\n🎉 关联类型学习完成！");
    println!("💡 关键要点：");
    println!("   • 关联类型提供了类型级别的抽象");
    println!("   • 每个实现只能指定一次关联类型");
    println!("   • 关联类型可以有约束和默认值");
    println!("   • 适用于类型之间有固定关系的场景");
    println!("   • 比泛型参数更加简洁和类型安全");
    println!("   • 是构建复杂类型系统的重要工具");
} 