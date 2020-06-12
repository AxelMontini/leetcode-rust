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

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let mut rev_head = ListNode::new(0);

    // This guy points to the next field of the last element in rev_head
    let mut last = &mut rev_head.next;

    loop {
        let (reversed, leftover) = reverse(head, k);

        match (reversed, leftover) {
            // Reversed some stuff and we might have extra elements left over
            (Some(reversed), leftover) => {
                head = leftover;
                *last = Some(reversed);

                println!("LOO");

                for _ in 0..k {
                    match last.as_mut() {
                        Some(last_ref) => last = &mut last_ref.next,
                        None => unreachable!(),
                    }
                }
            }
            // Nothing more has been reversed; we might have a leftover and the job is done
            (None, leftover) => {
                *last = leftover;
                break;
            }
        }
    }

    rev_head.next
}

/// # Returns
/// `(reversed, leftover)`
/// If there were more than `count` nodes, reversed contains the reversed nodes. Else, it's None.
/// Leftover contains the nodes that weren't reversed (if any).
fn reverse(
    mut head: Option<Box<ListNode>>,
    count: usize,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut rev_head = ListNode::new(0);

    // put the elements from head into target, in reversed order.
    let rev = |head: &mut Option<Box<ListNode>>, target: &mut ListNode| {
        let mut i = 0;

        while let Some(ref mut node) = head {
            let new_node = node.next.take();
            let back = target.next.take();
            node.next = back;
            target.next = head.take();
            *head = new_node;

            i += 1;

            if i == count {
                break;
            }
        }

        i
    };

    let r = rev(&mut head, &mut rev_head);

    let mut reversed = rev_head.next;

    // We haven't reversed enough nodes, thus we need to put them back into the original order
    // And return them as leftover
    if r < count {
        let mut undo_head = ListNode::new(0);

        // Not enough nodes, we have to put them back into the same order
        rev(&mut reversed, &mut undo_head);

        (None, undo_head.next)
    } else {
        // We've reversed enough nodes, return them as reversed and head as leftover
        (reversed, head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slice_to_node(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut acc = ListNode::new(0);

        slice.into_iter().fold(&mut acc, |mut cursor, &val| {
            cursor.next = Some(Box::new(ListNode::new(val)));
            cursor = cursor.next.as_mut().map(|next| next.as_mut()).unwrap();
            cursor
        });
        acc.next
    }

    fn node_to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        if let Some(mut first_node) = node {
            let mut curr = first_node.next.take();
            vec.push(first_node.val);

            while let Some(curr_node) = curr.take() {
                vec.push(curr_node.val);
                curr = curr_node.next;
            }
        }
        vec
    }

    #[test]
    fn reverse() {
        let dataset: &[(&[i32], usize, &[i32], &[i32])] = &[
            (&[1, 2, 3, 4, 5, 6], 3, &[3, 2, 1], &[4, 5, 6]),
            (&[1, 2], 3, &[], &[1, 2]),
            (&[], 3, &[], &[]),
        ];

        for (input, count, expected, leftover) in dataset {
            let output = super::reverse(slice_to_node(input), *count);
            let out_rev = &node_to_vec(output.0)[..];
            let out_left = &node_to_vec(output.1)[..];

            assert_eq!(
                (*expected, *leftover),
                (out_rev, out_left),
                "Expected {:?}, found {:?}",
                (*expected, *leftover),
                (out_rev, out_left)
            );
        }
    }

    const DATASET: &[(&[i32], usize, &[i32])] = &[
        (&[1, 2], 2, &[2, 1]),
        (&[1, 2, 3], 3, &[3, 2, 1]),
        (&[1, 2, 3], 2, &[2, 1, 3]),
    ];

    #[test]
    fn reverse_k_groups() {
        for (input, k, expected) in DATASET {
            let input_node = slice_to_node(input);
            println!("First input {:?}", input_node);

            let output_node = super::reverse_k_group(input_node, *k);
            let output_vec = node_to_vec(output_node);

            assert_eq!(
                *expected,
                &output_vec[..],
                "Expected {:?}, found {:?} (k={})",
                expected,
                output_vec,
                k
            );
        }
    }
}
