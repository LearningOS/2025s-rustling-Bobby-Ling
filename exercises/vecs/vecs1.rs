// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.


fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10, 20, 30, 40];// TODO: declare your vector here with the macro for vectors

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        // 由于 a 是固定长度的数组([i32; 4]), 而 v 是动态长度的向量(Vec<i32>), 直接比较 a 和 v 会因类型不同而报错.
        // 通过 v[..] 将向量转换为切片(&[i32]), 就可以与数组 a 进行比较(Rust 中数组可以自动解引用为切片).
        assert_eq!(a, v[..]);
    }
}
