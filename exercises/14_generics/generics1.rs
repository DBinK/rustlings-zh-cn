// `Vec<T>` 是针对类型 `T` 的泛型。
// 在大多数情况下，编译器能够推断出 `T`，例如在将具有具体类型的值推送到动态数组之后。
// 但在本练习中，需要通过类型注解来帮助编译器进行推断。

fn main() {
    // TODO: 通过为动态数组 `Vec<T>` 的类型添加注解来修复编译器错误。
    // 选择 `T` 作为某种可以由 `u8` 和 `i8` 创建的整数类型。
    let mut numbers: Vec<i32> = Vec::new();

    // 不要修改此行代码。
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}

// 核心原理：
// T.into() 会调用 From<T> trait 完成类型转换。
// i16 同时实现了 From<u8> 和 From<i8>，所以两种类型的值都能通过 .into() 转成 i16。
// 而 u8 和 i8 之间不能互相转换（i8 可以表示负数，u8 不能；
// u8 可以表示 128+，i8 不能），所以不能直接用 Vec<u8> 或 Vec<i8>。