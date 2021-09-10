use crate::{AppView, SurfaceView};

pub struct WgpuCanvas {
    pub app_view: AppView,
}

#[allow(dead_code)]
impl WgpuCanvas {
    pub fn new(app_view: AppView) -> Self {
        let instance = WgpuCanvas { app_view };
        if let Some(callback) = instance.app_view.callback_to_app {
            callback(0);
        }
        instance
    }

    pub fn reset(&mut self) {}
}

impl SurfaceView for WgpuCanvas {
    fn resize(&mut self) {}

    fn enter_frame(&mut self) {
        if let Some(callback) = self.app_view.callback_to_app {
            callback(123);
        }
    }
}
