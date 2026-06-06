trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: 通过修改此函数的签名来修复编译器错误。  
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {  // impl Trait 语法，用来表示"某个实现了 Licensed trait 的类型，但我不想指名具体是哪个类型"。
// fn compare_license_types<T1: Licensed, T2: Licensed>(software1: T1, software2: T2) -> bool {  // 等价实现
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
