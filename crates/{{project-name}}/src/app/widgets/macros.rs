//! Convenience macros for creating styled widgets
//!
//! This module provides ergonomic macros that wrap the builder pattern,
//! allowing for cleaner and more declarative widget creation syntax.

/// Creates an icon with optional size and color.
///
/// # Syntax
///
/// ```text
/// icon!(icon_value)
/// icon!(icon_value, size: size_value)
/// icon!(icon_value, color: color_value)
/// icon!(icon_value, size: size_value, color: color_value)
/// ```
///
/// # Examples
///
/// ```rust
/// use crate::core::types::Icon;
/// use iced::Color;
///
/// // Simple icon
/// let home_icon = icon!(Icon::Home);
///
/// // Icon with size
/// let large_icon = icon!(Icon::Settings, size: 32);
///
/// // Icon with color
/// let colored_icon = icon!(Icon::Search, color: Color::from_rgb(0.2, 0.6, 1.0));
///
/// // Icon with size and color
/// let styled_icon = icon!(
///     Icon::Heart,
///     size: 24,
///     color: Color::from_rgb(1.0, 0.0, 0.0)
/// );
/// ```
#[macro_export]
macro_rules! icon {
    ($icon:expr) => {
        $crate::app::widgets::IconBuilder::new($icon).build()
    };
    ($icon:expr, size: $size:expr) => {
        $crate::app::widgets::IconBuilder::new($icon).size($size).build()
    };
    ($icon:expr, color: $color:expr) => {
        $crate::app::widgets::IconBuilder::new($icon).color($color).build()
    };
    ($icon:expr, size: $size:expr, color: $color:expr) => {
        $crate::app::widgets::IconBuilder::new($icon).size($size).color($color).build()
    };
}

/// Creates a Nerd Font text widget with optional size and color.
///
/// Supports `format!`-like syntax for text formatting using parentheses syntax.
///
/// # Syntax
///
/// ```text
/// nerd_text!(text_value)
/// nerd_text!(("format string", args...))
/// nerd_text!(text_value, size: size_value)
/// nerd_text!(("format {}", arg), color: color_value)
/// nerd_text!(text_value, size: size_value, color: color_value)
/// ```
///
/// # Examples
///
/// ```rust
/// use iced::Color;
///
/// // Simple text
/// let text = nerd_text!("Hello");
///
/// // With formatting (note the parentheses)
/// let name = "Alice";
/// let formatted = nerd_text!(("Hello, {}!", name));
///
/// // Multiple arguments
/// let count = 42;
/// let status = nerd_text!(("Count: {} items", count));
///
/// // Text with size
/// let large_text = nerd_text!("Welcome", size: 24);
///
/// // Formatted with styling (note the parentheses)
/// let styled = nerd_text!(
///     ("User: {} (ID: {})", username, user_id),
///     size: 16,
///     color: Color::from_rgb(0.0, 1.0, 0.0)
/// );
/// ```
#[macro_export]
macro_rules! nerd_text {
    // Format with properties - using parentheses to wrap format args
    (($fmt:expr, $($arg:expr),+ $(,)?), $($prop:ident: $value:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::NerdTextBuilder::new(format!($fmt, $($arg),+));
        $(
            builder = builder.$prop($value);
        )*
        builder.build()
    }};
    // Format without properties - using parentheses to wrap format args
    (($fmt:expr, $($arg:expr),+ $(,)?)) => {
        $crate::app::widgets::NerdTextBuilder::new(format!($fmt, $($arg),+)).build()
    };
    // Text with properties
    ($text:expr, $($prop:ident: $value:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::NerdTextBuilder::new($text);
        $(
            builder = builder.$prop($value);
        )*
        builder.build()
    }};
    // Simple text
    ($text:expr) => {
        $crate::app::widgets::NerdTextBuilder::new($text).build()
    };
}

