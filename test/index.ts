import { Window, getWindow } from '../ts/index'

const win = new Window('main', {
  url: `file:${__dirname}/index.html`,
})

win.onCreated((id) => {
  getWindow('main')?.maximized()
})