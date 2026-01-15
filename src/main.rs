mod task;
mod storage;

use iced::{Element, Task, Theme};
use iced::widget::{button, column, container, row, text, text_input};
use iced::window;
use iced::{Color, Size};
use task::{Task as TodoTask};
use storage::Storage;

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

struct App {
    tasks: Vec<TodoTask>,
    storage: Storage,
    input: String,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let storage = Storage::new();
        let tasks = storage.load();
        
        (Self {
            tasks,
            storage,
            input: String::new(),
        }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
            }
            Message::AddTask => {
                if !self.input.trim().is_empty() {
                    let id = self.tasks.len() as u64;
                    let task = TodoTask::new(id, self.input.clone());
                    self.tasks.push(task);
                    self.input.clear();
                    self.storage.save(&self.tasks).ok();
                }
            }
            Message::ToggleTask(id) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    task.completed = !task.completed;
                    self.storage.save(&self.tasks).ok();
                }
            }
            Message::DeleteTask(id) => {
                self.tasks.retain(|t| t.id != id);
                self.storage.save(&self.tasks).ok();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_input("What needs to be done?", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::AddTask)
            .padding(10);

        let add_button = button("Add").on_press(Message::AddTask).padding(10);

        let input_row = row![input, add_button].spacing(10);

        let tasks_list = self.tasks.iter().fold(
            column![].spacing(5),
            |col, task| {
                let checkbox = button(if task.completed { "✓" } else { "○" })
                    .on_press(Message::ToggleTask(task.id));
                
                let title = text(&task.title).size(16);
                
                let delete_btn = button("×")
                    .on_press(Message::DeleteTask(task.id));

                let task_row = row![checkbox, title, delete_btn]
                    .spacing(10)
                    .padding(5);

                col.push(task_row)
            }
        );

        let content = column![
            text("Todo App").size(32),
            input_row,
            tasks_list,
        ]
        .padding(20)
        .spacing(20);

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
enum Message {
    InputChanged(String),
    AddTask,
    ToggleTask(u64),
    DeleteTask(u64),
}
