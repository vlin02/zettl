use app::start;

mod app;
pub mod shortcuts;
pub mod clipboard;
pub mod pasteboard;
pub mod db;
pub mod detection;
pub mod event;
pub mod lookup;
pub mod profile;
pub mod settings;
pub mod snippet;
pub mod syntax;
pub mod theme;
pub mod window;

fn main() {
    start();
}
