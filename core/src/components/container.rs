use crate::rendering::Scene;

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error::Result,
    style::Style,
};

pub struct Container {
    children: Vec<Box<dyn Component>>,
}

impl Component for Container {
    fn name(&self) -> &'static str {
        "Container"
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }
}

impl Container {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> Container {
        Container { children }
    }

    pub fn prepare_scene(&self, context: &mut ComponentContext) -> Result<Scene> {
        let style = self.parsed_style(None, context);
        let mut scene = Scene::new(
            (style.width * context.scale_factor) as u32,
            (style.height * context.scale_factor) as u32,
            context.font_renderer.clone(),
        );

        self.draw(
            &mut scene,
            context,
            RenderParams::default(),
            Style::default(),
            Style::default(),
        )?;

        Ok(scene)
    }
}
