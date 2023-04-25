use lemna::{self, widgets, *};

pub type Renderer = lemna::render::wgpu::WGPURenderer;
type Node = lemna::Node<Renderer>;

const DARK_GRAY: Color = color!(0x16, 0x16, 0x16);
const MID_GRAY: Color = color!(0x5F, 0x5F, 0x5F);
const LIGHT_GRAY: Color = color!(0xDE, 0xDE, 0xDE);
const BLUE: Color = color!(0x00, 0xE5, 0xEE);

#[derive(Debug)]
pub struct M8PlugApp {}

impl lemna::Component<Renderer> for M8PlugApp {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new().bg(DARK_GRAY),
                lay!(size: size_pct!(100.0), wrap: true,
                     padding: rect!(10.0),
                     axis_alignment: Alignment::Center, cross_alignment: Alignment::Center)
            )
            // Fileselector
            // Parameters
            // TrackDraggers
            // Footer
            .push(node!(
                AllTracksDragSource {},
                lay!(size: size!(Auto, 100.0)),
                0
            )),
        )
    }

    fn on_drag_drop(&mut self, event: &mut Event<event::DragDrop>) -> Vec<Message> {
        // TODO
        vec![]
    }

    fn on_drag_target(&mut self, _event: &mut Event<event::DragTarget>) -> Vec<Message> {
        // TODO Evaluate file type
        current_window().unwrap().set_drop_target_valid(true);
        vec![]
    }
}

impl lemna::App<Renderer> for M8PlugApp {
    fn new() -> Self {
        Self {}
    }
}

/// DragSource

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
