pub mod application;
pub mod window;
pub mod listen;
pub mod event;
use application::Application;

fn main() {
  let mut app = Application::new();
  let _ = app.run();
}