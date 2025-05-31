/**
 * Rust基础数据结构教程 - 结构体 (Struct)
 * 
 * 结构体是自定义数据类型，用于组合相关的数据
 * 包含三种类型：
 * - 具名字段结构体
 * - 元组结构体  
 * - 单元结构体
 */

// 具名字段结构体
#[derive(Debug, Clone)]  // 自动实现Debug和Clone trait
struct Person {
    name: String,
    age: u32,
    email: String,
    is_active: bool,
}

// 元组结构体
#[derive(Debug)]
struct Point(f64, f64);  // 二维坐标点

#[derive(Debug)]
struct Color(u8, u8, u8);  // RGB颜色

// 单元结构体（无字段）
#[derive(Debug)]
struct Unit;

// 带有生命周期的结构体
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
    pages: u32,
}

// 嵌套结构体
#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    country: String,
    postal_code: String,
}

#[derive(Debug)]
struct Employee {
    personal: Person,
    address: Address,
    salary: f64,
    department: String,
}

fn main() {
    println!("=== Rust 结构体教程 ===\n");

    // 1. 创建和使用具名字段结构体
    println!("1. 具名字段结构体：");
    
    // 创建结构体实例
    let person1 = Person {
        name: String::from("张三"),
        age: 25,
        email: String::from("zhangsan@example.com"),
        is_active: true,
    };
    
    println!("   人员信息: {:?}", person1);
    println!("   姓名: {}", person1.name);
    println!("   年龄: {}", person1.age);
    println!("   邮箱: {}", person1.email);
    println!("   是否活跃: {}", person1.is_active);

    // 2. 可变结构体
    println!("\n2. 可变结构体：");
    let mut person2 = Person {
        name: String::from("李四"),
        age: 30,
        email: String::from("lisi@example.com"),
        is_active: false,
    };
    
    println!("   修改前: {:?}", person2);
    person2.age = 31;  // 修改年龄
    person2.is_active = true;  // 修改状态
    println!("   修改后: {:?}", person2);

    // 3. 结构体更新语法
    println!("\n3. 结构体更新语法：");
    let person3 = Person {
        name: String::from("王五"),
        email: String::from("wangwu@example.com"),
        ..person1  // 其他字段从person1复制
    };
    println!("   基于person1创建的person3: {:?}", person3);
    // 注意：person1在这里被部分移动，某些字段不能再使用

    // 4. 元组结构体
    println!("\n4. 元组结构体：");
    let origin = Point(0.0, 0.0);
    let point1 = Point(3.0, 4.0);
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    
    println!("   原点: {:?}", origin);
    println!("   点1: {:?}", point1);
    println!("   红色: {:?}", red);
    println!("   绿色: {:?}", green);
    
    // 访问元组结构体字段
    println!("   点1的x坐标: {}", point1.0);
    println!("   点1的y坐标: {}", point1.1);
    println!("   红色的R值: {}", red.0);

    // 5. 单元结构体
    println!("\n5. 单元结构体：");
    let unit = Unit;
    println!("   单元结构体: {:?}", unit);

    // 6. 结构体方法
    println!("\n6. 结构体方法：");
    
    // 使用impl块添加方法
    impl Person {
        // 关联函数（类似于静态方法）
        fn new(name: String, age: u32, email: String) -> Person {
            Person {
                name,
                age,
                email,
                is_active: true,
            }
        }
        
        // 实例方法
        fn greet(&self) {
            println!("   你好，我是{}，今年{}岁", self.name, self.age);
        }
        
        fn is_adult(&self) -> bool {
            self.age >= 18
        }
        
        fn have_birthday(&mut self) {
            self.age += 1;
        }
        
        fn get_info(&self) -> String {
            format!("姓名: {}, 年龄: {}, 邮箱: {}", self.name, self.age, self.email)
        }
    }
    
    // 使用关联函数创建实例
    let mut person4 = Person::new(
        String::from("赵六"),
        28,
        String::from("zhaoliu@example.com")
    );
    
    println!("   新创建的人员: {:?}", person4);
    person4.greet();
    println!("   是否成年: {}", person4.is_adult());
    println!("   详细信息: {}", person4.get_info());
    
    person4.have_birthday();
    println!("   生日后的年龄: {}", person4.age);

    // 7. 为元组结构体添加方法
    impl Point {
        fn new(x: f64, y: f64) -> Point {
            Point(x, y)
        }
        
        fn distance_from_origin(&self) -> f64 {
            (self.0 * self.0 + self.1 * self.1).sqrt()
        }
        
        fn distance_to(&self, other: &Point) -> f64 {
            let dx = self.0 - other.0;
            let dy = self.1 - other.1;
            (dx * dx + dy * dy).sqrt()
        }
    }
    
    let point_a = Point::new(3.0, 4.0);
    let point_b = Point::new(0.0, 0.0);
    
    println!("\n   点A到原点的距离: {:.2}", point_a.distance_from_origin());
    println!("   点A到点B的距离: {:.2}", point_a.distance_to(&point_b));

    // 8. 嵌套结构体
    println!("\n8. 嵌套结构体：");
    let address = Address {
        street: String::from("中山路123号"),
        city: String::from("北京"),
        country: String::from("中国"),
        postal_code: String::from("100000"),
    };
    
    let employee = Employee {
        personal: Person::new(
            String::from("孙七"),
            32,
            String::from("sunqi@company.com")
        ),
        address,
        salary: 50000.0,
        department: String::from("技术部"),
    };
    
    println!("   员工信息: {:#?}", employee);  // 使用{:#?}进行美化打印
    println!("   员工姓名: {}", employee.personal.name);
    println!("   员工城市: {}", employee.address.city);

    // 9. 带生命周期的结构体
    println!("\n9. 带生命周期的结构体：");
    let title = "Rust编程语言";
    let author = "Steve Klabnik";
    
    let book = Book {
        title,
        author,
        pages: 552,
    };
    
    println!("   书籍信息: {:?}", book);

    // 10. 结构体作为函数参数
    println!("\n10. 结构体作为函数参数：");
    
    fn print_person_info(p: &Person) {
        println!("   函数接收的人员信息: {}", p.get_info());
    }
    
    fn update_person_age(p: &mut Person, new_age: u32) {
        p.age = new_age;
    }
    
    let mut person5 = Person::new(
        String::from("周八"),
        25,
        String::from("zhouba@example.com")
    );
    
    print_person_info(&person5);
    update_person_age(&mut person5, 26);
    print_person_info(&person5);

    // 11. 结构体向量
    println!("\n11. 结构体向量：");
    let people = vec![
        Person::new(String::from("用户1"), 20, String::from("user1@test.com")),
        Person::new(String::from("用户2"), 25, String::from("user2@test.com")),
        Person::new(String::from("用户3"), 30, String::from("user3@test.com")),
    ];
    
    println!("   人员列表:");
    for (i, person) in people.iter().enumerate() {
        println!("     {}. {}", i + 1, person.get_info());
    }
    
    // 筛选成年人
    let adults: Vec<&Person> = people.iter()
        .filter(|p| p.is_adult())
        .collect();
    println!("   成年人数量: {}", adults.len());

    // 12. 结构体的比较
    println!("\n12. 结构体的比较：");
    
    // 需要为结构体实现PartialEq trait
    #[derive(Debug, PartialEq)]
    struct SimplePoint {
        x: i32,
        y: i32,
    }
    
    let p1 = SimplePoint { x: 1, y: 2 };
    let p2 = SimplePoint { x: 1, y: 2 };
    let p3 = SimplePoint { x: 2, y: 3 };
    
    println!("   点p1: {:?}", p1);
    println!("   点p2: {:?}", p2);
    println!("   点p3: {:?}", p3);
    println!("   p1 == p2: {}", p1 == p2);
    println!("   p1 == p3: {}", p1 == p3);

    println!("\n=== 结构体教程结束 ===");
} 