import { describe, expect, it } from 'vitest'
import {
  ADVANCED_MACOS_TRAFFIC_LIGHT_TOP,
  ADVANCED_WINDOWS_TITLEBAR_HEIGHT,
  DESKTOP_FRAME_HEIGHT,
  DESKTOP_FRAME_MACOS_TRAFFIC_LIGHT_TOP,
  chromeGeometry,
} from '../src/runtime/window-chrome.ts'

describe('window chrome geometry', () => {
  it('keeps the 36px frame for compatibility and extended', () => {
    expect(chromeGeometry('compatibility')).toEqual({
      titlebarHeight: DESKTOP_FRAME_HEIGHT,
      macosTrafficLightTop: DESKTOP_FRAME_MACOS_TRAFFIC_LIGHT_TOP,
    })
    expect(chromeGeometry('extended').titlebarHeight).toBe(36)
  })

  it('uses the compact enhanced caption', () => {
    expect(chromeGeometry('advanced')).toEqual({
      titlebarHeight: ADVANCED_WINDOWS_TITLEBAR_HEIGHT,
      macosTrafficLightTop: ADVANCED_MACOS_TRAFFIC_LIGHT_TOP,
    })
  })
})
