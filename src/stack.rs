//! Ce module fournit la structure de pile (LIFO) avec ses opérations de base.

use std::fmt::Display;

use crate::linked_list::LinkedList;

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
pub struct Stack<T> {
    list: LinkedList<T>,
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
            list: LinkedList::new(),
        }
    }

    // Ajouter un élément à la pile
    pub fn push(&mut self, elem: T) {
        self.list.push_front(elem);
    }

    // Dépile un élément, retourne None si la pile est vide
    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    // Donne la valeur de l'élément en tête, retourne None si la pile est vide
    // Cette fonction est restreinte aux éléments dont le type définit Copy
    pub fn peek(&self) -> Option<&T> {
        self.list.peek_front()
    }

    // Dis si une pile est vide ou non
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    // Donne la taille de la pile
    pub fn len(&self) -> u32 {
        self.list.len()
    }

    // Affiche les éléments de la liste
    // Cette fonction est restreinte aux éléments dont le type définit Display
    pub fn display(&self)
    where
        T: Display,
    {
        self.list.display();
    }
}
