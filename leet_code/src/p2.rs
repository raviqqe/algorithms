use core::mem::swap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list_len(&l1) < list_len(&l2) {
        swap(&mut l1, &mut l2);
    }

    {
        let mut l1 = &mut l1;
        let mut x = 0;

        while let Some(n1) = l1 {
            if let Some(n2) = l2 {
                x += n2.val;
                l2 = n2.next;
            }

            let y = x + n1.val;

            n1.val = y % 10;
            x = y / 10;

            l1 = &mut n1.next;
        }

        if x != 0 {
            *l1 = Some(ListNode { val: x, next: None }.into());
        }
    }

    l1
}

fn list_len(l: &Option<Box<ListNode>>) -> usize {
    if let Some(l) = l {
        list_len(&l.next) + 1
    } else {
        0
    }
}
