// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    // 第一个字母
    match c.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + c.as_str()
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    words.iter().for_each(|x| {
        result.push(capitalize_first(x));
    });
    result
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // capitalize_words_vector(words)返回Vec<String>, 故这里每一个迭代的元素是&String
    // 由于没有实现FromIterator<&String>, 因此不能这么用
    // 注:
    // `String` implements `FromIterator<&char>`
    // `String` implements `FromIterator<&str>`
    // `String` implements `FromIterator<String>`
    // `String` implements `FromIterator<char>`
    // ...
    // capitalize_words_vector(words).iter().collect::<String>()

    // 这里每一个就是&str了
    words.iter().map(|x| capitalize_first(x)).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
