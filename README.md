# node-winit-wry
在node端使用winit和wry创建webview窗口

## 创建窗口
new Window(props: [WindowAttributes](#windowattributes))

## 方法
所有方法均为异步

### on
注册窗口事件监听   
[WindowEvent](#windowevent)内的所有方法都有别名，如winodw.on('close')可以使用window.onClose
> on\<T extends keyof [WindowEvent](#windowevent)\>(event:T, callback:(data: [WindowEvent](#windowevent)[T]) => void): void

### once
注册只触发一次的窗口事件监听
> once<T extends keyof [WindowEvent](#windowevent)>(event:T, callback:(data: [WindowEvent](#windowevent)[T]) => void): void

### close
关闭窗口
> close(): void

### setUrl
设置webview的URL
> setUrl(url: string): Promise\<void\>

### url
获取webview的URL
> url(): string

### evaluateScript
在webview内执行js代码
> evaluateScript(script: string): Promise\<void\>

### evaluateScriptReturnResult
在webview内执行js代码，并且以json字符串形式返回执行结果
> evaluateScriptReturnResult(script: string): Promise\<string\>

### zoom
设置webview的缩放
> zoom(scale: number): Promise\<void\>

### scaleFactor
获取当前窗口的缩放因子(屏幕的缩放因子可以通过[monitors](#monitors)方法获取)
> scaleFactor(): Promise\<number\>

### clearAllBrowsingData
清除webview浏览数据 
> clearAllBrowsingData(): Promise\<void\>

### setBackgroundColor
设置webview背景色(macOS不支持)
> setBackgroundColor(color: [number, number, number, number]): Promise\<void\>

### innerPosition
返回窗口客户区域(不包含边框和标题栏)左上角相对于桌面左上角的位置
> innerPosition(): Promise\<[Position](#position)\>

### outerPosition
返回窗口左上角相对于桌面左上角的位置
> outerPosition(): Promise\<[Position](#position)\>

### setOuterPosition
设置窗口左上角相对于桌面左上角的位置
> setOuterPosition(pos:[Position](#position)): Promise\<void\>

### innerSize
返回窗口客户端区域的物理大小(不包括标题栏和边框)
> innerSize(): Promise\<[Size](#size)\>

### outerSize
返回整个窗口的物理大小
> outerSize(): Promise\<[Size](#size)\>

### setMinInnerSize
设置窗口的最小尺寸
> setMinInnerSize(size:[Size](#size)): Promise\<void\>

### setMaxInnerSize
设置窗口的最大尺寸
> setMaxInnerSize(size:[Size](#size)): Promise\<void\>

### setTitle
设置窗口标题
> setTitle(title: string): Promise\<void\>

### title
返回窗口标题
> title(): Promise\<string\>

### setTransparent
设置窗口是否透明(如果要实现透明窗口效果需要将webview的背景也设置为透明)
> setTransparent(transparent: boolean = true): Promise\<void\>

### setBlur
更改窗口模糊状态(透明窗口毛玻璃效果)
> setBlur(blur: boolean = true): Promise\<void\>

### setVisible
修改窗口的可见性
> setVisible(visible: boolean): Promise\<void\>

### isVisible
获取窗口的可见性
> isVisible(): Promise\<boolean\>

### setResizable
设置窗口是否可调整大小
> setResizable(resizable: boolean): Promise\<void\>

### isResizable
获取窗口是否可调整大小
> isResizable(): Promise\<boolean\>

### setEnabledButtons
设置启用的窗口按钮
> setEnabledButtons(buttons: [WindowButton](#windowbutton)[] = ['close', 'maximize', 'minimize']): Promise\<void\>

### enabledButtons
获取启用的窗口按钮
> enabledButtons(): Promise\<[WindowButton](#windowbutton)[]\>

### minimized
最小化窗口
> minimized(): Promise\<void\>

### unminimized
窗口取消最小化
> unminimized(): Promise\<void\>

### isMinimized
获取窗口是否最小化
> isMinimized(): Promise\<boolean\>

### minimized
窗口最大化
> maximized(): Promise\<void\>

### unmaximized
窗口取消最小化
> unmaximized(): Promise\<void\>

### isMaximized
获取窗口是否最大化
> isMaximized(): Promise\<boolean\>

### monitors
获取显示器列表
> monitors(): Promise\<[Monitor](#monitor)[]\>

### currentMonitor
获取当前显示器
> currentMonitor(): Promise\<[Monitor](#monitor)\>

### primaryMonitor
获取主显示器
> primaryMonitor(): Promise\<[Monitor](#monitor)\>

### fullscreen
设置窗口全屏   
monitorId 为 null | undefined 时在当前窗口全屏  
传入monitorId则在指定显示器全屏  
monitorId可通过monitors方法获取  
> fullscreen(monitorId?: null | <[Monitor['monitorId']](#monitor)): Promise\<void\>

### unfullscreen
取消全屏
> unfullscreen(): Promise\<void\>

### isFullscreen
获取窗口是否全屏   
返回 true 表示在当前窗口全屏  
返回 monitorId 表示在指定显示器全屏  
返回 false 表示未全屏
> isFullscreen(): Promise\<[Monitor['monitorId']](#monitor) | boolean\>

### borderless
设置窗口是否无边框，默认 true 为无边框
borderless(borderless: boolean = true): Promise\<void\>

### isBorderless
获取是否是无边框窗口
> isBorderless(): Promise\<boolean\>

### setAlwaysOnTop
设置是否置顶窗口
> setAlwaysOnTop(top: boolean = true): Promise\<void\>

### setAlwaysOnBottom
设置是否置底窗口
> setAlwaysOnBottom(bottom: boolean = true): Promise\<void\>

### setIcon
设置窗口图标(仅支持Windows和X11)
> setIcon(icon: string): Promise\<void\>

### focus
聚焦窗口
> focus(): Promise\<void\>

### isFocus
获取窗口是否聚焦
> isFocus(): Promise\<boolean\>

### requestUserAttention
请求用户注意窗口(如果应用程序当前已经是焦点位置，则无效)
> requestUserAttention(type: [UserAttentionType](#userattentiontype) = 'critical'): Promise\<void\>

### cancelUserAttentionRequest
取消请求用户关注（macOS不支持）
> cancelUserAttentionRequest(): Promise\<void\>

### setTheme
设置窗口主题
> setTheme(theme: [Theme](#theme-1) | 'default'): Promise\<void\>

### theme
获取窗口主题   
null 表示当前系统无法确定主题 
> theme(): Promise\<[Theme](#theme-1)|null\>

### dragWindow
使用鼠标左键移动窗口，直到该按钮被释放(调用前鼠标左键需处于按下状态)
> dragWindow(): Promise\<void\>

### dragResizeWindow
使用鼠标左键调整窗口大小，直到该按钮被释放(调用前鼠标左键需处于按下状态)  
MacOS 不支持
> dragResizeWindow(direction: [ResizeDirection](#resizedirection)): Promise\<void\>

### showMenu
在指定位置显示窗口菜单(仅支持Windows)
> showMenu(pos: [Position](#position)): Promise\<void\>

## 类型

### WindowId
```
type WindowId = number
```

### Size
```
Size = {
  width: number
  height: number
}
```

### Position
```
type Position = {
  x: number
  y: number
}
```

### Monitor
```
type Monitor = {
  monitorId: number
  width: number
  height: number
  scaleFactor: number
}
```

### WindowButton
```
type WindowButton = 'close' | 'minimize' | 'maximize'
```

### Theme
```
type Theme = 'light' | 'dark'
```

### WindowLevel
```
WindowLevel = 'normal' | 'alwaysOnTop' | 'alwaysOnBottom'
```

### UserAttentionType
```
/**
 * 请求用户注意窗口
 *
 * critical
 ** Windows 同时闪烁窗口和任务栏按钮，直到应用程序处于焦点位置
 ** MacOS 弹出dock图标，直到应用程序处于焦点位置
 *
 * informational
 ** Windows 闪烁任务栏按钮，直到应用程序处于焦点位置
 ** MacOS 弹出dock图标，弹出一次dock图标
*/
type UserAttentionType = 'critical' | 'informational'
```

### ResizeDirection
```
type ResizeDirection = 'east' | 'north' | 'northEast' | 'northWest' | 'south' | 'southEast' | 'southWest' | 'west'
```

### WindowAttributes
```
/**创建窗口的参数 */
interface WindowAttributes {
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
```

### WindowEvent
```
interface WindowEvent {
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
```