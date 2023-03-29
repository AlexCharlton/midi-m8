use nih_plug::prelude::*;
use std::{marker::PhantomData, sync::Arc};

use lemna::backends::baseview::Window;
use lemna::{self, widgets, UI, *};
type Renderer = lemna::render::wgpu::WGPURenderer;
type Node = lemna::Node<Renderer>;

#[derive(Debug)]
pub struct HelloApp {}

impl lemna::Component<Renderer> for HelloApp {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new(),
                lay!(size: size_pct!(100.0), wrap: true,
                     padding: rect!(10.0),
                     axis_alignment: Alignment::Center, cross_alignment: Alignment::Center)
            )
            .push(node!(
                widgets::Div::new().bg(Color::rgb(1.0, 0.0, 0.0)),
                lay!(size: size!(200.0, 100.0), margin: rect!(5.0)),
                0
            ))
            .push(node!(
                widgets::Div::new().bg(Color::rgb(0.0, 1.0, 0.0)),
                lay!(size: size!(100.0), margin: rect!(5.0)),
                1
            ))
            .push(node!(
                widgets::RoundedRect {
                    background_color: [0.0, 0.0, 1.0].into(),
                    border_width: 1.0,
                    ..Default::default()
                }
                .radius(5.0),
                lay!(size: size!(100.0), margin: rect!(5.0)),
                2
            )),
        )
    }
}

impl lemna::App<Renderer> for HelloApp {
    fn new() -> Self {
        Self {}
    }
}

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
        let settings = baseview::WindowOpenOptions {
            title: "test".to_string(),
            size: baseview::Size::new(200.0, 200.0),
            scale: baseview::WindowScalePolicy::SystemScaleFactor,
        };

        let handle = Window::open_parented::<_, Renderer, HelloApp>(&parent, settings);
        Box::new(LemnaEditorHandle { window: handle })
    }

    fn size(&self) -> (u32, u32) {
        (200, 200)
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

struct LemnaEditorHandle {
    window: baseview::WindowHandle,
}

unsafe impl Send for LemnaEditorHandle {}
