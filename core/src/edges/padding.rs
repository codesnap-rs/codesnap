use super::edge::Edge;

#[derive(Clone, Default, Debug)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Edge for Padding {
    fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    fn vertical(&self) -> f32 {
        self.bottom + self.top
    }
}

impl Padding {
    pub fn from_value(value: f32) -> Padding {
        Padding {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn from_config(
        horizontal_background_padding: f32,
        vertical_background_padding: f32,
        background_padding: Option<f32>,
    ) -> Padding {
        match background_padding {
            Some(padding) => Padding::from_value(padding),
            None => Padding {
                top: vertical_background_padding,
                bottom: vertical_background_padding,
                left: horizontal_background_padding,
                right: horizontal_background_padding,
            },
        }
    }
}
