// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// 类似泛型, "声明"生命周期参数
// 生命周期通常在前, 泛型在后 fn function_name<'a, T>(x: &'a T, y: &'a T) -> &'a T
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 规则1: 每个输入引用参数都有自己的生命周期
    // 自动推断为: fn foo<'a, 'b>(x: &'a str, y: &'b str)
    // 规则2: 若只有一个输入生命周期(函数参数中只有一个引用类型), 那么该生命周期会被赋给所有的输出生命周期
    // 规则2: 若存在多个输入生命周期, 且其中一个是 &self 或 &mut self, 则 &self 的生命周期被赋给所有的输出生命周期
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
