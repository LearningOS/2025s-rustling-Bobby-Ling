// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // Some(p) 的模式匹配会将 Point 的所有权从 y 移动到 p
        // 因为 Point 没有实现 Copy trait
        // ref 只能用于模式匹配的上下文中（如 match、if let、while let、let 绑定等）
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
