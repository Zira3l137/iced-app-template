#![allow(dead_code)]

//! Builder pattern implementation for styled widgets
//!
//! This module provides a more idiomatic Rust approach to creating styled widgets
//! using the builder pattern instead of functions with many optional parameters.
//!
//! The builder pattern offers several advantages:
//! - **Type Safety**: Compile-time guarantees for correct usage
//! - **Clarity**: Self-documenting code with named methods
//! - **Flexibility**: Easy to add or omit optional properties
//! - **Chainability**: Fluent API for readable widget creation
//!
//! # Quick Start
//!
//! For convenience, use the macros from `crate::app::widgets::macros`:
//!
//! ```rust
//! use crate::icon;
//! use crate::button;
//! use crate::frame;
//! use crate::gradient;
//!
//! // Using macros
//! let my_icon = icon!(Icon::Home, size: 24, color: Color::WHITE);
//! let my_button = button!(
//!     "Click me",
//!     background_active: Color::from_rgb(0.2, 0.6, 1.0),
//!     border_radius: 8.0
//! );
//! ```
//!
//! # Direct Builder Usage
//!
//! For more control, use builders directly:
//!
//! ```rust
//! use crate::app::widgets::builder::*;
//!
//! let styled_button = ButtonBuilder::new("Click me")
//!     .background_active(iced::Color::from_rgb(0.2, 0.6, 1.0))
//!     .text_color_active(iced::Color::WHITE)
//!     .border_radius(8.0)
//!     .build();
//!
//! let icon = IconBuilder::new(Icon::Home)
//!     .size(24)
//!     .color(iced::Color::WHITE)
//!     .build();
//! ```
//!
//! # Available Builders
//!
//! - [`IconBuilder`]: Create icons with optional size and color
//! - [`NerdTextBuilder`]: Create text using Nerd Fonts
//! - [`FrameBuilder`]: Create containers with borders and shadows
//! - [`ButtonBuilder`]: Create buttons with state-specific styling
//! - [`GradientBuilder`]: Create linear gradients with color stops
//! - [`TextInputBuilder`]: Create text input fields with optional styling

pub mod macros;

use {{project-name}}_core::{constants::APP_FONT_FAMILY_NAME, types::Icon};

// ============================================================================
// Icon Builder
// ============================================================================

/// Builder for creating styled icon widgets.
///
/// Icons use the Nerd Fonts font family and can be customized with size and color.
///
/// # Examples
///
/// ```rust
/// use crate::app::widgets::builder::IconBuilder;
/// use crate::core::types::Icon;
/// use iced::Color;
///
/// // Simple icon
/// let icon = IconBuilder::new(Icon::Home).build();
///
/// // Icon with size
/// let large_icon = IconBuilder::new(Icon::Settings)
///     .size(32)
///     .build();
///
/// // Icon with color
/// let colored_icon = IconBuilder::new(Icon::Heart)
///     .color(Color::from_rgb(1.0, 0.0, 0.0))
///     .build();
///
/// // Fully customized icon
/// let custom_icon = IconBuilder::new(Icon::Star)
///     .size(24)
///     .color(Color::from_rgb(1.0, 0.84, 0.0))
///     .build();
/// ```
pub struct IconBuilder {
    icon: Icon,
    size: Option<iced::Pixels>,
    color: Option<iced::Color>,
}

impl IconBuilder {
    pub fn new(icon: Icon) -> Self {
        Self { icon, size: None, color: None }
    }

    pub fn size(mut self, size: impl Into<iced::Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn color(mut self, color: impl Into<iced::Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Builds the icon widget.
    ///
    /// Returns a styled `Text` widget configured with the Nerd Fonts font family.
    pub fn build<Renderer>(self) -> iced::widget::Text<'static, Renderer>
    where
        Renderer: iced::widget::text::Catalog + 'static,
        <Renderer as iced::widget::text::Catalog>::Class<'static>:
            From<Box<dyn for<'a> std::ops::Fn(&'a Renderer) -> iced::widget::text::Style>>,
    {
        let font = iced::font::Font::with_name(APP_FONT_FAMILY_NAME);
        let mut element = iced::widget::text(self.icon.to_string()).font(font);

        if let Some(size) = self.size {
            element = element.size(size);
        }

        if let Some(color) = self.color {
            element = element.color(color);
        }

        element
    }
}

// ============================================================================
// Nerd Text Builder
// ============================================================================

/// Builder for creating text widgets using Nerd Fonts.
///
/// This builder creates text widgets that use the Nerd Fonts font family,
/// allowing for rich icon and symbol display alongside regular text.
///
/// # Examples
///
/// ```rust
/// use crate::app::widgets::builder::NerdTextBuilder;
/// use iced::Color;
///
/// // Simple text
/// let text = NerdTextBuilder::new("Hello World").build();
///
/// // Text with size
/// let large_text = NerdTextBuilder::new("Welcome")
///     .size(24)
///     .build();
///
/// // Colored text
/// let error_text = NerdTextBuilder::new("Error!")
///     .color(Color::from_rgb(1.0, 0.0, 0.0))
///     .build();
///
/// // Fully customized text
/// let custom_text = NerdTextBuilder::new("Status: OK")
///     .size(16)
///     .color(Color::from_rgb(0.0, 1.0, 0.0))
///     .build();
/// ```
pub struct NerdTextBuilder<T> {
    text: T,
    size: Option<iced::Pixels>,
    color: Option<iced::Color>,
}

impl<T: iced::widget::text::IntoFragment<'static>> NerdTextBuilder<T> {
    pub fn new(text: T) -> Self {
        Self { text, size: None, color: None }
    }

