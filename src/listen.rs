use serde_json::{Map, Number, Value};
use winit::dpi::{ LogicalPosition, Size, LogicalSize };
use winit::event_loop::ActiveEventLoop;
use winit::window::{ WindowButtons, Fullscreen, WindowLevel, UserAttentionType, Theme, ResizeDirection, WindowAttributes };
use winit::monitor::MonitorHandle;
use std::io::{self, Write};
use crate::application::Application;
use std::path::Path;
use image::GenericImageView;
use wry::WebViewAttributes;

pub const IO_CHANNEL_PREFIX: &str = "_ioc:";

// 获取显示器信息
fn get_monitor_info(monitor:MonitorHandle) -> Value {
  let mut data = Map::new();
  let scale_factor = monitor.scale_factor();
  let name = monitor.name().unwrap();
  let monitor_id:Number = name["Monitor #".len()..].parse().unwrap();
  data.insert("width".to_string(), monitor.size().width.into());
  data.insert("height".to_string(), monitor.size().height.into());
  data.insert("scaleFactor".to_string(), Value::Number(Number::from_f64(scale_factor).unwrap()));
  data.insert("monitorId".to_string(), Value::Number(monitor_id));
  Value::Object(data)
}

// 发送IO消息
pub fn send_io_message(msg: Value) {
  let json_str = serde_json::to_string(&msg).unwrap();
  // 创建一个输出流
  let mut output = io::stdout();
  writeln!(output, "{}{}", IO_CHANNEL_PREFIX, json_str).unwrap();
  output.flush().unwrap();
}

