use app::start;

pub mod db;
pub mod detection;
#[macro_use]
pub mod handler;
pub mod lookup;
pub mod clipboard;
pub mod snippet;
pub mod syntax;
pub mod user;
pub mod theme;
mod app;

fn main() {
    start();
}