    pub fn size(mut self, size: impl Into<iced::Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn color(mut self, color: impl Into<iced::Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn build<Renderer>(self) -> iced::widget::Text<'static, Renderer>
    where
        Renderer: iced::widget::text::Catalog + 'static,
        <Renderer as iced::widget::text::Catalog>::Class<'static>:
            From<Box<dyn for<'a> std::ops::Fn(&'a Renderer) -> iced::widget::text::Style>>,
    {
        let font = iced::font::Font::with_name(APP_FONT_FAMILY_NAME);
        let mut element = iced::widget::text(self.text).font(font);

        if let Some(size) = self.size {
            element = element.size(size);
        }

        if let Some(color) = self.color {
            element = element.color(color);
        }

        element
    }
}

// ============================================================================
// Frame Builder
// ============================================================================

/// Builder for creating styled container (frame) widgets.
///
/// Frames are containers that can be styled with backgrounds, borders, shadows,
/// and text colors. They're useful for creating cards, panels, and other boxed content.
///
/// # Examples
///
/// ```rust
/// use crate::app::widgets::builder::FrameBuilder;
/// use iced::{Color, Vector, widget::text};
///
/// // Simple frame with background
/// let frame = FrameBuilder::new(text("Content"))
///     .background(Color::from_rgb(0.1, 0.1, 0.1))
///     .build();
///
/// // Frame with border
/// let bordered_frame = FrameBuilder::new(text("Bordered"))
///     .border_color(Color::from_rgb(0.5, 0.5, 0.5))
///     .border_width(2.0)
///     .border_radius(8.0)
///     .build();
///
/// // Card with shadow
/// let card = FrameBuilder::new(text("Card"))
///     .background(Color::WHITE)
///     .border_radius(12.0)
///     .shadow_color(Color::from_rgba(0.0, 0.0, 0.0, 0.2))
///     .shadow_blur_radius(10.0)
///     .shadow_offset(Vector::new(0.0, 4.0))
///     .build();
/// ```
pub struct FrameBuilder<'a, Message, B = iced::Background, C = iced::Color, V = iced::Vector>
where
    B: Into<iced::Background> + Clone + 'a,
    C: Into<iced::Color> + Clone + 'a,
    V: Into<iced::Vector> + Clone + 'a,
{
    content: iced::Element<'a, Message>,
    background: Option<B>,
    border_color: Option<C>,
    shadow_color: Option<C>,
    text_color: Option<C>,
    border_width: Option<f32>,
    border_radius: Option<f32>,
    shadow_blur_radius: Option<f32>,
    shadow_offset: Option<V>,
    snap: Option<bool>,
}

impl<'a, Message, B, C, V> FrameBuilder<'a, Message, B, C, V>
where
    B: Into<iced::Background> + Clone + 'a,
    C: Into<iced::Color> + Clone + 'a,
    V: Into<iced::Vector> + Clone + 'a,
{
    /// Creates a new frame builder with the specified content.
    ///
    /// # Arguments
    ///
    /// * `content` - The widget(s) to place inside the frame
    pub fn new(content: impl Into<iced::Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            background: None,
            border_color: None,
            shadow_color: None,
            text_color: None,
            border_width: None,
            border_radius: None,
            shadow_blur_radius: None,
            shadow_offset: None,
            snap: None,
        }
    }

    /// Sets the background of the frame.
    ///
    /// Can be a solid color or a gradient.
    pub fn background(mut self, background: B) -> Self {
        self.background = Some(background);
        self
    }

    /// Sets the border color of the frame.
    pub fn border_color(mut self, color: C) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Sets the shadow color of the frame.
    pub fn shadow_color(mut self, color: C) -> Self {
        self.shadow_color = Some(color);
        self
    }

