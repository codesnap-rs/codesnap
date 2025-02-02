use super::interface::{
    component::{Component, ComponentContext},
    style::{RawComponentStyle, Size, Style},
};

pub struct CodeBlock {
    children: Vec<Box<dyn Component>>,
}

impl Component for CodeBlock {
    fn name(&self) -> &'static str {
        "CodeBlock"
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        Style::default().size(Size::Inherit, Size::Dynamic)
    }
}

impl CodeBlock {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> CodeBlock {
        CodeBlock { children }
    }
}
