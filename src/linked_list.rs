use std::fmt::Display;

pub(crate) struct SimpleNode<T> {
    pub(crate) elem: T,
    pub(crate) next: Option<Box<SimpleNode<T>>>,
}

impl<T> SimpleNode<T> {
    pub(crate) fn new(elem: T, next: Option<Box<SimpleNode<T>>>) -> SimpleNode<T> {
        SimpleNode { elem, next }
    }

    fn reverse_aux(mut self: Box<Self>, prev: Box<SimpleNode<T>>) -> Box<SimpleNode<T>> {
        let current = self.next;
        self.next = Some(prev);
        match current {
            Some(next_node) => next_node.reverse_aux(self),
            None => self,
        }
    }

    pub(crate) fn reverse(mut self: Box<Self>) -> Box<SimpleNode<T>> {
        match self.next.take() {
            Some(node) => node.reverse_aux(self),
            None => self,
        }
    }

    pub(crate) fn last_node(&self) -> Option<&T> {
        match &self.next {
            Some(node) => node.last_node(),
            None => Some(&self.elem),
        }
    }

    pub(crate) fn display(&self)
    where
        T: Display,
    {
        print!("{} -> ", self.elem);
        if let Some(node) = &self.next {
            node.display()
        }
    }

    pub(crate) fn reverse_display(&self)
    where
        T: Display,
    {
        if let Some(node) = &self.next {
            node.display()
        }
        print!("{} -> ", self.elem);
    }
}

pub(crate) struct DoubleNode<T> {
    pub(crate) elem: T,
    pub(crate) next: Option<Box<DoubleNode<T>>>,
    pub(crate) last: Option<Box<DoubleNode<T>>>,
}

impl<T> DoubleNode<T> {
    pub(crate) fn new(
        elem: T,
        next: Option<Box<DoubleNode<T>>>,
        last: Option<Box<DoubleNode<T>>>,
    ) -> DoubleNode<T> {
        DoubleNode { elem, next, last }
    }

    pub(crate) fn display(&self)
    where
        T: Display,
    {
        print!("{} -> ", self.elem);
        match &self.next {
            Some(node) => node.display(),
            None => println!("END"),
        }
    }
}
