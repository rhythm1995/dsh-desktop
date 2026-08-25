import { describe, expect, it } from 'vitest'
import { NATIVE_EVENTS, NATIVE_METHODS } from '../src/rpc/protocol.ts'
import { LoopbackRpcTransport } from '../src/rpc/transport.ts'
import { IpcDesktopRuntime } from '../src/runtime/ipc-desktop-runtime.ts'
import type { DesktopShellPayload } from '../src/runtime/types.ts'

function payload(url = 'http://127.0.0.1:9/'): DesktopShellPayload {
  return {
    mode: 'compatibility',
    macosMaterial: 'transparent',
    windowsMaterial: 'acrylic',
    width: 1280,
    height: 800,
    minWidth: 900,
    minHeight: 600,
    material: 'transparent',
    url,
    productName: 'DSH Desktop',
    windowTitle: 'DSH Desktop',
    iconPath: '/tmp/icon.png',
    trayIcons: { templatePath: '/tmp/t.png', bluePath: '/tmp/b.png' },
    locale: 'en',
    themeSource: 'system',
  }
}

describe('IpcDesktopRuntime generation lifecycle', () => {
  it('schedules, mounts, then releases exactly one generation', async () => {
    const calls: string[] = []
    const transport = new LoopbackRpcTransport(async (method, params) => {
      calls.push(method)
      if (method === NATIVE_METHODS.schedule) return { generationId: 'g1' }
      if (method === NATIVE_METHODS.mount) {
        expect(params).toEqual({ generationId: 'g1' })
        return { ok: true }
      }
      if (method === NATIVE_METHODS.release) {
        expect(params).toEqual({ generationId: 'g1' })
        return { ok: true }
      }
      return {}
    })
    const runtime = new IpcDesktopRuntime(transport, { platform: 'darwin' })
    const release = runtime.schedule(payload())
    let interactive = false
    await runtime.mountScheduled(() => { interactive = true })
    expect(interactive).toBe(true)
    await release()
    expect(calls).toEqual([NATIVE_METHODS.schedule, NATIVE_METHODS.mount, NATIVE_METHODS.release])
    runtime.dispose()
  })

  it('serializes DesktopShellSpec functions into native schedule params', async () => {
    const urls: unknown[] = []
    const transport = new LoopbackRpcTransport(async (method, params) => {
      if (method === NATIVE_METHODS.schedule) {
        urls.push((params as { url?: string }).url)
        return { generationId: 'g-spec' }
      }
      return { ok: true }
    })
    const runtime = new IpcDesktopRuntime(transport, { platform: 'darwin' })
    runtime.schedule({
      ...payload('http://127.0.0.1:4242/'),
      readLocalePreference: () => 'zh',
      readThemeSource: () => 'dark',
      requestQuit: () => undefined,
      requestModeChange: async () => undefined,
    })
    await runtime.mountScheduled()
    expect(urls).toEqual(['http://127.0.0.1:4242/'])
    runtime.dispose()
  })

  it('rejects a second schedule while a generation is live', () => {
    const transport = new LoopbackRpcTransport(async () => ({ generationId: 'g1' }))
    const runtime = new IpcDesktopRuntime(transport, { platform: 'darwin' })
    runtime.schedule(payload())
    expect(() => runtime.schedule(payload())).toThrow('already registered')
    runtime.dispose()
  })

  it('invokes a tray item when the native shell emits trayInvoke', async () => {
    const invoked: string[] = []
    const transport = new LoopbackRpcTransport(async () => ({}))
    const runtime = new IpcDesktopRuntime(transport, { platform: 'darwin' })
    runtime.registerTrayItem({
      group: 'tools',
      order: 1,
      label: () => 'Open Terminal',
      invoke: () => { invoked.push('terminal') },
    })
    transport.emit(NATIVE_EVENTS.trayInvoke, { id: 'tray-1' })
    await Promise.resolve()
    expect(invoked).toEqual(['terminal'])
    runtime.dispose()
  })
})
