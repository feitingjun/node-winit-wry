import Window from './window'

/**根据label获取窗口 */
export const getWindow = (label: string):Window|undefined => {
  return globalThis.app?.windows[label]
}

export {
  Window
}