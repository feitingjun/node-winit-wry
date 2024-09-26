use std::collections::HashMap;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{WindowAttributes, WindowId };
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{Rect, WebViewAttributes, WebViewBuilder};
use std::io::{self, BufRead};
use std::thread;
use crate::window::Window;
use crate::listen::{IO_CHANNEL_PREFIX, handle_listen};
use crate::event::handle_window_event;

pub enum Action {
  ForwardMessage(String)
}

pub struct Application {
  is_resumed: bool,
  pub windows: HashMap<String, Window>,
  pub proxy: Option<EventLoopProxy<Action>>
}

impl Application {
  pub fn new () -> Self {
    Self {
      is_resumed: false,
      windows: HashMap::new(),
      proxy: None
    }
  }
  pub fn run(&mut self) {
    let event_loop = EventLoop::<Action>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();
    self.proxy = Some(proxy);
    let _ = event_loop.run_app(self);
  }
  // 监听IO
  fn listen(&self){
    let proxy = self.proxy.clone().unwrap();
    // 监听需要在子线程中进行，不然会阻塞主线程
    thread::spawn(move || loop {
      let stdin = io::stdin();
      let input = stdin.lock().lines().next();
      let line = input.unwrap();
      match line {
        Ok(line) => {
          if line.starts_with(IO_CHANNEL_PREFIX) {
            let string = &line[IO_CHANNEL_PREFIX.len()..];
            // 通过自定义user_event将消息转发给主线程的handleListen方法
            let _ = proxy.send_event(Action::ForwardMessage(string.to_string()));
          }
        },
        Err(e) => {
          println!("接受消息错误: {:?}", e)
        }
      }
    });
  }
  pub fn get_window(&self, label: String) -> Option<&Window> {
    self.windows.get(&label).clone()
  }
  pub fn close_window(&mut self, label: String){
    self.windows.remove(&label);
  }
  pub fn create_new_window(&mut self, event_loop: &ActiveEventLoop, label: String, mut window_attr:WindowAttributes, webview_attr:WebViewAttributes) -> WindowId {
    if window_attr.inner_size.is_none() {
      window_attr = window_attr.with_inner_size(LogicalSize::new(800, 600));
    }
    let size = window_attr.inner_size.unwrap();
    let window = event_loop.create_window(window_attr).unwrap();
    // 直接使用WebViewBuilder::new()创建的webview会导致winit窗口崩溃，需要创建child webview
    let mut webview_uilder = WebViewBuilder::new_as_child(&window)
      .with_bounds(Rect{
        position: LogicalPosition::new(0.0, 0.0).into(),
        size: size,
      });
    if webview_attr.url.is_some() {
      webview_uilder = webview_uilder.with_url(webview_attr.url.unwrap());
    }
    if webview_attr.background_color.is_some() {
      webview_uilder = webview_uilder.with_background_color(webview_attr.background_color.unwrap());
    }
    if webview_attr.html.is_some() {
      webview_uilder = webview_uilder.with_html(webview_attr.html.unwrap());
    }
    webview_uilder = webview_uilder
      .with_transparent(webview_attr.transparent)
      .with_devtools(webview_attr.devtools)
      .with_autoplay(webview_attr.autoplay);

    let webview = webview_uilder.build().unwrap();
    let id = window.id();
    self.windows.insert(label.clone(), Window::new(label, window, webview, id));
    id
  }
}

impl ApplicationHandler<Action> for Application {
  fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
    if self.is_resumed != true {
      self.is_resumed = true;
      // 程序启动时启动监听node进程发送过来的消息
      self.listen();
    }
  }
  fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Action) {
    match event {
      Action::ForwardMessage(string) => {
        handle_listen(self, string.as_str(), &event_loop);
      }
    }
  }
  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent,
  ) {
    handle_window_event(self, event_loop, window_id, event);
  }
  // fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
  //   use std::time;
  //   const WAIT_TIME: time::Duration = time::Duration::from_millis(100);
  //   event_loop.set_control_flow(ControlFlow::WaitUntil(time::Instant::now() + WAIT_TIME));
  // }
}
