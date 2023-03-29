use baseview::{Event, EventStatus, Window, WindowHandle, WindowHandler, WindowOpenOptions};
use nih_plug::prelude::*;
use std::sync::Arc;

pub fn create_lemna_editor() -> Option<Box<dyn Editor>> {
    Some(Box::new(LemnaEditor::new()))
}

#[derive(Clone)]
pub struct LemnaEditor {
    // TODO lemna_state: lemna::UI<>
    // TODO window state
}

impl LemnaEditor {
    fn new() -> Self {
        Self {}
    }
}

impl Editor for LemnaEditor {
    fn spawn(
        &self,
        parent: ParentWindowHandle,
        context: Arc<dyn GuiContext>,
    ) -> Box<dyn std::any::Any + Send> {
        let settings = WindowOpenOptions {
            title: "test".to_string(),
            size: baseview::Size::new(100.0, 100.0),
            scale: baseview::WindowScalePolicy::SystemScaleFactor,
        };

        let handle = LemnaWindow::open_parented(&parent, settings);
        Box::new(LemnaEditorHandle { window: handle })
    }

    fn size(&self) -> (u32, u32) {
        (100, 100)
    }
    fn set_scale_factor(&self, factor: f32) -> bool {
        true
    }
    fn param_value_changed(&self, id: &str, normalized_value: f32) {
        ()
    }
    fn param_modulation_changed(&self, id: &str, modulation_offset: f32) {
        ()
    }
    fn param_values_changed(&self) {
        ()
    }
}

pub struct LemnaWindow {}

impl LemnaWindow {
    fn new() -> Self {
        Self {}
    }

    fn open_parented(parent: &ParentWindowHandle, mut settings: WindowOpenOptions) -> WindowHandle {
        Window::open_parented(
            parent,
            settings,
            move |window: &mut baseview::Window<'_>| -> LemnaWindow { LemnaWindow::new() },
        )
    }
}

impl WindowHandler for LemnaWindow {
    fn on_frame(&mut self, window: &mut Window) {}
    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        EventStatus::Ignored
    }
}

struct LemnaEditorHandle {
    window: WindowHandle,
}

unsafe impl Send for LemnaEditorHandle {}
