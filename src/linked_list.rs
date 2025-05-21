//! Ce module fournit la structure de pile (LIFO) avec ses opérations de base.

use std::fmt::Display;

use crate::simple_node::SimpleNode;

/*
    Node définit de manière standard la structure interne de la pile
*/

/*
Pile générique définit par la tête d'une liste chaînée
    Cette structure permet d'implementer des meta-données qui permet
    d'avoir des informations en O(1).
    Dans le cas de cette structure, la taille de la liste ou savoir
    si elle est vide.
*/
pub struct LinkedList<T> {
    head: Option<Box<SimpleNode<T>>>,
    length: u32,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    // Créer une nouvelle liste chaînée vide
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    // Ajouter un élément à la liste chaînée
    pub fn push_front(&mut self, elem: T) {
        self.head = Some(Box::new(SimpleNode::new(elem, self.head.take())));
        self.length += 1;
    }

    pub fn push_back(&mut self, elem: T) {
        match self.length {
            0 => self.head = Some(Box::new(SimpleNode::new(elem, None))),
            _ => {
                self.head.as_mut().unwrap().new_last_node(elem);
            }
        }
        self.length += 1;
    }

    //  enlève l'élément de tête de la liste
    pub fn pop_front(&mut self) -> Option<T> {
        match self.length {
            0 => None,
            _ => {
                let node = self.head.take().unwrap();
                self.head = node.next;
                self.length -= 1;
                Some(node.elem)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.length {
            0 => None,
            1 => {
                self.length -= 1;
                Some(self.head.take().unwrap().elem)
            }
            _ => {
                self.length -= 1;
                self.head.as_mut().unwrap().give_last_node()
            }
        }
    }

    // Donne la valeur de l'élément en tête, retourne None si la pile est vide
    // Cette fonction est restreinte aux éléments dont le type définit Copy
    pub fn peek_front(&self) -> Option<&T> {
        match self.length {
            0 => None,
            _ => Some(&self.head.as_ref().unwrap().elem),
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        match self.length {
            0 => None,
            _ => self.head.as_ref().unwrap().last_node(),
        }
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        match self.length {
            size if (size <= index) => None,
            _ => self.head.as_ref().unwrap().index_node(0, index),
        }
    }

    pub fn get_mut(&mut self, index: u32) -> Option<&mut T> {
        match self.length {
            size if (size <= index) => None,
            _ => self.head.as_mut().unwrap().mut_index_node(0, index),
        }
    }

    pub fn contains(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        match self.length {
            0 => false,
            _ => self.head.as_ref().unwrap().contains_node(value),
        }
    }

    pub fn find<P>(&self, predicat: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        match self.length {
            0 => None,
            _ => self.head.as_ref().unwrap().find(predicat),
        }
    }

    pub fn insert(&mut self, index: u32, elem: T) {
        match index {
            0 => self.push_front(elem),
            idx if idx == self.length => self.push_back(elem),
            idx if idx > self.length => {
                unreachable!("It's not possible the linked list is too small !!")
            }
            _ => {
                self.length += 1;
                self.head.as_mut().unwrap().new_index_node(0, index, elem)
            }
        }
    }

    pub fn remove(&mut self, index: u32) -> Option<T> {
        match index {
            0 => self.pop_front(),
            idx if idx == self.length - 1 => self.pop_back(),
            idx if idx >= self.length => {
                unreachable!("It's not possible the linked list is too small !!")
            }
            _ => {
                self.length -= 1;
                self.head.as_mut().unwrap().remove_index_node(0, index)
            }
        }
    }

    pub fn reverse(&mut self) {
        self.head = self.head.take().map(|node| node.reverse());
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.length = 0;
    }

    // Dis si une pile est vide ou non
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Donne la taille de la pile
    pub fn len(&self) -> u32 {
        self.length
    }

    // Affiche les éléments de la liste
    // Cette fonction est restreinte aux éléments dont le type définit Display
    pub fn display(&self)
    where
        T: Display,
    {
        self.head.as_ref().unwrap().display();
        println!("END");
    }
}
