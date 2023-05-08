use std::sync::Arc;

use crate::{app::*, basic_param::BasicParam};
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

impl lemna::Component for Parameters {
    fn view(&self) -> Option<Node> {
        let label_style = TextStyle {
            color: MID_GRAY,
            ..Default::default()
        };

        let labels = node!(
            widgets::Div::new(),
            lay!(size: size_pct!(Auto, 100.0), direction: Direction::Column),
            0
        )
        .push(node!(widgets::Text::new(
            txt!("START"),
            label_style.clone()
        )))
        .push(node!(widgets::Text::new(
            txt!("MAX LEN"),
            label_style.clone()
        )))
        .push(node!(widgets::Text::new(txt!("TRANSPOSE"), label_style)));
        let params = node!(
            widgets::Div::new(),
            lay!(size: size_pct!(Auto, 100.0), direction: Direction::Column),
            1
        )
        .push(node!(BasicParam::new(self.params.start.clone()), lay!(), 0))
        .push(node!(
            BasicParam::new(self.params.max_len.clone()),
            lay!(),
            1
        ))
        .push(node!(
            BasicParam::new(self.params.transpose.clone()),
            lay!(),
            2
        ));

        Some(
            node!(
                widgets::Div::new(),
                lay!(
                    size: size_pct!(100.0),
                    padding: rect!(25.0, 15.0),
                    direction: Direction::Row
                )
            )
            .push(labels)
            .push(params),
        )
    }
}
