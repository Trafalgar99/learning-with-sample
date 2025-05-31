/**
 * Rust基础数据结构教程 - 枚举 (Enum)
 * 
 * 枚举是定义一组相关值的类型
 * 特点：
 * - 可以有不同的变体 (variants)
 * - 每个变体可以携带不同类型和数量的数据
 * - 非常适合模式匹配
 */

// 1. 简单枚举
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

// 2. 带数据的枚举
#[derive(Debug)]
enum Message {
    Quit,                           // 无数据
    Move { x: i32, y: i32 },       // 具名字段
    Write(String),                  // 元组形式
    ChangeColor(u8, u8, u8),       // 三个u8值
}

// 3. 更复杂的枚举示例
#[derive(Debug)]
enum WebEvent {
    PageLoad,                                     // 单元变体
    PageUnload,
    KeyPress(char),                               // 元组变体
    Paste(String),
    Click { x: i64, y: i64 },                    // 结构体变体
}

// 4. Option枚举的使用（标准库中最重要的枚举之一）
// Option<T>已在标准库中定义，这里仅作演示
/*
enum Option<T> {
    Some(T),
    None,
}
*/

// 5. Result枚举的使用（错误处理）
// Result<T, E>已在标准库中定义
/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

// 6. 自定义错误枚举
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput(String),
}