/// Creates a styled frame (container) with various styling options.
///
/// # Syntax
///
/// ```text
/// frame!(content, property: value, ...)
/// ```
///
/// # Properties
///
/// - `background`: Background color or gradient
/// - `border_color`: Border color
/// - `border_width`: Border width in pixels
/// - `border_radius`: Border radius in pixels
/// - `shadow_color`: Shadow color
/// - `shadow_blur_radius`: Shadow blur radius
/// - `shadow_offset`: Shadow offset as a vector
/// - `text_color`: Text color
///
/// # Examples
///
/// ```rust
/// use iced::{Color, Vector};
///
/// // Simple frame with background
/// let frame1 = frame!(
///     my_content,
///     background: Color::from_rgb(0.1, 0.1, 0.1)
/// );
///
/// // Frame with border
/// let frame2 = frame!(
///     my_content,
///     border_color: Color::from_rgb(0.5, 0.5, 0.5),
///     border_width: 2.0,
///     border_radius: 8.0
/// );
///
/// // Frame with shadow
/// let frame3 = frame!(
///     my_content,
///     background: Color::WHITE,
///     shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
///     shadow_blur_radius: 10.0,
///     shadow_offset: Vector::new(2.0, 2.0)
/// );
/// ```
#[macro_export]
macro_rules! frame {
    ($content:expr, $($prop:ident: $value:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::FrameBuilder::<_, iced::Background, iced::Color, iced::Vector>::new($content);
        $(
            builder = builder.$prop($value);
        )*
        builder.build()
    }};
    ($content:expr) => {
        $crate::app::widgets::FrameBuilder::<_, iced::Background, iced::Color, iced::Vector>::new($content).build()
    };
}

/// Creates a styled button with state-specific styling.
///
/// # Syntax
///
/// ```text
/// button!(content, property: value, ...)
/// ```
///
/// # Properties
///
/// ## Active State
/// - `background_active`: Background when button is active
/// - `text_color_active`: Text color when button is active
/// - `border_color_active`: Border color when button is active
///
/// ## Disabled State
/// - `background_disabled`: Background when button is disabled
/// - `text_color_disabled`: Text color when button is disabled
/// - `border_color_disabled`: Border color when button is disabled
///
/// ## Hovered State
/// - `background_hovered`: Background when button is hovered
/// - `text_color_hovered`: Text color when button is hovered
/// - `border_color_hovered`: Border color when button is hovered
///
/// ## Pressed State
/// - `background_pressed`: Background when button is pressed
/// - `text_color_pressed`: Text color when button is pressed
/// - `border_color_pressed`: Border color when button is pressed
///
/// ## Common Properties
/// - `border_width`: Border width in pixels
/// - `border_radius`: Border radius in pixels
/// - `shadow_offset`: Shadow offset as a vector
///
/// # Examples
///
/// ```rust
/// use iced::{Color, Background};
///
/// // Simple button
/// let btn1 = button!(
///     "Click me",
///     background_active: Color::from_rgb(0.2, 0.6, 1.0),
///     text_color_active: Color::WHITE
/// );
///
/// // Button with hover effect
/// let btn2 = button!(
///     "Hover me",
///     background_active: Color::from_rgb(0.2, 0.6, 1.0),
///     background_hovered: Color::from_rgb(0.3, 0.7, 1.0),
///     text_color_active: Color::WHITE,
///     border_radius: 8.0
/// );
///
/// // Button with all states
/// let btn3 = button!(
///     icon!(Icon::Save, size: 20),
///     background_active: Color::from_rgb(0.2, 0.8, 0.2),
///     background_hovered: Color::from_rgb(0.3, 0.9, 0.3),
///     background_pressed: Color::from_rgb(0.1, 0.7, 0.1),
///     background_disabled: Color::from_rgb(0.5, 0.5, 0.5),
///     text_color_active: Color::WHITE,
///     border_width: 2.0,
///     border_radius: 4.0
/// );
/// ```
#[macro_export]
macro_rules! button {
    ($content:expr, $($prop:ident: $value:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::ButtonBuilder::<_, iced::Background, iced::Color>::new($content);
        $(
            builder = builder.$prop($value);
        )*
        builder.build()
    }};
    ($content:expr) => {
        $crate::app::widgets::ButtonBuilder::<_, iced::Background, iced::Color>::new($content).build()
    };
}

