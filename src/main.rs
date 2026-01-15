mod task;
mod storage;

use iced::{Element, Task, Theme};
use iced::widget::{button, column, container, row, text, text_input};
use iced::window;
use iced::{Color, Size};
use task::{Task as TodoTask, Priority};
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
    selected_priority: Priority,
    filter: Filter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Filter {
    All,
    Pending,
    Completed,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let storage = Storage::new();
        let tasks = storage.load();
        
        (Self {
            tasks,
            storage,
            input: String::new(),
            selected_priority: Priority::Medium,
            filter: Filter::All,
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
                    let mut task = TodoTask::new(id, self.input.clone());
                    task.priority = self.selected_priority;
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
            Message::SetPriority(priority) => {
                self.selected_priority = priority;
            }
            Message::SetFilter(filter) => {
                self.filter = filter;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_input("What needs to be done?", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::AddTask)
            .padding(10)
            .style(|_theme, _status| text_input::Style {
                background: Color::from_rgba(0.2, 0.2, 0.25, 0.9).into(),
                border: iced::Border {
                    color: Color::from_rgba(0.4, 0.4, 0.5, 0.5),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: Color::from_rgba(0.6, 0.6, 0.7, 1.0),
                value: Color::WHITE,
                selection: Color::from_rgba(0.3, 0.5, 0.8, 0.5),
            });

        let priority_buttons = row![
            button("Low")
                .on_press(Message::SetPriority(Priority::Low))
                .padding(8)
                .style(move |_theme, _status| button::Style {
                    background: Some(if matches!(self.selected_priority, Priority::Low) {
                        Color::from_rgba(0.3, 0.6, 0.3, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.7)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }),
            button("Med")
                .on_press(Message::SetPriority(Priority::Medium))
                .padding(8)
                .style(move |_theme, _status| button::Style {
                    background: Some(if matches!(self.selected_priority, Priority::Medium) {
                        Color::from_rgba(0.6, 0.5, 0.2, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.7)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }),
            button("High")
                .on_press(Message::SetPriority(Priority::High))
                .padding(8)
                .style(move |_theme, _status| button::Style {
                    background: Some(if matches!(self.selected_priority, Priority::High) {
                        Color::from_rgba(0.8, 0.3, 0.3, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.7)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }),
        ].spacing(5);

        let add_button = button("Add")
            .on_press(Message::AddTask)
            .padding(10)
            .style(|_theme, _status| button::Style {
                background: Some(Color::from_rgba(0.2, 0.4, 0.7, 0.9).into()),
                text_color: Color::WHITE,
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            });

        let input_row = row![input, priority_buttons, add_button].spacing(10);

        let filter_buttons = row![
            button("All")
                .on_press(Message::SetFilter(Filter::All))
                .padding(6)
                .style(move |_theme, _status| button::Style {
                    background: Some(if self.filter == Filter::All {
                        Color::from_rgba(0.3, 0.4, 0.6, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.6)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }),
            button("Pending")
                .on_press(Message::SetFilter(Filter::Pending))
                .padding(6)
                .style(move |_theme, _status| button::Style {
                    background: Some(if self.filter == Filter::Pending {
                        Color::from_rgba(0.3, 0.4, 0.6, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.6)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }),
            button("Completed")
                .on_press(Message::SetFilter(Filter::Completed))
                .padding(6)
                .style(move |_theme, _status| button::Style {
                    background: Some(if self.filter == Filter::Completed {
                        Color::from_rgba(0.3, 0.4, 0.6, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.6)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }),
        ].spacing(5);

        let filtered_tasks: Vec<&TodoTask> = self.tasks.iter()
            .filter(|task| match self.filter {
                Filter::All => true,
                Filter::Pending => !task.completed,
                Filter::Completed => task.completed,
            })
            .collect();

        let tasks_list = filtered_tasks.iter().fold(
            column![].spacing(5),
            |col, task| {
                let priority_color = match task.priority {
                    Priority::Low => Color::from_rgb(0.3, 0.6, 0.3),
                    Priority::Medium => Color::from_rgb(0.6, 0.5, 0.2),
                    Priority::High => Color::from_rgb(0.8, 0.3, 0.3),
                };

                let checkbox = button(
                    text(if task.completed { "✓" } else { "○" })
                        .size(18)
                        .color(if task.completed { 
                            Color::from_rgb(0.3, 0.8, 0.3) 
                        } else { 
                            Color::from_rgb(0.6, 0.6, 0.7) 
                        })
                )
                .on_press(Message::ToggleTask(task.id))
                .padding(8)
                .style(move |_theme, _status| button::Style {
                    background: Some(Color::from_rgba(0.2, 0.2, 0.25, 0.7).into()),
                    border: iced::Border {
                        color: priority_color,
                        width: 2.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                });
                
                let title = text(&task.title).size(16);
                
                let delete_btn = button("×")
                    .on_press(Message::DeleteTask(task.id))
                    .padding(8)
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::from_rgba(0.8, 0.2, 0.2, 0.8).into()),
                        text_color: Color::WHITE,
                        border: iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 6.0.into(),
                        },
                        ..Default::default()
                    });

                let task_row = row![checkbox, title, delete_btn]
                    .spacing(10)
                    .padding(5);

                col.push(task_row)
            }
        );

        let content = column![
            text("Todo App").size(32),
            input_row,
            filter_buttons,
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
    SetPriority(Priority),
    SetFilter(Filter),
}
