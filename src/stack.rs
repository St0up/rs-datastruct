//! Ce module fournit la structure de pile (LIFO) avec ses opérations de base.

use std::fmt::Display;

/*
    Node définit de manière standard la structure interne de la pile
*/
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { elem, next }
    }

    fn display(&self)
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

/*
Pile générique définit par la tête d'une liste chaînée
    Cette structure permet d'implementer des meta-données qui permet
    d'avoir des informations en O(1).
    Dans le cas de cette structure, la taille de la liste ou savoir
    si elle est vide.
*/
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    length: i32,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    // Créer une nouvelle pile vide
    pub fn new() -> Stack<T> {
        Stack {
            head: None,
            length: 0,
        }
    }

    // Ajouter un élément à la pile
    pub fn push(&mut self, elem: T) {
        let new_elem: Node<T> = if self.length != 0 {
            Node::new(elem, self.head.take())
        } else {
            Node::new(elem, None)
        };
        self.head = Some(Box::new(new_elem));
        self.length += 1;
    }

    // Dépile un élément, retourne None si la pile est vide
    pub fn pop(&mut self) -> Option<T> {
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

    // Donne la valeur de l'élément en tête, retourne None si la pile est vide
    // Cette fonction est restreinte aux éléments dont le type définit Copy
    pub fn peek(&self) -> Option<T>
    where
        T: Copy,
    {
        match self.length {
            0 => None,
            _ => Some(self.head.as_ref().unwrap().elem),
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
        self.head.as_ref().unwrap().display();
    }
}