// 获取Size实例
fn get_size(obj:&Value) -> Option<Size> {
  if let Some(attr) = obj.as_object() {
    let width = attr.get("width").unwrap().as_f64().unwrap();
    let height = attr.get("height").unwrap().as_f64().unwrap();
    let size = LogicalSize::new(width, height);
    Some(Size::Logical(size))
  }else{
    None
  }
}
// 获取Position实例
fn get_position(obj:&Value) -> Option<LogicalPosition<i32>>{
  if let Some(attr) = obj.as_object() {
    let x = attr.get("x").unwrap().as_i64().unwrap() as i32;
    let y = attr.get("y").unwrap().as_i64().unwrap() as i32;
    Some(LogicalPosition::new(x, y))
  }else{
    None
  }
}
// 处理IO收到的信息
pub fn handle_listen(app:&mut Application, str:&str, event_loop: &ActiveEventLoop) {
  let message:Value = serde_json::from_str(str).unwrap();
  let id = message.get("id").unwrap().as_str().unwrap();
  let label = message.get("label").unwrap().as_str().unwrap();
  let method = message.get("method").unwrap().as_str().unwrap();
  let data = message.get("data").unwrap_or(&Value::Null);
  let window = app.get_window(label.to_string());
  // 返回消息
  let mut response = serde_json::Map::new();
  response.insert("id".to_string(), Value::String(id.to_string()));
  response.insert("label".to_string(), Value::String(label.to_string()));
  response.insert("method".to_string(), Value::String(method.to_string()));
  response.insert("type".to_string(), Value::String("response".to_string()));

  match method {
    "create" => {
      if let Some(_) = window {
        println!("窗口已存在");
        return ()
      }
      // 创建窗口
      let mut window_attr = WindowAttributes::default();
      let mut webview_attr = WebViewAttributes::default();
      if let Some(data) = data.as_object() {
        for key in data.keys() {
          match key.as_str() {
            // webview相关属性
            "url" => {
              if let Some(url) = data.get("url").unwrap().as_str() {
                webview_attr.url = Some(url.to_string());
              }
            },
            "backgroundColor" => {
              if let Some(color) = data.get("backgroundColor").unwrap().as_array() {
                let colors: Result<Vec<u8>, &str> = (0..4)
                  .map(|i| {
                    color.get(i)
                    .and_then(|v| v.as_u64())
                    .and_then(|v| Some(v as u8))
                    .ok_or("颜色值必须为0-255的整数")
                  })
                  .collect();
                match colors {
                  Ok(values) => {
                    let colors = (values[0] as u8, values[1] as u8, values[2] as u8, values[3] as u8);
                    webview_attr.background_color = Some(colors);
                  }
                  Err(e) => {
                    println!("错误: {}", e);
                  }
                }
              }
            },
            "html" => {
              if let Some(html) = data.get("html").unwrap().as_str() {
                webview_attr.html = Some(html.to_string());
              }
            },
            "devtools" => {
              if let Some(devtools) = data.get("devtools").unwrap().as_bool() {
                webview_attr.devtools = devtools;
              }
            },
            "autoplay" => {
              if let Some(autoplay) = data.get("autoplay").unwrap().as_bool() {
                webview_attr.autoplay = autoplay;
              }
            },
            "innerSize" => {
              if let Some(size) = get_size(data.get("innerSize").unwrap()) {
                window_attr = window_attr.with_inner_size(size);
              };
            },
            "minInnerSize" => {
              if let Some(size) = get_size(data.get("minInnerSize").unwrap()) {
                window_attr = window_attr.with_min_inner_size(size);
              };
            },
            "maxInnerSize" => {
              if let Some(size) = get_size(data.get("maxInnerSize").unwrap()) {
                window_attr = window_attr.with_max_inner_size(size);
              };
            },
            "position" => {
              if let Some(position) = get_position(data.get("position").unwrap()){
                window_attr = window_attr.with_position(position);
              }
            },
            "resizable" => {
              if let Some(resizable) = data.get("resizable").unwrap().as_bool() {
                window_attr.resizable = resizable;
              }
            },
            "enabledButtons" => {
              let mut buttons = WindowButtons::empty();
              if let Some(arr) = data.get("enabledButtons").unwrap().as_array() {
                let mut btns = Vec::new();
                for button in arr {
                  btns.push(button.as_str().unwrap());
                }
                if btns.contains(&"close") {
                  buttons |= WindowButtons::CLOSE;
                }
                if btns.contains(&"minimize") {
                  buttons |= WindowButtons::MINIMIZE;
                }
                if btns.contains(&"maximize") {
                  buttons |= WindowButtons::MAXIMIZE;
                }
              }else{
                buttons = WindowButtons::all();
              }
              window_attr = window_attr.with_enabled_buttons(buttons);
            },
            "title" => {
              if let Some(title) = data.get("title").unwrap().as_str(){
                window_attr = window_attr.with_title(title);
              }
            },
            "maximized" => {
              if let Some(maximized) = data.get("maximized").unwrap().as_bool() {
                window_attr = window_attr.with_maximized(maximized);
              }
            },
            "visible" => {
              if let Some(visible) = data.get("visible").unwrap().as_bool() {
                window_attr = window_attr.with_visible(visible);
              }
            },
            "transparent" => {
              if let Some(transparent) = data.get("transparent").unwrap().as_bool() {
                window_attr = window_attr.with_transparent(transparent);
                webview_attr.transparent = transparent;
              }
            },
            "blur" => {
              if let Some(blur) = data.get("blur").unwrap().as_bool() {
                window_attr = window_attr.with_blur(blur);
              }
            },
            "borderless" => {
              if let Some(borderless) = data.get("borderless").unwrap().as_bool() {
                window_attr = window_attr.with_decorations(!borderless);
              }
            },
            "windowIcon" => {
              if let Some(icon_path) = data.get("windowIcon").unwrap().as_str() {
                let icon_path = Path::new(&icon_path);
                let icon_image = image::open(icon_path).expect("Failed to load icon");
                let (width, height) = icon_image.dimensions();
                let rgba_image = icon_image.to_rgba8();
                let icon = winit::window::Icon::from_rgba(rgba_image.into_raw(), width, height).unwrap();
                window_attr = window_attr.with_window_icon(Some(icon));
              }
            },
            "theme" => {
              if let Some(theme) = data.get("theme").unwrap().as_str() {
                match theme {
                  "light" => {
                    window_attr.preferred_theme = Some(Theme::Light);
                  },
                  "dark" => {
                    window_attr.preferred_theme = Some(Theme::Dark);
                  },
                  _ => {}
                }
              }
            },
            "resizeIncrements" => {
              if let Some(s) = data.get("resizeIncrements").unwrap().as_object() {
                let mut width: f64 = 1.0;
                if let Some(w) = s.get("widht") {
                  width = w.as_f64().unwrap();
                }
                let mut height: f64 = 1.0;
                if let Some(h) = s.get("height") {
                  height = h.as_f64().unwrap();
                }
                let size = LogicalSize::new(width, height);
                Some(Size::Logical(size));
                window_attr = window_attr.with_resize_increments(size);
              }
            },
            "contentProtected" => {
              if let Some(protected) = data.get("contentProtected").unwrap().as_bool() {
                window_attr = window_attr.with_content_protected(protected);
              }
            },
            "windowLevel" => {
              if let Some(level) = data.get("windowLevel").unwrap().as_str() {
                match level {
                  "normal" => {
                    window_attr = window_attr.with_window_level(WindowLevel::Normal);
                  },
                  "alwaysOnTop" => {
                    window_attr = window_attr.with_window_level(WindowLevel::AlwaysOnTop);
                  },
                  "alwaysOnBottom" => {
                    window_attr = window_attr.with_window_level(WindowLevel::AlwaysOnBottom);
                  },
                  _ => {
                    window_attr = window_attr.with_window_level(WindowLevel::Normal);
                  }
                }
              }
            },
            "active" => {
              if let Some(active) = data.get("active").unwrap().as_bool() {
                window_attr = window_attr.with_active(active);
              }
            },
            "fullscreen" => {
              let full = data.get("fullscreen").unwrap();
              if full.is_number() {
                let monitor_name = String::from("Monitor #") + full.as_number().unwrap().to_string().as_str();
                let monitors = event_loop.available_monitors();
                let mut has = false;
                for m in monitors {
                  let name = m.clone().name().unwrap();
                  if name == monitor_name {
                    has = true;
                    window_attr = window_attr.with_fullscreen(Some(Fullscreen::Borderless(Some(m))));
                    break;
                  }
                }
                if !has {
                  window_attr = window_attr.with_fullscreen(Some(Fullscreen::Borderless(None)));
                }
              }else if full.is_boolean() && full.as_bool().unwrap() == true {
                window_attr = window_attr.with_fullscreen(Some(Fullscreen::Borderless(None)));
              }
            },
            _ => {}
          }
        }
      }
      let window_id = app.create_new_window(event_loop, label.to_string(), window_attr, webview_attr);
      let id:u64 = window_id.into();
      response.insert("data".to_string(), Value::String(id.to_string()));
      send_io_message(Value::Object(response));
    },
    "set_url" => {
      if data.is_string() {
        let url = data.as_str().unwrap();
        if let Some(window) = window {
          window.set_url(url.to_string());
          send_io_message(Value::Object(response));
        }
      }
    },
    "url" => {
      if let Some(window) = window {
        let url = window.url();
        response.insert("data".to_string(), Value::String(url));
        send_io_message(Value::Object(response));
      }
    },
    "evaluate_script" => {
      if data.is_string() {
        let script = data.as_str().unwrap();
        if let Some(window) = window {
          let _ = window.evaluate_script(script);
          send_io_message(Value::Object(response));
        }
      }
    },
    "evaluate_script_with_callback" => {
      if data.is_string() {
        let script = data.as_str().unwrap();
        if let Some(window) = window {
          window.evaluate_script_with_callback(script, move |str|{
            let mut res = response.clone();
            res.insert("data".to_string(), Value::String(str));
            send_io_message(Value::Object(res));
          });
        }
      }
    },
    // "open_devtools" => {
    //   if let Some(window) = window {
    //     let _ = window.open_devtools();
    //     send_io_message(Value::Object(response));
    //   }
    // },
    // "close_devtools" => {
    //   if let Some(window) = window {
    //     let _ = window.close_devtools();
    //     send_io_message(Value::Object(response));
    //   }
    // },
    // "is_devtools_open" => {
    //   if let Some(window) = window {
    //     let open = window.is_devtools_open();
    //     response.insert("data".to_string(), Value::Bool(open));
    //     send_io_message(Value::Object(response));
    //   }
    // },
    "zoom" => {
      if data.is_number() {
        let data = data.as_f64().unwrap();
        if let Some(window) = window {
          let _ = window.zoom(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "scale_factor" => {
      if let Some(window) = window {
        let scale_factor = window.scale_factor();
        response.insert("data".to_string(), Value::Number(Number::from_f64(scale_factor).unwrap()));
        send_io_message(Value::Object(response));
      }
    },
    "clear_all_browsing_data" => {
      if let Some(window) = window {
        let _ = window.clear_all_browsing_data();
        send_io_message(Value::Object(response));
      }
    },
    "set_background_color" => {
      if data.is_array() {
        let color = data.as_array().unwrap();
        if let Some(window) = window {
          let colors: Result<Vec<u8>, &str> = (0..4)
            .map(|i| {
              color.get(i)
              .and_then(|v| v.as_u64())
              .and_then(|v| Some(v as u8))
              .ok_or("颜色值必须为0-255的整数")
            })
            .collect();
          match colors {
            Ok(values) => {
              let colors = (values[0] as u8, values[1] as u8, values[2] as u8, values[3] as u8);
              window.set_background_color(colors);
              send_io_message(Value::Object(response));
            }
            Err(e) => {
              println!("错误: {}", e);
            }
          }
        }
      }
    },
    "inner_position" => {
      if let Some(window) = window {
        let position = window.inner_position().unwrap();
        let mut data = Map::new();
        data.insert("x".to_string(), position.x.into());
        data.insert("y".to_string(), position.y.into());
        response.insert("data".to_string(), Value::Object(data));
        send_io_message(Value::Object(response));
      }
    },
    "outer_position" => {
      if let Some(window) = window {
        let position = window.outer_position().unwrap();
        let mut data = Map::new();
        data.insert("x".to_string(), position.x.into());
        data.insert("y".to_string(), position.y.into());
        response.insert("data".to_string(), Value::Object(data));
        send_io_message(Value::Object(response));
      }
    },
    "set_outer_position" => {
      if data.is_object() {
        if let Some(window) = window {
          let position = data.as_object().unwrap();
          let x = position.get("x").unwrap().as_f64().unwrap();
          let y = position.get("y").unwrap().as_f64().unwrap();
          window.set_outer_position(LogicalPosition::new(x, y)); //LogicalPosition
          send_io_message(Value::Object(response));
        }
      }
    },
    "inner_size" => {
      if let Some(window) = window {
        let size = window.inner_size();
        let mut data = Map::new();
        data.insert("width".to_string(), size.width.into());
        data.insert("height".to_string(), size.height.into());
        response.insert("data".to_string(), Value::Object(data));
        send_io_message(Value::Object(response));
      }
    },
    "set_inner_size" => {
      if data.is_object() {
        let data = data.as_object().unwrap();
        if let Some(window) = window {
          let size = LogicalSize::new(data.get("width").unwrap().as_f64().unwrap(), data.get("height").unwrap().as_f64().unwrap());
          let size = window.set_inner_size(Size::Logical(size));
          if let Some(size) = size {
            let mut data = Map::new();
            data.insert("width".to_string(), size.width.into());
            data.insert("height".to_string(), size.height.into());
            response.insert("data".to_string(), Value::Object(data));
          }
          send_io_message(Value::Object(response));
        }
      }
    },
    "outer_size" => {
      if let Some(window) = window {
        let size = window.outer_size();
        let mut data = Map::new();
        data.insert("width".to_string(), size.width.into());
        data.insert("height".to_string(), size.height.into());
        response.insert("data".to_string(), Value::Object(data));
        send_io_message(Value::Object(response));
      }
    },
    "set_min_inner_size" => {
      if data.is_object() {
        let data = data.as_object().unwrap();
        if let Some(window) = window {
          let size = LogicalSize::new(data.get("width").unwrap().as_f64().unwrap(), data.get("height").unwrap().as_f64().unwrap());
          window.set_min_inner_size(Some(Size::Logical(size)));
          send_io_message(Value::Object(response));
        }
      }
    },
    "set_max_inner_size" => {
      if data.is_object() {
        let data = data.as_object().unwrap();
        if let Some(window) = window {
          let size = LogicalSize::new(data.get("width").unwrap().as_f64().unwrap(), data.get("height").unwrap().as_f64().unwrap());
          window.set_max_inner_size(Some(Size::Logical(size)));
          send_io_message(Value::Object(response));
        }
      }
    },
    "set_title" => {
      if data.is_string() {
        let title = data.as_str().unwrap();
        if let Some(window) = window {
          window.set_title(&title);
          send_io_message(Value::Object(response));
        }
      }
    },
    "title" => {
      if let Some(window) = window {
        let title = window.title();
        response.insert("data".to_string(), Value::String(title));
        send_io_message(Value::Object(response));
      }
    },
    "set_transparent" => {
      if data.is_boolean() {
        let data = data.as_bool().unwrap();
        if let Some(window) = window {
          window.set_transparent(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "set_blur" => {
      if data.is_boolean() {
        let data = data.as_bool().unwrap();
        if let Some(window) = window {
          window.set_blur(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "set_visible" => {
      if data.is_boolean() {
        let data = data.as_bool().unwrap();
        if let Some(window) = window {
          window.set_visible(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "is_visible" => {
      if let Some(window) = window {
        let visible = window.is_visible().unwrap();
        response.insert("data".to_string(), Value::Bool(visible));
        send_io_message(Value::Object(response));
      }
    },
    "set_resizable" => {
      if data.is_boolean() {
        let data = data.as_bool().unwrap();
        if let Some(window) = window {
          window.set_resizable(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "is_resizable" => {
      if let Some(window) = window {
        let resizable = window.is_resizable();
        response.insert("data".to_string(), Value::Bool(resizable));
        send_io_message(Value::Object(response));
      }
    },
    "set_enabled_buttons" => {
      if data.is_array() {
        let data = data.as_array().unwrap();
        if let Some(window) = window {
          let mut buttons = Vec::new();
          let mut btns = WindowButtons::empty();
          for button in data {
            buttons.push(button.as_str().unwrap());
          }
          if buttons.contains(&"close") {
            btns |= WindowButtons::CLOSE;
          }
          if buttons.contains(&"minimize") {
            btns |= WindowButtons::MINIMIZE;
          }
          if buttons.contains(&"maximize") {
            btns |= WindowButtons::MAXIMIZE;
          }
          window.set_enabled_buttons(btns);
          send_io_message(Value::Object(response));
        }
      }
    },
    "enabled_buttons" => {
      if let Some(window) = window {
        let buttons = window.enabled_buttons();
        let mut btns = Vec::new();
        if buttons.contains(WindowButtons::CLOSE) {
          btns.push("close");
        }
        if buttons.contains(WindowButtons::MINIMIZE) {
          btns.push("minimize");
        }
        if buttons.contains(WindowButtons::MAXIMIZE) {
          btns.push("maximize");
        }
        response.insert("data".to_string(), Value::Array(btns.into_iter().map(|s| Value::String(s.to_string())).collect()));
        send_io_message(Value::Object(response));
      }
    },
    "set_minimized" => {
      if !data.is_null() {
        let data = data.as_bool().unwrap();
        if let Some(window) = window {
          window.set_minimized(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "is_minimized" => {
      if let Some(window) = window {
        let minimized = window.is_minimized().unwrap();
        response.insert("data".to_string(), Value::Bool(minimized));
        send_io_message(Value::Object(response));
      }
    },
    "set_maximized" => {
      if !data.is_null() {
        let data = data.as_bool().unwrap();
        if let Some(window) = window {
          window.set_maximized(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "is_maximized" => {
      if let Some(window) = window {
        let maximized = window.is_maximized();
        response.insert("data".to_string(), Value::Bool(maximized));
        send_io_message(Value::Object(response));
      }
    },
    "current_monitor" => {
      if let Some(window) = window {
        let monitor = window.current_monitor();
        if let Some(monitor) = monitor {
          let data = get_monitor_info(monitor.clone());
          response.insert("data".to_string(), data);
        }
        send_io_message(Value::Object(response));
      }
    },
    "primary_monitor" => {
      let primary = event_loop.primary_monitor();
      if let Some(monitor) = primary {
        let data = get_monitor_info(monitor.clone());
        response.insert("data".to_string(), data);
      }
      send_io_message(Value::Object(response));
    },
    "get_monitor_list" => {
      let monitors = event_loop.available_monitors();
      let mut data = Vec::new();
      for monitor in monitors {
        data.push(get_monitor_info(monitor.clone()));
      }
      response.insert("data".to_string(), Value::Array(data));
      send_io_message(Value::Object(response));
    },
    "fullscreen" => {
      if data.is_number() || data.is_null() {
        if let Some(window) = window {
          let mut monitor:Option<MonitorHandle> = None;
          if data.is_number() {
            let monitor_name = String::from("Monitor #") + data.as_number().unwrap().to_string().as_str();
            let monitors = event_loop.available_monitors();
            for m in monitors {
              let name = m.clone().name().unwrap();
              if name == monitor_name {
                monitor = Some(m);
                break;
              }
            }
          }
          window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
          send_io_message(Value::Object(response));
        }
      }
    },
    "unfullscreen" => {
      if let Some(window) = window {
        window.set_fullscreen(None);
        send_io_message(Value::Object(response));
      }
    },
    "is_fullscreen" => {
      if let Some(window) = window {
        let fullscreen = window.fullscreen();
        if let Some(Fullscreen::Borderless(monitor)) = fullscreen {
          match monitor {
            Some(name) => {
              let name = name.name().unwrap();
              let monitor_id = name["Monitor #".len()..].parse().unwrap();
              response.insert("data".to_string(), Value::Number(monitor_id));
            }
            None => {
              response.insert("data".to_string(), Value::Bool(true));
            }
          }
        }else{
          response.insert("data".to_string(), Value::Bool(false));
        }
        send_io_message(Value::Object(response));
      }
    },
    "set_decorations" => {
      if data.is_boolean() {
        let data = data.as_bool().unwrap();
        if let Some(window) = window {
          window.set_decorations(data);
          send_io_message(Value::Object(response));
        }
      }
    },
    "is_decorated" => {
      if let Some(window) = window {
        let decorated = window.is_decorated();
        response.insert("data".to_string(), Value::Bool(!decorated));
        send_io_message(Value::Object(response));
      }
    },
    "set_window_level" => {
      if data.is_string() {
        let data = data.as_str().unwrap();
        if let Some(window) = window {
          match data {
            "alwaysOnBottom" => {
              window.set_window_level(WindowLevel::AlwaysOnBottom);
            },
            "normal" => {
              window.set_window_level(WindowLevel::Normal);
            },
            "alwaysOnTop" => {
              window.set_window_level(WindowLevel::AlwaysOnTop);
            },
            _ => {
              window.set_window_level(WindowLevel::Normal);
            }
          }
          send_io_message(Value::Object(response));
        }
      }
    },
    "set_window_icon" => {
      if data.is_string() {
        let data = data.as_str().unwrap();
        if let Some(window) = window {
          window.set_window_icon(data.to_string());
          send_io_message(Value::Object(response));
        }
      }
    },
    "focus_window" => {
      if let Some(window) = window {
        window.focus_window();
        send_io_message(Value::Object(response));
      }
    },
    "has_focus" => {
      if let Some(window) = window {
        let focused = window.has_focus();
        response.insert("data".to_string(), Value::Bool(focused));
        send_io_message(Value::Object(response));
      }
    },
    "request_user_attention" => {
      if data.is_string() {
        let data = data.as_str().unwrap();
        if let Some(window) = window {
          match data {
            "critical" => {
              window.request_user_attention(Some(UserAttentionType::Critical));
            },
            "informational" => {
              window.request_user_attention(Some(UserAttentionType::Informational));
            },
            _ => {
              window.request_user_attention(Some(UserAttentionType::Critical));
            }
          }
          send_io_message(Value::Object(response));
        }
      }
      else if data.is_null() {
        if let Some(window) = window {
          window.request_user_attention(None);
          send_io_message(Value::Object(response));
        }
      }
    },
    "set_theme" => {
      if data.is_string() {
        let data = data.as_str().unwrap();
        if let Some(window) = window {
          match data {
            "light" => {
              window.set_theme(Some(Theme::Light));
            },
            "dark" => {
              window.set_theme(Some(Theme::Dark));
            },
            _ => {
              window.set_theme(None);
            }
          }
          send_io_message(Value::Object(response));
        }
      }
    },
    "theme" => {
      if let Some(window) = window {
        let theme = window.theme();
        match theme {
          Some(Theme::Light) => {
            response.insert("data".to_string(), Value::String("light".to_string()));
          },
          Some(Theme::Dark) => {
            response.insert("data".to_string(), Value::String("dark".to_string()));
          },
          None => {
            response.insert("data".to_string(), Value::Null);
          }
        }
        send_io_message(Value::Object(response));
      }
    },
    "drag_window" => {
      if let Some(window) = window {
        let _ = window.drag_window();
        send_io_message(Value::Object(response));
      }
    },
    "drag_resize_window" => {
      if data.is_string() {
        let data = data.as_str().unwrap();
        if let Some(window) = window {
          match data {
            "east" => {
              let _ = window.drag_resize_window(ResizeDirection::East);
              send_io_message(Value::Object(response));
            },
            "north" => {
              let _ = window.drag_resize_window(ResizeDirection::North);
            },
            "northEast" => {
              let _ = window.drag_resize_window(ResizeDirection::NorthEast);
            },
            "northWest" => {
              let _ = window.drag_resize_window(ResizeDirection::NorthWest);
            },
            "south" => {
              let _ = window.drag_resize_window(ResizeDirection::South);
            },
            "southEast" => {
              let _ = window.drag_resize_window(ResizeDirection::SouthEast);
            },
            "southWest" => {
              let _ = window.drag_resize_window(ResizeDirection::SouthWest);
            },
            "west" => {
              let _ = window.drag_resize_window(ResizeDirection::West);
            },
            _ => {

            }
          }
        }
      }
    },
    "show_window_menu" => {
      if data.is_object() {
        if let Some(window) = window {
          let position = data.as_object().unwrap();
          let x = position.get("x").unwrap().as_f64().unwrap();
          let y = position.get("y").unwrap().as_f64().unwrap();
          window.show_window_menu(LogicalPosition::new(x, y)); //LogicalPosition
          send_io_message(Value::Object(response));
        }
      }
    },
    "close" => {
      if let Some(_) = window {
        app.close_window(label.to_string())
      }
    },
    _ => {
      println!("方法 {} 不存在", method);
    }
  }
}