import { createServer, type IncomingMessage, type Server, type ServerResponse } from 'node:http'
import type { AddressInfo } from 'node:net'
import { RENDERER_BOOT_REPORT_PATH } from './constants.ts'
import type { LogJournal } from '../runtime/log-journal.ts'
import type { NetworkJournal } from '../runtime/network-journal.ts'
import { renderDevtoolsPage } from './devtools-page.ts'

export const RENDERER_BOOT_PATH = RENDERER_BOOT_REPORT_PATH

export interface LoopbackDevtools {
  readonly enabled: boolean
  readonly logs?: LogJournal
  readonly network?: NetworkJournal
}

export interface LoopbackOptions {
  readonly pageTitle?: string
  readonly devtools?: LoopbackDevtools
}

export interface LoopbackCarrier {
  readonly url: string
  readonly origin: string
  readonly port: number
  close(): Promise<void>
}

function json(res: ServerResponse, status: number, value: unknown, cors: boolean): void {
  res.writeHead(status, {
    'content-type': 'application/json; charset=utf-8',
    ...(cors ? { 'access-control-allow-origin': '*' } : {}),
  })
  res.end(JSON.stringify(value))
}

function handleDevtools(
  url: URL,
  res: ServerResponse,
  devtools: LoopbackDevtools | undefined,
): boolean {
  if (!url.pathname.startsWith('/_dsh/dev')) return false
  if (devtools?.enabled !== true) {
    res.writeHead(404)
    res.end()
    return true
  }
  if (url.pathname === '/_dsh/dev/meta') {
    json(res, 200, {
      enabled: true,
      logsPath: devtools.logs?.filePath,
      networkPath: devtools.network?.filePath,
    }, true)
    return true
  }
  if (url.pathname === '/_dsh/dev/logs') {
    const q = url.searchParams.get('q') ?? ''
    const rawLevel = url.searchParams.get('level')
    const level = rawLevel === 'debug' || rawLevel === 'info' || rawLevel === 'warn' || rawLevel === 'error'
      ? rawLevel
      : undefined
    json(res, 200, {
      records: devtools.logs?.query({
        ...(q.length === 0 ? {} : { q }),
        ...(level === undefined ? {} : { level }),
        limit: Number(url.searchParams.get('limit') ?? 200) || 200,
      }) ?? [],
    }, true)
    return true
  }
  if (url.pathname === '/_dsh/dev/network') {
    const q = url.searchParams.get('q') ?? ''
    const method = url.searchParams.get('method') ?? ''
    json(res, 200, {
      records: devtools.network?.query({
        ...(q.length === 0 ? {} : { q }),
        ...(method.length === 0 ? {} : { method }),
        limit: Number(url.searchParams.get('limit') ?? 200) || 200,
      }) ?? [],
    }, true)
    return true
  }
  if (url.pathname === '/_dsh/dev/ui') {
    res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' })
    res.end(renderDevtoolsPage(url.searchParams.get('tab') === 'network' ? 'network' : 'logs'))
    return true
  }
  res.writeHead(404)
  res.end()
  return true
}

export function startLoopbackCarrier(input: string | LoopbackOptions = 'DSH Desktop'): Promise<LoopbackCarrier> {
  const options: LoopbackOptions = typeof input === 'string' ? { pageTitle: input } : input
  const pageTitle = options.pageTitle ?? 'DSH Desktop'
  const devtools = options.devtools
  return new Promise((resolve, reject) => {
    const server: Server = createServer((req: IncomingMessage, res: ServerResponse) => {
      const started = Date.now()
      const host = req.headers.host ?? '127.0.0.1'
      const url = new URL(req.url ?? '/', `http://${host}`)
      const finish = (status: number): void => {
        if (url.pathname.startsWith('/_dsh/dev')) return
        devtools?.network?.append({
          method: req.method ?? 'GET',
          url: url.href,
          status,
          durationMs: Date.now() - started,
        })
      }
      if (handleDevtools(url, res, devtools)) {
        return
      }
      if (url.pathname === '/health') {
        res.writeHead(200, { 'content-type': 'application/json; charset=utf-8' })
        res.end(JSON.stringify({ ok: true }))
        finish(200)
        return
      }
      if (url.pathname === RENDERER_BOOT_REPORT_PATH && req.method === 'POST') {
        res.writeHead(204)
        res.end()
        finish(204)
        return
      }
      if (url.pathname !== '/') {
        res.writeHead(404)
        res.end()
        finish(404)
        return
      }
      res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' })
      res.end(`<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>${pageTitle}</title>
    <style>
      html, body { margin: 0; height: 100%; background: #141414; color: #e8e8ea; font: 14px/1.45 ui-sans-serif, system-ui, sans-serif; }
      [data-tauri-drag-region] { height: 36px; display: flex; align-items: center; padding: 0 16px; user-select: none; }
      main { padding: 24px; }
      [data-dsh-workspace-drop-target] { min-height: 120px; border: 1px dashed #3a3a3c; border-radius: 10px; padding: 16px; }
    </style>
  </head>
  <body>
    <div data-tauri-drag-region>${pageTitle}</div>
    <main>
      <p>Host loopback is running. The official DSH Web client loads here when the upstream Host is composed.</p>
      <div data-dsh-workspace-drop-target>Drop one folder to add a workspace.</div>
    </main>
    <script>
      window.addEventListener('dsh-desktop-folder-drop', (event) => {
        const detail = event.detail || {}
        const paths = detail.paths || []
        const target = document.querySelector('[data-dsh-workspace-drop-target]')
        if (target) target.textContent = paths.length === 1 ? paths[0] : 'Drop exactly one folder'
        window.__DSH_DESKTOP_FILE_PATH__ = { getPathForFile: function () { return paths[0] || '' } }
      })
      document.addEventListener('click', (event) => {
        const node = event.target && event.target.closest ? event.target.closest('a[href]') : null
        if (!node) return
        const href = node.getAttribute('href') || ''
        if (href.startsWith('http') && !href.startsWith(location.origin)) {
          event.preventDefault()
        }
      })
    </script>
  </body>
</html>`)
      finish(200)
    })
    server.once('error', reject)
    server.listen(0, '127.0.0.1', () => {
      const address = server.address() as AddressInfo
      const origin = `http://127.0.0.1:${String(address.port)}`
      resolve({
        url: `${origin}/`,
        origin,
        port: address.port,
        close: () => new Promise((closeResolve, closeReject) => {
          server.close(error => { error ? closeReject(error) : closeResolve() })
        }),
      })
    })
  })
}
