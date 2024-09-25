export type WindowId = number
export type Size = {
  width: number
  height: number
}
export type Position = {
  x: number
  y: number
}
// 显示器信息
export type Monitor = {
  monitorId: number
  width: number
  height: number
  scaleFactor: number
}
export type WindowButton = 'close' | 'minimize' | 'maximize'
export type Theme = 'light' | 'dark'
export type WindowLevel = 'normal' | 'alwaysOnTop' | 'alwaysOnBottom'

/**
 * 请求用户注意窗口
 *
 * critical
 ** Windows 同时闪烁窗口和任务栏按钮，直到应用程序进入焦点
 ** MacOS 弹出dock图标，直到应用程序处于焦点位置
 *
 * informational
 ** Windows 闪烁任务栏按钮，直到应用程序处于焦点位置
 ** MacOS 弹出dock图标，弹出一次dock图标
*/
export type UserAttentionType = 'critical' | 'informational'

/**窗口调整尺寸的方向 */
export type ResizeDirection = 'east' | 'north' | 'northEast' | 'northWest' | 'south' | 'southEast' | 'southWest' | 'west'

/**创建窗口的参数 */
export interface WindowAttributes {
  /**webview URL */
  url?: string
  /**webview HTML */
  html?: string
  /**webview的背景色(MacOS不支持) */
  backgroundColor?: [number, number, number, number]
  /**是否打开webbiew开发工具 */
  devtools?: boolean
  /**是否运行媒体自动播放 */
  autoplay?: boolean
  /**窗口尺寸(不包含标题栏和边框) */
  innerSize?: Size
  /**窗口最小尺寸 */
  minInnerSize?: Size
  /**窗口最大尺寸 */
  maxInnerSize?: Size
  /**窗口位置 */
  position?: Position
  /**窗口是否可调整大小 */
  resizable?: boolean
  /**窗口控制按钮列表 */
  enabledButtons?: WindowButton[]
  /**窗口标题 */
  title?: String
  /**窗口是否最大化 */
  maximized?: boolean
  /**窗口是否显示 */
  visible?: boolean
  /**窗口是否透明 */
  transparent?: boolean
  /**透明窗口是否模糊 */
  blur?: boolean
  /**是否是无边框窗口 */
  borderless?: boolean
  /**窗口图标 */
  windowIcon?: string
  /**窗口主题 */
  theme?: Theme
  /**调整窗口大小时的增量(大小按特定步长变化，比如width=10则宽度以10个像素的增量变化) */
  resizeIncrements?: {
    width?: number
    height?: number
  }
  /**内容保护 */
  contentProtected?: boolean
  /**窗口层级 */
  windowLevel?: WindowLevel
  /**是否处于活动状态 */
  active?: boolean
  /**是否全屏
   ** 传入true在当前显示器全屏
   ** 传入显示器id则在指定显示器上全屏(显示器id不存在则在当前显示器全屏)
   */
  fullscreen?: boolean | Monitor['monitorId']
}

