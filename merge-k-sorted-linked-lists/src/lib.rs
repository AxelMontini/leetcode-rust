// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::cmp::Ordering;

struct Entry(i32, Box<ListNode>, usize);

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Entry {}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut first_junk = ListNode::new(0);
    let mut cursor: &mut ListNode = &mut first_junk;

    let mut heap = BinaryHeap::with_capacity(lists.len());

    // FIll heap with initial values
    for (index, node_initial) in lists.iter_mut().enumerate() {
        if let Some(node) = node_initial {
            let next = node.next.take();
            let val = node.val;
            let node_replaced = std::mem::replace(node_initial, next).unwrap();
            heap.push(Reverse(Entry(val, node_replaced, index)));
        }
    }

    while let Some(Reverse(Entry(val, node, index))) = heap.pop() {
        cursor.next = Some(node);
        cursor = cursor.next.as_mut().unwrap();

        if let Some(mut node) = lists[index].take() {
            lists[index] = node.next.take();
            heap.push(Reverse(Entry(node.val, node, index)))
        }
    }

    first_junk.next
}

pub fn merge_k_lists_binary_heap(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::cmp::Reverse;

    let mut heap = std::collections::BinaryHeap::with_capacity(lists.len());
    // Fill heap with initial values
    for (index, initial_node) in lists.iter_mut().enumerate() {
        if let Some(mut node) = initial_node.take() {
            heap.push(Reverse((node.val, index)));
            *initial_node = node.next.take();
        }
    }

    while let Some(Reverse((val, index))) = heap.pop() {}

    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
