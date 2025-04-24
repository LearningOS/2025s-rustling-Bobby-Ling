/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T: Ord + Clone> {
    val: T,
    // NonNull<T> 是一个非空指针的包装类型, 保证:
    // 1. 需要 unsafe 块解引用
    // 2. 指针非空
    // 构造: NonNull::new(ptr: *mut T) -> Option<NonNull<T>>
    // 转换: as_ptr(self) -> *mut T: 转为裸指针. as_ref(&self) -> &T: (需 unsafe 解引用).
    next: Option<NonNull<Node<T>>>,
}

impl<T: Ord + Clone> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T: Ord + Clone> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Ord + Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        // 考虑: RUST Vect::push方法为什么对T没有Copy Clone的要求?
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn add_node(&mut self, node: Node<T>) {
        let obj = node.val;
        self.add(obj);
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    /// 消耗 list_a 和 list_b 的所有权
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged = LinkedList::new();
        let mut read_ptr_a = list_a.start;
        let mut read_ptr_b = list_b.start;
        // 首先处理直到某一侧为Null
        while let (Some(ptr_a), Some(ptr_b)) = (read_ptr_a, read_ptr_b) {
            // 可以保证现在的 ptr_a 和 ptr_b 都是NonNull
            let value_a = unsafe { ptr_a.as_ref() };
            let value_b = unsafe { ptr_b.as_ref() };
            // let value_a = unsafe { ptr_a.as_mut() };
            // let value_b = unsafe { ptr_a.as_mut() };
            // 副本, 必须要能够Copy
            // move occurs because value has type `Node<T>`, which does not implement the `Copy` trait
            // NonNull 裸指针, 无法直接转移底层数据的所有权
            // let value_a = unsafe { *ptr_a.as_ptr() };
            // let value_b = unsafe { *ptr_b.as_ptr() };
            match value_a.val < value_b.val {
                true => {
                    merged.add(value_a.val.clone());
                    read_ptr_a = value_a.next;
                },
                false => {
                    merged.add(value_b.val.clone());
                    read_ptr_b = value_b.next;
                },
            }
        }
        // 每侧剩下的
        while let Some(ptr_a) = read_ptr_a {
            let value_a = unsafe { ptr_a.as_ref() };
            merged.add(value_a.val.clone());
            read_ptr_a = value_a.next;
        }
        while let Some(ptr_b) = read_ptr_b {
            let value_b = unsafe { ptr_b.as_ref() };
            merged.add(value_b.val.clone());
            read_ptr_b = value_b.next;
        }
        merged
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display + Ord + Clone,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display + Ord + Clone,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
