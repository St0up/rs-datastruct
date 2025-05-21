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

    pub(crate) fn contains_node(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        self.elem == value
            || self
                .next
                .as_ref()
                .map_or(false, |node| node.contains_node(value))
    }

    pub fn find<P>(&self, predicat: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        Some(&self.elem)
            .filter(|e| predicat(e))
            .or_else(|| self.next.as_ref().and_then(|node| node.find(predicat)))
    }

    pub(crate) fn index_node(&self, current_index: u32, research_index: u32) -> Option<&T> {
        match current_index {
            x if (x == research_index) => Some(&self.elem),
            _ => match &self.next {
                Some(node) => node.index_node(current_index + 1, research_index),
                None => unreachable!(
                    "Problème d'implementation, cet élément doit forcément avoir un next"
                ),
            },
        }
    }

    pub(crate) fn mut_index_node(
        &mut self,
        current_index: u32,
        research_index: u32,
    ) -> Option<&mut T> {
        match current_index {
            x if (x == research_index) => Some(&mut self.elem),
            _ => match &mut self.next {
                Some(node) => node.mut_index_node(current_index + 1, research_index),
                None => unreachable!(
                    "Problème d'implementation, cet élément doit forcément avoir un next"
                ),
            },
        }
    }

    pub(crate) fn new_index_node(&mut self, current_index: u32, research_index: u32, elem: T) {
        match current_index {
            x if (x + 1 == research_index) => {
                self.next = Some(Box::new(SimpleNode::new(elem, self.next.take())))
            }
            _ => match &mut self.next {
                Some(node) => node.new_index_node(current_index + 1, research_index, elem),
                None => unreachable!(
                    "Problème d'implementation, cet élément doit forcément avoir un next"
                ),
            },
        }
    }

    pub(crate) fn remove_index_node(
        &mut self,
        current_index: u32,
        research_index: u32,
    ) -> Option<T> {
        match current_index {
            x if (x + 1 == research_index) => self.next.take().map(|mut next_node| {
                self.next = next_node.next.take();
                next_node.elem
            }),
            _ => match &mut self.next {
                Some(node) => node.remove_index_node(current_index + 1, research_index),
                None => unreachable!(
                    "Problème d'implementation, cet élément doit forcément avoir un next"
                ),
            },
        }
    }

    pub(crate) fn last_node(&self) -> Option<&T> {
        match &self.next {
            Some(node) => node.last_node(),
            None => Some(&self.elem),
        }
    }

    pub(crate) fn new_last_node(&mut self, elem: T) {
        match &mut self.next {
            Some(node) => node.new_last_node(elem),
            None => self.next = Some(Box::new(SimpleNode::new(elem, None))),
        }
    }

    pub(crate) fn give_last_node(&mut self) -> Option<T> {
        match self.next.as_mut() {
            Some(node) => match node.next {
                Some(_) => node.give_last_node(),
                None => Some(self.next.take().unwrap().elem),
            },
            None => unreachable!(
                "Problème d'implementation, le premier élément doit forcément avoir un next"
            ),
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
