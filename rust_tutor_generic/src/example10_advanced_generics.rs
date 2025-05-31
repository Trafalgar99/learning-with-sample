// 例子10: 高级泛型技巧
// 这个例子将展示高级泛型技巧和实际应用

pub fn run() {
    println!("\n🎯 例子10: 高级泛型技巧");
    println!("========================");
    
    // 1. 幻影类型(Phantom Types)
    println!("\n📖 1. 幻影类型(Phantom Types)");
    
    use std::marker::PhantomData;
    
    // 使用幻影类型来区分不同的度量单位
    struct Measurement<T, U> {
        value: T,
        _unit: PhantomData<U>,
    }
    
    // 单位类型
    struct Meters;
    struct Feet;
    struct Celsius;
    struct Fahrenheit;
    
    impl<T, U> Measurement<T, U> {
        fn new(value: T) -> Self {
            Measurement {
                value,
                _unit: PhantomData,
            }
        }
        
        fn value(&self) -> &T {
            &self.value
        }
    }
    
    // 类型安全的单位转换
    impl Measurement<f64, Meters> {
        fn to_feet(self) -> Measurement<f64, Feet> {
            Measurement::new(self.value * 3.28084)
        }
    }
    
    impl Measurement<f64, Celsius> {
        fn to_fahrenheit(self) -> Measurement<f64, Fahrenheit> {
            Measurement::new(self.value * 9.0 / 5.0 + 32.0)
        }
    }
    
    let distance_m = Measurement::<f64, Meters>::new(10.0);
    let distance_ft = distance_m.to_feet();
    println!("10米 = {:.2}英尺", distance_ft.value());
    
    let temp_c = Measurement::<f64, Celsius>::new(25.0);
    let temp_f = temp_c.to_fahrenheit();
    println!("25°C = {:.1}°F", temp_f.value());
    
    // 2. 类型级编程
    println!("\n📖 2. 类型级编程");
    
    // 使用类型来表示编译时常量
    trait TypeNum {
        const VALUE: usize;
    }
    
    struct Zero;
    struct One;
    struct Two;
    struct Three;
    
    impl TypeNum for Zero { const VALUE: usize = 0; }
    impl TypeNum for One { const VALUE: usize = 1; }
    impl TypeNum for Two { const VALUE: usize = 2; }
    impl TypeNum for Three { const VALUE: usize = 3; }
    
    // 固定大小的数组，大小在类型中编码
    struct FixedArray<T, N: TypeNum> {
        data: Vec<T>,
        _size: PhantomData<N>,
    }
    
    impl<T, N: TypeNum> FixedArray<T, N> {
        fn new() -> Self {
            FixedArray {
                data: Vec::with_capacity(N::VALUE),
                _size: PhantomData,
            }
        }
        
        fn push(&mut self, item: T) -> Result<(), &'static str> {
            if self.data.len() < N::VALUE {
                self.data.push(item);
                Ok(())
            } else {
                Err("数组已满")
            }
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn capacity(&self) -> usize {
            N::VALUE
        }
    }
    
    let mut arr: FixedArray<i32, Three> = FixedArray::new();
    println!("固定数组容量: {}", arr.capacity());
    
    arr.push(1).unwrap();
    arr.push(2).unwrap();
    arr.push(3).unwrap();
    
    match arr.push(4) {
        Ok(_) => println!("添加成功"),
        Err(e) => println!("添加失败: {}", e),
    }
    
    // 3. 高阶类型构造器模拟
    println!("\n📖 3. 高阶类型构造器模拟");
    
    trait Functor<T> {
        type Wrapped<U>;
        fn map<U, F>(self, f: F) -> Self::Wrapped<U> 
        where 
            F: FnOnce(T) -> U;
    }
    
    impl<T> Functor<T> for Option<T> {
        type Wrapped<U> = Option<U>;
        
        fn map<U, F>(self, f: F) -> Self::Wrapped<U> 
        where 
            F: FnOnce(T) -> U,
        {
            self.map(f)
        }
    }
    
    impl<T, E> Functor<T> for Result<T, E> {
        type Wrapped<U> = Result<U, E>;
        
        fn map<U, F>(self, f: F) -> Self::Wrapped<U> 
        where 
            F: FnOnce(T) -> U,
        {
            self.map(f)
        }
    }
    
    let opt = Some(42);
    let mapped_opt = opt.map(|x| x * 2);
    println!("映射Option: {:?}", mapped_opt);
    
    let res: Result<i32, String> = Ok(21);
    let mapped_res = res.map(|x| x * 2);
    println!("映射Result: {:?}", mapped_res);
    
    // 4. 类型状态模式
    println!("\n📖 4. 类型状态模式");
    
    // 状态类型
    struct Locked;
    struct Unlocked;
    
    struct SafeBox<T, State> {
        content: Option<T>,
        _state: PhantomData<State>,
    }
    
    impl<T> SafeBox<T, Locked> {
        fn new(content: T) -> Self {
            SafeBox {
                content: Some(content),
                _state: PhantomData,
            }
        }
        
        fn unlock(self, _password: &str) -> SafeBox<T, Unlocked> {
            SafeBox {
                content: self.content,
                _state: PhantomData,
            }
        }
    }
    
    impl<T> SafeBox<T, Unlocked> {
        fn take(&mut self) -> Option<T> {
            self.content.take()
        }
        
        fn put(&mut self, content: T) {
            self.content = Some(content);
        }
        
        fn lock(self) -> SafeBox<T, Locked> {
            SafeBox {
                content: self.content,
                _state: PhantomData,
            }
        }
    }
    
    let locked_box = SafeBox::new("秘密文档");
    println!("创建了锁定的保险箱");
    
    let mut unlocked_box = locked_box.unlock("password123");
    println!("保险箱已解锁");
    
    if let Some(content) = unlocked_box.take() {
        println!("取出内容: {}", content);
    }
    
    unlocked_box.put("新文档");
    let _locked_again = unlocked_box.lock();
    println!("保险箱重新锁定");
    
    // 5. 泛型常量参数
    println!("\n📖 5. 泛型常量参数");
    
    struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }
    
    impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        fn new() -> Self {
            Matrix {
                data: [[T::default(); COLS]; ROWS],
            }
        }
        
        fn set(&mut self, row: usize, col: usize, value: T) {
            if row < ROWS && col < COLS {
                self.data[row][col] = value;
            }
        }
        
        fn get(&self, row: usize, col: usize) -> Option<T> {
            if row < ROWS && col < COLS {
                Some(self.data[row][col])
            } else {
                None
            }
        }
        
        fn dimensions(&self) -> (usize, usize) {
            (ROWS, COLS)
        }
    }
    
    let mut matrix: Matrix<i32, 3, 3> = Matrix::new();
    matrix.set(0, 0, 1);
    matrix.set(1, 1, 2);
    matrix.set(2, 2, 3);
    
    println!("矩阵维度: {:?}", matrix.dimensions());
    println!("对角线元素: {:?}, {:?}, {:?}", 
        matrix.get(0, 0), matrix.get(1, 1), matrix.get(2, 2));
    
    // 6. 异构列表(HList)模拟
    println!("\n📖 6. 异构列表(HList)模拟");
    
    struct HNil;
    struct HCons<H, T> {
        head: H,
        tail: T,
    }
    
    trait HList {
        fn len(&self) -> usize;
    }
    
    impl HList for HNil {
        fn len(&self) -> usize {
            0
        }
    }
    
    impl<H, T: HList> HList for HCons<H, T> {
        fn len(&self) -> usize {
            1 + self.tail.len()
        }
    }
    
    impl<H, T> HCons<H, T> {
        fn new(head: H, tail: T) -> Self {
            HCons { head, tail }
        }
    }
    
    // 创建异构列表: (i32, String, bool)
    let hlist = HCons::new(
        42,
        HCons::new(
            String::from("hello"),
            HCons::new(true, HNil),
        ),
    );
    
    println!("异构列表长度: {}", hlist.len());
    println!("第一个元素: {}", hlist.head);
    println!("第二个元素: {}", hlist.tail.head);
    println!("第三个元素: {}", hlist.tail.tail.head);
    
    // 7. 类型级别的计算
    println!("\n📖 7. 类型级别的计算");
    
    trait Add<Rhs> {
        type Output;
    }
    
    impl Add<Zero> for Zero { type Output = Zero; }
    impl Add<One> for Zero { type Output = One; }
    impl Add<Zero> for One { type Output = One; }
    impl Add<One> for One { type Output = Two; }
    impl Add<Two> for One { type Output = Three; }
    impl Add<One> for Two { type Output = Three; }
    
    fn type_add<A, B>() -> <A as Add<B>>::Output 
    where 
        A: Add<B>,
        <A as Add<B>>::Output: Default,
    {
        Default::default()
    }
    
    // 这里我们只是演示类型级计算的概念
    println!("类型级计算演示完成");
    
    // 8. 泛型单例模式
    println!("\n📖 8. 泛型单例模式");
    
    use std::sync::{Arc, Mutex};
    
    struct Singleton<T> {
        data: Arc<Mutex<Option<T>>>,
    }
    
    impl<T> Singleton<T> {
        fn new() -> Self {
            Singleton {
                data: Arc::new(Mutex::new(None)),
            }
        }
        
        fn initialize(&self, value: T) -> Result<(), &'static str> {
            let mut data = self.data.lock().unwrap();
            if data.is_none() {
                *data = Some(value);
                Ok(())
            } else {
                Err("单例已经初始化")
            }
        }
        
        fn get<F, R>(&self, f: F) -> Option<R> 
        where 
            F: FnOnce(&T) -> R,
        {
            let data = self.data.lock().unwrap();
            data.as_ref().map(f)
        }
    }
    
    let singleton: Singleton<String> = Singleton::new();
    singleton.initialize(String::from("全局配置")).unwrap();
    
    if let Some(config) = singleton.get(|s| s.clone()) {
        println!("单例值: {}", config);
    }
    
    // 9. 泛型构建器模式
    println!("\n📖 9. 泛型构建器模式");
    
    struct ConfigBuilder<T> {
        name: Option<String>,
        value: Option<T>,
        enabled: bool,
    }
    
    impl<T> ConfigBuilder<T> {
        fn new() -> Self {
            ConfigBuilder {
                name: None,
                value: None,
                enabled: false,
            }
        }
        
        fn name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }
        
        fn value(mut self, value: T) -> Self {
            self.value = Some(value);
            self
        }
        
        fn enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        
        fn build(self) -> Result<Config<T>, &'static str> {
            Ok(Config {
                name: self.name.ok_or("缺少名称")?,
                value: self.value.ok_or("缺少值")?,
                enabled: self.enabled,
            })
        }
    }
    
    struct Config<T> {
        name: String,
        value: T,
        enabled: bool,
    }
    
    impl<T: std::fmt::Display> Config<T> {
        fn display(&self) {
            println!("配置 '{}': {} (启用: {})", 
                self.name, self.value, self.enabled);
        }
    }
    
    let config = ConfigBuilder::new()
        .name("数据库连接".to_string())
        .value("localhost:5432".to_string())
        .enabled(true)
        .build()
        .unwrap();
    
    config.display();
    
    // 10. 实际应用：类型安全的状态机
    println!("\n📖 10. 实际应用：类型安全的状态机");
    
    // 状态类型
    struct Idle;
    struct Running;
    struct Paused;
    struct Stopped;
    
    struct StateMachine<State> {
        _state: PhantomData<State>,
    }
    
    impl StateMachine<Idle> {
        fn new() -> Self {
            println!("状态机创建 - 空闲状态");
            StateMachine { _state: PhantomData }
        }
        
        fn start(self) -> StateMachine<Running> {
            println!("状态转换: 空闲 -> 运行");
            StateMachine { _state: PhantomData }
        }
    }
    
    impl StateMachine<Running> {
        fn pause(self) -> StateMachine<Paused> {
            println!("状态转换: 运行 -> 暂停");
            StateMachine { _state: PhantomData }
        }
        
        fn stop(self) -> StateMachine<Stopped> {
            println!("状态转换: 运行 -> 停止");
            StateMachine { _state: PhantomData }
        }
    }
    
    impl StateMachine<Paused> {
        fn resume(self) -> StateMachine<Running> {
            println!("状态转换: 暂停 -> 运行");
            StateMachine { _state: PhantomData }
        }
        
        fn stop(self) -> StateMachine<Stopped> {
            println!("状态转换: 暂停 -> 停止");
            StateMachine { _state: PhantomData }
        }
    }
    
    impl StateMachine<Stopped> {
        fn reset(self) -> StateMachine<Idle> {
            println!("状态转换: 停止 -> 空闲");
            StateMachine { _state: PhantomData }
        }
    }
    
    let machine = StateMachine::new();
    let machine = machine.start();
    let machine = machine.pause();
    let machine = machine.resume();
    let machine = machine.stop();
    let _machine = machine.reset();
    
    println!("\n🎉 高级泛型技巧学习完成！");
    println!("💡 关键要点：");
    println!("   • 幻影类型提供编译时类型安全");
    println!("   • 类型级编程实现编译时计算");
    println!("   • 状态模式确保状态转换的正确性");
    println!("   • 泛型常量参数支持编译时大小检查");
    println!("   • 高级泛型技巧提供强大的抽象能力");
    println!("   • 类型系统是Rust最强大的特性之一");
} 