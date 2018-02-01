//!
//! Types a functionality for handling Canvas and Widget theming.
//!

use Scalar;
use color::{Color, BLACK, WHITE};
use position::{Align, Direction, Padding, Position, Relative};
use fnv;
use std;
use std::any::Any;
use text;
use widget;

/// `std::collections::HashMap` with `fnv::FnvHasher` for unique styling
/// of each widget, index-able by the **Widget::Style**.
pub type StyleMap = fnv::FnvHashMap<std::any::TypeId, WidgetStyle>;

/// A `Widget`'s style and its variations
#[derive(Debug)]
pub struct WidgetStyle {
    default: DynamicWidgetStyle,
    special: Vec<(InteractionState, DynamicWidgetStyle)>,
}

impl WidgetStyle {
    /// Creates a new `WidgetStyle` without special cases.
    pub fn new(default: DynamicWidgetStyle) -> Self {
        WidgetStyle {
            default: default,
            special: Vec::new(),
        }
    }

    /// Adds a special case.
    /// This style is only applied, when the given `InteractionState` `applies_for` the `Widget`'s `InteractionState`.
    pub fn when(mut self, interaction: InteractionState, style: DynamicWidgetStyle) -> Self {
        self.special.push((interaction, style));
        self
    }

    /// Returns all styles that apply to the given `InteractionState`
    pub fn for_interaction(&self, interaction: &InteractionState) -> Vec<&DynamicWidgetStyle> {
        let mut result = Vec::new();

        result.push(&self.default);

        for case in &self.special {
            if case.0.applies_for(interaction) {
                result.push(&case.1);
            }
        }

        return result;
    }
}

/// The states of a `Widget` that are interesting for theming
#[allow(missing_docs)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InteractionState {
    pub hovered  : Option<bool>,
    pub pressed  : Option<bool>,
    pub focused  : Option<bool>,
    pub selected : Option<bool>,
    pub enabled  : Option<bool>,
    pub empty    : Option<bool>,
}

#[allow(missing_docs)]
impl InteractionState {

    pub fn     hovered ( mut self) -> Self { self.hovered  = Some(true ); self }
    pub fn     pressed ( mut self) -> Self { self.pressed  = Some(true ); self }
    pub fn     focused ( mut self) -> Self { self.focused  = Some(true ); self }
    pub fn     selected( mut self) -> Self { self.selected = Some(true ); self }
    pub fn     enabled ( mut self) -> Self { self.enabled  = Some(true ); self }
    pub fn     empty   ( mut self) -> Self { self.empty    = Some(true ); self }

    pub fn not_hovered ( mut self) -> Self { self.hovered  = Some(false); self }
    pub fn not_pressed ( mut self) -> Self { self.pressed  = Some(false); self }
    pub fn not_focused ( mut self) -> Self { self.focused  = Some(false); self }
    pub fn not_selected( mut self) -> Self { self.selected = Some(false); self }
    pub fn not_enabled ( mut self) -> Self { self.enabled  = Some(false); self }
    pub fn not_empty   ( mut self) -> Self { self.empty    = Some(false); self }

    pub fn set_hovered (&mut self, state: bool) { self.hovered  = Some(state); }
    pub fn set_pressed (&mut self, state: bool) { self.pressed  = Some(state); }
    pub fn set_focused (&mut self, state: bool) { self.focused  = Some(state); }
    pub fn set_selected(&mut self, state: bool) { self.selected = Some(state); }
    pub fn set_enabled (&mut self, state: bool) { self.enabled  = Some(state); }
    pub fn set_empty   (&mut self, state: bool) { self.empty    = Some(state); }

