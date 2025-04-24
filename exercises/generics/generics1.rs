// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // Vec<str> = Vec::new();
    // str: DST, 动态大小类型, 表示一个不确定长度的字符串切片, 编译时无法确定其大小(例如 "hello" 和 "world" 长度不同).
    // 存储在 Vec 中的元素必须有固定大小
    // &str 是一个指向 str 的 胖指针(包含数据指针和长度), 大小固定
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
