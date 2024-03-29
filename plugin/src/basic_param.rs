use std::sync::Arc;

use crate::app::*;
use lemna::{self, widgets, *};
use lemna_nih_plug::nih_plug::params::*;

#[derive(Debug, Default)]
struct ParamState {
    last_drag_position: Option<f32>,
    // Tracked separately from the quantized param value
    raw_norm_value: f32,
}

#[derive(Debug)]
#[component(State = "ParamState")]
pub struct BasicParam<P: Param> {
    param: Arc<P>,
}

impl<P: Param> BasicParam<P> {
    const PIXELS_OVER_RANGE: f32 = 100.0;

    pub fn new(param: Arc<P>) -> Self {
        Self {
            state: Some(ParamState {
                last_drag_position: None,
                raw_norm_value: param.modulated_normalized_value(),
            }),
            dirty: false,
            param,
        }
    }

    fn shift_value(&self, delta: f32) -> f32 {
        let scale = 1.0 / Self::PIXELS_OVER_RANGE;
        let value = self.state_ref().raw_norm_value;
        lemna::clamp(value + delta * scale, 0.0, 1.0)
    }
}

#[state_component_impl(ParamState)]
impl<P: Param> lemna::Component for BasicParam<P> {
    fn view(&self) -> Option<Node> {
        Some(node!(widgets::Text::new(txt!(self
            .param
            .normalized_value_to_string(
                self.param.modulated_normalized_value(),
                false
            )),)
        .style("color", LIGHT_GRAY)))
    }

    fn on_mouse_motion(&mut self, event: &mut event::Event<event::MouseMotion>) {
        event.stop_bubbling();
    }

    fn on_mouse_enter(&mut self, _event: &mut event::Event<event::MouseEnter>) {
        if let Some(w) = lemna::current_window() {
            w.set_cursor("SizeNS")
        }
    }

    fn on_mouse_leave(&mut self, _event: &mut event::Event<event::MouseLeave>) {
        if self.state_ref().last_drag_position.is_none() {
            if let Some(w) = lemna::current_window() {
                w.unset_cursor()
            }
        }
    }

    fn on_drag_start(&mut self, event: &mut event::Event<event::DragStart>) {
        self.state_mut().last_drag_position = Some(event.relative_physical_position().y);
        event.stop_bubbling();
        event.emit(msg!(AppMsg::BeginSettingParam {
            param: self.param.as_ptr()
        }));
    }

    fn on_drag_end(&mut self, event: &mut event::Event<event::DragEnd>) {
        self.state_mut().last_drag_position = None;
        if !event
            .current_physical_aabb()
            .is_under(event.physical_mouse_position())
        {
            if let Some(w) = lemna::current_window() {
                w.unset_cursor()
            }
        }
        event.emit(msg!(AppMsg::EndSettingParam {
            param: self.param.as_ptr()
        }));
    }

    fn on_drag(&mut self, event: &mut event::Event<event::Drag>) {
        let delta =
            self.state_ref().last_drag_position.unwrap() - event.relative_physical_position().y;
        self.state_mut().last_drag_position = Some(event.relative_physical_position().y);
        let new_value =
            self.shift_value(delta * if event.modifiers_held.shift { 0.1 } else { 1.0 });
        self.state_mut().raw_norm_value = new_value;
        if new_value != self.param.modulated_normalized_value() {
            event.emit(msg!(AppMsg::SetParam {
                param: self.param.as_ptr(),
                norm_value: new_value
            }));
        }
    }

    fn on_double_click(&mut self, event: &mut Event<event::DoubleClick>) {
        event.emit(msg!(AppMsg::SetParam {
            param: self.param.as_ptr(),
            norm_value: self.param.default_normalized_value(),
        }));
    }
}
