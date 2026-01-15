use crate::task::Task;
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("todo-app");
        fs::create_dir_all(&path).ok();
        path.push("tasks.json");
        
        Self { path }
    }

    pub fn load(&self) -> Vec<Task> {
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, tasks: &[Task]) -> Result<(), String> {
        let data = serde_json::to_string_pretty(tasks)
            .map_err(|e| e.to_string())?;
        fs::write(&self.path, data)
            .map_err(|e| e.to_string())
    }
}
