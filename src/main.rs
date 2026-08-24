use iced::{
    theme::palette::mix,
    Background,
    Element,
    Length,
    Color,
    widget::{
        container,
        button,
        column,
        slider,
        text,
        row,
    },
};

// kinda heatmap-based colouring, though still just placeholders for now
pub const MIN_COLOUR: iced::Color = iced::Color::from_rgb(0.0, 0.0, 0.0);
pub const MAX_COLOUR: iced::Color = iced::Color::from_rgb(1.0, 1.0, 1.0);

pub struct DMXUniverse {
    pub data: [u8; 512]
}

impl DMXUniverse {
    pub const fn default() -> Self {
        Self {
            data: [0; 512]
        }
    }
}

fn main() -> iced::Result {
    iced::application(DMXDirect::default, DMXDirect::update, DMXDirect::view)
        .title("DMXDirect")
        .run()
}

struct DMXDirect {
    pub universe: DMXUniverse,
    pub selected_channel: Option<u16>,
}

impl DMXDirect {
    pub const fn default() -> Self {
        Self {
            universe: DMXUniverse::default(),
            selected_channel: None,
        }
    }

    /// Sets an address within the DMX Universe.
    /// Takes a human-indexed (1 -> 512) address number,
    /// 
    /// # Safety
    /// Address must be within 1 and 512 (inclusive)
    #[inline]
    pub fn set_address(&mut self, address: u16, value: u8) {
        self.universe.data[(address - 1) as usize] = value;
    }

    #[inline]
    pub fn get_address(&self, address: u16) -> u8 {
        self.universe.data[(address - 1) as usize]
    }

    #[inline]
    pub fn get_colour(&self, address: u16) -> iced::Color {
        mix(MIN_COLOUR, MAX_COLOUR, self.get_address(address) as f32 / u8::MAX as f32)
    }
}

#[derive(Debug, Clone)]
enum Message {
    DummyOne,
    Reset,
    DummyThree,
    SetChannel { channel: u16, value: u8 },
    SelectChannel { channel: u16 },
    DeselectChannels,
}

impl DMXDirect {
    fn update(&mut self, message: Message) {
        match message {
            Message::Reset => {
                self.universe = DMXUniverse::default();
                self.selected_channel = None;
            }
            Message::SetChannel { channel, value } => {
                self.set_address(channel, value);
            }
            Message::DeselectChannels => {
                self.selected_channel = None;
            }
            Message::SelectChannel { channel } => {
                self.selected_channel = Some(channel)
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut toolbar = row([
            button(text(format!("{:?}", self.selected_channel))).on_press(Message::DummyOne).into(),
            button(text("Reset")).on_press(Message::Reset).into(),
            button(text("Set Output")).on_press(Message::DummyThree).into(),
        ])
        .spacing(8)
        .padding(8)
        .width(Length::Fill);

        if let Some(channel) = self.selected_channel {
            toolbar = toolbar.push(
                slider(
                    0..=255,
                    self.get_address(channel),
                    move |value| Message::SetChannel { channel, value },
                ),
            );
        }

        let rows = (0..16).map(|y| {
            let cells = (0..32).map(|x| {
                let cell_channel = (y * 32 + x) + 1;
                let cell_color = self.get_colour(cell_channel);

                button(container(text(cell_channel))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center(Length::Fill)
                    .style(move |_theme| container::Style {
                        background: Some(Background::Color(cell_color)),
                        text_color: Some(contrasting_color(cell_color)),
                        ..Default::default()
                    })
                )
                    .padding(0)
                    .style(button::text)
                    .on_press(Message::SelectChannel { channel: cell_channel })
                    .into()
            });

            row(cells)
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(1)
                .into()
        });

        let grid: Element<'_, Message> = column(rows)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(1)
            .into();

        column(vec![toolbar.into(), grid])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn contrasting_color(color: Color) -> Color {
    let luminance =
        0.2126 * color.r +
        0.7152 * color.g +
        0.0722 * color.b;

    if luminance > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}