# Rust 基础数据类型和数据结构教程

这是一个标准的Cargo项目，包含了Rust中常用的基础数据类型和数据结构示例。每个示例都是一个可独立运行的程序，包含详细的中文注释和实际应用示例。

## 项目结构

```
rust-data-structures-tutorial/
├── Cargo.toml          # 项目配置文件
├── README.md           # 项目说明文档
├── .gitignore          # Git忽略文件
├── src/
│   └── lib.rs          # 库入口文件
└── examples/           # 教程示例文件
    ├── 01_vector.rs    # Vector（动态数组）教程
    ├── 02_basic_types.rs # 基本数据类型教程
    ├── 03_string.rs    # 字符串类型教程
    ├── 04_array_tuple.rs # 数组、切片和元组教程
    ├── 05_struct.rs    # 结构体教程
    ├── 06_enum.rs      # 枚举类型教程
    ├── 07_hashmap.rs   # HashMap教程
    └── 08_hashset.rs   # HashSet教程
```

## 教程内容

### 基础数据类型
1. **basic_types** - 基本数据类型（整数、浮点数、布尔值、字符）
2. **string** - 字符串类型（String 和 &str）
3. **array_tuple** - 数组、切片和元组
4. **vector** - Vector（动态数组）- 最常用的集合类型

### 自定义数据类型
5. **struct** - 结构体（具名字段、元组结构体、单元结构体）
6. **enum** - 枚举类型（简单枚举、带数据枚举、Option、Result）

### 集合数据结构
7. **hashmap** - HashMap（哈希映射）- 键值对存储
8. **hashset** - HashSet（哈希集合）- 唯一值集合

## 运行方式

### 使用Cargo运行示例（推荐）

```bash
# 运行基本数据类型教程
cargo run --example basic_types

# 运行字符串教程
cargo run --example string

# 运行数组和元组教程
cargo run --example array_tuple

# 运行Vector教程
cargo run --example vector

# 运行结构体教程
cargo run --example struct

# 运行枚举教程
cargo run --example enum

# 运行HashMap教程
cargo run --example hashmap

# 运行HashSet教程
cargo run --example hashset
```

### 查看项目信息

```bash
# 检查项目配置
cargo check

# 运行测试
cargo test

# 生成文档
cargo doc --open

# 查看所有可用示例
cargo run --example
```

### 构建项目

```bash
# 构建库
cargo build

# 构建发布版本
cargo build --release

# 构建所有示例
cargo build --examples
```

## 学习路径建议

### 初学者路径：
1. `basic_types` - 了解Rust的基本数据类型
2. `string` - 掌握字符串的使用
3. `array_tuple` - 学习数组和元组
4. `vector` - 掌握最常用的Vector
5. `struct` - 学习自定义结构体
6. `enum` - 理解枚举和模式匹配

### 进阶路径：
1. `hashmap` - 掌握键值对存储
2. `hashset` - 学习集合操作

## 每个教程包含的内容

- **详细的代码注释**：每行关键代码都有中文解释
- **完整的示例**：从基础用法到高级特性
- **实际应用场景**：展示如何在真实项目中使用
- **最佳实践**：Rust编程的推荐做法
- **性能考虑**：内存使用和效率优化
- **错误处理**：安全的Rust编程模式

## 重要概念解释

### 所有权（Ownership）
- 每个值都有一个所有者
- 值在所有者离开作用域时被释放
- 移动（Move）vs 借用（Borrow）

### 生命周期（Lifetime）
- 引用的有效期
- 防止悬垂指针
- 生命周期标注

### 模式匹配（Pattern Matching）
- `match` 表达式
- `if let` 和 `while let`
- 解构赋值

### 错误处理
- `Option<T>` 处理可能为空的值
- `Result<T, E>` 处理可能失败的操作
- `unwrap()` vs 安全的错误处理

## 实际应用示例

教程中包含了许多实际应用场景：
- **用户管理系统**（结构体和HashMap）
- **标签系统**（HashSet）
- **权限管理**（枚举和集合）
- **缓存实现**（HashMap）
- **任务管理**（枚举状态机）
- **数据去重**（HashSet）

## 性能提示

1. **Vector vs Array**：Vector可变长度，Array固定长度
2. **String vs &str**：String可变，&str不可变引用
3. **HashMap预分配**：使用`with_capacity`提高性能
4. **克隆 vs 引用**：避免不必要的数据克隆

## 常见错误和解决方案

1. **借用检查器错误**：理解可变性和生命周期
2. **字符串索引**：使用`.chars()`或`.bytes()`而不是直接索引
3. **Option解包**：使用`match`或`if let`而不是`unwrap()`
4. **集合修改**：遍历时不能修改集合

## 下一步学习

完成这些教程后，建议学习：
- 迭代器（Iterator）和闭包（Closure）
- 智能指针（Box, Rc, RefCell）
- 并发编程（threads, channels）
- 异步编程（async/await）
- 宏编程（macro_rules!）
- 错误处理（thiserror, anyhow）
- 序列化（serde）

## 贡献和反馈

如果你发现任何问题或有改进建议，欢迎提出！这些教程旨在帮助中文Rust学习者更好地掌握语言基础。

## 许可证

这个项目采用MIT或Apache-2.0双重许可证。

## 作者

欢迎更新Cargo.toml中的作者信息为你的信息。 