    /// Sets the text color for content inside the frame.
    pub fn text_color(mut self, color: C) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the border width in pixels.
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Sets the border radius in pixels.
    ///
    /// Higher values create more rounded corners.
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Sets the shadow blur radius in pixels.
    ///
    /// Higher values create softer shadows.
    pub fn shadow_blur_radius(mut self, radius: f32) -> Self {
        self.shadow_blur_radius = Some(radius);
        self
    }

    /// Sets the shadow offset as a vector.
    ///
    /// # Arguments
    ///
    /// * `offset` - Vector specifying horizontal and vertical shadow offset
    pub fn shadow_offset(mut self, offset: V) -> Self {
        self.shadow_offset = Some(offset);
        self
    }

    /// Builds the frame widget.
    ///
    /// Returns a styled `Container` widget with all specified properties applied.
    pub fn build(self) -> iced::widget::Container<'a, Message> {
        let background = self.background;
        let border_color = self.border_color;
        let shadow_color = self.shadow_color;
        let text_color = self.text_color;
        let border_width = self.border_width;
        let border_radius = self.border_radius;
        let shadow_blur_radius = self.shadow_blur_radius;
        let shadow_offset = self.shadow_offset;
        let snap = self.snap;

        iced::widget::container(self.content).style(move |theme| {
            let palette_ext = theme.extended_palette();

            let border_color = match border_color.clone() {
                Some(color) => color.into(),
                None => palette_ext.primary.base.color,
            };

            let border_radius = match border_radius {
                Some(radius) => radius.into(),
                None => iced::border::Radius::default(),
            };

            let shadow_color = match shadow_color.clone() {
                Some(color) => color.into(),
                None => palette_ext.background.weak.color,
            };

            let shadow_offset = match shadow_offset.clone() {
                Some(offset) => offset.into(),
                None => iced::Vector::new(0.0, 0.0),
            };

            let text_color = match text_color.clone() {
                Some(color) => color.into(),
                None => palette_ext.background.base.text,
            };

            let background = match background.clone() {
                Some(bg) => bg.into(),
                None => iced::Background::Color(palette_ext.background.base.color),
            };

            let border_width = border_width.unwrap_or(1.0);
            let shadow_blur_radius = shadow_blur_radius.unwrap_or(0.0);
            let snap = snap.unwrap_or(false);

            iced::widget::container::Style {
                text_color: Some(text_color),
                background: Some(background),
                border: iced::border::Border {
                    color: border_color,
                    width: border_width,
                    radius: border_radius,
                },
                shadow: iced::Shadow {
                    color: shadow_color,
                    offset: shadow_offset,
                    blur_radius: shadow_blur_radius,
                },
                snap,
            }
        })
    }
}

// ============================================================================
// Button State Helper (for complex button builders)
// ============================================================================

struct ButtonState<B, C>
where
    B: Clone,
    C: Clone,
{
    background: Option<B>,
    text_color: Option<C>,
    border_color: Option<C>,
}

impl<B, C> ButtonState<B, C>
where
    B: Clone,
    C: Clone,
{
    fn new() -> Self {
        Self { background: None, text_color: None, border_color: None }
    }

    fn with_background(mut self, background: B) -> Self {
        self.background = Some(background);
        self
    }

    fn with_text_color(mut self, color: C) -> Self {
        self.text_color = Some(color);
        self
    }

    fn with_border_color(mut self, color: C) -> Self {
        self.border_color = Some(color);
        self
    }
}

// ============================================================================
// Button Builder
// ============================================================================

/// Builder for creating styled button widgets with state-specific styling.
///
/// Buttons can have different styles for each interaction state (active, hovered,
/// pressed, and disabled), allowing for rich interactive experiences.
///
/// # Examples
///
/// ```rust
/// use crate::app::widgets::builder::ButtonBuilder;
/// use iced::{Color, widget::text};
///
/// // Simple button
/// let button = ButtonBuilder::new(text("Click me"))
///     .background_active(Color::from_rgb(0.2, 0.6, 1.0))
///     .text_color_active(Color::WHITE)
///     .build();
///
/// // Button with hover effect
/// let hover_button = ButtonBuilder::new(text("Hover me"))
///     .background_active(Color::from_rgb(0.2, 0.6, 1.0))
///     .background_hovered(Color::from_rgb(0.3, 0.7, 1.0))
///     .text_color_active(Color::WHITE)
///     .border_radius(8.0)
///     .build();
///
/// // Full-featured button
/// let fancy_button = ButtonBuilder::new(text("Save"))
///     .background_active(Color::from_rgb(0.2, 0.8, 0.2))
///     .background_hovered(Color::from_rgb(0.3, 0.9, 0.3))
///     .background_pressed(Color::from_rgb(0.1, 0.7, 0.1))
///     .background_disabled(Color::from_rgb(0.5, 0.5, 0.5))
///     .text_color_active(Color::WHITE)
///     .border_width(2.0)
///     .border_radius(4.0)
///     .build();
/// ```
pub struct ButtonBuilder<'a, Message, B = iced::Background, C = iced::Color>
where
    B: Into<iced::Background> + Clone + 'a,
    C: Into<iced::Color> + Clone + 'a,
{
    content: iced::Element<'a, Message>,
    active: ButtonState<B, C>,
    disabled: ButtonState<B, C>,
    hovered: ButtonState<B, C>,
    pressed: ButtonState<B, C>,
    border_width: Option<f32>,
    border_radius: Option<f32>,
    shadow_offset: Option<iced::Vector>,
    snap: Option<bool>,
}

