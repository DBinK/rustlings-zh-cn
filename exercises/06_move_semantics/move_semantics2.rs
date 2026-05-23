fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut owned_vec = vec.to_owned();

    owned_vec.push(88);

    owned_vec
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: 使动态数组 `vec0` 和 `vec1` 能够被同时访问，以修复测试中的编译器错误。
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        
        let vec1 = fill_vec(&vec0);
        // let vec1 = fill_vec(vec0.clone());  // .clone() 会执行深拷贝操作（分配堆内存 + 复制数据），在性能敏感的场景下应避免不必要的开销

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
