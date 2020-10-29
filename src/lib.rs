use cgmath::Point2;
use iced_native::renderer::Renderer;
use iced_native::widget::{button, text};
use libremarkable::framebuffer::{common::color, core::Framebuffer};

#[derive(Default)]
pub struct RemarkableRenderer;

#[derive(Debug)]
pub enum Primitive {
    Line(Point2<u32>, u32, color),
    Text(String),
    Group(Vec<Primitive>),
    Offset(Point2<u32>, Box<Primitive>),
    Nothing,
}
#[derive(Default)]
pub struct Style;

impl RemarkableRenderer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn render(&mut self, framebuffer: &mut Framebuffer, primitive: &Primitive) {
        dbg!(primitive);
    }
}

impl Renderer for RemarkableRenderer {
    type Output = Primitive;

    type Defaults = Style;
}

impl button::Renderer for RemarkableRenderer {
    const DEFAULT_PADDING: u16 = 0;

    type Style = Style;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: iced_core::Rectangle,
        cursor_position: iced_core::Point,
        is_disabled: bool,
        is_pressed: bool,
        style: &Self::Style,
        content: &iced_native::Element<'_, Message, Self>,
        content_layout: iced_native::Layout<'_>,
    ) -> Self::Output {
        Primitive::Line((100, 100).into(), 5, color::BLACK)
    }
}

impl text::Renderer for RemarkableRenderer {
    const DEFAULT_SIZE: u16 = 0;

    fn measure(
        &self,
        content: &str,
        size: u16,
        font: iced_core::Font,
        bounds: iced_core::Size,
    ) -> (f32, f32) {
        (100.0, 20.0)
    }

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: iced_core::Rectangle,
        content: &str,
        size: u16,
        font: iced_core::Font,
        color: Option<iced_core::Color>,
        horizontal_alignment: iced_core::HorizontalAlignment,
        vertical_alignment: iced_core::VerticalAlignment,
    ) -> Self::Output {
        Primitive::Text(String::from(content))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