impl<'a, Message, B, C> ButtonBuilder<'a, Message, B, C>
where
    Message: Clone + 'a,
    B: Into<iced::Background> + Clone + 'a,
    C: Into<iced::Color> + Clone + 'a,
{
    /// Creates a new button builder with the specified content.
    ///
    /// # Arguments
    ///
    /// * `content` - The widget(s) to display inside the button
    pub fn new(content: impl Into<iced::Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            active: ButtonState::new(),
            disabled: ButtonState::new(),
            hovered: ButtonState::new(),
            pressed: ButtonState::new(),
            border_width: None,
            border_radius: None,
            shadow_offset: None,
            snap: None,
        }
    }

    // Active state

    /// Sets the background color/gradient for the active (default) state.
    pub fn background_active(mut self, background: B) -> Self {
        self.active = self.active.with_background(background);
        self
    }

    /// Sets the text color for the active (default) state.
    pub fn text_color_active(mut self, color: C) -> Self {
        self.active = self.active.with_text_color(color);
        self
    }

    /// Sets the border color for the active (default) state.
    pub fn border_color_active(mut self, color: C) -> Self {
        self.active = self.active.with_border_color(color);
        self
    }

    // Disabled state

    /// Sets the background color/gradient for the disabled state.
    pub fn background_disabled(mut self, background: B) -> Self {
        self.disabled = self.disabled.with_background(background);
        self
    }

    /// Sets the text color for the disabled state.
    pub fn text_color_disabled(mut self, color: C) -> Self {
        self.disabled = self.disabled.with_text_color(color);
        self
    }

    /// Sets the border color for the disabled state.
    pub fn border_color_disabled(mut self, color: C) -> Self {
        self.disabled = self.disabled.with_border_color(color);
        self
    }

    // Hovered state

    /// Sets the background color/gradient for the hovered state.
    pub fn background_hovered(mut self, background: B) -> Self {
        self.hovered = self.hovered.with_background(background);
        self
    }

    /// Sets the text color for the hovered state.
    pub fn text_color_hovered(mut self, color: C) -> Self {
        self.hovered = self.hovered.with_text_color(color);
        self
    }

    /// Sets the border color for the hovered state.
    pub fn border_color_hovered(mut self, color: C) -> Self {
        self.hovered = self.hovered.with_border_color(color);
        self
    }

    // Pressed state

    /// Sets the background color/gradient for the pressed state.
    pub fn background_pressed(mut self, background: B) -> Self {
        self.pressed = self.pressed.with_background(background);
        self
    }

    /// Sets the text color for the pressed state.
    pub fn text_color_pressed(mut self, color: C) -> Self {
        self.pressed = self.pressed.with_text_color(color);
        self
    }

    /// Sets the border color for the pressed state.
    pub fn border_color_pressed(mut self, color: C) -> Self {
        self.pressed = self.pressed.with_border_color(color);
        self
    }

    // Common properties

    /// Sets the border width in pixels (applies to all states).
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Sets the border radius in pixels (applies to all states).
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Sets the shadow offset as a vector (applies to all states).
    pub fn shadow_offset(mut self, offset: iced::Vector) -> Self {
        self.shadow_offset = Some(offset);
        self
    }

    /// Builds the button widget.
    ///
    /// Returns a styled `Button` widget with all state-specific styles applied.
    ///
    /// Unspecified states automatically fall back to sensible defaults based on the active state:
    /// - **Hovered**: Active with 90% opacity and enhanced shadow
    /// - **Pressed**: Active with darker background (80%) and no shadow
    /// - **Disabled**: Active with 50% opacity
    pub fn build(self) -> iced::widget::Button<'a, Message> {
        let active = self.active;
        let disabled = self.disabled;
        let hovered = self.hovered;
        let pressed = self.pressed;
        let border_width = self.border_width;
        let border_radius = self.border_radius;
        let shadow_offset = self.shadow_offset;
        let snap = self.snap;

        iced::widget::button(self.content).style(move |theme, status| {
            let palette_ext = theme.extended_palette();

            // Get base active values for fallback
            let base_bg: iced::Background = active
                .background
                .clone()
                .map(|b| b.into())
                .unwrap_or_else(|| iced::Background::Color(palette_ext.primary.strong.color));

            let base_text: iced::Color =
                active.text_color.clone().map(|c| c.into()).unwrap_or(palette_ext.background.base.text);

            let base_border: iced::Color =
                active.border_color.clone().map(|c| c.into()).unwrap_or(palette_ext.primary.strong.color);

            let base_shadow_offset = shadow_offset.unwrap_or_else(|| iced::Vector::new(0.0, 1.0));

            let (background, text_color, border_color, shadow) = match status {
                iced::widget::button::Status::Active => {
                    let bg = base_bg;
                    let text = base_text;
                    let border = base_border;
                    let shadow = iced::Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: base_shadow_offset,
                        blur_radius: 0.0,
                    };
                    (bg, text, border, shadow)
                }
                iced::widget::button::Status::Hovered => {
                    // Fallback: Use active with reduced opacity and enhanced shadow
                    let bg = hovered
                        .background
                        .clone()
                        .map(|b| b.into())
                        .unwrap_or_else(|| base_bg.scale_alpha(0.9));

                    let text = hovered.text_color.clone().map(|c| c.into()).unwrap_or(base_text);

                    let border = hovered.border_color.clone().map(|c| c.into()).unwrap_or(base_border);

                    let shadow = iced::Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: base_shadow_offset + iced::Vector::new(0.0, 1.0),
                        blur_radius: 2.0,
                    };
                    (bg, text, border, shadow)
                }
                iced::widget::button::Status::Pressed => {
                    // Fallback: Use darkened active background and no shadow
                    let bg = pressed.background.clone().map(|b| b.into()).unwrap_or_else(|| {
                        if let iced::Background::Color(c) = base_bg {
                            iced::Background::Color(iced::Color::from_rgb(c.r * 0.8, c.g * 0.8, c.b * 0.8))
                        } else {
                            base_bg
                        }
                    });

                    let text = pressed.text_color.clone().map(|c| c.into()).unwrap_or(base_text);

                    let border = pressed.border_color.clone().map(|c| c.into()).unwrap_or(base_border);

                    let shadow = iced::Shadow {
                        color: iced::Color::TRANSPARENT,
                        offset: iced::Vector::default(),
                        blur_radius: 0.0,
                    };
                    (bg, text, border, shadow)
                }
                iced::widget::button::Status::Disabled => {
                    // Fallback: Use active with 50% opacity
                    let bg = disabled
                        .background
                        .clone()
                        .map(|b| b.into())
                        .unwrap_or_else(|| base_bg.scale_alpha(0.5));

                    let text = disabled
                        .text_color
                        .clone()
                        .map(|c| c.into())
                        .unwrap_or(iced::Color { a: base_text.a * 0.5, ..base_text });

                    let border = disabled
                        .border_color
                        .clone()
                        .map(|c| c.into())
                        .unwrap_or(iced::Color { a: base_border.a * 0.5, ..base_border });

                    let shadow = iced::Shadow {
                        color: iced::Color::TRANSPARENT,
                        offset: iced::Vector::default(),
                        blur_radius: 0.0,
                    };
                    (bg, text, border, shadow)
                }
            };

            let border_radius =
                border_radius.map(|r| r.into()).unwrap_or_else(|| iced::border::Radius::default());
            let snap = snap.unwrap_or(false);

            iced::widget::button::Style {
                background: Some(background),
                text_color,
                border: iced::border::Border {
                    color: border_color,
                    width: border_width.unwrap_or(0.0),
                    radius: border_radius,
                },
                shadow,
                snap,
            }
        })
    }
}

