use crate::application::Application;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::event::WindowEvent;
use serde_json::{Map, Value};
use crate::listen::send_io_message;

pub fn handle_window_event(
  app:&mut Application,
  event_loop:&ActiveEventLoop,
  window_id: WindowId,
  event: WindowEvent
) {
  
  let len = app.windows.len();
  let windows_clone = app.windows.clone();
  let mut windows = windows_clone.values();
  let window = windows.find(|w| w.id() == window_id).unwrap();

  let mut response = serde_json::Map::new();
  response.insert("type".to_string(), Value::String("windowEvent".to_string()));
  response.insert("label".to_string(), Value::String(window.label.to_string()));

  match event {
    WindowEvent::CloseRequested => {
      response.insert("method".to_string(), Value::String("close".to_string()));
      send_io_message(Value::Object(response));
      // 如果只有一个窗口，直接退出进程
      if len == 1 {
        event_loop.exit();
        return;
      }
      // 关闭窗口
      app.close_window(window.label.clone()); 
    },
    WindowEvent::Moved(position) => {
      response.insert("method".to_string(), Value::String("move".to_string()));
      let mut obj = Map::new();
      obj.insert("x".to_string(), position.x.into());
      obj.insert("y".to_string(), position.y.into());
      response.insert("data".to_string(), Value::Object(Map::from(obj)));
      send_io_message(Value::Object(response));
    },
    WindowEvent::Destroyed => {
      response.insert("method".to_string(), Value::String("destroy".to_string()));
      send_io_message(Value::Object(response));
    },
    WindowEvent::Focused(focused) => {
      if focused {
        response.insert("method".to_string(), Value::String("focus".to_string()));
      }else {
        response.insert("method".to_string(), Value::String("blur".to_string()));
      }
      send_io_message(Value::Object(response));
    },
    WindowEvent::CursorMoved { device_id:_, position } => {
      response.insert("method".to_string(), Value::String("cursorMove".to_string()));
      let mut pos = Map::new();
      pos.insert("x".to_string(), position.x.into());
      pos.insert("y".to_string(), position.y.into());
      response.insert("data".to_string(), Value::Object(pos));
      send_io_message(Value::Object(response));
    },
    WindowEvent::CursorEntered { device_id:_ } => {
      response.insert("method".to_string(), Value::String("cursorEnter".to_string()));
      send_io_message(Value::Object(response));
    },
    WindowEvent::CursorLeft { device_id:_ } => {
      response.insert("method".to_string(), Value::String("cursorOut".to_string()));
      send_io_message(Value::Object(response));
    },
    WindowEvent::ThemeChanged(theme) => {
      response.insert("method".to_string(), Value::String("theme".to_string()));
      match theme {
        winit::window::Theme::Light => {
          response.insert("data".to_string(), Value::String("light".to_string()));
        },
        winit::window::Theme::Dark => {
          response.insert("data".to_string(), Value::String("dark".to_string()));
        }
      }
      send_io_message(Value::Object(response));
    },
    WindowEvent::Occluded(occluded) => {
      response.insert("method".to_string(), Value::String("occluded".to_string()));
      response.insert("data".to_string(), Value::Bool(occluded));
      send_io_message(Value::Object(response));
    },
    WindowEvent::Resized(size) => {
      response.insert("method".to_string(), Value::String("resize".to_string()));
      let mut data = Map::new();
      data.insert("width".to_string(), size.width.into());
      data.insert("height".to_string(), size.height.into());
      response.insert("data".to_string(), Value::Object(data));
      send_io_message(Value::Object(response));
    },
    _ => (),
  }
}