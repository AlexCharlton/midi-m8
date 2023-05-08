use std::sync::Arc;

use crate::app::*;
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

impl lemna::Component for DragSources {
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

impl Component for AllTracksDragSource {
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

impl Component for TracksDragSource {
    fn view(&self) -> Option<Node> {
        let mut container = node!(
            widgets::Div::new(),
            lay!(
                size: size_pct!(100.0),
                margin: rect!(0.0, 5.0),
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
            )
        );
        for i in 0..8 {
            let has_data = self
                .song
                .as_ref()
                .map(|f| f.tracks[i].is_some())
                .unwrap_or(false);
            container = container.push(
                node!(
                    widgets::Div::new(),
                    lay!(
                        size: size_pct!(Auto, 100.0),
                        direction: Direction::Column,
                        axis_alignment: Alignment::Stretch,
                        margin: rect!(5.0),
                    )
                )
                .push(node!(
                    widgets::Text::new(
                        txt!(format!("{}", i + 1)),
                        widgets::TextStyle {
                            h_alignment: HorizontalAlign::Left,
                            color: if has_data { LIGHT_GRAY } else { MID_GRAY },
                            ..widgets::TextStyle::default()
                        }
                    ),
                    lay!(margin: rect!(3.0))
                ))
                .push(node!(
                    TrackDragSource {
                        song: self.song.clone(),
                        track: i,
                    },
                    lay!(size: size_pct!(100.0, Auto))
                )),
            );
        }
        Some(container)
    }
}

#[derive(Debug)]
pub struct TrackDragSource {
    // Why do we pass in the whole song? If we clone a temp file,
    // it will get dropped, which will remove the file
    song: Option<Arc<MidiTempFiles>>,
    track: usize,
}

impl Component for TrackDragSource {
    fn view(&self) -> Option<Node> {
        let has_data = self
            .song
            .as_ref()
            .map(|f| f.tracks[self.track].is_some())
            .unwrap_or(false);

        Some(node!(
            widgets::Div::new().bg(if has_data { LIGHT_GRAY } else { MID_GRAY }),
            lay!(size: size_pct!(100.0))
        ))
    }

    fn on_drag_start(&mut self, event: &mut Event<event::DragStart>) -> Vec<Message> {
        if let Some(f) = self
            .song
            .as_ref()
            .and_then(|s| s.tracks[self.track].as_ref())
        {
            current_window()
                .unwrap()
                .start_drag(Data::Filepath(f.path().into()));
            event.stop_bubbling();
        }
        vec![]
    }
}
