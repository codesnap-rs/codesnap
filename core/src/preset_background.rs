use once_cell::sync::Lazy;

use crate::config::{
    Background, DimensionValue, GradientPoint, LinearGradient, LinearGradientStop,
};

// This file contains the preset background themes that can be used in CodeSnap
// If you want to add more beautiful background themes, please feel free to contribute

pub static BAMBOO: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            LinearGradientStop::new(0.22, "#6bcba5"),
            LinearGradientStop::new(0.95, "#caf4c2"),
        ],
    })
});

pub static SEA: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            LinearGradientStop::new(0., "#1fa2ff"),
            LinearGradientStop::new(0.4, "#12d8fa"),
            LinearGradientStop::new(0.95, "#a6ffcb"),
        ],
    })
});

// The CLASSIC background was default background of CodeSnap.nvim ^^
pub static CLASSIC: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            LinearGradientStop::new(0.0, "#3a1c71"),
            LinearGradientStop::new(0.5, "#d76d77"),
            LinearGradientStop::new(0.95, "#ffb07c"),
        ],
    })
});

pub static GRAPE: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            LinearGradientStop::new(0.28, "#675af7"),
            LinearGradientStop::new(0.95, "#bd65fa"),
        ],
    })
});

pub static PEACH: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            LinearGradientStop::new(0.22, "#dd5e89"),
            LinearGradientStop::new(0.95, "#f7bb97"),
        ],
    })
});

pub static SUMMER: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            LinearGradientStop::new(0.28, "#f8a5c2"),
            LinearGradientStop::new(0.95, "#74b9ff"),
        ],
    })
});

pub static DUSK: Lazy<Background> = Lazy::new(|| {
    Background::Gradient(LinearGradient {
        start: GradientPoint {
            x: DimensionValue::Num(0.),
            y: DimensionValue::Num(0.),
        },
        end: GradientPoint {
            x: DimensionValue::Max,
            y: DimensionValue::Num(0.),
        },
        stops: vec![
            LinearGradientStop::new(0.22, "#ff626e"),
            LinearGradientStop::new(0.95, "#ffbe71"),
        ],
    })
});
