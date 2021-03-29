use anyhow::Result;
use iced::{Align, Application, Column, Command, Container, Element, Length, Row, Settings, Text};

mod fetch;

macro_rules! format_entry {
    ($k:expr, $v:expr) => {
        Row::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(Text::new($k).size(30))
            .push(Text::new($v).size(30))
    };
}

enum SysInfoApp {
    Loading,
    Loaded(SysInfo),
    Error(String),
}

#[derive(Debug)]
enum Message {
    Found(Result<SysInfo>),
}

#[derive(Debug)]
pub(crate) struct SysInfo {
    systemd_version: String,
    desktop: String,
    distro: String,
    ram: String,
    hostname: String,
    kernel_version: String,
}

impl SysInfo {
    pub fn format(&self) -> Element<Message> {
        Column::new()
            .push(format_entry!("Hostname:", &self.hostname))
            .push(format_entry!("Total RAM:", &self.ram))
            .push(format_entry!("Kernel:", &self.kernel_version))
            .push(format_entry!("Distro:", &self.distro))
            .push(format_entry!("Systemd:", &self.systemd_version))
            .push(format_entry!("DE:", &self.desktop))
            .into()
    }
}

impl Application for SysInfoApp {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self::Loading,
            Command::perform(fetch::fetch_system_info(), Message::Found),
        )
    }

    fn title(&self) -> String {
        "System Information".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Found(Ok(info)) => {
                *self = SysInfoApp::Loaded(info);

                Command::none()
            }
            Message::Found(Err(e)) => {
                *self = SysInfoApp::Error(e.to_string());

                Command::none()
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content = match self {
            Self::Loading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Loading...").size(40)),
            Self::Error(e) => Column::new()
                .width(Length::Shrink)
                .push(Text::new(format!("Error: {}", e)).size(40)),
            Self::Loaded(info) => Column::new()
                .max_width(500)
                .spacing(20)
                .align_items(Align::Start)
                .push(info.format()),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    SysInfoApp::run(Settings::default())
}
