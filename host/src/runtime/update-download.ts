import { mkdirSync, writeFileSync } from 'node:fs'
import { dirname } from 'node:path'
import { DESKTOP_VERSION_ENDPOINT } from './update-checker.ts'

export const DESKTOP_DOWNLOAD_URLS = {
  darwin: 'https://www.dshdesktop.cn/api/downloads/mac',
  win32: 'https://www.dshdesktop.cn/api/downloads/windows',
} as const

export const MAX_UPDATE_DOWNLOAD_BYTES = 1024 * 1024 * 1024

export type UpdateArtifactRequest = (url: string, init: RequestInit) => Promise<Response>

export class UpdateDownloadError extends Error {
  readonly code: 'empty-body' | 'http-status' | 'network' | 'response-too-large'
  constructor(code: UpdateDownloadError['code'], message: string) {
    super(message)
    this.name = 'UpdateDownloadError'
    this.code = code
  }
}

export async function downloadDesktopUpdate(options: {
  readonly platform: 'darwin' | 'win32'
  readonly version: string
  readonly destinationPath: string
  readonly request: UpdateArtifactRequest
  readonly signal?: AbortSignal
}): Promise<string> {
  let response: Response
  try {
    response = await options.request(DESKTOP_DOWNLOAD_URLS[options.platform], {
      method: 'GET',
      redirect: 'error',
      ...(options.signal === undefined ? {} : { signal: options.signal }),
    })
  } catch {
    throw new UpdateDownloadError('network', 'update download failed')
  }
  if (response.status !== 200) {
    throw new UpdateDownloadError('http-status', `unexpected status ${String(response.status)}`)
  }
  const bytes = Buffer.from(await response.arrayBuffer())
  if (bytes.length === 0) throw new UpdateDownloadError('empty-body', 'installer body was empty')
  if (bytes.length > MAX_UPDATE_DOWNLOAD_BYTES) {
    throw new UpdateDownloadError('response-too-large', 'installer exceeded 1 GiB')
  }
  mkdirSync(dirname(options.destinationPath), { recursive: true })
  writeFileSync(options.destinationPath, bytes)
  return options.destinationPath
}

void DESKTOP_VERSION_ENDPOINT
