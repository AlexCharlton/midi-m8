use std::path::PathBuf;

use crate::app::*;
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

impl lemna::Component for FileSelection {
    fn view(&self) -> Option<Node> {
        let mut selector = widgets::FileSelector::new("Choose a file".to_string())
            .style("font_size", 9.0)
            .style("padding", 0.0)
            .filter(vec!["*.m8s".into()], "M8 song file".into())
            .on_select(Box::new(|f| msg!(AppMsg::FileSelected { selection: f })));

        if let Some(p) = &self.file {
            selector = selector.default_path(p.clone());
        }

        Some(
            node!(widgets::Div::new(), [size_pct: [100]])
                .push(node!(
                    selector,
                    [margin: [5], size: [22, 22]]
                ))
                .push(node!(
                    widgets::Text::new(
                        txt!(self
                            .file
                            .as_ref()
                            .map(|p| p.file_stem().unwrap().to_str().unwrap())
                            .unwrap_or("Select a file")))
                        .style("color", BLUE),
                    [margin: [7, 7]]
                )),
        )
    }
}
