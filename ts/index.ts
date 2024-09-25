import { spawn } from 'child_process'
import { join } from 'path'
import {
  SendMessage,
  ReceiveMessage,
  MessageMethodParams,
  MessageMethodResponse,
  WindowAttributes,
  MessageMethodKey,
  Position,
  WindowButton,
  Monitor,
  UserAttentionType,
  Theme,
  Size,
  ResizeDirection,
  WindowEvent,
  WindowId
} from './types'
import { ChildProcessWithoutNullStreams } from 'child_process'
import { uid, getPlatform } from './utils'

const IO_CHANNEL_PREFIX = '_ioc:';

export class Window {
  /**window子进程 */
  static childProcess: ChildProcessWithoutNullStreams
  childProcess: ChildProcessWithoutNullStreams
  /**给窗口发送消息的回调 */
  callbacks: { [key: string]: Function } = {}
  /**窗口唯一标识 */
  label: string
  /**监听窗口事件 */
  listeners: { [key: string]: Function[] } = {}
  /**
   * 创建窗口
   * @param label 窗口唯一标识
   * @param props 窗口属性
   */
  constructor(label: string, props?: WindowAttributes) {
    let platform = getPlatform()
    if (!Window.childProcess) {
      const path = join(__dirname, '../binary', platform)
      Window.childProcess = spawn(path, [], {})
    }
    this.childProcess = Window.childProcess
    this.label = label
    this.create(props);
    this.event()
  }
  /**创建窗口 */
  private async create(props?: WindowAttributes) {
    let id = await this.send('create', props)
    this.listeners['created']?.forEach((cb) => cb(id))
    return id
  }
  /**监听窗口事件 */
  on<T extends keyof WindowEvent>(event:T, callback:(data: WindowEvent[T]) => void){
    if(!this.listeners[event]){
      this.listeners[event] = []
    }
    this.listeners[event].push(callback)
    return () => this.off(event, callback)
  }
  /**监听一次窗口事件 */
  once(event:keyof WindowEvent, callback:Function){
    if(!this.listeners[event]){
      this.listeners[event] = []
    }
    this.listeners[event].push((...args) => {
      callback(...args)
      this.off(event, callback)
    })
  }
  /**窗口创建完成 */
  onCreated(callback:(id:WindowId) => void){
    return this.on('created', callback)
  }
  /**监听窗口移动 */
  onMove(callback:(data:Position) => void){
    return this.on('move', callback)
  }
  /**监听窗口关闭 */
  onClose(callback:() => void){
    return this.on('close', callback)
  }
  /**监听窗口销毁 */
  onDestroy(callback:() => void){
    return this.on('destroy', callback)
  }
  /**监听窗口失去焦点 */
  onBlur(callback:() => void){
    return this.on('blur', callback)
  }
  /**监听窗口获得焦点 */
  onFocus(callback:() => void){
    return this.on('focus', callback)
  }
  /**监听窗口光标移动 */
  onCursorMove(callback:(data:Position) => void){
    return this.on('cursorMove', callback)
  }
  /**监听窗口光标进入 */
  onCursorEnter(callback:() => void){
    return this.on('cursorEnter', callback)
  }
  /**监听窗口光标离开 */
  onCursorOut(callback:() => void){
    return this.on('cursorOut', callback)
  }
  /**监听窗口主题改变 */
  onTheme(callback:(data:Theme) => void){
    return this.on('theme', callback)
  }
  /**监听窗口被遮挡(关闭、最小化、设置为不可见或被其他窗口遮挡) */
  onOccluded(callback:(data:boolean) => void){
    return this.on('occluded', callback)
  }
  /**监听窗口大小改变 */
  onResize(callback:(data:Size) => void){
    return this.on('resize', callback)
  }
  /**取消监听窗口事件 */
  private off(event:keyof WindowEvent, callback:Function){
    if(!this.listeners[event]){
      return
    }
    this.listeners[event] = this.listeners[event].filter(item => item !== callback)
  }
  /**关闭窗口 */
  close() {
    return this.send('close')
  }
  /**设置webview URL */
  setUrl(url: string) {
    return this.send('set_url', url)
  }
  /**获取webview URL */
  url() {
    return this.send('url')
  }
  /**在webview上执行js代码 */
  evaluateScript(script: string) {
    return this.send('evaluate_script', script)
  }
  /**
   * 在webview上执行js代码，并且以json字符串形式返回执行结果
   * 
   * 例如 
   * window.evaluateScriptReturnResult('1+1') 将返回 '2'
   * */
  evaluateScriptReturnResult(script: string) {
    return this.send('evaluate_script_with_callback', script)
  }
  // /**打开调试工具 */
  // openDevtools() {
  //   return this.send('open_devtools')
  // }
  // /**关闭调试工具 */
  // closeDevtools() {
  //   return this.send('close_devtools')
  // }
  // /**调试工具是否打开 */
  // isDevtoolsOpen() {
  //   return this.send('is_devtools_open')
  // }
  /**设置webview缩放等级 */
  zoom(scale: number) {
    return this.send('zoom', scale)
  }
  /**获取当前窗口的缩放因子(屏幕的缩放因子可以通过monitors方法获取) */
  scaleFactor() {
    return this.send('scale_factor')
  }
  /**清除webview浏览数据 */
  clear_all_browsing_data() {
    return this.send('clear_all_browsing_data')
  }
  /**设置webview背景色(macOS/iOS不支持) */
  setBackgroundColor(color: [number, number, number, number]) {
    return this.send('set_background_color', color)
  }
  /**返回窗口客户区左上角相对于桌面左上角的位置 */
  innerPosition() {
    return this.send('inner_position')
  }
  /**返回窗口左上角相对于桌面左上角的位置 */
  outerPosition() {
    return this.send('outer_position')
  }
  /**设置窗口左上角相对于桌面左上角的位置 */
  setOuterPosition(pos: Position) {
    return this.send('set_outer_position', pos)
  }
  /**返回窗口客户端区域的物理大小(不包括标题栏和边框) */
  innerSize() {
    return this.send('inner_size')
  }
  /**返回整个窗口的物理大小 */
  outerSize() {
    return this.send('outer_size')
  }
  /**设置窗口的最小尺寸 */
  setMinInnerSize(size: Size) {
    return this.send('set_min_inner_size', size)
  }
  /**设置窗口的最大尺寸 */
  setMaxInnerSize(size: Size) {
    return this.send('set_max_inner_size', size)
  }
  /**设置窗口标题 */
  setTitle(title: string) {
    return this.send('set_title', title)
  }
  /**返回窗口标题 */
  title() {
    return this.send('title')
  }
  /**设置窗口是否透明 */
  setTransparent(transparent: boolean = true) {
    return this.send('set_transparent', transparent)
  }
  /**更改窗口模糊状态 */
  setBlur(blur: boolean = true) {
    return this.send('set_blur', blur)
  }
  /**修改窗口的可见性 */
  setVisible(visible: boolean) {
    return this.send('set_visible', visible)
  }
  /**获取窗口是否可见 */
  isVisible() {
    return this.send('is_visible')
  }
  /**设置窗口是否可调整大小 */
  setResizable(resizable: boolean) {
    return this.send('set_resizable', resizable)
  }
  /**获取窗口是否可调整大小 */
  isResizable() {
    return this.send('is_resizable')
  }
  /**设置启用的窗口按钮 */
  setEnabledButtons(buttons: WindowButton[] = ['close', 'maximize', 'minimize']) {
    return this.send('set_enabled_buttons', buttons)
  }
  /**获取启用的窗口按钮 */
  enabledButtons() {
    return this.send('enabled_buttons')
  }
  /**窗口最小化 */
  minimized() {
    return this.send('set_minimized', true)
  }
  /**窗口取消最小化 */
  unminimized() {
    return this.send('set_minimized', false)
  }
  /**获取窗口是否最小化 */
  isMinimized() {
    return this.send('is_minimized')
  }
  /**窗口最大化 */
  maximized() {
    return this.send('set_maximized', true)
  }
  /**窗口取消最大化 */
  unmaximized() {
    return this.send('set_maximized', false)
  }
  /**获取窗口是否最大化 */
  isMaximized() {
    return this.send('is_maximized')
  }
  /**获取显示器列表(返回显示器id列表) */
  monitors() {
    return this.send('get_monitor_list')
  }
  /**获取当前显示器id */
  currentMonitor() {
    return this.send('current_monitor')
  }
  /**获取主显示器id */
  primaryMonitor() {
    return this.send('primary_monitor')
  }
  /**
   * 设置窗口全屏
   * 
   ** monitorId 为 null | undefined 时在当前窗口全屏
   ** 传入monitorId则在指定显示器全屏
   ** monitorId通过monitors方法获取
  */
  fullscreen(monitorId?: null | Monitor['monitorId']) {
    return this.send('fullscreen', monitorId ?? null)
  }
  /**取消全屏 */
  unfullscreen() {
    return this.send('unfullscreen')
  }
  /**
   * 获取窗口是否全屏
   * 
   ** 返回 true 表示在当前窗口全屏
   ** 返回 显示器id 表示在指定显示器全屏
   ** 返回 false 表示未全屏
   */
  isFullscreen() {
    return this.send('is_fullscreen')
  }
  /**设置窗口是否无边框，默认 true 为无边框 */
  borderless(borderless: boolean = true) {
    return this.send('set_decorations', !borderless)
  }
  /**获取是否是无边框窗口 */
  isBorderless() {
    return this.send('is_decorated')
  }
  /**是否置顶窗口 */
  setAlwaysOnTop(top: boolean = true) {
    return this.send('set_window_level', top ? 'alwaysOnTop' : 'normal')
  }
  /**是否置底窗口 */
  setAlwaysOnBottom(bottom: boolean = true) {
    return this.send('set_window_level', bottom ? 'alwaysOnBottom' : 'normal')
  }
  /**设置窗口图标(仅支持Windows和X11) */
  setIcon(icon: string) {
    return this.send('set_window_icon', icon)
  }
  /**窗口聚焦 */
  focus() {
    return this.send('focus_window')
  }
  /**获取窗口是否聚焦 */
  isFocus() {
    return this.send('has_focus')
  }
  /**
   * 请求用户注意窗口(如果应用程序当前已经是焦点位置，则无效)

   * type = critical
   ** Windows：同时闪烁窗口和任务栏按钮，直到应用程序进入焦点
   ** MacOS：弹出dock图标，直到应用程序处于焦点位置

   * type = informational
   ** Windows：闪烁任务栏按钮，直到应用程序处于焦点位置
   ** MacOS：弹出dock图标，弹出一次dock图标
   * */
  requestUserAttention(type: UserAttentionType = 'critical') {
    return this.send('request_user_attention', type)
  }
  /**取消请求用户关注（macOS不支持） */
  cancelUserAttentionRequest() {
    return this.send('request_user_attention', null)
  }
  /**设置窗口主题
   ** light 为亮色主题
   ** dark 为暗色主题
   ** default 为系统默认值
  */
  setTheme(theme: Theme | 'default') {
    return this.send('set_theme', theme)
  }
  /**
   * 获取窗口主题
   ** null 表示当前系统无法确定主题 
  */
  theme() {
    return this.send('theme')
  }
  /**使用鼠标左键移动窗口，直到该按钮被释放(调用前鼠标左键需处于按下状态) */
  dragWindow() {
    return this.send('drag_window')
  }
  /**
   * 使用鼠标左键调整窗口大小，直到该按钮被释放(调用前鼠标左键需处于按下状态)
   ** MacOS 不支持 
   */
  dragResizeWindow(direction: ResizeDirection) {
    return this.send('drag_resize_window', direction)
  }
  /**在指定位置显示窗口菜单(仅支持Windows) */
  showMenu(position: Position) {
    return this.send('show_window_menu', position)
  }
  private send<T extends MessageMethodKey>(method: T, data?: MessageMethodParams<T>): Promise<MessageMethodResponse<T> extends never ? void : MessageMethodResponse<T>> {
    return new Promise((resolve, reject) => {
      const id = uid()
      const callback = (data: MessageMethodResponse<T> extends never ? void : MessageMethodResponse<T>) => {
        resolve(data)
      }
      this.callbacks[id] = callback
      this.emit({ id, method, data, label: this.label })
    })
  }
  private emit<T extends MessageMethodKey>(msg: SendMessage<T>) {
    this.childProcess.stdin.write(`${IO_CHANNEL_PREFIX}${JSON.stringify(msg)}` + '\n')
  }
  private event() {
    this.childProcess.stdout.on('data', (data) => {
      let str:string = data.toString()
      str?.split('\n').forEach((item) => {
        if (!item) return
        try {
          let msg: ReceiveMessage<MessageMethodKey|keyof WindowEvent> = JSON.parse(item.replace(IO_CHANNEL_PREFIX, ''))
          if (msg.label !== this.label) return
          // 处理响应
          if (msg.type === 'response'){
            const callback = this.callbacks[msg.id]
            if (callback) callback(msg.data)
          }
          // 处理窗口事件
          else if(msg.type === 'windowEvent'){
            const callbacks = this.listeners[msg.method]??[]
            callbacks.forEach(cb => cb(msg.data))
          }
        } catch (e) {
          console.error(`响应消息格式错误：${item}`)
        }
      })
    })
  }
}