use std::sync::Arc;

use crate::{app::*, Node, Renderer};
use lemna::{self, widgets, *};
use lemna_nih_plug::nih_plug::params::*;

#[derive(Debug)]
pub struct BasicParam<P: Param> {
    param: Arc<P>,
}

impl<P: Param> BasicParam<P> {
    pub fn new(param: Arc<P>) -> Self {
        Self { param }
    }
}

impl<P: Param> lemna::Component<Renderer> for BasicParam<P> {
    fn view(&self) -> Option<Node> {
        dbg!(self
            .param
            .normalized_value_to_string(self.param.modulated_normalized_value(), false));
        Some(node!(widgets::Text::new(
            txt!(self
                .param
                .normalized_value_to_string(self.param.modulated_normalized_value(), false)),
            TextStyle {
                color: LIGHT_GRAY,
                ..Default::default()
            }
        )))
    }

    // fn on_mouse_motion(&mut self, event: &mut event::Event<event::MouseMotion>) -> Vec<Message> {
    //     event.stop_bubbling();
    //     vec![]
    // }

    // fn on_mouse_enter(&mut self, _event: &mut event::Event<event::MouseEnter>) -> Vec<Message> {
    //     if self.min != self.max {
    //         ui::current_window().map(|w| w.set_cursor("SizeNS"));
    //     }

    //     vec![]
    // }

    // fn on_mouse_leave(&mut self, _event: &mut event::Event<event::MouseLeave>) -> Vec<Message> {
    //     if self.state_ref().last_drag_position.is_none() {
    //         ui::current_window().map(|w| w.unset_cursor());
    //     }
    //     vec![]
    // }

    // fn on_drag_start(&mut self, event: &mut event::Event<event::DragStart>) -> Vec<Message> {
    //     self.state_mut().last_drag_position = Some(event.relative_position_unscaled().y);
    //     event.stop_bubbling();
    //     vec![]
    // }

    // fn on_drag_end(&mut self, event: &mut event::Event<event::DragEnd>) -> Vec<Message> {
    //     self.state_mut().last_drag_position = None;
    //     if !event.current_aabb().is_under(event.mouse_position) {
    //         ui::current_window().map(|w| w.unset_cursor());
    //     }
    //     vec![]
    // }

    // fn on_drag(&mut self, event: &mut event::Event<event::Drag>) -> Vec<Message> {
    //     if self.min == self.max {
    //         return vec![];
    //     }
    //     let delta =
    //         self.state_ref().last_drag_position.unwrap() - event.relative_position_unscaled().y;
    //     self.state_mut().last_drag_position = Some(event.relative_position_unscaled().y);
    //     let new_value = self.shift_value(delta);
    //     if new_value == self.value {
    //         vec![]
    //     } else {
    //         if let Some(change_fn) = &self.on_change {
    //             event.dirty();
    //             let x = change_fn(new_value);
    //             vec![x]
    //         } else {
    //             vec![]
    //         }
    //     }
    // }
}
