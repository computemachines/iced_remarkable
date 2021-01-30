use iced_graphics::backend::{Backend, Text};
use iced_graphics::Primitive;

use libremarkable::framebuffer::{
    common::{display_temp, dither_mode, mxcfb_rect, waveform_mode},
    core::Framebuffer,
    refresh::PartialRefreshMode,
    FramebufferBase, FramebufferRefresh,
};

pub const DISPLAYWIDTH: u16 = 1404;
pub const DISPLAYHEIGHT: u16 = 1872;

pub struct RemarkableBackend<'a> {
    framebuffer: Framebuffer<'a>,
}

impl Backend for RemarkableBackend<'_> {
    fn trim_measurements(&mut self) {
        println!("trim measurement cache?",);
    }
}

impl Text for RemarkableBackend<'_> {
    const ICON_FONT: iced_graphics::Font = iced_graphics::Font::Default;

    const CHECKMARK_ICON: char = 'X';

    const ARROW_DOWN_ICON: char = 'V';

    fn default_size(&self) -> u16 {
        60
    }

    fn measure(
        &self,
        contents: &str,
        size: f32,
        font: iced_graphics::Font,
        bounds: iced_graphics::Size,
    ) -> (f32, f32) {
        dbg!(contents);
        dbg!(size);
        dbg!(&bounds);
        (contents.len() as f32 * size, size)
    }
}

impl Default for RemarkableBackend<'_> {
    fn default() -> Self {
        Self {
            framebuffer: Framebuffer::new("/dev/fb0"),
        }
    }
}

impl RemarkableBackend<'_> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn render(&mut self, primitive: &Primitive) {
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
