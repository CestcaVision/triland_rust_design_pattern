//! # Singleton
//! Singleton in rust just use one_cell or lazy_static crate, of which the former is recommended. One_cell is now joined into the standard libaray in the nightly version of Rust.
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// The `SINGLETON` static variable is a `Lazy` static variable that holds a `Mutex` of a `GameManager` instance. The `Lazy` static variable is used to ensure that the `GameManager` instance is only created once when it is first accessed. The `Mutex` is used to ensure that the `GameManager` instance is thread-safe.
pub static SINGLETON: Lazy<Mutex<GameManager>> = Lazy::new(|| Mutex::new(GameManager::new()));

/// This is an example of a singleton pattern in Rust. The `GameManager` struct is a simple struct that holds the state of a game. The `GameManager` struct has methods to get and modify the state of the game, such as the level and money.
pub struct GameManager {
    level: i32,
    money: i32,
}

impl GameManager {
    fn new() -> Self {
        Self { level: 1, money: 0 }
    }

    fn get_level(&self) -> i32 {
        self.level
    }

    fn increase_level(&mut self) {
        self.level += 1;
    }

    fn get_money(&self) -> i32 {
        self.money
    }

    // 增加金钱
    fn add_money(&mut self, amount: i32) {
        self.money += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_functionality() {
        {
            // 首先获取单例并通过 Mutex 锁定进行操作
            let mut game_manager = SINGLETON.lock().unwrap();
            // 验证初始状态
            assert_eq!(game_manager.get_level(), 1);
            assert_eq!(game_manager.get_money(), 0);
            // 修改状态
            game_manager.increase_level();
            game_manager.add_money(100);
        } // Mutex 锁在这里自动释放

        // 再次获取单例验证是否保持修改后的状态
        let game_manager = SINGLETON.lock().unwrap();
        assert_eq!(game_manager.get_level(), 2);
        assert_eq!(game_manager.get_money(), 100);
    }
}