// ============================================================================
// Gradient Builder
// ============================================================================

/// Builder for creating linear gradients with color stops.
///
/// Gradients can be used as backgrounds for frames and buttons, providing
/// smooth color transitions at specified angles.
///
/// # Examples
///
/// ```rust
/// use crate::app::widgets::builder::GradientBuilder;
/// use iced::Color;
///
/// // Horizontal gradient (default 0 degrees)
/// let gradient1 = GradientBuilder::new()
///     .stop(Color::from_rgb(1.0, 0.0, 0.0), 0.0)
///     .stop(Color::from_rgb(0.0, 0.0, 1.0), 1.0)
///     .build();
///
/// // Vertical gradient (90 degrees)
/// let gradient2 = GradientBuilder::new()
///     .angle(90.0)
///     .stop(Color::WHITE, 0.0)
///     .stop(Color::BLACK, 1.0)
///     .build();
///
/// // Multi-stop gradient with angle
/// let gradient3 = GradientBuilder::new()
///     .angle(45.0)
///     .stop(Color::from_rgb(1.0, 0.0, 0.0), 0.0)
///     .stop(Color::from_rgb(0.0, 1.0, 0.0), 0.5)
///     .stop(Color::from_rgb(0.0, 0.0, 1.0), 1.0)
///     .build();
/// ```
pub struct GradientBuilder {
    angle: Option<f32>,
    stops: Vec<(iced::Color, f32)>,
}

