use super::{
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle, Size, Style},
};
use crate::rendering::Scene;
use crate::{
    config::SnapshotConfig,
    edges::edge::Edge,
    utils::{text::FontRenderer, theme_provider::ThemeProvider},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct ComponentContext {
    pub scale_factor: f32,
    pub take_snapshot_params: SnapshotConfig,
    pub theme_provider: ThemeProvider,
    pub font_renderer: Arc<Mutex<FontRenderer>>,
    pub style_map: HashMap<&'static str, Style<f32>>,
}

#[derive(Default, Clone)]
pub struct RenderParams {
    pub x: f32,
    pub y: f32,
}

pub trait Component {
    fn children(&self) -> &Vec<Box<dyn Component>>;

    // The render_condition determines whether the component should be rendered or not
    fn render_condition(&self, _context: &ComponentContext) -> bool {
        true
    }

    // The difference with render_condition is that self_render_condition still renders childrens
    fn self_render_condition(&self, _context: &ComponentContext) -> bool {
        true
    }

    fn draw_self(
        &self,
        _scene: &mut Scene,
        _context: &ComponentContext,
        _render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        // Stub component means this component no need to cache its style
        // For instance, "Row" and "Col" component, they are just layout components
        // and their style is determined by their children, so they don't need to cache
        // their style
        "STUB_COMPONENT"
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        RawComponentStyle::default()
    }

    fn parse_size(&self, size: Size, dynamic_value: f32, inherit_value: Option<f32>) -> f32 {
        match size {
            Size::Num(num) => num,
            Size::Dynamic => dynamic_value,
            Size::Inherit => inherit_value.unwrap_or(dynamic_value),
        }
    }

    fn parsed_style(
        &self,
        parent_style: Option<&ComponentStyle>,
        context: &mut ComponentContext,
    ) -> Style<f32> {
        let name = self.name();

        if name != "STUB_COMPONENT" {
            if let Some(style) = context.style_map.get(name) {
                return style.clone();
            }
        }

        // If render_condition return false, the whole component shouldn't rendered,
        // includes its children
        if !self.render_condition(context) {
            return ComponentStyle::default();
        }

        // If self_render_condition return false, the component shouldn't rendered,
        // so the corresponding style should be cleared
        let style = if self.self_render_condition(context) {
            self.style(context)
        } else {
            RawComponentStyle::default()
        };
        let (width, height) = self.get_dynamic_wh(&style, context);
        let width = self.parse_size(style.width, width, parent_style.map(|s| s.width))
            + style.padding.horizontal()
            + style.margin.horizontal();

        let style = Style {
            min_width: style.min_width,
            width: if width < style.min_width {
                style.min_width
            } else {
                width
            },
            height: self.parse_size(style.height, height, parent_style.map(|s| s.height))
                + style.padding.vertical()
                + style.margin.vertical(),
            align: style.align,
            padding: style.padding,
            margin: style.margin,
        };

        if name != "STUB_COMPONENT" {
            context.style_map.insert(name, style.clone());
        }

        style
    }

    fn draw(
        &self,
        scene: &mut Scene,
        context: &mut ComponentContext,
        sibling_render_params: RenderParams,
        parent_style: ComponentStyle,
        sibling_style: ComponentStyle,
    ) -> render_error::Result<RenderParams> {
        let style = self.parsed_style(Some(&parent_style), context);
        let render_params = match parent_style.align {
            ComponentAlign::Row => RenderParams {
                x: sibling_render_params.x
                    + sibling_style.width
                    + style.margin.left
                    + sibling_style.padding.horizontal(),
                y: sibling_render_params.y + style.margin.top,
            },
            ComponentAlign::Column => RenderParams {
                x: sibling_render_params.x + style.margin.left,
                y: sibling_render_params.y
                    + style.margin.top
                    + sibling_style.height
                    + sibling_style.padding.vertical(),
            },
        };

        if !self.render_condition(context) {
            return Ok(render_params);
        }

        if self.self_render_condition(context) {
            self.draw_self(scene, context, &render_params, &style, &parent_style)?;
        }

        let mut sibling_render_params = RenderParams {
            x: render_params.x + style.padding.left,
            y: render_params.y + style.padding.top,
        };
        let mut sibling_style = ComponentStyle::default();

        for child in self.children() {
            sibling_render_params = child.draw(
                scene,
                context,
                sibling_render_params,
                style.clone(),
                sibling_style,
            )?;
            sibling_style = child.parsed_style(Some(&style), context);
        }

        Ok(render_params)
    }

    // Dynamic calculate width and height of children, if the children is empty, get_dynamic_wh
    // will return (0., 0.)
    fn get_dynamic_wh(
        &self,
        style: &RawComponentStyle,
        context: &mut ComponentContext,
    ) -> (f32, f32) {
        let (mut width, mut height) = (0f32, 0f32);
        for child in self.children() {
            let child_style = child.parsed_style(None, context);
            match style.align {
                ComponentAlign::Row => {
                    width += child_style.width;
                    height = height.max(child_style.height);
                }
                ComponentAlign::Column => {
                    width = width.max(child_style.width);
                    height += child_style.height;
                }
            }
        }

        (width, height)
    }
}
