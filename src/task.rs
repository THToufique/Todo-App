use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
    pub due_date: Option<SystemTime>,
    pub project: Option<String>,
    pub tags: Vec<String>,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Task {
    pub fn new(id: u64, title: String) -> Self {
        Self {
            id,
            title,
            description: String::new(),
            completed: false,
            priority: Priority::Medium,
            due_date: None,
            project: None,
            tags: Vec::new(),
            created_at: SystemTime::now(),
        }
    }
}
