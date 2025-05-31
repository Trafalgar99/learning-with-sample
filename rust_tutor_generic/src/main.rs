use std::io;

// 导入所有例子模块
mod example01_basic_generics;
mod example02_generic_functions;
mod example03_generic_structs;
mod example04_generic_enums;
mod example05_generic_traits;
mod example06_trait_bounds;
mod example07_where_clause;
mod example08_associated_types;
mod example09_lifetime_generics;
mod example10_advanced_generics;

fn main() {
    println!("🦀 Rust 泛型教程 - 交互式学习系统");
    println!("=====================================");
    
    loop {
        print_menu();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        
        match input.trim() {
            "1" => example01_basic_generics::run(),
            "2" => example02_generic_functions::run(),
            "3" => example03_generic_structs::run(),
            "4" => example04_generic_enums::run(),
            "5" => example05_generic_traits::run(),
            "6" => example06_trait_bounds::run(),
            "7" => example07_where_clause::run(),
            "8" => example08_associated_types::run(),
            "9" => example09_lifetime_generics::run(),
            "10" => example10_advanced_generics::run(),
            "0" => {
                println!("感谢使用Rust泛型教程！再见！👋");
                break;
            }
            _ => println!("❌ 无效选择，请重新输入"),
        }
        
        println!("\n按回车键继续...");
        let mut _temp = String::new();
        io::stdin().read_line(&mut _temp).ok();
    }
}

fn print_menu() {
    println!("\n📚 请选择要学习的泛型主题：");
    println!("1.  基础泛型概念");
    println!("2.  泛型函数");
    println!("3.  泛型结构体");
    println!("4.  泛型枚举");
    println!("5.  泛型特征(Traits)");
    println!("6.  特征约束(Trait Bounds)");
    println!("7.  Where子句");
    println!("8.  关联类型");
    println!("9.  生命周期与泛型");
    println!("10. 高级泛型技巧");
    println!("0.  退出");
    print!("请输入选择 (0-10): ");
}
