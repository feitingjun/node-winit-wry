import { ChildProcessWithoutNullStreams } from 'child_process'
import { uid, getBinaryPath } from './utils'
import { spawn } from 'child_process'
import { ReceiveMessage } from './types'
import Window from './window'

const IO_CHANNEL_PREFIX = '_ioc:';

export default class App {
  /**给窗口发送消息的回调 */
  callbacks: { [key: string]: Function } = {}
  /**监听窗口事件 */
  listeners: { [label: string]: {
    [key: string]: Function[]
  } } = {}
  /**窗口列表 */
  windows: { [key: string]: Window } = {}
  /**子进程 */
  childProcess: ChildProcessWithoutNullStreams

  async init(){
    if(this.childProcess) return
    const path = await getBinaryPath()
    this.childProcess = spawn(path, [], {})
    // 监听子进程消息
    this.childProcess.stdout.on('data', (data) => {
      let str:string = data.toString()
      str?.split('\n').forEach((item) => {
        if (!item) return
        let msg;
        try {
          msg = JSON.parse(item.replace(IO_CHANNEL_PREFIX, ''))
        } catch (e) {
          console.error(`响应消息格式错误：${item}`)
          return
        }
        this.handleIoMessage(msg)
      })
    })
    this.childProcess.stderr.on('data', (data) => {
      console.error(`错误：${data.toString()}`)
    })
  }
  // 处理子进程消息
  handleIoMessage(msg: ReceiveMessage){
    const instance = this.windows[msg.label]
    if (!instance) return
    switch (msg.type) {
      case 'response':
        const callback = this.callbacks[msg.id as string]
        if (callback) callback(msg.data)
        break
      case 'windowEvent':
        const listeners = this.listeners[msg.label][msg.method]??[]
        listeners.forEach(cb => cb(msg.data))
        break
    }
  }
  // 向子进程发送消息
  sendIoMessage(msg, callback) {
    const id = uid()
    msg.id = id
    this.callbacks[id] = callback
    this.childProcess.stdin.write(`${IO_CHANNEL_PREFIX}${JSON.stringify(msg)}` + '\n')
  }
  // 添加事件监听
  on(label, event, callback){
    if(!this.listeners[label]) this.listeners[label] = {}
    if(!this.listeners[label][event]) this.listeners[label][event] = []
    this.listeners[label][event].push(callback)
    return () => this.off(label, event, callback)
  }
  // 监听一次
  once(label, event, callback){
    this.on(label, event, (...args) => {
      callback(...args)
      this.off(label, event, callback)
    })
  }
  // 删除事件监听
  off(label, event, callback){
    if(!this.listeners[label]) return
    if(!this.listeners[label][event]) return
    this.listeners[label][event] = this.listeners[label][event].filter(item => item !== callback)
  }
}