import { mkdtempSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { NATIVE_METHODS } from '../src/rpc/protocol.ts'
import { LoopbackRpcTransport } from '../src/rpc/transport.ts'
import { startLoopbackCarrier } from '../src/host/loopback.ts'
import { runHostMain } from '../src/main.ts'

describe('Host loopback carrier', () => {
  it('serves the desktop page on 127.0.0.1', async () => {
    const carrier = await startLoopbackCarrier('DSH Desktop')
    expect(carrier.origin.startsWith('http://127.0.0.1:')).toBe(true)
    const response = await fetch(carrier.url)
    expect(response.ok).toBe(true)
    expect(await response.text()).toContain('data-tauri-drag-region')
    await carrier.close()
  })

  it('mounts a shell generation against the native RPC', async () => {
    const methods: string[] = []
    const transport = new LoopbackRpcTransport(async method => {
      methods.push(method)
      if (method === NATIVE_METHODS.schedule) return { generationId: 'g-test' }
      if (method === NATIVE_METHODS.mount) return { ok: true }
      if (method === NATIVE_METHODS.release) return { ok: true }
      if (method === NATIVE_METHODS.writeBootstrap) return { ok: true }
      return {}
    })
    const userDataDir = mkdtempSync(join(tmpdir(), 'dsh-host-main-'))
    const handle = await runHostMain({
      transport,
      userDataDir,
      platform: 'darwin',
    })
    expect(handle.url.startsWith('http://127.0.0.1:')).toBe(true)
    expect(handle.profileName).toBe('desktop')
    expect(methods).toContain(NATIVE_METHODS.schedule)
    expect(methods).toContain(NATIVE_METHODS.mount)
    await handle.stop()
  })
})