/// Creates a gradient with optional angle and color stops.
///
/// # Syntax
///
/// ```text
/// gradient!()                                          // Empty gradient
/// gradient!(angle;)                                    // Gradient with angle only
/// gradient!(color => offset, ...)                      // Gradient with color stops
/// gradient!(angle; color => offset, ...)               // Gradient with angle and color stops
/// ```
///
/// # Examples
///
/// ```rust
/// use iced::Color;
///
/// // Simple horizontal gradient
/// let grad1 = gradient!(
///     Color::from_rgb(1.0, 0.0, 0.0) => 0.0,
///     Color::from_rgb(0.0, 0.0, 1.0) => 1.0
/// );
///
/// // Angled gradient (45 degrees)
/// let grad2 = gradient!(
///     45.0;
///     Color::from_rgb(1.0, 0.0, 0.0) => 0.0,
///     Color::from_rgb(0.0, 1.0, 0.0) => 0.5,
///     Color::from_rgb(0.0, 0.0, 1.0) => 1.0
/// );
///
/// // Gradient with RGB array syntax
/// let grad3 = gradient!(
///     90.0;
///     [1.0, 0.0, 0.0] => 0.0,
///     [0.0, 0.0, 1.0] => 1.0
/// );
///
/// // Vertical gradient (90 degrees)
/// let grad4 = gradient!(
///     90.0;
///     Color::WHITE => 0.0,
///     Color::BLACK => 1.0
/// );
/// ```
#[macro_export]
macro_rules! gradient {
    // No angle, just color points
    ($($color:expr => $offset:expr),+ $(,)?) => {{
        let mut builder = $crate::app::widgets::GradientBuilder::new();
        $(
            builder = builder.stop($color, $offset);
        )+
        builder.build()
    }};

    // With angle and color points
    ($angle:expr; $($color:expr => $offset:expr),+ $(,)?) => {{
        let mut builder = $crate::app::widgets::GradientBuilder::new()
            .angle($angle);
        $(
            builder = builder.stop($color, $offset);
        )+
        builder.build()
    }};

    // Just angle, no color points
    ($angle:expr;) => {
        $crate::app::widgets::GradientBuilder::new()
            .angle($angle)
            .build()
    };

    // No arguments
    () => {
        $crate::app::widgets::GradientBuilder::new().build()
    };
}

/// Quick gradient creation using builder pattern directly.
///
/// This is an alternative to the `gradient!` macro that provides
/// more flexibility at the cost of slightly more verbose syntax.
///
/// # Examples
///
/// ```rust
/// use crate::app::widgets::builder::GradientBuilder;
/// use iced::Color;
///
/// let gradient = GradientBuilder::new()
///     .angle(45.0)
///     .stop(Color::from_rgb(1.0, 0.0, 0.0), 0.0)
///     .stop(Color::from_rgb(0.0, 0.0, 1.0), 1.0)
///     .build();
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! gradient_builder {
    // No angle, just color points
    ($($color:expr => $offset:expr),+ $(,)?) => {{
        let mut builder = $crate::app::widgets::GradientBuilder::new();
        $(
            builder = builder.stop($color, $offset);
        )+
        builder.build()
    }};

    // With angle and color points
    ($angle:expr; $($color:expr => $offset:expr),+ $(,)?) => {{
        let mut builder = $crate::app::widgets::GradientBuilder::new().angle($angle);
        $(
            builder = builder.stop($color, $offset);
        )+
        builder.build()
    }};
}

/// Creates a clickable text widget with optional styling.
///
/// Supports `format!`-like syntax for text formatting using parentheses syntax.
///
/// # Syntax
///
/// ```text
/// clickable_text!(text_value)
/// clickable_text!(("format string", args...))
/// clickable_text!(text_value, size: size_value)
/// clickable_text!(("format {}", arg), color: color_value)
/// clickable_text!(text_value, size: size_value, color: color_value, color_hovered: hover_color)
/// ```
///
/// # Examples
///
/// ```rust
/// use iced::Color;
///
/// // Simple clickable text
/// let link = clickable_text!("Click me").on_press(Message::Clicked);
///
/// // With formatting (note the parentheses)
/// let url = "example.com";
/// let link = clickable_text!(("Visit {}", url)).on_press(Message::OpenUrl);
///
/// // With color
/// let colored_link = clickable_text!(
///     "Visit Site",
///     color: Color::from_rgb(0.2, 0.6, 1.0)
/// ).on_press(Message::OpenLink);
///
/// // Formatted with styling (note the parentheses)
/// let page_num = 5;
/// let nav_link = clickable_text!(
///     ("Go to page {}", page_num),
///     color: Color::from_rgb(0.2, 0.6, 1.0),
///     color_hovered: Color::from_rgb(0.3, 0.7, 1.0)
/// ).on_press(Message::GoToPage(page_num));
/// ```
#[macro_export]
macro_rules! clickable_text {
    // Format with properties - using parentheses to wrap format args
    (($fmt:expr, $($arg:expr),+ $(,)?), $($prop:ident: $value:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::ClickableTextBuilder::new(format!($fmt, $($arg),+));
        $(
            builder = builder.$prop($value);
        )*
        builder.build()
    }};
    // Format without properties - using parentheses to wrap format args
    (($fmt:expr, $($arg:expr),+ $(,)?)) => {
        $crate::app::widgets::ClickableTextBuilder::new(format!($fmt, $($arg),+)).build()
    };
    // Text with properties
    ($text:expr, $($prop:ident: $value:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::ClickableTextBuilder::new($text);
        $(
            builder = builder.$prop($value);
        )*
        builder.build()
    }};
    // Simple text
    ($text:expr) => {
        $crate::app::widgets::ClickableTextBuilder::new($text).build()
    };
}

