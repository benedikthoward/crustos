
pub struct List {
    list_ends: ListNode, // next is the first member, prev is the tail
    length: u16
}

pub struct ListNode {
    prev: Option<*mut ListNode>, 
    next: Option<*mut ListNode>, 

    value: u32, 
    owner: Option<*const TaskControlBlock>, // the list_ends list node in the List does not have a
                                            // TCB owner
    container: Option<*mut List>,
}

impl List {
    pub fn new() -> Self {
        List {
            length: 0,
            list_ends: ListNode {
                prev: None,
                next: None,
                value: 0,
                owner: None,
                container: None,
            }
        }
    }
    pub fn insert_sorted(&mut self, node: &mut ListNode);
    pub fn insert_end(&mut self, node: &mut ListNode);
    pub fn remove(node: &mut ListNode);
    pub fn is_empty(&self) -> bool;
    pub fn head(&self) -> Option<*const ListNode>;

}

impl ListNode {
    pub fn new(value: u32, owner: Option<*const TaskControlBlock>) -> Self {
        ListNode {
            prev: None,
            next: None,
            value,
            owner, 
            container: None,
        }
    }
}

fn insert_sorted(&mut self, node: &mut ListNode) {
    debug_assert!(node.container.is_none(), "Already in a list");

    self.length += 1;
    node.container = Some(self);

    match self.list_ends.next {
        None => {

            self.list_ends.next = Some(node);
            self.list_ends.prev = Some(node);

            node.prev = None;
            node.next = None;

        },

        Some(mut current_node) => unsafe {

            if node.value > (*current_node).value {
                self.list_ends.next = Some(node);
                
                node.prev = None;
                node.next = Some(current_node);

                (*current_node).prev = Some(node);
                
                return;
            }


            while node.value <= (*current_node).value{
                match (*current_node).next {
                    Some(val) => {current_node = val},
                    None => {
                        (*current_node).next = Some(node);
                        self.list_ends.prev = Some(node);

                        node.prev = Some(current_node);
                        return;
                    }
                }
            }

            let predecessor = current_node.prev.unwrap();
            (*predecessor.next) = Some(node);
            current_node.prev = Some(node);

            node.prev = Some(predecessor);
            node.next = Some(current_node);

        }
    }
}


fn insert_end(&mut self, node: &mut ListNode) {

    debug_assert!(node.container.is_none(), "Already in a list");

    node.container = Some(self);
    node.next = None;
    self.length += 1;

    match self.list_ends.prev {
        None => {
            self.list_ends.next = Some(node);
            self.list_ends.prev = Some(node);

            node.prev = None;

        },

        Some(mut current_node) => {

            current_node.next = Some(node);

            self.list_ends.prev = Some(node);

            node.prev = Some(current_node);

        }
    }
}

fn remove(node: &mut ListNode) {

    debug_assert!(node.container.is_none(), "not in a list");

    unsafe {
        let list = &mut *node.container.unwrap();
        list.length -=1;

        match node.next {
            Some(next) => {
                next.prev = node.prev;
            },
            None => {
                list.list_ends.prev = node.prev;
            }
        }

        match node.prev {
            Some(prev) => {
                prev.next = node.next;
            },
            None => {
                list.list_ends.next = node.next;
            },
        }
    }

    node.container = None;
    node.prev = None;
    node.next = None;

}

fn is_empty(&self) -> bool {
    self.list_ends.next.is_none()
}

fn head(&self) -> Option<*const TaskControlBlock> {
    self.list_ends.next.map(|ptr| unsafe { (*ptr).owner.unwrap()})
}
