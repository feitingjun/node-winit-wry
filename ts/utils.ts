import { platform, arch, cwd } from 'process'
import { existsSync, createWriteStream } from 'fs'
import { join } from 'path'
import { get } from 'https'
import { IncomingMessage } from 'http'

const URL = 'https://github.com/feitingjun/node-winit-wry/releases/latest/download'
const suffix = platform.indexOf('windows') > -1 ? '.exe' : ''

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
// 下载二进制文件
const writeFile = async (res: IncomingMessage, filename: string) => {
  return new Promise((resolve, reject) => {
    if(res.statusCode === 200){
      let stream = createWriteStream(join(__dirname, filename), { mode: 0o755 })
      res.pipe(stream)
      stream.on('finish', () => {
        stream.close();
        resolve(true)
      })
      stream.on('error', (err) => {
        console.error('写入文件错误:', err)
        reject(err)
      })
    }
  })
}
// 下载二进制文件
const getBinary = (url: string):Promise<IncomingMessage> => {
  return new Promise((resolve, reject) => {
    get(url, (res) => {
      if(res.statusCode === 200){
        resolve(res)
      }else if(res.statusCode === 302) {
        getBinary(res.headers.location).then(resolve).catch(reject)
      }else{
        console.error('下载二进制文件失败', res)
        reject(new Error('下载错误'))
      }
    }).on('error', (err) => {
      console.error('下载二进制文件失败', err)
      reject(err)
    })
  })
}
// 获取二进制文件路径
export const getBinaryPath = async () => {
  // 写入的本地二进制文件名称
  let filename = 'app'
  // @ts-ignore
  if (process.pkg){
    // @ts-ignore
    filename = process.pkg.name
  }else if(existsSync(join(cwd(), 'package.json'))){
    const path = join(cwd(), 'package.json')
    filename = (await import(path)).name
  }
  filename = filename + suffix
  // 判断文件是否存在
  if (!existsSync(join(__dirname, filename))){
    // 下载二进制文件
    const gitfilename = getPlatform()
    const url = `${URL}/${gitfilename}${suffix}`
    const res = await getBinary(url)
    await writeFile(res, filename)
  }
  return join(__dirname, filename)
}