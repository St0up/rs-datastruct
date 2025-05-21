//! Ce module fournit la structure de file (FIFO) avec ses opérations de base.

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
pub struct Queue<T> {
    exit: Option<Box<SimpleNode<T>>>,
    length_exit: i32,
    enter: Option<Box<SimpleNode<T>>>,
    length_enter: i32,
    length: i32,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    // Créer une nouvelle pile vide
    pub fn new() -> Queue<T> {
        Queue {
            exit: None,
            length_exit: 0,
            enter: None,
            length_enter: 0,
            length: 0,
        }
    }

    fn exit_to_enter(&mut self) {
        self.exit = Some(self.enter.take().unwrap().reverse());
        self.length_exit = self.length_enter;
        self.enter = None;
        self.length_enter = 0;
    }

    // Ajouter un élément à la pile
    pub fn enqueue(&mut self, elem: T) {
        self.enter = Some(Box::new(SimpleNode::new(elem, self.enter.take())));
        self.length_enter += 1;
        self.length += 1;
    }

    // Dépile un élément, retourne None si la pile est vide
    pub fn dequeue(&mut self) -> Option<T>
    where
        T: Display,
    {
        match self.length {
            0 => None,
            _ => {
                if self.length_exit == 0 {
                    self.exit_to_enter();
                }
                self.length -= 1;
                self.length_exit -= 1;
                self.exit.take().map(|mut node| {
                    self.exit = node.next.take();
                    node.elem
                })
            }
        }
    }

    // Donne la valeur de l'élément en tête, retourne None si la pile est vide
    // Cette fonction est restreinte aux éléments dont le type définit Copy
    pub fn peek(&self) -> Option<&T> {
        match self.length {
            0 => None,
            _ => match self.length_exit {
                0 => self.enter.as_ref().unwrap().last_node(),
                _ => Some(&self.exit.as_ref().unwrap().elem),
            },
        }
    }

    // Dis si une pile est vide ou non
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Donne la taille de la pile
    pub fn len(&self) -> i32 {
        self.length
    }

    // Affiche les éléments de la liste
    // Cette fonction est restreinte aux éléments dont le type définit Display
    pub fn display(&self)
    where
        T: Display,
    {
        if self.length_enter != 0 {
            self.enter.as_ref().unwrap().display();
        }
        if self.length_exit != 0 {
            self.exit.as_ref().unwrap().reverse_display();
        }
        println!("EXIT");
    }
}