/**向窗口发送消息的方法 */
export interface MessageMethod {
  /**设置url */
  set_url: {
    params: string
  }
  /**获取url */
  url: {
    response: string
  }
  /**在webview上执行js代码 */
  evaluate_script: {
    params: string
  }
  /**在webview上执行js代码并返回执行结果 */
  evaluate_script_with_callback: {
    params: string
    response: string
  }
  /**打开调试工具 */
  open_devtools: {}
  /**关闭调试工具 */
  close_devtools: {}
  /**调试工具是否打开 */
  is_devtools_open: {
    response: boolean
  }
  /**设置webview缩放等级 */
  zoom: {
    params: number
  }
  /**获取屏幕DPI缩放比例 */
  scale_factor: {
    response: number
  }
  /**清除所有浏览数据 */
  clear_all_browsing_data: {}
  /**设置webview背景色 */
  set_background_color: {
    params: [number, number, number, number]
  }
  /**创建窗口 */
  create: {
    params: WindowAttributes
    response: WindowId
  }
  /**关闭窗口 */
  close: {}
  /**返回窗口客户区左上角相对于桌面左上角的位置 */
  inner_position: {
    response: Position
  }
  /**返回窗口左上角相对于桌面左上角的位置 */
  outer_position: {
    response: Position
  }
  set_outer_position: {
    params: Position
  }
  /**返回窗口客户端区域的物理大小(不包括标题栏和边框) */
  inner_size: {
    response: Size
  }
  /**返回整个窗口的物理大小 */
  outer_size: {
    response: Size
  }
  /**设置窗口的最小尺寸 */
  set_min_inner_size: {
    params: Size
  }
  /**设置窗口的最大尺寸 */
  set_max_inner_size: {
    params: Size
  }
  /**设置窗口标题 */
  set_title: {
    params: string
  }
  /**返回窗口标题 */
  title: {
    response: string
  }
  /**设置窗口是否透明 */
  set_transparent: {
    params: boolean
  }
  /**更改窗口模糊状态 */
  set_blur: {
    params: boolean
  }
  /**修改窗口的可见性 */
  set_visible: {
    params: boolean
  }
  /**获取窗口的可见性 */
  is_visible: {
    response: boolean | null
  }
  /**设置窗口是否可调整大小 */
  set_resizable: {
    params: boolean
  }
  /**获取窗口是否可调整大小 */
  is_resizable: {
    response: boolean
  }
  /**设置启用的窗口按钮 */
  set_enabled_buttons: {
    params: WindowButton[]
  }
  /**获取启用的窗口按钮 */
  enabled_buttons: {
    response: WindowButton
  }
  /**设置窗口最小化 */
  set_minimized: {
    params: boolean
  }
  /**获取窗口最小化 */
  is_minimized: {
    response: boolean
  }
  /**最大化窗口 */
  set_maximized: {
    params: boolean
  }
  /**获取窗口最大化 */
  is_maximized: {
    response: boolean
  }
  /**获取显示器列表 */
  get_monitor_list: {
    response: Monitor[]
  }
  /**获取当前显示器id */
  current_monitor: {
    response: Monitor
  }
  /**获取主显示器id */
  primary_monitor: {
    response: Monitor
  }
  /**设置窗口全屏 */
  fullscreen: {
    params: null | undefined | Monitor['monitorId']
  }
  unfullscreen: {}
  /**获取窗口是否全屏 */
  is_fullscreen: {
    response: boolean | Monitor['monitorId']
  }
  /**打开或关闭窗户装饰 */
  set_decorations: {
    params: boolean
  }
  /**获取窗口装饰状态 */
  is_decorated: {
    response: boolean
  }
  /**设置窗口级别 */
  set_window_level: {
    params: WindowLevel
  }
  /**设置窗口图标 */
  set_window_icon: {
    params: string
  }
  /**设置窗口聚焦 */
  focus_window: {}
  /**获取窗口是否聚焦 */
  has_focus: {
    response: boolean
  }
  /**请求用户注意窗口，这对应用程序没有影响 */
  request_user_attention: {
    params: UserAttentionType | null
  }
  /**设置窗口主题 */
  set_theme: {
    params: Theme | 'default'
  }
  /**获取窗口主题 */
  theme: {
    response: Theme | null
  }
  /**使用鼠标左键移动窗口，直到该按钮被释放(调用前按下鼠标左键) */
  drag_window: {}
  /**使用鼠标左键调整窗口大小，直到该按钮被释放 */
  drag_resize_window: {
    params: ResizeDirection
  }
  /**在指定的位置显示窗口菜单 */
  show_window_menu: {
    params: Position
  }
}
/**窗口触发的事件 */
export interface WindowEvent {
  created: WindowId
  /**窗口被移动 */
  move: Position
  /**窗口被关闭 */
  close: void
  /**窗口被销毁 */
  destroy: void
  /**窗口失去焦点 */
  blur: void
  /**窗口获得焦点 */
  focus: void
  /**鼠标在窗口上移动 */
  cursorMove: Position
  /**鼠标进入窗口 */
  cursorEnter: void
  /**鼠标离开窗口 */
  cursorOut: void
  /**主题变更 */
  theme: Theme
  /**窗口被遮住(关闭、最小化、设置为不可见或被其他窗口遮挡) */
  occluded: boolean
  /**窗口大小变更 */
  resize: Size
}
export type MessageMethodKey = keyof MessageMethod
export type MessageMethodParams<T extends MessageMethodKey> = 'params' extends keyof MessageMethod[T] ? MessageMethod[T]['params'] : never
export type MessageMethodResponse<T extends MessageMethodKey> = 'response' extends keyof MessageMethod[T] ? MessageMethod[T]['response'] : never

/**向窗口进程发送的消息格式 */
export interface SendMessage<T extends MessageMethodKey> {
  id: string
  method: T
  label: string
  data?: MessageMethodParams<T>
}
/**接受窗口进程发送的消息格式 */
export interface ReceiveMessage<T> {
  id?: string
  type: T extends MessageMethodKey ? 'response' : 'windowEvent'
  method: T
  label: string
  data?: T extends MessageMethodKey ? MessageMethodResponse<T> : WindowEvent[T]
}
