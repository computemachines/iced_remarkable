use std::cell::RefCell;

use iced_core::Rectangle;
use iced_graphics::Primitive;

#[derive(Debug)]
pub enum Node<'a> {
    Primitive(&'a Primitive, Rectangle),
    BoundingBox {
        children: Vec<Node<'a>>,
        region: Rectangle,
    },
}

impl<'a> Node<'a> {
    pub fn insert_unchecked(&mut self, primitive: &'a Primitive) {}
}
// trying to be able to mutate self leaf into branch
// pub struct Tree {
//     children:
// }
