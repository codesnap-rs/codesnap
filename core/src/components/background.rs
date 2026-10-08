use crate::rendering::Scene;

use crate::edges::{edge::Edge, padding::Padding};

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error::{self},
    style::{ComponentAlign, ComponentStyle, RawComponentStyle},
};

pub struct Background {
    children: Vec<Box<dyn Component>>,
    padding: Padding,
}

impl Background {
    pub fn new(padding: Padding, children: Vec<Box<dyn Component>>) -> Background {
        Background { children, padding }
    }

    pub fn has_background(padding: &Padding) -> bool {
        return padding.horizontal() != 0. || padding.vertical() != 0.;
    }
}

impl Component for Background {
    fn name(&self) -> &'static str {
        "Background"
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        RawComponentStyle::default()
            .align(ComponentAlign::Column)
            .padding(self.padding.clone())
    }

    fn self_render_condition(&self, _context: &ComponentContext) -> bool {
        Self::has_background(&self.padding)
    }

    fn draw_self(
        &self,
        scene: &mut Scene,
        context: &ComponentContext,
        _render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        scene.background(&context.take_snapshot_params.background);

        Ok(())
    }
}
