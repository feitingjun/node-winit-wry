import { platform, arch } from 'process'
/**生成一个id */
export const uid = () => {
  return new Date().getTime() + '' + Math.floor(Math.random() * 100000)
}

/**判断平台 */
export const getPlatform = () => {
  let SYS, ARCH
  switch (platform) {
    case 'darwin':
      SYS = 'apple-darwin'
      break
    case 'win32':
      SYS = 'pc-windows-gnu'
      break
    case 'linux':
      SYS = 'unknown-linux-gnu'
      break
    default:
      SYS = 'unknown-unknown'
  }
  switch (arch) {
    case 'x64':
      ARCH = 'x86_64'
      break
    case 'arm64':
      ARCH = 'aarch64'
      break
    default:
      ARCH = 'x86_64'
  }
  return ARCH + '-' + SYS
}