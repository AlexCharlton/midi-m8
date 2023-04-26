use std::sync::Arc;

use crate::{app::*, Node, Renderer};
use lemna::{self, widgets, *};

#[derive(Debug)]
pub struct Parameters {
    params: Arc<M8Params>,
}

impl Parameters {
    pub fn new(params: Arc<M8Params>) -> Self {
        Self { params }
    }
}

impl lemna::Component<Renderer> for Parameters {
    fn view(&self) -> Option<Node> {
        Some(node!(widgets::Div::new(), lay!(size: size_pct!(100.0))))
    }
}
