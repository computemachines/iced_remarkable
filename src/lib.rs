#![allow(dead_code, unused_variables)]

use backend::RemarkableBackend;
use iced_graphics::Renderer;

pub mod application;
pub mod backend;
pub mod subscription_pool;

pub type RemarkableRenderer = Renderer<RemarkableBackend<'static>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
