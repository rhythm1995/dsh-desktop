import { mkdirSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import type { DesktopPlatform, DesktopTerminalSpec } from './types.ts'

export interface TerminalLaunchRecord {
  readonly profileName: string
  readonly command: readonly string[]
  readonly recordPath: string
  readonly scriptPath: string
}

export function terminalCommand(platform: DesktopPlatform, scriptPath: string): string[] {
  if (platform === 'darwin') return ['open', '-a', 'Terminal', scriptPath]
  if (platform === 'win32') return ['cmd', '/c', 'start', '', scriptPath]
  return ['x-terminal-emulator', '-e', scriptPath]
}

export function recordTerminalLaunch(
  userDataDir: string,
  platform: DesktopPlatform,
  spec: DesktopTerminalSpec,
): TerminalLaunchRecord {
  const dir = join(userDataDir, 'terminal-launches')
  mkdirSync(dir, { recursive: true })
  const stamp = String(Date.now())
  const scriptPath = join(dir, platform === 'win32' ? `${stamp}.cmd` : `${stamp}.sh`)
  const script = platform === 'win32'
    ? `@echo off\r\ncd /d ${spec.profileDir}\r\nset DSH_HOME=${spec.homeDir}\r\n`
    : `#!/bin/sh\ncd ${JSON.stringify(spec.profileDir)}\nexport DSH_HOME=${JSON.stringify(spec.homeDir)}\nexec "$SHELL"\n`
  writeFileSync(scriptPath, script)
  const command = terminalCommand(platform, scriptPath)
  const recordPath = join(dir, `${stamp}.json`)
  writeFileSync(recordPath, `${JSON.stringify({
    profileName: spec.profileName,
    profileDir: spec.profileDir,
    homeDir: spec.homeDir,
    command,
    scriptPath,
  }, undefined, 2)}\n`)
  return { profileName: spec.profileName, command, recordPath, scriptPath }
}
