#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
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
    fn new_all() -> Node {
        Node {
            bot: Id::min_value(),
            top: Id::max_value(),

            left: None,
            right: None,
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

    fn find_leaf(&mut self, parent: Option<Box<Node>>) -> Option<Id> {
        if self.left.is_none() && self.right.is_none() {
            if parent.is_none() || {
                match parent {
                    Some(p) => {
                        match p.right {
                            Some(r) => self == r,
                            None => false,
                        }
                    },
                    None => false,
                }
            } {

            }
        }
        None
    }
}

struct IdManager {
    node: Node,
}

impl IdManager {
    fn new() -> IdManager {
        IdManager {
            node: Node::new_all(),
        }
    }

    fn alloc(&mut self) -> Option<Id> {
        let lonely = self.node.find_lonely();
        if lonely.is_some() {
            return lonely;
        }
        None
    }

    fn dealloc(&mut self, id: Id) {

    }
}
