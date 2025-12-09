use iced::Element;
use iced::Length;
use iced::Pixels;
use iced::Rectangle;
use iced::Size;
use iced::Theme;
use iced::advanced::Layout;
use iced::advanced::Widget;
use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::Tree;
use iced::alignment;
use iced::widget::text::Fragment;
use iced::widget::text::LineHeight;
use iced::widget::text::Shaping;
use iced::widget::text::Wrapping;

/// Internal state for tracking mouse press
struct State<P: iced::advanced::text::Paragraph> {
    text_state: iced::advanced::widget::text::State<P>,
    is_pressed: bool,
}

pub struct ClickableText<'a, Renderer, Message>
where
    Renderer: text::Renderer,
{
    width: Length,
    height: Length,
    shaping: Shaping,
    wrapping: Wrapping,
    size: Option<Pixels>,
    fragment: Fragment<'a>,
    color_idle: Option<iced::Color>,
    color_hovered: Option<iced::Color>,
    color_pressed: Option<iced::Color>,
    color_disabled: Option<iced::Color>,
    passed_message: Option<Message>,
    line_height: LineHeight,
    font: Option<Renderer::Font>,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
}

impl<'a, Renderer, Message> ClickableText<'a, Renderer, Message>
where
    Renderer: text::Renderer,
{
    pub fn new(fragment: impl text::IntoFragment<'a>) -> Self {
        ClickableText {
            fragment: fragment.into_fragment(),
            color_idle: None,
            color_hovered: None,
            color_pressed: None,
            color_disabled: None,
            size: None,
            line_height: LineHeight::default(),
            font: None,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            shaping: Shaping::default(),
            wrapping: Wrapping::default(),
            passed_message: None,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn align_x(mut self, alignment: impl Into<iced::alignment::Horizontal>) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    pub fn align_y(mut self, alignment: impl Into<iced::alignment::Vertical>) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    pub fn color(mut self, color: impl Into<iced::Color>) -> Self {
        self.color_idle = Some(color.into());
        self
    }

    pub fn color_hovered(mut self, color: impl Into<iced::Color>) -> Self {
        self.color_hovered = Some(color.into());
        self
    }

    pub fn color_pressed(mut self, color: impl Into<iced::Color>) -> Self {
        self.color_pressed = Some(color.into());
        self
    }

    pub fn color_disabled(mut self, color: impl Into<iced::Color>) -> Self {
        self.color_disabled = Some(color.into());
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.passed_message = Some(message);
        self
    }

    pub fn on_press_maybe(mut self, message: impl FnOnce() -> Option<Message>) -> Self {
        self.passed_message = message();
        self
    }
}

impl<'a, Renderer, Message> Widget<Message, Theme, Renderer> for ClickableText<'a, Renderer, Message>
where
    Renderer: text::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size { width: self.width, height: self.height }
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(State {
            text_state: iced::advanced::text::paragraph::Plain::<Renderer::Paragraph>::default(),
            is_pressed: false,
        })
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        iced::advanced::widget::text::layout(
            &mut state.text_state,
            renderer,
            limits,
            &self.fragment,
            iced::advanced::widget::text::Format {
                width: self.width,
                height: self.height,
                line_height: self.line_height,
                size: self.size,
                font: self.font,
                shaping: self.shaping,
                wrapping: self.wrapping,
                align_x: self.horizontal_alignment.into(),
                align_y: self.vertical_alignment,
            },
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();
        let palette_ext = theme.extended_palette();
        let paragraph = state.text_state.raw();

        let bounds = layout.bounds();

        let x = match self.horizontal_alignment {
            alignment::Horizontal::Left => bounds.x,
            alignment::Horizontal::Center => bounds.center_x(),
            alignment::Horizontal::Right => bounds.x + bounds.width,
        };

        let y = match self.vertical_alignment {
            alignment::Vertical::Top => bounds.y,
            alignment::Vertical::Center => bounds.center_y(),
            alignment::Vertical::Bottom => bounds.y + bounds.height,
        };

        let color_idle = self.color_idle.unwrap_or(palette_ext.background.base.text);

        let color_hovered = self.color_hovered.unwrap_or(iced::Color::from_rgb(
            color_idle.r * 2.0,
            color_idle.g * 2.0,
            color_idle.b * 2.0,
        ));

        let color_disabled = self.color_disabled.unwrap_or(iced::Color::from_rgba(
            color_idle.r * 0.5,
            color_idle.g * 0.5,
            color_idle.b * 0.5,
            0.5,
        ));

        let color_pressed = self.color_pressed.unwrap_or(iced::Color::from_rgb(
            color_idle.r * 0.8,
            color_idle.g * 0.8,
            color_idle.b * 0.8,
        ));

        let is_over = cursor.is_over(layout.bounds());

        let draw_color = if self.passed_message.is_none() {
            color_disabled
        } else if state.is_pressed && is_over {
            color_pressed
        } else if is_over {
            color_hovered
        } else {
            color_idle
        };

        renderer.fill_paragraph(paragraph, iced::Point::new(x, y), draw_color, *viewport);
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.passed_message.is_none() {
            mouse::Interaction::default()
        } else {
            if cursor.is_over(layout.bounds()) {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            }
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if let Some(passed_message) = &self.passed_message {
            let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
            let is_over = cursor.is_over(layout.bounds());

            match event {
                iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if is_over {
                        state.is_pressed = true;
                    }
                }
                iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    if state.is_pressed {
                        state.is_pressed = false;
                        if is_over {
                            shell.publish(passed_message.clone());
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

impl<'a, Renderer, Message> From<ClickableText<'a, Renderer, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(widget: ClickableText<'a, Renderer, Message>) -> Self {
        Self::new(widget)
    }
}
