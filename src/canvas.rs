use std::sync::Mutex;

use libremarkable::framebuffer::{
    common::display_temp, common::dither_mode, common::mxcfb_rect, common::waveform_mode,
    core::Framebuffer, refresh::PartialRefreshMode, FramebufferRefresh,
};

#[allow(dead_code)]
pub struct Canvas<'a> {
    pub framebuffer: Framebuffer<'a>,
}

impl<'a> Canvas<'a> {
    pub fn update_full(&self) {
        self.framebuffer.lock().unwrap().full_refresh(
            waveform_mode::WAVEFORM_MODE_GC16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
            0,
            true,
        );
    }
    pub fn update_partial(&self, region: &mxcfb_rect) {
        self.framebuffer.lock().unwrap().partial_refresh(
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
