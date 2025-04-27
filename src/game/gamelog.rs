use bracket_lib::prelude::RGB;
use specs::World;
use specs::WorldExt;
use std::collections::VecDeque;

pub enum LogLevel {
    Info,
    Warning,
    Combat,
    Critical,
    System,
}

pub struct LogEntry {
    pub text: String,
    pub level: LogLevel,
    pub turn: u32,
}

pub struct GameLog {
    pub entries: VecDeque<LogEntry>,
    pub max_entries: usize,
}

impl GameLog {
    pub fn new(max_entries: usize) -> Self {
        GameLog { entries: VecDeque::with_capacity(max_entries),
        max_entries, }
    }

    pub fn add_entry(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn get_color(&self, level: &LogLevel) -> RGB {
        match level {
            LogLevel::Info => RGB::named(bracket_lib::prelude::WHITE),
            LogLevel::Warning => RGB::named(bracket_lib::prelude::YELLOW),
            LogLevel::Combat => RGB::named(bracket_lib::prelude::RED),
            LogLevel::Critical => RGB::from_f32(1.0, 0.0, 0.0), // Bright red
            LogLevel::System => RGB::named(bracket_lib::prelude::CYAN),
        }
    }
}

#[derive(Default)]
pub struct GameTurn(pub u32);

pub fn log_entry(world: &mut World, message: impl ToString, level: LogLevel) {
    let current_turn = world.read_resource::<GameTurn>().0;
    let mut log = world.write_resource::<GameLog>();
    
    let entry = LogEntry {
        text: message.to_string(),
        level,
        turn: current_turn,
    };
    
    log.add_entry(entry);
}

// Convenience functions for different log levels
pub fn info(world: &mut World, message: impl ToString) {
    log_entry(world, message, LogLevel::Info);
}

pub fn warning(world: &mut World, message: impl ToString) {
    log_entry(world, message, LogLevel::Warning);
}

pub fn combat(world: &mut World, message: impl ToString) {
    log_entry(world, message, LogLevel::Combat);
}

pub fn critical(world: &mut World, message: impl ToString) {
    log_entry(world, message, LogLevel::Critical);
}

pub fn system(world: &mut World, message: impl ToString) {
    log_entry(world, message, LogLevel::System);
}

