use std::path::PathBuf;

use crate::{app::*, Node, Renderer};
use lemna::{self, widgets, *};

#[derive(Debug)]
pub struct FileSelection {
    file: Option<PathBuf>,
}

impl FileSelection {
    pub fn new(file: Option<PathBuf>) -> Self {
        Self { file }
    }
}

impl lemna::Component<Renderer> for FileSelection {
    fn view(&self) -> Option<Node> {
        let mut selector = widgets::FileSelector::new(
            "Choose a file".to_string(),
            widgets::FileSelectorStyle {
                button_style: ButtonStyle {
                    text_color: BLUE,
                    font_size: 9.0,
                    background_color: DARK_GRAY,
                    highlight_color: DARK_GRAY,
                    active_color: MID_GRAY,
                    border_color: BLUE,
                    padding: 0.0,
                    ..Default::default()
                },
            },
        )
        .filter(vec!["*.m8s".into()], "M8 song file".into())
        .on_select(Box::new(|f| msg!(AppMsg::FileSelected { selection: f })));

        if let Some(p) = &self.file {
            selector = selector.default_path(p.clone());
        }

        Some(
            node!(widgets::Div::new(), lay!(size: size_pct!(100.0)))
                .push(node!(
                    selector,
                    lay!(margin: rect!(5.0), size: size!(22.0, 22.0))
                ))
                .push(node!(
                    widgets::Text::new(
                        txt!(self
                            .file
                            .as_ref()
                            .map(|p| p.file_stem().unwrap().to_str().unwrap())
                            .unwrap_or("Select a file")),
                        TextStyle {
                            color: BLUE,
                            ..Default::default()
                        }
                    ),
                    lay!(margin: rect!(7.0, 7.0))
                )),
        )
    }
}