/// Creates a styled text input widget with state-specific styling.
///
/// Supports `format!`-like syntax for placeholder text formatting using parentheses syntax.
///
/// # Syntax
///
/// ```text
/// text_input!(placeholder, value, property: value, ...)
/// text_input!(("format {}", arg), value, property: value, ...)
/// ```
///
/// # Properties
///
/// ## Active State
/// - `background_active`: Background when input is active
/// - `icon_color_active`: Icon color when active
/// - `value_color_active`: Text value color when active
/// - `placeholder_color_active`: Placeholder color when active
/// - `selection_color_active`: Selection color when active
/// - `border_color_active`: Border color when active
///
/// ## Disabled State
/// - `background_disabled`, `icon_color_disabled`, `value_color_disabled`
/// - `placeholder_color_disabled`, `selection_color_disabled`, `border_color_disabled`
///
/// ## Focused State
/// - `background_focused`, `icon_color_focused`, `value_color_focused`
/// - `placeholder_color_focused`, `selection_color_focused`, `border_color_focused`
///
/// ## Hovered State
/// - `background_hovered`, `icon_color_hovered`, `value_color_hovered`
/// - `placeholder_color_hovered`, `selection_color_hovered`, `border_color_hovered`
///
/// ## Common Properties
/// - `border_width`: Border width in pixels
/// - `border_radius`: Border radius in pixels
///
/// # Examples
///
/// ```rust
/// use iced::Color;
///
/// // Simple text input
/// let input = text_input!("Enter name", &name)
///     .on_input(Message::NameChanged);
///
/// // With formatted placeholder (note the parentheses)
/// let field_name = "username";
/// let input = text_input!(("Enter your {}", field_name), &value)
///     .on_input(Message::ValueChanged);
///
/// // With border styling
/// let styled_input = text_input!(
///     "Search...",
///     &search_query,
///     border_color_focused: Color::from_rgb(0.2, 0.6, 1.0),
///     border_width: 2.0,
///     border_radius: 8.0
/// ).on_input(Message::SearchChanged);
///
/// // Formatted placeholder with styling (note the parentheses)
/// let max_len = 100;
/// let custom_input = text_input!(
///     ("Enter text (max {} chars)", max_len),
///     &text,
///     border_color_focused: Color::from_rgb(0.2, 0.6, 1.0),
///     border_width: 1.0,
///     border_radius: 4.0
/// ).on_input(Message::TextChanged);
/// ```
#[macro_export]
macro_rules! text_input {
    // Format placeholder with properties - using parentheses to wrap format args
    (($fmt:expr, $($farg:expr),+ $(,)?), $value:expr, $($prop:ident: $val:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::TextInputBuilder::<'_, iced::Background, iced::Color>::new(
            &format!($fmt, $($farg),+),
            $value
        );
        $(
            builder = builder.$prop($val);
        )*
        builder.build()
    }};
    // Format placeholder without properties - using parentheses to wrap format args
    (($fmt:expr, $($farg:expr),+ $(,)?), $value:expr $(,)?) => {
        $crate::app::widgets::TextInputBuilder::<'_, iced::Background, iced::Color>::new(
            &format!($fmt, $($farg),+),
            $value
        ).build()
    };
    // Text with properties
    ($placeholder:expr, $value:expr, $($prop:ident: $val:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut builder = $crate::app::widgets::TextInputBuilder::<'_, iced::Background, iced::Color>::new($placeholder, $value);
        $(
            builder = builder.$prop($val);
        )*
        builder.build()
    }};
    // Simple text
    ($placeholder:expr, $value:expr) => {
        $crate::app::widgets::TextInputBuilder::<_, iced::Background, iced::Color>::new($placeholder, $value).build()
    };
}
