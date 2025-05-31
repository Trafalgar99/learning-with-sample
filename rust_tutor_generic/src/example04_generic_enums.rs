// 例子4: 泛型枚举
// 这个例子将详细讲解如何定义和使用泛型枚举

pub fn run() {
    println!("\n🎯 例子4: 泛型枚举");
    println!("===================");
    
    // 1. 基本泛型枚举
    println!("\n📖 1. 基本泛型枚举");
    
    // 定义一个简单的泛型枚举
    #[derive(Debug)]
    enum MyOption<T> {
        Some(T),
        None,
    }
    
    let some_number = MyOption::Some(42);
    let some_string = MyOption::Some(String::from("Hello"));
    let none_value: MyOption<i32> = MyOption::None;
    
    println!("数字选项: {:?}", some_number);
    println!("字符串选项: {:?}", some_string);
    println!("空值选项: {:?}", none_value);
    
    // 2. 多个泛型参数的枚举
    println!("\n📖 2. 多个泛型参数的枚举");
    
    #[derive(Debug)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }
    
    let success: MyResult<i32, String> = MyResult::Ok(100);
    let failure: MyResult<i32, String> = MyResult::Err(String::from("出错了"));
    
    println!("成功结果: {:?}", success);
    println!("失败结果: {:?}", failure);
    
    // 3. 泛型枚举的方法实现
    println!("\n📖 3. 泛型枚举的方法实现");
    
    impl<T> MyOption<T> {
        fn is_some(&self) -> bool {
            match self {
                MyOption::Some(_) => true,
                MyOption::None => false,
            }
        }
        
        fn is_none(&self) -> bool {
            !self.is_some()
        }
        
        fn unwrap(self) -> T {
            match self {
                MyOption::Some(value) => value,
                MyOption::None => panic!("在None值上调用unwrap"),
            }
        }
    }
    
    let option = MyOption::Some("测试值");
    println!("是否有值: {}", option.is_some());
    println!("是否为空: {}", option.is_none());
    println!("解包值: {}", option.unwrap());
    
    // 4. 带约束的泛型枚举方法
    println!("\n📖 4. 带约束的泛型枚举方法");
    
    impl<T> MyOption<T> 
    where 
        T: std::fmt::Display,
    {
        fn display(&self) {
            match self {
                MyOption::Some(value) => println!("值: {}", value),
                MyOption::None => println!("无值"),
            }
        }
    }
    
    let displayable = MyOption::Some(42);
    displayable.display();
    
    let empty: MyOption<i32> = MyOption::None;
    empty.display();
    
    // 5. 复杂的泛型枚举示例
    println!("\n📖 5. 复杂的泛型枚举示例");
    
    // 二叉树枚举
    #[derive(Debug)]
    enum BinaryTree<T> {
        Empty,
        Node {
            value: T,
            left: Box<BinaryTree<T>>,
            right: Box<BinaryTree<T>>,
        },
    }
    
    impl<T> BinaryTree<T> {
        fn new() -> Self {
            BinaryTree::Empty
        }
        
        fn leaf(value: T) -> Self {
            BinaryTree::Node {
                value,
                left: Box::new(BinaryTree::Empty),
                right: Box::new(BinaryTree::Empty),
            }
        }
        
        fn node(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> Self {
            BinaryTree::Node {
                value,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        
        fn count_nodes(&self) -> usize {
            match self {
                BinaryTree::Empty => 0,
                BinaryTree::Node { left, right, .. } => {
                    1 + left.count_nodes() + right.count_nodes()
                }
            }
        }
    }
    
    let tree = BinaryTree::node(
        1,
        BinaryTree::leaf(2),
        BinaryTree::node(
            3,
            BinaryTree::leaf(4),
            BinaryTree::Empty,
        ),
    );
    
    println!("二叉树: {:?}", tree);
    println!("节点数量: {}", tree.count_nodes());
    
    // 6. 泛型枚举与模式匹配
    println!("\n📖 6. 泛型枚举与模式匹配");
    
    #[derive(Debug)]
    enum Message<T> {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
        Custom(T),
    }
    
    fn process_message<T: std::fmt::Debug>(msg: Message<T>) {
        match msg {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
            Message::Custom(data) => println!("自定义消息: {:?}", data),
        }
    }
    
    process_message(Message::<()>::Quit);
    process_message(Message::<()>::Move { x: 10, y: 20 });
    process_message(Message::<()>::Write(String::from("Hello World")));
    process_message(Message::<()>::ChangeColor(255, 0, 0));
    process_message(Message::<Vec<i32>>::Custom(vec![1, 2, 3]));
    
    // 7. 链表实现
    println!("\n📖 7. 链表实现");
    
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    
    impl<T> List<T> {
        fn new() -> Self {
            List::Nil
        }
        
        fn prepend(self, elem: T) -> Self {
            List::Cons(elem, Box::new(self))
        }
        
        fn len(&self) -> usize {
            match self {
                List::Cons(_, tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }
    }
    
    impl<T: std::fmt::Display> List<T> {
        fn stringify(&self) -> String {
            match self {
                List::Cons(head, tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                List::Nil => {
                    format!("Nil")
                }
            }
        }
    }
    
    let list = List::new()
        .prepend(1)
        .prepend(2)
        .prepend(3);
    
    println!("链表: {}", list.stringify());
    println!("链表长度: {}", list.len());
    
    // 8. 状态机枚举
    println!("\n📖 8. 状态机枚举");
    
    #[derive(Debug)]
    enum State<T> {
        Idle,
        Processing(T),
        Completed(T),
        Error(String),
    }
    
    impl<T> State<T> {
        fn is_idle(&self) -> bool {
            matches!(self, State::Idle)
        }
        
        fn is_processing(&self) -> bool {
            matches!(self, State::Processing(_))
        }
        
        fn is_completed(&self) -> bool {
            matches!(self, State::Completed(_))
        }
        
        fn is_error(&self) -> bool {
            matches!(self, State::Error(_))
        }
    }
    
    let states = vec![
        State::Idle,
        State::Processing("任务1"),
        State::Completed("任务1结果"),
        State::Error(String::from("网络错误")),
    ];
    
    for (i, state) in states.iter().enumerate() {
        println!("状态 {}: {:?}", i, state);
        println!("  空闲: {}, 处理中: {}, 完成: {}, 错误: {}", 
            state.is_idle(), state.is_processing(), 
            state.is_completed(), state.is_error());
    }
    
    // 9. 泛型枚举的转换
    println!("\n📖 9. 泛型枚举的转换");
    
    impl<T, E> MyResult<T, E> {
        fn map<U, F>(self, f: F) -> MyResult<U, E> 
        where 
            F: FnOnce(T) -> U,
        {
            match self {
                MyResult::Ok(value) => MyResult::Ok(f(value)),
                MyResult::Err(err) => MyResult::Err(err),
            }
        }
        
        fn map_err<F, G>(self, f: G) -> MyResult<T, F> 
        where 
            G: FnOnce(E) -> F,
        {
            match self {
                MyResult::Ok(value) => MyResult::Ok(value),
                MyResult::Err(err) => MyResult::Err(f(err)),
            }
        }
    }
    
    let result: MyResult<i32, String> = MyResult::Ok(10);
    let doubled = result.map(|x| x * 2);
    println!("映射结果: {:?}", doubled);
    
    let error_result: MyResult<i32, String> = MyResult::Err(String::from("原始错误"));
    let mapped_error = error_result.map_err(|e| format!("映射的错误: {}", e));
    println!("映射错误: {:?}", mapped_error);
    
    // 10. 实际应用：JSON值枚举
    println!("\n📖 10. 实际应用：JSON值枚举");
    
    #[derive(Debug)]
    enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(std::collections::HashMap<String, JsonValue>),
    }
    
    impl JsonValue {
        fn type_name(&self) -> &'static str {
            match self {
                JsonValue::Null => "null",
                JsonValue::Bool(_) => "boolean",
                JsonValue::Number(_) => "number",
                JsonValue::String(_) => "string",
                JsonValue::Array(_) => "array",
                JsonValue::Object(_) => "object",
            }
        }
        
        fn is_truthy(&self) -> bool {
            match self {
                JsonValue::Null => false,
                JsonValue::Bool(b) => *b,
                JsonValue::Number(n) => *n != 0.0,
                JsonValue::String(s) => !s.is_empty(),
                JsonValue::Array(arr) => !arr.is_empty(),
                JsonValue::Object(obj) => !obj.is_empty(),
            }
        }
    }
    
    let json_values = vec![
        JsonValue::Null,
        JsonValue::Bool(true),
        JsonValue::Number(42.0),
        JsonValue::String(String::from("Hello")),
        JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
    ];
    
    for value in json_values {
        println!("JSON值: {:?}", value);
        println!("  类型: {}, 真值: {}", value.type_name(), value.is_truthy());
    }
    
    println!("\n🎉 泛型枚举学习完成！");
    println!("💡 关键要点：");
    println!("   • 泛型枚举让枚举更加灵活和通用");
    println!("   • 可以有多个泛型参数");
    println!("   • 支持复杂的数据结构如树和链表");
    println!("   • 模式匹配是处理泛型枚举的主要方式");
    println!("   • 可以为泛型枚举实现转换和映射方法");
    println!("   • 泛型枚举常用于错误处理和状态管理");
} 