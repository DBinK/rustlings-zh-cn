#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}


#[derive(Debug)]
enum Message {
    // TODO: 定义下面所使用的不同变体(variants)。
    Resize {width: u64, height: u64},
    Quit,
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8)
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

// Hint
// 在 Rust 中，你可以创建包含不同类型变体的枚举，这些变体可以支持多种数据结构形式，例如:
// ​匿名结构体: `Move { x: i32, y: i32 }`
// ​具名结构体: 通过`struct Point`定义后绑定到`Move(Point)`变体
// ​单字符串: `Echo(String)`
// ​元组: `ChangeColor(u8, u8, u8)`
// ​无数据变体: `Quit`
// ...
// 这种设计允许枚举灵活地封装不同场景下的数据形态，并通过模式匹配统一处理。

