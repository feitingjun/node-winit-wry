use std::sync::Arc;
use winit::window::{Window as WinitWindow, WindowId, WindowButtons, Fullscreen, WindowLevel, UserAttentionType, Theme, ResizeDirection};
use wry::WebView;
use wry::dpi::LogicalPosition;
use wry::Rect;
use winit::dpi::{PhysicalPosition, PhysicalSize, Position, Size};
use winit::error::NotSupportedError;
use winit::monitor::MonitorHandle;
use std::path::Path;
use image::GenericImageView;

#[derive(Clone)]
pub struct Window {
  pub label: String,
  pub window: Arc<WinitWindow>,
  pub webview: Arc<WebView>,
  id: WindowId
}

impl Window{
  pub fn new (
    label: String,
    window:WinitWindow,
    webview: WebView,
    id: WindowId
  ) -> Self {
    Self {
      label,
      window: Arc::new(window),
      webview: Arc::new(webview),
      id
    }
  }
  pub fn id(&self) -> WindowId {
    self.id
  }
  // 设置webview的url
  pub fn set_url(&self, url: String) {
    let _ = self.webview.load_url(&url);
  }
  // 获取webview的url
  pub fn url(&self) -> String {
    self.webview.url().unwrap()
  }
  // 执行js
  pub fn evaluate_script(&self, js: &str) {
    let _ = self.webview.evaluate_script(js);
  }
  // 执行js并返回回调
  pub fn evaluate_script_with_callback(
    &self,
    js: &str,
    callback: impl Fn(String) + Send + 'static,
  ) {
    let _ = self.webview.evaluate_script_with_callback(js, callback);
  }
  // 打开调试工具(打成正式包没有这个方法)
  // pub fn open_devtools(&self) {
  //   self.webview.open_devtools();
  // }
  // // 关闭调试工具
  // pub fn close_devtools(&self) {
  //   self.webview.close_devtools();
  // }
  // // 调试工具是否打开
  // pub fn is_devtools_open(&self) -> bool {
  //   self.webview.is_devtools_open()
  // }
  // 设置webview缩放级别
  pub fn zoom(&self, scale_factor: f64) {
    let _ = self.webview.zoom(scale_factor);
  }
  pub fn scale_factor(&self) -> f64 {
    self.window.scale_factor()
  }
  // 清除所有浏览数据
  pub fn clear_all_browsing_data(&self) {
    let _ = self.webview.clear_all_browsing_data();
  }
  // 设置背景色
  pub fn set_background_color(&self, color: (u8, u8, u8, u8)) {
    let _ = self.webview.set_background_color(color);
  }
  // 返回窗口客户区左上角相对于桌面左上角的位置
  pub fn inner_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
    self.window.inner_position()
  }
  // 返回窗口左上角相对于桌面左上角的位置
  pub fn outer_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
    self.window.outer_position()
  }
  // 设置窗口左上角相对于桌面左上角的位置
  pub fn set_outer_position<P: Into<Position>>(&self, position: P){
    self.window.set_outer_position(position)
  }
  // 返回窗口客户端区域的物理大小(不包括标题栏和边框)
  pub fn inner_size(&self) -> PhysicalSize<u32> {
    self.window.inner_size()
  }
  // 设置窗口客户端区域的物理大小
  pub fn set_inner_size<S: Into<Size>>(&self, size: S) -> Option<PhysicalSize<u32>> {
    self.window.request_inner_size(size)
  }
  // 返回整个窗口的物理大小
  pub fn outer_size(&self) -> PhysicalSize<u32> {
    self.window.outer_size()
  }
  // 设置窗口的最小尺寸
  pub fn set_min_inner_size<S: Into<Size>>(&self, min_size: Option<S>) {
    self.window.set_min_inner_size(min_size)
  }
  // 设置窗口的最大尺寸
  pub fn set_max_inner_size<S: Into<Size>>(&self, max_size: Option<S>) {
    self.window.set_max_inner_size(max_size) 
  }
  // 设置窗口标题
  pub fn set_title(&self, title: &str) {
    self.window.set_title(title)
  }
  // 返回窗口标题
  pub fn title(&self) -> String {
    self.window.title()
  }
  // 设置窗口是否透明
  pub fn set_transparent(&self, transparent: bool) {
    self.window.set_transparent(transparent)
  }
  // 更改窗口模糊状态
  pub fn set_blur(&self, blur: bool) {
    self.window.set_blur(blur)
  }
  // 修改窗口的可见性
  pub fn set_visible(&self, visible: bool) {
    self.window.set_visible(visible)
  }
  // 获取窗口的可见性
  pub fn is_visible(&self) -> Option<bool> {
    self.window.is_visible()
  }
  // 设置窗口是否可调整大小
  pub fn set_resizable(&self, resizable: bool) {
    self.window.set_resizable(resizable)
  }
  // 获取窗口是否可调整大小
  pub fn is_resizable(&self) -> bool {
    self.window.is_resizable()
  }
  // 设置启用的窗口按钮
  pub fn set_enabled_buttons(&self, buttons: WindowButtons) {
    self.window.set_enabled_buttons(buttons)
  }
  // 获取启用的窗口按钮
  pub fn enabled_buttons(&self) -> WindowButtons {
    self.window.enabled_buttons()
  }
  // 设置窗口最小化
  pub fn set_minimized(&self, minimized: bool) {
    self.window.set_minimized(minimized)
  }
  // 获取窗口最小化
  pub fn is_minimized(&self) -> Option<bool> {
    self.window.is_minimized()
  }
  // 设置窗口最大化
  pub fn set_maximized(&self, maximized: bool) {
    self.window.set_maximized(maximized)
  }
  // 获取窗口最大化
  pub fn is_maximized(&self) -> bool {
    self.window.is_maximized()
  }
  // 设置窗口全屏
  pub fn set_fullscreen(&self, fullscreen: Option<Fullscreen>) {
    self.window.set_fullscreen(fullscreen)
  }
  // 获取窗口全屏
  pub fn fullscreen(&self) -> Option<Fullscreen> {
    self.window.fullscreen()
  }
  // 打开或关闭窗户装饰
  pub fn set_decorations(&self, decorations: bool) {
    self.window.set_decorations(decorations)
  }
  // 获取窗口装饰状态
  pub fn is_decorated(&self) -> bool {
    self.window.is_decorated()
  }
  // 设置窗口级别
  pub fn set_window_level(&self, level: WindowLevel) {
    self.window.set_window_level(level)
  }
  // 设置窗口图标
  pub fn set_window_icon(&self, icon_path: String) {
    let icon_path = Path::new(&icon_path);
    let icon_image = image::open(icon_path).expect("Failed to load icon");
    let (width, height) = icon_image.dimensions();
    let rgba_image = icon_image.to_rgba8();
    let icon = winit::window::Icon::from_rgba(rgba_image.into_raw(), width, height).unwrap();
    self.window.set_window_icon(Some(icon))
  }
  // 窗口聚焦
  pub fn focus_window(&self) {
    self.window.focus_window()
  }
  // 获取窗口是否聚焦
  pub fn has_focus(&self) -> bool {
    self.window.has_focus()
  }
  // 请求用户注意窗口，这对应用程序没有影响
  pub fn request_user_attention(&self, request_type: Option<UserAttentionType>) {
    self.window.request_user_attention(request_type);
  }
  // 设置窗口主题
  pub fn set_theme(&self, theme: Option<Theme>) {
    self.window.set_theme(theme);
  }
  // 获取窗口主题
  pub fn theme(&self) -> Option<Theme> {
    self.window.theme()
  }
  // 使用鼠标左键移动窗口，直到该按钮被释放(调用前按下鼠标左键)
  pub fn drag_window(&self) {
   let _ = self.window.drag_window();
  }
  // 使用鼠标左键调整窗口大小，直到该按钮被释放
  pub fn drag_resize_window(&self, direction: ResizeDirection) {
    let _ = self.window.drag_resize_window(direction);
  }
  // 在指定的位置显示窗口菜单
  pub fn show_window_menu(&self, position: impl Into<Position>) {
    self.window.show_window_menu(position);
  }
  // 获取当前显示器
  pub fn current_monitor(&self) -> Option<MonitorHandle> {
    self.window.current_monitor()
  }
  // 更改webview的大小
  pub fn resize(&self, size: Size) {
    let _ = self.webview.set_bounds(Rect{
      position: LogicalPosition::new(0.0, 0.0).into(),
      size
    });
  }
}
