use cgmath::{Point2, Vector2};
use iced_core::{Point, Rectangle, Vector};
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
use std::{cell::RefCell, sync::mpsc::Receiver};
mod primitive_utils;
use primitive_utils::primitive_children;
mod bvh;

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
        // log::info!("measure {:?}", contents);
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
    pub fn render(&mut self, root: &Primitive, cached: &Primitive) {
        let mut stack = vec![(root, Vector::new(0f32, 0f32))];
        let mut bvh = bvh::Node::BoundingBox {
            children: vec![],
            region: Rectangle::new(
                iced_core::Point::new(0f32, 0f32),
                iced_core::Size::new(DISPLAYWIDTH as f32, DISPLAYHEIGHT as f32),
            ),
        };

        // an iterator would be more idiomatic here
        while let Some((parent, origin)) = stack.pop() {
            // log::debug!("parent = {:?}", parent);
            for primitive in primitive_children(parent) {
                match primitive {
                    // ---- Group Primitives ----
                    Primitive::Translate {
                        translation,
                        content,
                    } => stack.push((content, (origin + *translation))),
                    Primitive::Group { primitives } => {
                        stack.extend(primitives.iter().map(|p| (p, origin)))
                    }
                    Primitive::Cached { cache } => unimplemented!(),

                    // ---- Leaf Primitives ----
                    _ => bvh.insert_unchecked(primitive),
                }
            }
        }
        dbg!(bvh);
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
