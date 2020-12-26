#![allow(dead_code, unused_variables)]

use backend::RemarkableBackend;
use iced_graphics::Renderer;

pub mod backend;

pub type RemarkableRenderer<'a> = Renderer<RemarkableBackend<'a>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
