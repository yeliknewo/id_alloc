#[test]
fn alloc_1() {
    let mut node = Node::new_all();
    assert!(node.alloc() == Some(0));
}

#[test]
fn alloc_2() {
    let mut node = Node::new_all();
    assert!(node.alloc() == Some(0));
    assert!(node.alloc() == Some(1));
}

#[test]
fn alloc_3() {
    let mut node = Node::new_all();
    assert!(node.alloc() == Some(0));
    assert!(node.alloc() == Some(1));
    assert!(node.alloc() == Some(2));
}

#[test]
fn dealloc_1() {
    let mut node = Node::new_all();
    assert!(node.alloc() == Some(0));
    node.dealloc(0);
    assert!(node.alloc() == Some(0));
}

#[test]
fn dealloc_2() {
    let mut node = Node::new_all();
    assert!(node.alloc() == Some(0));
    assert!(node.alloc() == Some(1));
    node.dealloc(0);
    assert!(node.alloc() == Some(0));
}

#[test]
fn dealloc_3() {
    let mut node = Node::new_all();
    assert!(node.alloc() == Some(0));
    assert!(node.alloc() == Some(1));
    node.dealloc(1);
    assert!(node.alloc() == Some(1));
    node.dealloc(0);
    assert!(node.alloc() == Some(0));
}

type Id = u64;

#[derive(Eq, PartialEq)]
struct Node {
    bot: Id,
    top: Id,

    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(id: Id) -> Node {
        Node {
            bot: id,
            top: id,
            left: None,
            right: None,
        }
    }

    fn new_all() -> Node {
        Node {
            bot: Id::min_value(),
            top: Id::max_value(),

            left: None,
            right: None,
        }
    }

    fn alloc(&mut self) -> Option<Id> {
        if let Some(lonely) = self.find_lonely() {
            Some(lonely)
        } else if let Some(leaf) = self.find_leaf(true) {
            Some(leaf)
        } else {
            None
        }
    }

    fn dealloc(&mut self, id: Id) {
        assert!(id < self.bot || id > self.top);
        if {
            match self.bot.checked_sub(1) {
                Some(temp) => temp == id,
                None => false,
            }
        } {
            self.bot = self.compact_left(id);
        } else if {
            match self.bot.checked_add(1) {
                Some(temp) => temp == id,
                None => false,
            }
        } {
            self.top = self.compact_right(id);
        } else if id < self.bot {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new(id)));
            } else {
                self.left.as_mut().unwrap().dealloc(id);
            }
        } else if id > self.top {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(id)))
            } else {
                self.right.as_mut().unwrap().dealloc(id);
            }
        } else {
            panic!("IDK");
        }
    }

    fn compact_left(&mut self, id: Id) -> Id {
        if {
            match id.checked_sub(1) {
                Some(temp) => temp == self.top,
                None => false,
            }
        } {
            self.bot
        } else if self.right.is_some() {
            self.right.as_mut().unwrap().compact_left(id)
        } else {
            id
        }
    }

    fn compact_right(&mut self, id: Id) -> Id {
        if {
            match id.checked_add(1) {
                Some(temp) => temp == self.bot,
                None => false,
            }
        } {
            self.top
        } else if self.left.is_some() {
            self.left.as_mut().unwrap().compact_right(id)
        } else {
            id
        }
    }

    fn find_lonely(&mut self) -> Option<Id> {
        if self.left.is_none() && self.right.is_none() {
            if self.bot == self.top {
                return Some(self.bot);
            } else {
                return None;
            }
        } else if self.left.is_some() {
            return self.left.as_mut().unwrap().find_lonely();
        } else if self.right.is_some() {
            return self.right.as_mut().unwrap().find_lonely();
        } else {
            return None;
        }
    }

    fn find_leaf(&mut self, right: bool) -> Option<Id> {
        if self.left.is_none() && self.right.is_none() {
            if right {
                let value = self.bot;
                self.bot += 1;
                Some(value)
            } else {
                let value = self.top;
                self.top -= 1;
                Some(value)
            }
        } else if self.left.is_some() {
            self.left.as_mut().unwrap().find_leaf(false)
        } else if self.right.is_some() {
            self.right.as_mut().unwrap().find_leaf(true)
        } else {
            None
        }
    }
}
