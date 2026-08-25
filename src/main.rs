use iced::{
    theme::palette::mix,
    Subscription,
    Background,
    keyboard,
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

use crate::dmx::{DmxChannel, DmxUniverse};

mod artnet;
mod net;
mod dmx;

// kinda heatmap-based colouring, though still just placeholders for now
pub const MIN_COLOUR: iced::Color = iced::Color::from_rgb(0.0, 0.0, 0.0);
pub const MAX_COLOUR: iced::Color = iced::Color::from_rgb(1.0, 1.0, 1.0);

fn main() -> iced::Result {
    iced::application(DMXDirect::default, DMXDirect::update, DMXDirect::view)
        .title("DMXDirect")
        .subscription(DMXDirect::subscription)
        .run()
}

struct DMXDirect {
    pub universe: DmxUniverse,
    pub selected_channels: Vec<DmxChannel>,
    pub multi_select: bool,
    pub slider_value: u8,
}

impl DMXDirect {
    pub const fn default() -> Self {
        Self {
            universe: DmxUniverse::default(),
            selected_channels: Vec::new(),
            multi_select: false,
            slider_value: 0,
        }
    }

    #[inline]
    pub fn get_colour(&self, address: DmxChannel) -> iced::Color {
        mix(MIN_COLOUR, MAX_COLOUR, self.universe.get_channel(address) as f32 / u8::MAX as f32)
    }
}

#[derive(Clone)]
enum Message {
    Reset,
    SetChannel { channel: DmxChannel, value: u8 },
    SetSelectedChannels { value: u8 },
    SelectChannel { channel: DmxChannel },
    DeselectChannels,
    StartMultiChannelSelection,
    StopMultiChannelSelection,
    StartContiguousChannelSelection,
    StopContiguousChannelSelection,
    // delta change channel
}

impl DMXDirect {
    fn update(&mut self, message: Message) {
        match message {
            Message::Reset => {
                self.universe = DmxUniverse::default();
                self.selected_channels.clear();
            }
            Message::SetChannel { channel, value } => {
                self.universe.set_channel(channel, value);
            }
            Message::SetSelectedChannels { value } => {
                self.slider_value = value;
                for channel in self.selected_channels.iter() {
                    self.universe.set_channel(*channel, value);
                }
            }
            Message::DeselectChannels => {
                self.selected_channels.clear();
            }
            Message::SelectChannel { channel } => {
                if self.multi_select {
                    self.selected_channels.push(channel);
                } else {
                    self.selected_channels.clear();
                    self.selected_channels.push(channel);
                }
            }
            Message::StartMultiChannelSelection => {
                self.multi_select = true;
            }
            Message::StopMultiChannelSelection => {
                self.multi_select = false;
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut toolbar = row([
            button(text("Reset")).on_press(Message::Reset).into(),
            button(text("Set Output")).into(),
        ])
        .spacing(8)
        .padding(8)
        .width(Length::Fill);

        toolbar = toolbar.push(
            slider(
                0..=255,
                self.slider_value,
                move |value| Message::SetSelectedChannels { value },
            ),
        );

        let rows = (0..16).map(|y| {
            let cells = (0..32).map(|x| {
                let cell_channel = DmxChannel::new((y * 32 + x) + 1).unwrap(); // i mean its safe, but its not ideal
                let cell_colour = self.get_colour(cell_channel);
                let text_colour = if self.selected_channels.contains(&cell_channel) {
                    iced::Color::from_rgb(0.0, 0.0, 1.0)
                } else {
                    contrasting_color(cell_colour)
                };

                button(container(text(cell_channel.to_string()))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center(Length::Fill)
                    .style(move |_theme| container::Style {
                        background: Some(Background::Color(cell_colour)),
                        text_color: Some(text_colour),
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

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            match event {
                keyboard::Event::KeyPressed { key, .. } => {
                    match key {
                        keyboard::Key::Named(keyboard::key::Named::Escape) => {
                            Some(Message::DeselectChannels)
                        }
                        keyboard::Key::Named(keyboard::key::Named::Control) => {
                            Some(Message::StartMultiChannelSelection)
                        }
                        keyboard::Key::Named(keyboard::key::Named::Shift) => {
                            Some(Message::StartContiguousChannelSelection)
                        }
                        _ => None,
                    }
                }
                keyboard::Event::KeyReleased { key, .. } => {
                    match key {
                        keyboard::Key::Named(keyboard::key::Named::Control) => {
                            Some(Message::StopMultiChannelSelection)
                        }
                        keyboard::Key::Named(keyboard::key::Named::Shift) => {
                            Some(Message::StopContiguousChannelSelection)
                        }
                        _ => None
                    }
                }
                _ => None,
            }
        })
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