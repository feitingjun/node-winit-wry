import { Window } from './ts/index'

const window = new Window('window', {
  url: 'https://www.baidu.com'
})
window.onCreated((id) => {
  console.log('窗口创建成功')
})
window.on('created', (id) => {
  console.log('窗口创建成功')
})