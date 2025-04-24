/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		assert_eq!(self.size, self.data.len());
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		if 0 == self.size {
			None
		} else {
			self.size -= 1;
			self.data.pop()
		}
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { stack: Vec::new() };
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { stack: Vec::new() };
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;
			self.0.data.pop()
		} else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool {
	let pairs: HashMap<char, char> = [('}', '{'), (']', '['), (')', '(')]
		.iter()
		.cloned()
		.collect();
	let left_set = pairs.values().cloned().collect::<HashSet<_>>();
	let right_set = pairs.keys().cloned().collect::<HashSet<_>>();
	let mut stack: Stack<char> = Stack::new();
	// 对于 (char, char) 类型的元组, collect() 会将第一个字符作为键(Key), 第二个字符作为值(Value)
	println!("{:?}", bracket.chars());
	for ch in bracket.chars() {
		// 迭代器: 	all 方法 所有元素是否满足给定的条件
		// 		   any 方法 至少有一个元素满足条件
		println!("{}", ch);
		if left_set.contains(&ch) || right_set.contains(&ch) {
			println!("  {}", ch);
			if left_set.contains(&ch) {
				// ch在左侧
				stack.push(ch);
			} else if right_set.contains(&ch) {
				// ch在右侧, 检查top是否为其左侧
				// copied: Option<&T> to an Option<T>
				if stack.pop() != pairs.get(&ch).copied() {
					// 这里保证pairs.get(&ch)不会为None
					return false;
				}
			}
		}
	}
	println!("{:?}", stack);
	println!("{}", stack.is_empty());
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bracket_matching_1() {
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s), true);
	}
	#[test]
	fn bracket_matching_2() {
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_3() {
		let s = "{{([])}}";
		assert_eq!(bracket_match(s), true);
	}
	#[test]
	fn bracket_matching_4() {
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_5() {
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_6() {
		let s = "";
		assert_eq!(bracket_match(s), true);
	}
}
