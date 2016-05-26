extern crate num;

#[derive(Eq, PartialEq)]
pub struct Node<T: num::Num + num::Bounded + Ord + num::CheckedAdd + num::CheckedSub + num::One + Copy> {
    bot: T,
    top: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: num::Num + num::Bounded + Ord + num::CheckedAdd + num::CheckedSub + num::One + Copy> Node<T> {
    fn new(id: T) -> Node<T> {
        Node {
            bot: id,
            top: id,
            left: None,
            right: None,
        }
    }

    pub fn new_all() -> Node<T> {
        Node {
            bot: T::min_value(),
            top: T::max_value(),

            left: None,
            right: None,
        }
    }

    pub fn alloc(&mut self) -> Option<T> {
        if let Some(lonely) = self.find_lonely() {
            Some(lonely)
        } else if let Some(leaf) = self.find_leaf(true) {
            Some(leaf)
        } else {
            None
        }
    }

    pub fn dealloc(&mut self, id: T) {
        assert!(id < self.bot || id > self.top);
        if {
            match self.bot.checked_sub(&T::one()) {
                Some(temp) => temp == id,
                None => false,
            }
        } {
            self.bot = self.compact_left(id);
        } else if {
            match self.bot.checked_add(&T::one()) {
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

    fn compact_left(&mut self, id: T) -> T {
        if {
            match id.checked_sub(&T::one()) {
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

    fn compact_right(&mut self, id: T) -> T {
        if {
            match id.checked_add(&T::one()) {
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

    fn find_lonely(&mut self) -> Option<T> {
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

    fn find_leaf(&mut self, right: bool) -> Option<T> {
        if self.left.is_none() && self.right.is_none() {
            if right {
                let value = self.bot;
                self.bot = self.bot + T::one();
                Some(value)
            } else {
                let value = self.top;
                self.top = self.bot - T::one();
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