    /// Checks wheter the coresponding `Style` should be applied for a `Widget` with the `other` `InteractionState`
    pub fn applies_for(&self, other: &InteractionState) -> bool {
        if self.hovered .is_some() && self.hovered  != other.hovered  { return false; }
        if self.pressed .is_some() && self.pressed  != other.pressed  { return false; }
        if self.focused .is_some() && self.focused  != other.focused  { return false; }
        if self.selected.is_some() && self.selected != other.selected { return false; }
        if self.enabled .is_some() && self.enabled  != other.enabled  { return false; }
        if self.empty   .is_some() && self.empty    != other.empty    { return false; }

        true
    }
}

/// A serializable collection of canvas and widget styling defaults.
#[derive(Debug)]
pub struct Theme {
    /// A name for the theme used for identification.
    pub name: String,
    /// Padding for Canvas layout and positioning.
    pub padding: Padding,
    /// A default widget position along the *x* axis.
    pub x_position: Position,
    /// A default widget position along the *y* axis.
    pub y_position: Position,
    /// A default background for the theme.
    pub background_color: Color,
    /// A default color for widget shapes.
    pub shape_color: Color,
    /// A default color for widget borders.
    pub border_color: Color,
    /// A default width for widget borders.
    pub border_width: Scalar,
    /// A default color for widget labels.
    pub label_color: Color,
    /// The `Id` of the default font used for text widgets when one is not specified.
    pub font_id: Option<text::font::Id>,
    /// A default "large" font size.
    pub font_size_large: u32,
    /// A default "medium" font size.
    pub font_size_medium: u32,
    /// A default "small" font size.
    pub font_size_small: u32,
    /// `StyleMap` for unique styling
    /// of each widget, index-able by the **Widget::kind**.
    pub widget_styling: StyleMap,
    /// Mouse Drag distance threshold determines the minimum distance from the mouse-down point
    /// that the mouse must move before starting a drag operation.
    pub mouse_drag_threshold: Scalar,
    /// Once the `Duration` that separates two consecutive `Click`s is greater than this value, a
    /// `DoubleClick` event will no longer be generated.
    pub double_click_threshold: std::time::Duration,
}

/// A wrapper around a `Widget::Style` to store it in a `HashMap`
#[derive(Debug)]
pub struct DynamicWidgetStyle(Box<Any + Send>);

impl DynamicWidgetStyle {
    /// Wrap the given `Widget::Style`
    pub fn from<T: widget::Style + Send>(style: T) -> Self {
        DynamicWidgetStyle(Box::new(style))
    }

    /// Unwrap into the `Widget::Style` type `T`
    pub fn specific<T: widget::Style>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

impl Theme {

    /// The default theme if not loading from file.
    pub fn default() -> Theme {
        Theme {
            name: "Demo Theme".to_string(),
            padding: Padding::none(),
            x_position: Position::Relative(Relative::Align(Align::Start), None),
            y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color: BLACK,
            shape_color: WHITE,
            border_color: BLACK,
            border_width: 1.0,
            label_color: BLACK,
            font_id: None,
            font_size_large: 26,
            font_size_medium: 18,
            font_size_small: 12,
            widget_styling: fnv::FnvHashMap::default(),
            mouse_drag_threshold: 0.0,
            double_click_threshold: std::time::Duration::from_millis(500),
        }
    }

    /// Retrieve the unique default styling for a widget.
    ///
    /// Attempts to cast the `Box<WidgetStyle>` to the **Widget**'s unique associated style **T**.
    pub fn widget_style<T: widget::Style>(&self) -> Option<&T> {
        let style_id = std::any::TypeId::of::<T>();
        self.widget_styling.get(&style_id).and_then(|boxed_default| boxed_default.default.specific())
    }

    /// Retrieve all styles that apply to the widget
    pub fn widget_styles<T: widget::Style>(&self, interaction: &InteractionState) -> Vec<&T> {
        let style_id = std::any::TypeId::of::<T>();

        if let Some(wstyle) = self.widget_styling.get(&style_id) {
            let styles: Vec<&DynamicWidgetStyle> = wstyle.for_interaction(&interaction);
            return styles.into_iter()
                .map(DynamicWidgetStyle::specific)
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect();
        }

        return Vec::new();
    }

}
