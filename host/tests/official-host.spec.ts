import { existsSync } from 'node:fs'
import { spawnSync } from 'node:child_process'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { officialHostDisabled, officialHostEntry } from '../src/runtime/official-host.ts'
import { dshPluginDesktopRoot } from '../src/runtime/paths.ts'

describe('official dsh-plugin-desktop Host', () => {
  it('selects the compiled launcher unless DSH_OFFICIAL_HOST is off', () => {
    const root = dshPluginDesktopRoot()
    expect(officialHostDisabled({ DSH_OFFICIAL_HOST: '0' })).toBe(true)
    expect(officialHostEntry(root, { DSH_OFFICIAL_HOST: '0' })).toBeUndefined()
    expect(officialHostEntry(root, {})).toBe(join(root, 'lib/tauri-host.js'))
  })

  it('exposes startTauriHost from the compiled upstream launcher', () => {
    const root = dshPluginDesktopRoot()
    const entry = join(root, 'lib/tauri-host.js')
    expect(existsSync(entry), `missing ${entry}`).toBe(true)
    const result = spawnSync(process.execPath, ['--input-type=module', '-e', `
      import { pathToFileURL } from 'node:url'
      const loaded = await import(pathToFileURL(${JSON.stringify(entry)}).href)
      if (typeof loaded.startTauriHost !== 'function') process.exit(2)
    `], { encoding: 'utf8' })
    expect(result.status, result.stderr).toBe(0)
  })
})
