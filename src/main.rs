use iced::{Element, Task, Theme};
use iced::widget::{column, container, text};
use iced::window;
use iced::{Color, Size};

fn main() -> iced::Result {
    iced::application("Todo App", App::update, App::view)
        .theme(|_| Theme::TokyoNightStorm)
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            transparent: true,
            ..Default::default()
        })
        .run_with(App::new)
}

struct App;

impl App {
    fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Todo App").size(32),
        ]
        .padding(20);

        container(content)
            .center(800.0)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgba(0.1, 0.1, 0.15, 0.85).into()),
                ..Default::default()
            })
            .into()
    }
}

#[derive(Debug, Clone)]
enum Message {}
