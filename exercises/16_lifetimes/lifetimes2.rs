// 不要修改此函数。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: 通过移动一行代码，修复编译器错误。

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");  // 移动到作用域外
    let result;
    {  // 花括号创建了一个新的作用域（scope）。变量在作用域结束时被销毁（drop）
        // let string2 = String::from("xyz");  // 移动到作用域外
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
