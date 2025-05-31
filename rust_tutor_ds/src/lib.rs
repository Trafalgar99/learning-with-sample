//! # Rust 基础数据类型和数据结构教程
//!
//! 这个库包含了Rust中常用的基础数据类型和数据结构的学习资源。
//! 每个示例都包含详细的中文注释和实际应用场景。
//!
//! ## 使用方法
//!
//! ### 运行示例
//! ```bash
//! # 运行Vector教程
//! cargo run --example vector
//!
//! # 运行基本数据类型教程
//! cargo run --example basic_types
//!
//! # 运行字符串教程
//! cargo run --example string
//!
//! # 运行数组和元组教程
//! cargo run --example array_tuple
//!
//! # 运行结构体教程
//! cargo run --example struct
//!
//! # 运行枚举教程
//! cargo run --example enum
//!
//! # 运行HashMap教程
//! cargo run --example hashmap
//!
//! # 运行HashSet教程
//! cargo run --example hashset
//! ```
//!
//! ### 查看所有示例
//! ```bash
//! # 列出所有可用的示例
//! cargo run --example
//! ```
//!
//! ## 教程结构
//!
//! 本教程按照从基础到高级的顺序组织：
//!
//! 1. **基础数据类型** - 了解Rust的原始数据类型
//! 2. **字符串** - 掌握String和&str的使用
//! 3. **数组和元组** - 学习固定大小的数据结构
//! 4. **Vector** - 掌握最常用的动态数组
//! 5. **结构体** - 创建自定义数据类型
//! 6. **枚举** - 理解代数数据类型和模式匹配
//! 7. **HashMap** - 键值对存储
//! 8. **HashSet** - 唯一值集合
//!
//! ## 学习建议
//!
//! - 每个示例都是独立的，可以按任意顺序学习
//! - 建议先从基础数据类型开始
//! - 运行代码并观察输出
//! - 尝试修改代码来加深理解
//! - 关注代码中的注释，它们解释了重要概念

pub mod data_structures {
    //! 数据结构相关的工具函数和类型定义
    
    /// 用于演示的学生信息结构体
    #[derive(Debug, Clone)]
    pub struct Student {
        pub name: String,
        pub age: u32,
        pub grade: f64,
    }
    
    impl Student {
        /// 创建新的学生实例
        pub fn new(name: String, age: u32, grade: f64) -> Self {
            Student { name, age, grade }
        }
        
        /// 检查学生是否及格（60分以上）
        pub fn is_passing(&self) -> bool {
            self.grade >= 60.0
        }
    }
    
    /// 用于演示的坐标点结构体
    #[derive(Debug, Clone, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }
    
    impl Point {
        /// 创建新的坐标点
        pub fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
        
        /// 计算到原点的距离
        pub fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }
}

pub mod examples {
    //! 示例代码的工具函数
    
    /// 打印分隔线，用于美化输出
    pub fn print_separator(title: &str) {
        println!("\n{}", "=".repeat(50));
        println!("  {}", title);
        println!("{}", "=".repeat(50));
    }
    
    /// 打印小节标题
    pub fn print_section(title: &str) {
        println!("\n{}", "-".repeat(30));
        println!("{}", title);
        println!("{}", "-".repeat(30));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_student_creation() {
        let student = data_structures::Student::new("测试学生".to_string(), 20, 85.0);
        assert_eq!(student.name, "测试学生");
        assert_eq!(student.age, 20);
        assert_eq!(student.grade, 85.0);
        assert!(student.is_passing());
    }
    
    #[test]
    fn test_point_creation() {
        let point = data_structures::Point::new(3.0, 4.0);
        assert_eq!(point.x, 3.0);
        assert_eq!(point.y, 4.0);
        assert_eq!(point.distance_from_origin(), 5.0);
    }
} 