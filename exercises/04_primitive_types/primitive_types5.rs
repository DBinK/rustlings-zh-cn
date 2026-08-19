fn main() {
    let cat = ("Furry McFurson", 3.5);

// TODO: 用一条语句解构 `cat` 元组，让下方 `println!` 能够正常执行。
    let (name, age) = cat;

    println!("{name} 今年 {age} 岁了");
}
