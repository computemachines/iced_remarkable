#![allow(dead_code, unused_variables)]

use cgmath::Point2;
use iced_native::renderer::Renderer;
use iced_native::widget::{button, text};
use libremarkable::framebuffer::{
    common::{color, display_temp, dither_mode, mxcfb_rect, waveform_mode},
    core::Framebuffer,
    refresh::PartialRefreshMode,
    FramebufferBase, FramebufferRefresh,
};

pub struct RemarkableRenderer<'a> {
    framebuffer: Framebuffer<'a>,
}

impl Default for RemarkableRenderer<'_> {
    fn default() -> Self {
        Self {
            framebuffer: Framebuffer::new("/dev/fb0"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Line(Point2<u32>, u32, color),
    Text(String),
    Group(Vec<Primitive>),
    Offset(Point2<u32>, Box<Primitive>),
    Nothing,
}
#[derive(Default)]
pub struct Style;

impl RemarkableRenderer<'_> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn render(&mut self, primitive: &Primitive) {
        // draw to self.framebuffer
        dbg!(primitive);
    }
    pub fn update_full(&self) {
        self.framebuffer.full_refresh(
            waveform_mode::WAVEFORM_MODE_GC16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
            0,
            true,
        );
    }
    pub fn update_partial(&self, region: &mxcfb_rect) {
        self.framebuffer.partial_refresh(
            region,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_GLR16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0,
            false,
        );
    }
}

impl Renderer for RemarkableRenderer<'_> {
    type Output = Primitive;

    type Defaults = ();
}

impl button::Renderer for RemarkableRenderer<'_> {
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
        dbg!(bounds);
        let outline = Primitive::Line((100, 100).into(), 5, color::BLACK);
        let inner = content.draw(self, defaults, content_layout, cursor_position);
        Primitive::Group(vec![outline, inner])
    }
}

impl text::Renderer for RemarkableRenderer<'_> {
    const DEFAULT_SIZE: u16 = 0;

    fn measure(
        &self,
        content: &str,
        size: u16,
        font: iced_core::Font,
        bounds: iced_core::Size,
    ) -> (f32, f32) {
        println!("measure({})", content);
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
