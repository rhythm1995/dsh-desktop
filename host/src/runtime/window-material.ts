import type {
  DesktopPlatform,
  DesktopShellMode,
  DesktopWindowMaterial,
  MacosWindowMaterial,
  WindowsWindowMaterial,
} from './types.ts'
import { release as osRelease } from 'node:os'

export const DEFAULT_MACOS_WINDOW_MATERIAL: MacosWindowMaterial = 'transparent'
export const DEFAULT_WINDOWS_WINDOW_MATERIAL: WindowsWindowMaterial = 'acrylic'
export const WINDOWS_ACRYLIC_MIN_BUILD = 17_763
export const WINDOWS_ROUNDED_CORNERS_MIN_BUILD = 22_000
export const WINDOWS_MICA_MIN_BUILD = 22_621

export function windowsBuildNumber(value: string = osRelease()): number | undefined {
  const match = /^(?:\d+\.){2}(\d+)(?:\.|$)/.exec(value)
  if (match === null) return undefined
  const build = Number(match[1])
  return Number.isSafeInteger(build) ? build : undefined
}

export function windowsSupportsAcrylic(build: number | undefined): boolean {
  return build !== undefined && build >= WINDOWS_ACRYLIC_MIN_BUILD
}

export function windowsSupportsRoundedCorners(build: number | undefined): boolean {
  return build !== undefined && build >= WINDOWS_ROUNDED_CORNERS_MIN_BUILD
}

export function windowsSupportsSystemBackdrop(build: number | undefined): boolean {
  return build !== undefined && build >= WINDOWS_MICA_MIN_BUILD
}

export function windowsSupportsMica(build: number | undefined): boolean {
  return windowsSupportsSystemBackdrop(build)
}

/** Whether Acrylic must use the legacy transparent-window implementation. */
export function windowsUsesLegacyAcrylic(build: number | undefined): boolean {
  return windowsSupportsAcrylic(build) && !windowsSupportsRoundedCorners(build)
}

export function parseMacosWindowMaterial(value: unknown): MacosWindowMaterial {
  if (value === undefined) return DEFAULT_MACOS_WINDOW_MATERIAL
  if (value === 'off' || value === 'transparent') return value
  throw new Error('dsh-desktop.macosMaterial must be "off" or "transparent"')
}

export function parseWindowsWindowMaterial(value: unknown): WindowsWindowMaterial {
  if (value === undefined) return DEFAULT_WINDOWS_WINDOW_MATERIAL
  if (value === 'off' || value === 'acrylic' || value === 'mica') return value
  throw new Error('dsh-desktop.windowsMaterial must be "off", "acrylic", or "mica"')
}

export function effectiveDesktopWindowMaterial(
  mode: DesktopShellMode,
  platform: DesktopPlatform,
  macosMaterial: MacosWindowMaterial,
  windowsMaterial: WindowsWindowMaterial,
  windowsBuild: number | undefined,
): DesktopWindowMaterial {
  void mode
  if (platform === 'linux') return 'off'
  if (platform === 'darwin') return macosMaterial
  if (windowsMaterial === 'mica' && !windowsSupportsSystemBackdrop(windowsBuild)) {
    return windowsUsesLegacyAcrylic(windowsBuild) ? 'acrylic' : 'off'
  }
  if (windowsMaterial === 'acrylic') {
    return windowsSupportsSystemBackdrop(windowsBuild) || windowsUsesLegacyAcrylic(windowsBuild)
      ? 'acrylic'
      : 'off'
  }
  return windowsMaterial
}
