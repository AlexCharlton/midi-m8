use std::sync::Arc;

use crate::{app::*, Node, Renderer};
use lemna::{self, widgets, *};
use m8_files::Song;

#[derive(Debug)]
pub struct DragSources {
    song: Option<Arc<Song>>,
}

impl DragSources {
    pub fn new(song: Option<Arc<Song>>) -> Self {
        Self { song }
    }
}

impl lemna::Component<Renderer> for DragSources {
    fn view(&self) -> Option<Node> {
        Some(node!(
            widgets::Div::new().bg([0.5, 0.0, 0.5]),
            lay!(size: size_pct!(100.0))
        ))
    }
}

#[derive(Debug)]
pub struct AllTracksDragSource {}

impl Component<Renderer> for AllTracksDragSource {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new().bg(BLUE).border(MID_GRAY, 2.0),
                lay!(
                    size: size_pct!(100.0),
                    margin: rect!(10.0),
                    padding: rect!(5.0),
                    cross_alignment: layout::Alignment::Center,
                    axis_alignment: layout::Alignment::Center
                ),
                0
            )
            .push(node!(widgets::Text::new(
                txt!("ALL TRACKS"),
                widgets::TextStyle {
                    h_alignment: HorizontalAlign::Center,
                    color: DARK_GRAY,
                    ..widgets::TextStyle::default()
                }
            ))),
        )
    }

    fn on_drag_start(&mut self, event: &mut Event<event::DragStart>) -> Vec<Message> {
        current_window()
            .unwrap()
            .start_drag(Data::Filepath("/test/file.txt".into()));
        event.stop_bubbling();
        vec![]
    }
}
