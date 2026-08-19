// 这个函数的所有调用都应替换为对 `string_slice` 或 `string` 的调用。
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 这里有一堆值，有些是 `String` 类型，有些是 `&str` 类型。
// 你的任务是根据你对每个值类型的判断，将 `placeholder(…)` 替换为 `string_slice(…)`
// 或 `string(…)`。
fn main() {
    string_slice("blue");  // "blue" 是字符串字面量，类型为 &str

    string("red".to_string());  // "red".to_string() 创建 String 类型

    string(String::from("hi"));  // String::from("hi") 返回 String 类型

    string("rust is fun!".to_owned());  // to_owned() 将 &str 转换为 String

string("nice weather".into());  // into() 将 &str 转换为 String（通过 Into<String> trait）

    string(format!("Interpolation {}", "Station"));  // format! 宏返回 String 类型

    // 警告：这是字节索引（byte indexing），而非字符索引（character indexing）。
    // 字符索引可以通过 `s.chars().nth(INDEX)` 来完成。
    string_slice(&String::from("abc")[0..1]);  // [0..1] 切片操作产生 &str 类型

    string_slice("  hello there ".trim());  // trim() 方法返回 &str 类型

    string("Happy Monday!".replace("Mon", "Tues"));  // replace() 方法返回 String 类型

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());  // to_lowercase() 方法返回 String 类型
}
