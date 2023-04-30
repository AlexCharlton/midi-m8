use std::sync::Arc;

use crate::{app::*, Node, Renderer};
use lemna::{self, widgets, *};

#[derive(Debug)]
pub struct DragSources {
    song: Option<Arc<MidiTempFiles>>,
}

impl DragSources {
    pub fn new(song: Option<Arc<MidiTempFiles>>) -> Self {
        Self { song }
    }
}

impl lemna::Component<Renderer> for DragSources {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new(),
                lay!(
                    size: size_pct!(100.0),
                    direction: Direction::Column,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch
                )
            )
            .push(node!(
                TracksDragSource {
                    song: self.song.clone()
                },
                lay!(size: size!(Auto, 70.0))
            ))
            .push(node!(AllTracksDragSource {
                song: self.song.clone()
            })),
        )
    }
}

#[derive(Debug)]
pub struct AllTracksDragSource {
    song: Option<Arc<MidiTempFiles>>,
}

impl Component<Renderer> for AllTracksDragSource {
    fn view(&self) -> Option<Node> {
        let has_data = self.song.is_some();
        Some(
            node!(
                widgets::Div::new().bg(if has_data { BLUE } else { MID_GRAY }),
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
                txt!(if has_data { "ALL TRACKS" } else { "NO DATA" }),
                widgets::TextStyle {
                    h_alignment: HorizontalAlign::Center,
                    color: if has_data { DARK_GRAY } else { LIGHT_GRAY },
                    ..widgets::TextStyle::default()
                }
            ))),
        )
    }

    fn on_drag_start(&mut self, event: &mut Event<event::DragStart>) -> Vec<Message> {
        if let Some(f) = &self.song {
            current_window()
                .unwrap()
                .start_drag(Data::Filepath(f.all.path().into()));
            event.stop_bubbling();
        }
        vec![]
    }
}

#[derive(Debug)]
pub struct TracksDragSource {
    song: Option<Arc<MidiTempFiles>>,
}

impl Component<Renderer> for TracksDragSource {
    fn view(&self) -> Option<Node> {
        Some(node!(
            widgets::Div::new().bg([0.5, 0.0, 0.5]),
            lay!(size: size_pct!(100.0))
        ))
    }
}
