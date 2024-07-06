#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
     let main_head = head.clone();
    let mut current = head.unwrap().next;
    let mut i = 0;
    while current.is_some() {
        let v = current.clone().unwrap().val;
        if v == 0 {

        } else {
            i += v;
        }

        current = current.unwrap().next
    }

    main_head
}