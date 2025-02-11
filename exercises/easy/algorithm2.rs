/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
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
	pub fn reverse(&mut self){
    // 如果链表为空或只有一个元素，不需要反转
    if self.length <= 1 {
        return;
    }

    let mut current = self.start;
    let mut prev: Option<NonNull<Node<T>>> = None;

    // 更新 end 指针为原来的 start
    self.end = current;

    while let Some(mut current_ptr) = current {
        unsafe {
            // 保存下一个节点
            let next = (*current_ptr.as_ptr()).next;
            
            // 反转当前节点的指针
            (*current_ptr.as_ptr()).next = prev;
            (*current_ptr.as_ptr()).prev = next;

            // 移动指针
            prev = current;
            current = next;
        }
    }

    // 更新 start 指针为原来的 end
    self.start = prev;
}
关键修改：

get 方法改为迭代实现，避免递归导致的栈溢出
reverse 方法更加明确地处理 start 和 end 指针
使用迭代方式反转链表的指针
这个实现应该能够：

避免栈溢出
正确反转双向链表
通过提供的测试用例
完整的修改示例：

Copy
impl<T> LinkedList<T> {
    pub fn get(&self, index: i32) -> Option<&T> {
        if index < 0 || index >= self.length as i32 {
            return None;
        }

        let mut current = self.start;
        let mut current_index = 0;

        while let Some(node_ptr) = current {
            unsafe {
                if current_index == index {
                    return Some(&(*node_ptr.as_ptr()).val);
                }
                current = (*node_ptr.as_ptr()).next;
                current_index += 1;
            }
        }

        None
    }

    pub fn reverse(&mut self) {
        // 如果链表为空或只有一个元素，不需要反转
        if self.length <= 1 {
            return;
        }

        let mut current = self.start;
        let mut prev: Option<NonNull<Node<T>>> = None;

        // 更新 end 指针为原来的 start
        self.end = current;

        while let Some(mut current_ptr) = current {
            unsafe {
                // 保存下一个节点
                let next = (*current_ptr.as_ptr()).next;
                
                // 反转当前节点的指针
                (*current_ptr.as_ptr()).next = prev;
                (*current_ptr.as_ptr()).prev = next;

                // 移动指针
                prev = current;
                current = next;
            }
        }

        // 更新 start 指针为原来的 end
        self.start = prev;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
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
    T: Display,
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
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}