// 7. 状态机枚举
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    println!("=== Rust 枚举教程 ===\n");

    // 1. 基本枚举使用
    println!("1. 基本枚举使用：");
    let north = Direction::North;
    let south = Direction::South;
    
    println!("   方向1: {:?}", north);
    println!("   方向2: {:?}", south);
    println!("   是否相等: {}", north == south);
    println!("   是否相等: {}", north == Direction::North);

    // 2. 枚举作为函数参数
    println!("\n2. 枚举作为函数参数：");
    
    fn describe_direction(dir: Direction) -> &'static str {
        match dir {
            Direction::North => "向北",
            Direction::South => "向南", 
            Direction::East => "向东",
            Direction::West => "向西",
        }
    }
    
    for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
        println!("   {}: {}", format!("{:?}", direction), describe_direction(direction));
    }

    // 3. 带数据的枚举
    println!("\n3. 带数据的枚举：");
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 128, 0),
    ];
    
    println!("   消息列表:");
    for (i, msg) in messages.iter().enumerate() {
        println!("     消息{}: {:?}", i + 1, msg);
    }

    // 4. 枚举方法
    println!("\n4. 枚举方法：");
    
    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("   执行退出操作"),
                Message::Move { x, y } => println!("   移动到坐标 ({}, {})", x, y),
                Message::Write(text) => println!("   写入文本: {}", text),
                Message::ChangeColor(r, g, b) => println!("   改变颜色为 RGB({}, {}, {})", r, g, b),
            }
        }
    }
    
    for msg in &messages {
        msg.call();
    }

    // 5. 复杂枚举的模式匹配
    println!("\n5. 复杂枚举的模式匹配：");
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::PageUnload,
        WebEvent::KeyPress('q'),
        WebEvent::Paste(String::from("复制的文本")),
        WebEvent::Click { x: 100, y: 200 },
    ];
    
    fn handle_event(event: &WebEvent) {
        match event {
            WebEvent::PageLoad => println!("   页面加载"),
            WebEvent::PageUnload => println!("   页面卸载"),
            WebEvent::KeyPress(c) => println!("   按键按下: '{}'", c),
            WebEvent::Paste(s) => println!("   粘贴操作: {}", s),
            WebEvent::Click { x, y } => println!("   鼠标点击位置: ({}, {})", x, y),
        }
    }
    
    println!("   处理Web事件:");
    for event in &events {
        handle_event(event);
    }

    // 6. Option枚举的使用
    println!("\n6. Option枚举的使用：");
    
    fn find_user(id: u32) -> Option<String> {
        if id == 1 {
            Some(String::from("张三"))
        } else if id == 2 {
            Some(String::from("李四"))
        } else {
            None
        }
    }
    
    let user_ids = vec![1, 2, 3, 4];
    for id in user_ids {
        match find_user(id) {
            Some(name) => println!("   用户ID {}: {}", id, name),
            None => println!("   用户ID {}: 未找到", id),
        }
    }
    
    // 使用if let简化匹配
    println!("\n   使用if let:");
    if let Some(user) = find_user(1) {
        println!("   找到用户: {}", user);
    }
    
    // 使用unwrap_or提供默认值
    let user = find_user(999).unwrap_or(String::from("匿名用户"));
    println!("   用户名: {}", user);

    // 7. Result枚举的使用
    println!("\n7. Result枚举的使用：");
    
    fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    fn sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }
    
    let calculations = vec![
        ("10 / 2", divide(10.0, 2.0)),
        ("10 / 0", divide(10.0, 0.0)),
        ("sqrt(16)", sqrt(16.0)),
        ("sqrt(-4)", sqrt(-4.0)),
    ];
    
    for (desc, result) in calculations {
        match result {
            Ok(value) => println!("   {}: {:.2}", desc, value),
            Err(err) => println!("   {}: 错误 - {:?}", desc, err),
        }
    }

    // 8. 枚举的嵌套匹配
    println!("\n8. 枚举的嵌套匹配：");
    
    #[derive(Debug)]
    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
    }
    
    #[derive(Debug)]
    enum Shape {
        Circle { radius: f64, color: Color },
        Rectangle { width: f64, height: f64, color: Color },
    }
    
    let shapes = vec![
        Shape::Circle { 
            radius: 5.0, 
            color: Color::Rgb(255, 0, 0) 
        },
        Shape::Rectangle { 
            width: 10.0, 
            height: 20.0, 
            color: Color::Hsv(240, 100, 100) 
        },
    ];
    
    for shape in &shapes {
        match shape {
            Shape::Circle { radius, color } => {
                println!("   圆形 - 半径: {:.1}", radius);
                match color {
                    Color::Rgb(r, g, b) => println!("     RGB颜色: ({}, {}, {})", r, g, b),
                    Color::Hsv(h, s, v) => println!("     HSV颜色: ({}, {}, {})", h, s, v),
                }
            },
            Shape::Rectangle { width, height, color } => {
                println!("   矩形 - 宽: {:.1}, 高: {:.1}", width, height);
                match color {
                    Color::Rgb(r, g, b) => println!("     RGB颜色: ({}, {}, {})", r, g, b),
                    Color::Hsv(h, s, v) => println!("     HSV颜色: ({}, {}, {})", h, s, v),
                }
            },
        }
    }

    // 9. 状态机示例
    println!("\n9. 状态机示例：");
    
    impl TrafficLight {
        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Green => TrafficLight::Yellow,
            }
        }
        
        fn duration(&self) -> u32 {
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 25,
            }
        }
        
        fn description(&self) -> &str {
            match self {
                TrafficLight::Red => "停止",
                TrafficLight::Yellow => "准备",
                TrafficLight::Green => "通行",
            }
        }
    }
    
    let mut current_light = TrafficLight::Red;
    println!("   交通灯状态变化:");
    
    for cycle in 1..=6 {
        println!("     周期{}: {:?} - {} ({}秒)", 
               cycle, current_light, current_light.description(), current_light.duration());
        current_light = current_light.next();
    }

    // 10. 枚举的匹配守卫
    println!("\n10. 枚举的匹配守卫：");
    
    #[derive(Debug)]
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }
    
    fn describe_temperature(temp: Temperature) {
        match temp {
            Temperature::Celsius(t) if t > 30 => println!("   {}°C - 很热", t),
            Temperature::Celsius(t) if t > 20 => println!("   {}°C - 温暖", t),
            Temperature::Celsius(t) if t > 0 => println!("   {}°C - 凉爽", t),
            Temperature::Celsius(t) => println!("   {}°C - 寒冷", t),
            Temperature::Fahrenheit(t) if t > 86 => println!("   {}°F - 很热", t),
            Temperature::Fahrenheit(t) if t > 68 => println!("   {}°F - 温暖", t),
            Temperature::Fahrenheit(t) if t > 32 => println!("   {}°F - 凉爽", t),
            Temperature::Fahrenheit(t) => println!("   {}°F - 寒冷", t),
        }
    }
    
    let temperatures = vec![
        Temperature::Celsius(35),
        Temperature::Celsius(25),
        Temperature::Celsius(10),
        Temperature::Celsius(-5),
        Temperature::Fahrenheit(95),
        Temperature::Fahrenheit(75),
    ];
    
    for temp in temperatures {
        describe_temperature(temp);
    }

    // 11. 枚举与Vec的组合
    println!("\n11. 枚举与Vec的组合：");
    
    #[derive(Debug)]
    enum Task {
        Todo(String),
        InProgress(String, u8),  // 任务名，完成百分比
        Done(String),
    }
    
    let mut tasks = vec![
        Task::Todo(String::from("学习Rust")),
        Task::InProgress(String::from("写代码"), 60),
        Task::Done(String::from("看文档")),
    ];
    
    // 统计不同状态的任务
    let (todo_count, in_progress_count, done_count) = tasks.iter().fold((0, 0, 0), |acc, task| {
        match task {
            Task::Todo(_) => (acc.0 + 1, acc.1, acc.2),
            Task::InProgress(_, _) => (acc.0, acc.1 + 1, acc.2),
            Task::Done(_) => (acc.0, acc.1, acc.2 + 1),
        }
    });
    
    println!("   任务统计:");
    println!("     待办: {} 个", todo_count);
    println!("     进行中: {} 个", in_progress_count);
    println!("     完成: {} 个", done_count);
    
    // 显示所有任务
    println!("   任务列表:");
    for (i, task) in tasks.iter().enumerate() {
        match task {
            Task::Todo(name) => println!("     {}. [ ] {}", i + 1, name),
            Task::InProgress(name, progress) => println!("     {}. [{}%] {}", i + 1, progress, name),
            Task::Done(name) => println!("     {}. [✓] {}", i + 1, name),
        }
    }

    println!("\n=== 枚举教程结束 ===");
} 