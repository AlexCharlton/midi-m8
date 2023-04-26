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
        Some(node!(
            widgets::Div::new().bg([0.0, 0.0, 1.0]),
            lay!(size: size_pct!(100.0))
        ))
    }
}
