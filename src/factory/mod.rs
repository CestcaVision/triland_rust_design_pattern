//! # Factory Pattern
//! Factory pattern is a creational pattern that provides an interface for creating objects in a superclass, but allows subclasses to alter the type of objects that will be created.

/// Basic trait for game console, including some basic methods for game console. This trait will be implemented by different game consoles.
pub trait GameConsole {
    /// show greeting message for gamers when they start the game console.
    fn greeting(&self) -> String;
}

pub struct PlayStation;
pub struct Xbox;
pub struct NintendoSwitch;

/// PlayStation implementation for GameConsole trait.
///
/// ```rust,ignore
///     impl GameConsole for PlayStation {
///     fn greeting(&self) -> String {
///         "PlayStation: Welcome to the game world!".to_string()
///     }
/// }
/// ```
impl GameConsole for PlayStation {
    fn greeting(&self) -> String {
        "PlayStation: Welcome to the game world!".to_string()
    }
}
impl GameConsole for Xbox {
    fn greeting(&self) -> String {
        "Xbox: Welcome to the game world!".to_string()
    }
}
impl GameConsole for NintendoSwitch {
    fn greeting(&self) -> String {
        "NintendoSwitch: Welcome to the game world!".to_string()
    }
}

/// Enum for different game console types.
pub enum GameConsoleType {
    PlayStation,
    Xbox,
    NintendoSwitch,
}

/// Factory struct for creating different game consoles.
pub struct GameConsoleFactory;

impl GameConsoleFactory {
    fn create_game_console(game_console: GameConsoleType) -> Box<dyn GameConsole> {
        match game_console {
            GameConsoleType::PlayStation => Box::new(PlayStation),
            GameConsoleType::Xbox => Box::new(Xbox),
            GameConsoleType::NintendoSwitch => Box::new(NintendoSwitch),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::factory::*;

    #[test]
    fn test_game_console_factory() {
        let ps = GameConsoleFactory::create_game_console(GameConsoleType::PlayStation);
        assert_eq!(ps.greeting(), "PlayStation: Welcome to the game world!");

        let xbox = GameConsoleFactory::create_game_console(GameConsoleType::Xbox);
        assert_eq!(xbox.greeting(), "Xbox: Welcome to the game world!");

        let switch = GameConsoleFactory::create_game_console(GameConsoleType::NintendoSwitch);
        assert_eq!(
            switch.greeting(),
            "NintendoSwitch: Welcome to the game world!"
        );
    }
}
