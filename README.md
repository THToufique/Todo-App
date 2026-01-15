# Todo App

A fast, beautiful desktop todo application for Linux built with Rust and iced.

## Features

- **Clean Interface**: Modern translucent design with Fira Code font
- **Priority Management**: Color-coded task priorities (Low, Medium, High)
- **Smart Filtering**: View all tasks, pending only, or completed
- **Search**: Find tasks by title or description
- **Flexible Sorting**: Sort by date, priority, or alphabetically
- **Keyboard Shortcuts**: Quick priority selection with Ctrl+1/2/3
- **Offline First**: All data stored locally, no internet required
- **Fast Startup**: Lightweight single binary

## Installation

Download the latest release binary and make it executable:

```bash
chmod +x todo-app
./todo-app
```

Or build from source:

```bash
git clone https://github.com/THToufique/todo-app.git
cd todo-app
cargo build --release
./target/release/todo-app
```

## Usage

- **Add Task**: Type in the input field and press Enter or click Add
- **Set Priority**: Click Low/Med/High buttons or use Ctrl+1/2/3
- **Complete Task**: Click the checkbox [X] / [ ]
- **Delete Task**: Click the × button
- **Filter**: Use All/Pending/Completed buttons
- **Search**: Type in the search box to find tasks
- **Sort**: Choose Date/Priority/A-Z sorting

## Data Storage

Tasks are automatically saved to `~/.local/share/todo-app/tasks.json`

## Requirements

- Linux (any modern distribution)
- No additional dependencies required

## Building

Requires Rust 1.70+ and cargo:

```bash
cargo build --release
```

The binary will be created at `target/release/todo-app`

## License

MIT License - see LICENSE file for details