impl GradientBuilder {
    pub fn new() -> Self {
        Self { angle: None, stops: Vec::new() }
    }

    pub fn angle(mut self, degrees: f32) -> Self {
        self.angle = Some(degrees);
        self
    }

    pub fn stop(mut self, color: impl Into<iced::Color>, offset: f32) -> Self {
        self.stops.push((color.into(), offset));
        self
    }

    pub fn build(self) -> iced::Gradient {
        let angle_radians = self.angle.unwrap_or(0.0).to_radians();

        let mut linear = iced::gradient::Linear::new(angle_radians);

        for (color, offset) in self.stops {
            linear = linear.add_stop(offset, color);
        }

        iced::Gradient::Linear(linear)
    }
}

impl Default for GradientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TextInput State Helper
// ============================================================================

struct InputState<B, C>
where
    B: Clone,
    C: Clone,
{
    background: Option<B>,
    icon_color: Option<C>,
    value_color: Option<C>,
    placeholder_color: Option<C>,
    selection_color: Option<C>,
    border_color: Option<C>,
}

impl<B, C> InputState<B, C>
where
    B: Clone,
    C: Clone,
{
    fn new() -> Self {
        Self {
            selection_color: None,
            border_color: None,
            background: None,
            icon_color: None,
            value_color: None,
            placeholder_color: None,
        }
    }

    fn with_background(mut self, background: B) -> Self {
        self.background = Some(background);
        self
    }

    fn with_icon_color(mut self, color: C) -> Self {
        self.icon_color = Some(color);
        self
    }

    fn with_value_color(mut self, color: C) -> Self {
        self.value_color = Some(color);
        self
    }

    fn with_placeholder_color(mut self, color: C) -> Self {
        self.placeholder_color = Some(color);
        self
    }

    fn with_selection_color(mut self, color: C) -> Self {
        self.selection_color = Some(color);
        self
    }

    fn with_border_color(mut self, color: C) -> Self {
        self.border_color = Some(color);
        self
    }
}

// ============================================================================
// TextInput Builder
// ============================================================================

/// Builder for creating styled text input widgets with state-specific styling.
///
/// Text inputs can have different styles for each interaction state (active, focused,
/// hovered, and disabled).
///
/// # Examples
///
/// ```rust
/// use crate::app::widgets::builder::TextInputBuilder;
/// use iced::Color;
///
/// // Simple text input
/// let input = TextInputBuilder::new("Enter name", "")
///     .build()
///     .on_input(Message::NameChanged);
///
/// // Styled text input
/// let styled_input = TextInputBuilder::new("Search...", &search_query)
///     .background_active(Color::WHITE)
///     .border_color_focused(Color::from_rgb(0.2, 0.6, 1.0))
///     .border_width(2.0)
///     .border_radius(8.0)
///     .build()
///     .on_input(Message::SearchChanged);
/// ```
pub struct TextInputBuilder<'a, B = iced::Background, C = iced::Color>
where
    B: Into<iced::Background> + Clone,
    C: Into<iced::Color> + Clone,
{
    placeholder: &'a str,
    value: &'a str,
    active: InputState<B, C>,
    disabled: InputState<B, C>,
    focused: InputState<B, C>,
    hovered: InputState<B, C>,
    border_width: Option<f32>,
    border_radius: Option<f32>,
}

