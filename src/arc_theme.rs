//! Conrod implementation of the Arc theme from https://github.com/horst3180/arc-theme

use theme::{DynamicWidgetStyle, InteractionState, StyleMap, Theme, WidgetStyle};
use color::Color;
use widget;
use position::{Align, Direction, Padding, Position, Relative};
use Scalar;
use std;
use std::any::TypeId;

/// A `Button` style for dangerous actions
#[derive(Clone, Copy, Debug)]
pub struct DangerButton;

/// Conrod implementation of the Arc theme.
/// https://github.com/horst3180/arc-theme
#[derive(Debug)]
#[allow(missing_docs)]
pub struct ArcTheme {
    pub is_light: bool,
    pub window: Color,
    pub background: Color,
    pub dark_text: Color,
    pub light_text: Color,
    pub border: Color,
    pub active: Color,
    pub accent: Color,
    pub invalid: Color,
    pub radius: Scalar,

    additions: Vec<(TypeId, DynamicWidgetStyle)>,
}

impl ArcTheme {

    /// Creates a new 'light' instance of this theme
    pub fn light() -> Self {
        ArcTheme {
            is_light: true,
            window: make_color!(245, 246, 247),
            background: make_color!(245, 246, 247),
            dark_text: make_color!(92, 97, 108),
            light_text: make_color!(255, 255, 255),
            border: make_color!(207, 214, 230),
            active: make_color!(255, 255, 255),
            accent: make_color!(80, 145, 225),
            invalid: make_color!(244, 67, 54),
            radius: 3.0,
            additions: Vec::new(),
        }
    }

    /// Creates a new 'dark' instance of this theme
    pub fn dark() -> Self {
        ArcTheme {
            is_light: false,
            window: make_color!(56, 60, 74),
            background: make_color!(60, 65, 78),
            dark_text: make_color!(255, 255, 255),
            light_text: make_color!(255, 255, 255),
            border: make_color!(43, 46, 57),
            active: make_color!(80, 86, 102),
            accent: make_color!(82, 148, 226),
            invalid: make_color!(244, 67, 54),
            radius: 3.0,
            additions: Vec::new(),
        }
    }

    /// Can be used to specify a custom accent color
    pub fn accent(mut self, color: Color) -> Self {
        self.accent = color;
        self
    }

    /// Can be used to add custom styles that adapt to the themes variables
    pub fn with<F>(mut self, func: F) -> Self
        where F: FnOnce(&Self) -> (TypeId, DynamicWidgetStyle)
    {
        let addition = func(&self);
        self.additions.push(addition);
        self
    }

    /// Finish theme creation
    pub fn build(self) -> Theme {
        Theme {
            name: "Arc".to_string(),
            padding: Padding::none(),
            x_position: Position::Relative(Relative::Align(Align::Start), None),
            y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color: self.window,
            shape_color: self.background,
            border_color: self.border,
            border_width: 1.0,
            label_color: self.dark_text,
            font_id: None,
            font_size_large: 22,
            font_size_medium: 16,
            font_size_small: 12,
            widget_styling: map_styles! {
                widget::canvas::Style => self.canvas(),
                widget::button::Style => self.button(),
                widget::toggle::Style => self.toggle(),
                widget::text_box::Style => self.text_box(),
                widget::scrollbar::Style => self.scrollbar(),
                widget::envelope_editor::Style => self.envelope_editor(),

                DangerButton => self.danger_button(),
            },
            mouse_drag_threshold: 0.0,
            double_click_threshold: std::time::Duration::from_millis(500),
        }.and_all(self.additions)
    }

    fn active_border(&self) -> Color {
        if self.is_light {
            self.accent
        } else {
            self.border
        }
    }

    fn canvas(&self) -> DynamicWidgetStyle {
        theme! { widget::canvas::Style,
            default {
                border = 0.0;
            }
        }
    }

    fn button(&self) -> DynamicWidgetStyle {
        theme! { widget::button::Style,
            hovered {
                color = self.active;
            }
            hovered, pressed {
                color = self.accent;
                label_color = self.light_text;
                border_color = self.active_border();
            }
        }
    }

    fn danger_button(&self) -> DynamicWidgetStyle {
        const RED: Color = make_color!(244, 67, 54);
        theme! { widget::button::Style,
            default {
                color = RED;
                border = if self.is_light { 0.0 } else { 1.0 };
                label_color = self.light_text;
            }
            hovered {
                color = RED.with_luminance(0.62);
            }
            hovered, pressed {
                color = RED.with_luminance(0.66);
            }
        }
    }

    fn toggle(&self) -> DynamicWidgetStyle {
        theme! { widget::toggle::Style,
            hovered {
                color = self.active;
            }
            hovered, pressed {
                color = self.accent;
                label_color = self.light_text;
                border_color = self.active_border();
            }
            selected {
                color = self.accent;
                label_color = self.light_text;
                border_color = self.active_border();
            }
        }
    }

    fn text_box(&self) -> DynamicWidgetStyle {
        theme! { widget::text_box::Style,
            empty {
                text_color = self.dark_text.with_luminance(0.8);
            }
            focused {
                //color = self.active;
                border_color = self.accent;
            }
            not_valid {
                border_color = self.invalid;
            }
        }
    }

    fn scrollbar(&self) -> DynamicWidgetStyle {
        theme! { widget::scrollbar::Style,
            default {
                thickness = 7.0;
                color = self.border;
            }
            hovered {
                thickness = 10.0;
                color = self.accent;
            }
            pressed {
                color = self.accent;
            }
        }
    }

    fn envelope_editor(&self) -> DynamicWidgetStyle {
        theme! { widget::envelope_editor::Style,
            default {
                point_color = self.accent;
            }
        }
    }
}


