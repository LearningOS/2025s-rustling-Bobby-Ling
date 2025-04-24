/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;

// 二叉堆的性质: 父亲的权值不小于儿子的权值(大顶堆为例)
pub struct Heap<T>
where
    T: Default + Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 二叉堆满足完全二叉树, 因此插入时先插入在数组尾部(即"最右侧"的叶子节点), 然后逐步调整所在的子树
        // (因为只需要保证parent > childs就行, 因此不需要考虑另一边)
        print!("{:?} inserting: {:?}", self.items, value);

        self.items.push(value);
        self.count += 1;

        // let items = &self.items;

        let mut current_idx = self.len();
        let mut parent_idx = self.parent_idx(current_idx);

        while parent_idx!=0 && (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
            print!(" [swap: ({}, {})] ", current_idx, parent_idx);
            // comparator: < ; MinHeap;
            // std::mem::swap(&mut self.items[current_idx], &mut self.items[parent_idx]);
            self.items.swap(current_idx, parent_idx);
            current_idx = self.parent_idx(current_idx);
            parent_idx = self.parent_idx(current_idx);
        }
        println!(" {:?}", self.items);
    }

    pub fn heapify(&mut self) {
        // 从堆顶开始逐步向下调整
        // 假定是 < ; MinHeap
        // root begins from index 1
        let mut current_idx = 1;
        println!("heapify: {:?}", self.items);
        while self.children_present(current_idx) {
            // 当存在孩子时
            // 正常情况下current要比较"小"的child还小
            let smallest_idx = self.smallest_child_idx(current_idx);
            if (self.comparator)(&self.items[smallest_idx], &self.items[current_idx]) {
                println!("swap: ({:?}, {:?}) of {:?}", self.items[current_idx], self.items[smallest_idx], self.items);
                self.items.swap(current_idx, smallest_idx);
                current_idx = smallest_idx;
            } else {
                break;
            }
        }
    }

    pub fn del(&mut self, idx: usize) -> Option<T> {
        // 将 idx 与 last 交换
        let last_idx = self.len();
        if (1..(last_idx + 1)).contains(&idx) {
            // 需要T具备Copy
            // let target= self.items[idx];
            if idx != last_idx {
                print!("swap: ({:?}, {:?}) of {:?}", self.items[idx], self.items[last_idx], self.items);
                self.items.swap(idx, last_idx);
                print!(" now {:?} ", self.items);
            }
            let target = self.items.remove(last_idx);
            self.count -= 1;
            println!("remove {:?} {:?}", target, self.items);
            self.heapify();
            Some(target)
        } else {
            None
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // smaller_child_idx, idx节点较"小"的那个
        assert_eq!(self.children_present(idx), true);
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        match right_idx > self.len() || (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            // < MinHeap
            true => left_idx,
            false => right_idx,
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Debug + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Debug + Ord,
{
    type Item = T;

    // 返回当前堆顶值, 并将该值从堆中移除
    fn next(&mut self) -> Option<T> {
        let root_idx = 1;
        let last_idx = self.len();
        if (1..(last_idx + 1)).contains(&root_idx) {
            println!("{:?} popping {:?}", self.items, self.items[root_idx]);
            self.del(root_idx)
        } else {
            None
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Debug + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

pub fn test_empty_heap() {
    let mut heap = MaxHeap::new::<i32>();
    assert_eq!(heap.next(), None);
}

pub fn test_min_heap() {
    let mut heap = MinHeap::new();
    heap.add(4);
    heap.add(2);
    heap.add(9);
    heap.add(11);
    assert_eq!(heap.len(), 4);
    assert_eq!(heap.next(), Some(2));
    assert_eq!(heap.next(), Some(4));
    assert_eq!(heap.next(), Some(9));
    heap.add(1);
    assert_eq!(heap.next(), Some(1));
}

pub fn test_max_heap() {
    let mut heap = MaxHeap::new();
    heap.add(4);
    heap.add(2);
    heap.add(9);
    heap.add(11);
    assert_eq!(heap.len(), 4);
    assert_eq!(heap.next(), Some(11));
    assert_eq!(heap.next(), Some(9));
    assert_eq!(heap.next(), Some(4));
    heap.add(1);
    assert_eq!(heap.next(), Some(2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_empty_heap() {
        super::test_empty_heap();
    }

    #[test]
    pub fn test_min_heap() {
        super::test_min_heap();
    }

    #[test]
    pub fn test_max_heap() {
        super::test_max_heap();
    }
}

fn main() {
    // test_empty_heap();
    // test_min_heap();
    test_max_heap();
}