impl<'a, B, C> TextInputBuilder<'a, B, C>
where
    B: Into<iced::Background> + Clone,
    C: Into<iced::Color> + Clone,
{
    /// Creates a new text input builder with placeholder and value.
    ///
    /// # Arguments
    ///
    /// * `placeholder` - Placeholder text shown when input is empty
    /// * `value` - Current value of the input
    pub fn new(placeholder: &'a str, value: &'a str) -> Self {
        Self {
            placeholder,
            value,
            active: InputState::new(),
            disabled: InputState::new(),
            focused: InputState::new(),
            hovered: InputState::new(),
            border_width: None,
            border_radius: None,
        }
    }

    // Active state
    pub fn background_active(mut self, background: B) -> Self {
        self.active = self.active.with_background(background);
        self
    }

    pub fn icon_color_active(mut self, color: C) -> Self {
        self.active = self.active.with_icon_color(color);
        self
    }

    pub fn value_color_active(mut self, color: C) -> Self {
        self.active = self.active.with_value_color(color);
        self
    }

    pub fn placeholder_color_active(mut self, color: C) -> Self {
        self.active = self.active.with_placeholder_color(color);
        self
    }

    pub fn selection_color_active(mut self, color: C) -> Self {
        self.active = self.active.with_selection_color(color);
        self
    }

    pub fn border_color_active(mut self, color: C) -> Self {
        self.active = self.active.with_border_color(color);
        self
    }

    // Disabled state
    pub fn background_disabled(mut self, background: B) -> Self {
        self.disabled = self.disabled.with_background(background);
        self
    }

    pub fn icon_color_disabled(mut self, color: C) -> Self {
        self.disabled = self.disabled.with_icon_color(color);
        self
    }

    pub fn value_color_disabled(mut self, color: C) -> Self {
        self.disabled = self.disabled.with_value_color(color);
        self
    }

    pub fn placeholder_color_disabled(mut self, color: C) -> Self {
        self.disabled = self.disabled.with_placeholder_color(color);
        self
    }

    pub fn selection_color_disabled(mut self, color: C) -> Self {
        self.disabled = self.disabled.with_selection_color(color);
        self
    }

    pub fn border_color_disabled(mut self, color: C) -> Self {
        self.disabled = self.disabled.with_border_color(color);
        self
    }

    // Focused state
    pub fn background_focused(mut self, background: B) -> Self {
        self.focused = self.focused.with_background(background);
        self
    }

    pub fn icon_color_focused(mut self, color: C) -> Self {
        self.focused = self.focused.with_icon_color(color);
        self
    }

    pub fn value_color_focused(mut self, color: C) -> Self {
        self.focused = self.focused.with_value_color(color);
        self
    }

    pub fn placeholder_color_focused(mut self, color: C) -> Self {
        self.focused = self.focused.with_placeholder_color(color);
        self
    }

    pub fn selection_color_focused(mut self, color: C) -> Self {
        self.focused = self.focused.with_selection_color(color);
        self
    }

    pub fn border_color_focused(mut self, color: C) -> Self {
        self.focused = self.focused.with_border_color(color);
        self
    }

    // Hovered state
    pub fn background_hovered(mut self, background: B) -> Self {
        self.hovered = self.hovered.with_background(background);
        self
    }

    pub fn icon_color_hovered(mut self, color: C) -> Self {
        self.hovered = self.hovered.with_icon_color(color);
        self
    }

    pub fn value_color_hovered(mut self, color: C) -> Self {
        self.hovered = self.hovered.with_value_color(color);
        self
    }

    pub fn placeholder_color_hovered(mut self, color: C) -> Self {
        self.hovered = self.hovered.with_placeholder_color(color);
        self
    }

    pub fn selection_color_hovered(mut self, color: C) -> Self {
        self.hovered = self.hovered.with_selection_color(color);
        self
    }

    pub fn border_color_hovered(mut self, color: C) -> Self {
        self.hovered = self.hovered.with_border_color(color);
        self
    }

    // Common properties
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Builds the text input widget.
    ///
    /// Returns a styled `TextInput` widget ready to have `.on_input()` called.
    ///
    /// Unspecified states automatically fall back to sensible defaults based on the active state:
    /// - **Hovered**: Active colors with 1.5x border width
    /// - **Focused**: Active colors with 2.0x border width
    /// - **Disabled**: Active colors with 50% opacity
    pub fn build<'b, Message: Clone>(self) -> iced::widget::TextInput<'b, Message>
    where
        'a: 'b,
        C: 'b,
        B: 'b,
    {
        let active = self.active;
        let disabled = self.disabled;
        let focused = self.focused;
        let hovered = self.hovered;
        let border_width = self.border_width;
        let border_radius = self.border_radius;

        let font = iced::font::Font::with_name(APP_FONT_FAMILY_NAME);
        iced::widget::text_input(self.placeholder, self.value).font(font).style(
            move |theme: &iced::Theme, status| {
                let palette_ext = theme.extended_palette();

                // Get base active values for fallback
                let base_bg: iced::Background = active
                    .background
                    .clone()
                    .map(|b| b.into())
                    .unwrap_or_else(|| iced::Background::Color(palette_ext.background.base.color));

                let base_icon: iced::Color =
                    active.icon_color.clone().map(|c| c.into()).unwrap_or(palette_ext.primary.base.color);

                let base_value: iced::Color =
                    active.value_color.clone().map(|c| c.into()).unwrap_or(palette_ext.background.base.text);

                let base_placeholder: iced::Color = active
                    .placeholder_color
                    .clone()
                    .map(|c| c.into())
                    .unwrap_or(palette_ext.primary.base.color);

                let base_selection: iced::Color = active
                    .selection_color
                    .clone()
                    .map(|c| c.into())
                    .unwrap_or(palette_ext.primary.base.color);

                let base_border: iced::Color =
                    active.border_color.clone().map(|c| c.into()).unwrap_or(palette_ext.primary.base.color);

                let base_border_width = border_width.unwrap_or(1.0);

                let (
                    background,
                    icon_color,
                    value_color,
                    placeholder_color,
                    selection_color,
                    border_color,
                    width_multiplier,
                ) = match status {
                    iced::widget::text_input::Status::Active => {
                        (base_bg, base_icon, base_value, base_placeholder, base_selection, base_border, 1.0)
                    }
                    iced::widget::text_input::Status::Hovered => {
                        // Fallback: Use active colors with enhanced border
                        let bg = hovered.background.clone().map(|b| b.into()).unwrap_or(base_bg);
                        let icon = hovered.icon_color.clone().map(|c| c.into()).unwrap_or(base_icon);
                        let value = hovered.value_color.clone().map(|c| c.into()).unwrap_or(base_value);
                        let placeholder =
                            hovered.placeholder_color.clone().map(|c| c.into()).unwrap_or(base_placeholder);
                        let selection =
                            hovered.selection_color.clone().map(|c| c.into()).unwrap_or(base_selection);
                        let border = hovered.border_color.clone().map(|c| c.into()).unwrap_or(base_border);
                        (bg, icon, value, placeholder, selection, border, 1.5)
                    }
                    iced::widget::text_input::Status::Focused { is_hovered: false } => {
                        // Fallback: Use active colors with enhanced border
                        let bg = focused.background.clone().map(|b| b.into()).unwrap_or(base_bg);
                        let icon = focused.icon_color.clone().map(|c| c.into()).unwrap_or(base_icon);
                        let value = focused.value_color.clone().map(|c| c.into()).unwrap_or(base_value);
                        let placeholder =
                            focused.placeholder_color.clone().map(|c| c.into()).unwrap_or(base_placeholder);
                        let selection =
                            focused.selection_color.clone().map(|c| c.into()).unwrap_or(base_selection);
                        let border = focused.border_color.clone().map(|c| c.into()).unwrap_or(base_border);
                        (bg, icon, value, placeholder, selection, border, 2.0)
                    }
                    iced::widget::text_input::Status::Focused { is_hovered: true } => {
                        // Fallback: Use active colors with enhanced border
                        let bg = focused.background.clone().map(|b| b.into()).unwrap_or(base_bg);
                        let icon = focused.icon_color.clone().map(|c| c.into()).unwrap_or(base_icon);
                        let value = focused.value_color.clone().map(|c| c.into()).unwrap_or(base_value);
                        let placeholder =
                            focused.placeholder_color.clone().map(|c| c.into()).unwrap_or(base_placeholder);
                        let selection =
                            focused.selection_color.clone().map(|c| c.into()).unwrap_or(base_selection);
                        let border = focused.border_color.clone().map(|c| c.into()).unwrap_or(base_border);
                        (bg, icon, value, placeholder, selection, border, 2.0)
                    }
                    iced::widget::text_input::Status::Disabled => {
                        // Fallback: Use active with 50% opacity
                        let bg = disabled.background.clone().map(|b| b.into()).unwrap_or_else(|| {
                            iced::Background::Color(iced::Color {
                                a: 0.5,
                                ..palette_ext.background.base.color
                            })
                        });
                        let icon = disabled
                            .icon_color
                            .clone()
                            .map(|c| c.into())
                            .unwrap_or(iced::Color { a: 0.5, ..base_icon });
                        let value = disabled
                            .value_color
                            .clone()
                            .map(|c| c.into())
                            .unwrap_or(iced::Color { a: 0.5, ..base_value });
                        let placeholder = disabled
                            .placeholder_color
                            .clone()
                            .map(|c| c.into())
                            .unwrap_or(iced::Color { a: 0.5, ..base_placeholder });
                        let selection = disabled
                            .selection_color
                            .clone()
                            .map(|c| c.into())
                            .unwrap_or(iced::Color { a: 0.5, ..base_selection });
                        let border = disabled
                            .border_color
                            .clone()
                            .map(|c| c.into())
                            .unwrap_or(iced::Color { a: 0.5, ..base_border });
                        (bg, icon, value, placeholder, selection, border, 1.0)
                    }
                };

                let border_radius =
                    border_radius.map(|r| r.into()).unwrap_or_else(|| iced::border::Radius::default());

                iced::widget::text_input::Style {
                    background,
                    border: iced::Border {
                        color: border_color,
                        width: base_border_width * width_multiplier,
                        radius: border_radius,
                    },
                    icon: icon_color,
                    placeholder: placeholder_color,
                    value: value_color,
                    selection: selection_color,
                }
            },
        )
    }
}
