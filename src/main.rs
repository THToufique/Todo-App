mod task;
mod storage;

use iced::{Element, Task, Theme, Font, Event, Subscription};
use iced::widget::{button, column, container, row, text, text_input};
use iced::window;
use iced::{Color, Size};
use iced::keyboard::{self, Key};
use task::{Task as TodoTask, Priority};
use storage::Storage;

const FIRA_CODE: Font = Font::with_name("Fira Code");

fn main() -> iced::Result {
    iced::application("Todo App", App::update, App::view)
        .subscription(App::subscription)
        .theme(|_| Theme::TokyoNightStorm)
        .font(include_bytes!("../fonts/FiraCode-Regular.ttf").as_slice())
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
    search: String,
    sort_by: SortBy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Filter {
    All,
    Pending,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SortBy {
    Created,
    Priority,
    Alphabetical,
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
            search: String::new(),
            sort_by: SortBy::Created,
        }, Task::none())
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::EventOccurred)
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
            Message::SearchChanged(value) => {
                self.search = value;
            }
            Message::SetSort(sort) => {
                self.sort_by = sort;
            }
            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                    match key {
                        Key::Character(c) if c == "1" && modifiers.control() => {
                            self.selected_priority = Priority::Low;
                        }
                        Key::Character(c) if c == "2" && modifiers.control() => {
                            self.selected_priority = Priority::Medium;
                        }
                        Key::Character(c) if c == "3" && modifiers.control() => {
                            self.selected_priority = Priority::High;
                        }
                        Key::Character(c) if c == "f" && modifiers.control() => {
                            // Focus search - handled by UI
                        }
                        _ => {}
                    }
                }
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

        let search_input = text_input("Search tasks...", &self.search)
            .on_input(Message::SearchChanged)
            .padding(8)
            .style(|_theme, _status| text_input::Style {
                background: Color::from_rgba(0.2, 0.2, 0.25, 0.9).into(),
                border: iced::Border {
                    color: Color::from_rgba(0.4, 0.4, 0.5, 0.5),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: Color::from_rgba(0.6, 0.6, 0.7, 1.0),
                value: Color::WHITE,
                selection: Color::from_rgba(0.3, 0.5, 0.8, 0.5),
            });

        let mut filtered_tasks: Vec<&TodoTask> = self.tasks.iter()
            .filter(|task| match self.filter {
                Filter::All => true,
                Filter::Pending => !task.completed,
                Filter::Completed => task.completed,
            })
            .filter(|task| {
                if self.search.is_empty() {
                    true
                } else {
                    task.title.to_lowercase().contains(&self.search.to_lowercase()) ||
                    task.description.to_lowercase().contains(&self.search.to_lowercase())
                }
            })
            .collect();

        // Sort tasks
        match self.sort_by {
            SortBy::Created => filtered_tasks.sort_by_key(|task| task.created_at),
            SortBy::Priority => filtered_tasks.sort_by_key(|task| match task.priority {
                Priority::High => 0,
                Priority::Medium => 1,
                Priority::Low => 2,
            }),
            SortBy::Alphabetical => filtered_tasks.sort_by_key(|task| &task.title),
        }

        let sort_buttons = row![
            button("Date")
                .on_press(Message::SetSort(SortBy::Created))
                .padding(4)
                .style(move |_theme, _status| button::Style {
                    background: Some(if self.sort_by == SortBy::Created {
                        Color::from_rgba(0.4, 0.3, 0.6, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.6)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
            button("Priority")
                .on_press(Message::SetSort(SortBy::Priority))
                .padding(4)
                .style(move |_theme, _status| button::Style {
                    background: Some(if self.sort_by == SortBy::Priority {
                        Color::from_rgba(0.4, 0.3, 0.6, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.6)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
            button("A-Z")
                .on_press(Message::SetSort(SortBy::Alphabetical))
                .padding(4)
                .style(move |_theme, _status| button::Style {
                    background: Some(if self.sort_by == SortBy::Alphabetical {
                        Color::from_rgba(0.4, 0.3, 0.6, 0.9)
                    } else {
                        Color::from_rgba(0.2, 0.2, 0.25, 0.6)
                    }.into()),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
        ].spacing(3);

        let tasks_list = filtered_tasks.iter().fold(
            column![].spacing(5),
            |col, task| {
                let priority_color = match task.priority {
                    Priority::Low => Color::from_rgb(0.3, 0.6, 0.3),
                    Priority::Medium => Color::from_rgb(0.6, 0.5, 0.2),
                    Priority::High => Color::from_rgb(0.8, 0.3, 0.3),
                };

                let checkbox = button(
                    text(if task.completed { "[X]" } else { "[ ]" })
                        .size(16)
                        .font(FIRA_CODE)
                        .color(if task.completed { 
                            Color::from_rgb(0.3, 0.9, 0.3) 
                        } else { 
                            Color::from_rgb(0.7, 0.7, 0.8) 
                        })
                )
                .on_press(Message::ToggleTask(task.id))
                .padding(4)
                .style(move |_theme, _status| button::Style {
                    background: Some(Color::from_rgba(0.2, 0.2, 0.25, 0.7).into()),
                    border: iced::Border {
                        color: priority_color,
                        width: 2.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                });
                
                let title = text(&task.title).size(18).font(FIRA_CODE);
                
                let delete_btn = button("×")
                    .on_press(Message::DeleteTask(task.id))
                    .padding(4)
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
            container(text("Todo App").size(32).font(FIRA_CODE))
                .width(iced::Length::Fill)
                .center_x(iced::Length::Fill),
            input_row,
            row![filter_buttons, search_input].spacing(20),
            container(sort_buttons).width(iced::Length::Fill).center_x(iced::Length::Fill),
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
    SearchChanged(String),
    SetSort(SortBy),
    EventOccurred(Event),
}
