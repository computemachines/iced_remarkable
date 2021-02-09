use cgmath::{Point2, Vector2};
use iced_graphics::backend::{Backend, Text};
use iced_graphics::Primitive;
use libremarkable::{
    framebuffer::{
        common::{
            color::{BLACK, GRAY, RGB, WHITE},
            display_temp, dither_mode, mxcfb_rect, waveform_mode,
        },
        core::Framebuffer,
        refresh::PartialRefreshMode,
        FramebufferBase, FramebufferDraw, FramebufferRefresh,
    },
    input::{ev::EvDevContext, InputDevice, InputEvent},
};
use log;
use logging_timer::{stime, time};
use std::sync::mpsc::Receiver;

pub const DISPLAYWIDTH: u16 = 1404;
pub const DISPLAYHEIGHT: u16 = 1872;

pub struct RemarkableBackend<'a> {
    framebuffer: Framebuffer<'a>,
    pub input_rx: Receiver<InputEvent>,
}

impl Backend for RemarkableBackend<'_> {
    fn trim_measurements(&mut self) {
        log::info!("trim measurement cache?",);
    }
}

impl Text for RemarkableBackend<'_> {
    const ICON_FONT: iced_graphics::Font = iced_graphics::Font::Default;

    const CHECKMARK_ICON: char = 'X';

    const ARROW_DOWN_ICON: char = 'V';

    fn default_size(&self) -> u16 {
        // log::info!("default_size()");
        60
    }

    fn measure(
        &self,
        contents: &str,
        size: f32,
        font: iced_graphics::Font,
        bounds: iced_graphics::Size,
    ) -> (f32, f32) {
        log::info!("measure {:?}", contents);
        (contents.len() as f32 * size * 0.5, size)
    }
}

impl RemarkableBackend<'_> {
    pub fn new() -> Self {
        let (input_tx, input_rx) = std::sync::mpsc::channel::<InputEvent>();
        EvDevContext::new(InputDevice::GPIO, input_tx.clone()).start();
        EvDevContext::new(InputDevice::Multitouch, input_tx.clone()).start();
        EvDevContext::new(InputDevice::Wacom, input_tx.clone()).start();
        Self {
            framebuffer: Framebuffer::from_path("/dev/fb0"),
            input_rx,
        }
    }
    pub fn clear(&mut self) {
        self.framebuffer.clear();
    }
    pub fn render(&mut self, primitive: &Primitive) {
        match primitive {
            Primitive::None => {}
            Primitive::Group { primitives } => {
                for primitive in primitives {
                    self.render(primitive);
                }
            }
            Primitive::Text {
                content,
                bounds,
                color,
                size,
                font,
                horizontal_alignment,
                vertical_alignment,
            } => {
                self.framebuffer.draw_text(
                    Point2::new(bounds.x, bounds.y + bounds.height - 6.0),
                    content.clone(),
                    *size,
                    RGB(
                        (color.r * 256.0) as u8,
                        (color.g * 256.0) as u8,
                        (color.b * 256.0) as u8,
                    ),
                    false,
                );
            }
            Primitive::Quad {
                bounds,
                background,
                border_radius,
                border_width,
                border_color,
            } => self.framebuffer.draw_rect(
                Point2::new(bounds.x as i32, bounds.y as i32),
                Vector2::new(bounds.width as u32, bounds.height as u32),
                2,
                BLACK,
            ),
            Primitive::Image { handle, bounds } => {}
            Primitive::Svg { handle, bounds } => {}
            Primitive::Clip {
                bounds,
                offset,
                content,
            } => {}
            Primitive::Translate {
                translation,
                content,
            } => {
                unimplemented!("translation",);
            }
            Primitive::Mesh2D { buffers, size } => {}
            Primitive::Cached { cache } => unimplemented!("cache",),
        }
    }
    #[stime]
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
