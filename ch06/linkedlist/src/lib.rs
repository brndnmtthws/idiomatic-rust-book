//! # linkedlist crate
//!
//! This crate provides a simple linked list implementation.
//!
//! The crates serves as a teaching example for the book [_Code Like a Pro in
//! Rust_](https://www.manning.com/books/code-like-a-pro-in-rust).
//!
//! ## Example usage
//!
//! ```rust
//! use linkedlist::LinkedList;
//!
//! let mut animals = LinkedList::new();
//! animals.append("chicken");
//! animals.append("ostrich");
//! animals.append("antelope");
//! animals.append("axolotl");
//! animals.append("okapi");
//! ```

use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

struct ListItem<T> {
    data: ItemData<T>,
    next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T> {
    fn new(t: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(t)),
            next: None,
        }
    }
}

/// Provides a singly linked list implementation with iterator access.
pub struct LinkedList<T> {
    head: Option<ListItemPtr<T>>,
}

impl<T> LinkedList<T> {
    /// Constructs a new, empty [`LinkedList<T>`].
    pub fn new() -> Self {
        Self { head: None }
    }
    /// Appends an element to the end of of the list. If the list is empty,
    /// the element becomes the first element of the list.
    pub fn append(&mut self, t: T) {
        match &self.head {
            Some(head) => {
                let mut next = head.clone();
                while next.as_ref().borrow().next.is_some() {
                    let n = next
                        .as_ref()
                        .borrow()
                        .next
                        .as_ref()
                        .unwrap()
                        .clone();
                    next = n;
                }
                next.as_ref().borrow_mut().next =
                    Some(Rc::new(RefCell::new(ListItem::new(t))));
            }
            None => {
                self.head = Some(Rc::new(RefCell::new(ListItem::new(t))));
            }
        }
    }
    /// Returns an iterator over the list.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
            data: None,
            phantom: PhantomData,
        }
    }
    /// Returns an iterator over the list that allows mutation.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.clone(),
            data: None,
            phantom: PhantomData,
        }
    }
    /// Consumes this list returning an iterator over its values.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: self.head.clone(),
        }
    }
}

/// Provides an iterator over `&T` for [`LinkedList<T>`].
pub struct Iter<'a, T> {
    next: Option<ListItemPtr<T>>,
    data: Option<ItemData<T>>,
    phantom: PhantomData<&'a T>,
}
/// Provides an iterator with mutability over `&mut T` for [`LinkedList<T>`].
pub struct IterMut<'a, T> {
    next: Option<ListItemPtr<T>>,
    data: Option<ItemData<T>>,
    phantom: PhantomData<&'a T>,
}
/// Provides an iterator to access owned elements from [`LinkedList<T>`].
pub struct IntoIter<T> {
    next: Option<ListItemPtr<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                self.data = Some(ptr.as_ref().borrow().data.clone());
                unsafe { Some(&*self.data.as_ref().unwrap().as_ptr()) }
            }
            None => None,
        }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                self.data = Some(ptr.as_ref().borrow().data.clone());
                unsafe { Some(&mut *self.data.as_ref().unwrap().as_ptr()) }
            }
            None => None,
        }
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                let listitem =
                    Rc::try_unwrap(ptr).map(|refcell| refcell.into_inner());
                match listitem {
                    Ok(listitem) => Rc::try_unwrap(listitem.data)
                        .map(|refcell| refcell.into_inner())
                        .ok(),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
impl<'a, T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut cloned = Self::new();
        cloned.clone_from(self);
        cloned
    }
    fn clone_from(&mut self, source: &Self) {
        self.head = None;
        source.iter().for_each(|item| self.append(item.clone()));
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list().entries(self.iter()).finish()
    }
}
