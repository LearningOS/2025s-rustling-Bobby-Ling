// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.

// fn main() {
//     println!("Hello world!");
// }

/* use std::ptr::NonNull;
fn main() {
    let mut x = 1_u32;
    let mut ptr = NonNull::new(&mut x as *mut u32).expect("Invalid pointer");

    unsafe { *ptr.as_ptr() += 1; }
    // x_value为副本
    let x_value = unsafe { *ptr.as_ptr() };
    assert_eq!(x_value, 2);
    assert_eq!(x, 2);

    unsafe { *ptr.as_ptr() += 1; }
    assert_eq!(x_value, 2);
    assert_eq!(x, 3);

    let x_mut_ref = unsafe { ptr.as_mut() };
    *x_mut_ref += 1;
    assert_eq!(x, 4);

    let x_ref = unsafe { ptr.as_ref() };
    x += 1;
    assert_eq!(*x_ref, 5);

    // 5 5 2 5 5
    print!("{:?} {:?} {:?} {:?} {:?}", x, unsafe { *ptr.as_ptr() }, x_value, x_mut_ref, x_ref);
}
 */

// 自定义 Vector 类型
#[derive(Debug)]
struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    // 创建一个新的 Vector
    fn new(data: Vec<T>) -> Self {
        Vector { data }
    }

    // 添加单个元素
    fn add(&mut self, item: T) {
        self.data.push(item);
    }

    // 合并两个 Vector，消耗输入的所有权
    fn combine(a: Vector<T>, b: Vector<T>) -> Vector<T> {
        let mut result = a;
        // 将 b 的所有数据移动到 result 中
        result.data.extend(b.data.into_iter());
        result
    }
}

fn main() {
    let mut v1 = Vector::new(vec![1, 2, 3]);
    v1.add(4); // 添加单个元素
    println!("v1 after add: {:?}", v1); // 输出: v1 after add: Vector { data: [1, 2, 3, 4] }

    let v2 = Vector::new(vec![5, 6, 7]);
    let combined = Vector::combine(v1, v2); // 合并 v1 和 v2
    println!("Combined Vector: {:?}", combined); // 输出: Combined Vector: Vector { data: [1, 2, 3, 4, 5, 6, 7] }
}