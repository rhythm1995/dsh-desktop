import { NATIVE_EVENTS, NATIVE_METHODS } from '../rpc/protocol.ts'
import type { RpcTransport } from '../rpc/transport.ts'
import type {
  DesktopDialogOptions,
  DesktopDialogResult,
  DesktopLocale,
  DesktopNotification,
  DesktopPlatform,
  DesktopShellMode,
  DesktopShellPayload,
  DesktopShellSpec,
  DesktopTerminalSpec,
  DesktopThemeSource,
  DesktopTrayItemGroup,
  DesktopTrayItemSnapshot,
  DesktopTraySubmenuSnapshot,
  ProfileCreateWindowOptions,
  RendererBootReport,
} from './types.ts'
import { DesktopRendererHealthGate, type RendererHealthFailureReason, type RendererHealthVerdict } from './renderer-health.ts'

export interface DesktopTraySubmenuItem {
  label(): string
  type?: 'normal' | 'checkbox' | 'radio'
  enabled?(): boolean
  checked?(): boolean
  invoke(): void | Promise<void>
}

export interface DesktopTrayItem {
  group: DesktopTrayItemGroup
  order: number
  label(): string
  enabled?(): boolean
  invoke(): void | Promise<void>
  submenu?(): readonly DesktopTraySubmenuItem[]
}

export interface DesktopTrayItemRegistration {
  refresh(): void
  dispose(): void
}

export interface DesktopUpdateAdapter {
  readonly isPackaged: boolean
  readonly canDownload: boolean
  readonly currentVersion: string
  readonly statePath: string
  readonly request: (url: string, init: RequestInit) => Promise<Response>
  confirmDownload(version: string): Promise<boolean>
  showManualCheckResult(result: { status: string, currentVersion: string, latestVersion?: string } | null): Promise<void>
  downloadAndOpen(version: string, signal: AbortSignal): Promise<void>
  notify(notification: DesktopNotification): void
}

export interface DesktopRuntime {
  readonly platform: DesktopPlatform
  readonly windowsBuild: number | undefined
  readonly locale: DesktopLocale
  readonly updates: DesktopUpdateAdapter
  schedule(spec: DesktopShellPayload | DesktopShellSpec): () => Promise<void>
  mountScheduled(beforeInteractive?: () => void): Promise<void>
  show(): void
  notifyAttention(notification: DesktopNotification): void
  registerTrayItem(item: DesktopTrayItem): DesktopTrayItemRegistration
  openTerminal(): void
  reloadRenderer(): void
  toggleDeveloperTools(): void
  exportDiagnostics(): Promise<void>
  pickDirectory(): Promise<string | null>
  validateDirectory(path: string): Promise<boolean>
  openProfileCreateWindow(options: ProfileCreateWindowOptions): void
  reportRendererBoot(report: RendererBootReport): void
  setLocalePreference(preference: DesktopLocale | undefined): void
  setThemeSource(source: DesktopThemeSource): void
  requestRestart(): Promise<void>
  requestRecoveryRestart(): Promise<void>
  prepareToQuit(): void
  openDialog(options: DesktopDialogOptions): Promise<DesktopDialogResult>
  openRecovery(state: unknown): Promise<void>
  openProfileCreate(state: unknown): void
  openDevtools(options: { tab: 'logs' | 'network', origin: string }): void
  configureTerminal(spec: DesktopTerminalSpec): void
  beginRendererBootMonitoring(
    options: { commitHealthy: () => Promise<void> },
    timeoutMs?: number,
  ): Promise<RendererHealthVerdict>
  stopRendererBootMonitoring(): void
  readonly rendererBootFailureReason: RendererHealthFailureReason | undefined
}

interface RegisteredTray {
  readonly id: string
  readonly item: DesktopTrayItem
  readonly submenu: DesktopTraySubmenuItem[]
}

export class IpcDesktopRuntime implements DesktopRuntime {
  readonly platform: DesktopPlatform
  readonly windowsBuild: number | undefined
  readonly updates: DesktopUpdateAdapter
  locale: DesktopLocale = 'en'
  private scheduled: DesktopShellPayload | undefined
  private scheduledSpec: DesktopShellSpec | undefined
  private mountTask: Promise<void> | undefined
  private generationId: string | undefined
  private terminalSpec: DesktopTerminalSpec | undefined
  private readonly tray = new Map<string, RegisteredTray>()
  private traySeq = 1
  private readonly unsubscribers: Array<() => void> = []
  private rendererHealthGate: DesktopRendererHealthGate | undefined
  private profileCreate?: ProfileCreateWindowOptions

  constructor(
    private readonly transport: RpcTransport,
    options: {
      platform: DesktopPlatform
      windowsBuild?: number
      locale?: DesktopLocale
      userDataDir?: string
      isPackaged?: boolean
      currentVersion?: string
    } = {
      platform: process.platform === 'darwin' || process.platform === 'win32' || process.platform === 'linux'
        ? process.platform
        : 'linux',
    },
  ) {
    this.platform = options.platform
    this.windowsBuild = options.windowsBuild
    if (options.locale !== undefined) this.locale = options.locale
    const userDataDir = options.userDataDir ?? ''
    const currentVersion = options.currentVersion ?? '0.1.0'
    const isPackaged = options.isPackaged ?? false
    this.updates = {
      isPackaged,
      canDownload: this.platform === 'darwin' || this.platform === 'win32',
      currentVersion,
      statePath: `${userDataDir}/update-state.json`,
      request: (url, init) => fetch(url, init),
      confirmDownload: async version => {
        const result = await this.openDialog({
          type: 'info',
          title: 'Update available',
          message: version,
          buttons: ['Download', 'Later'],
          defaultId: 1,
          cancelId: 1,
        })
        return result.response === 0
      },
      showManualCheckResult: async result => {
        await this.transport.request(NATIVE_METHODS.showUpdateResult, result ?? { status: 'failed' })
      },
      downloadAndOpen: async (version, _signal) => {
        await this.transport.request(NATIVE_METHODS.downloadUpdate, { version })
      },
      notify: notification => { this.native(NATIVE_METHODS.notify, notification) },
    }
    this.unsubscribers.push(transport.on(NATIVE_EVENTS.trayInvoke, params => {
      void this.handleTrayInvoke(params)
    }))
  }

  get rendererBootFailureReason(): RendererHealthFailureReason | undefined {
    return this.rendererHealthGate?.failureReason
  }

  beginRendererBootMonitoring(
    options: { commitHealthy: () => Promise<void> },
    timeoutMs = 30_000,
  ): Promise<RendererHealthVerdict> {
    if (this.rendererHealthGate !== undefined) {
      throw new Error('dsh-desktop: renderer boot monitoring already started')
    }
    const gate = new DesktopRendererHealthGate(options)
    this.rendererHealthGate = gate
    return gate.begin(timeoutMs)
  }

  stopRendererBootMonitoring(): void {
    this.rendererHealthGate?.stop()
  }

  schedule(spec: DesktopShellPayload | DesktopShellSpec): () => Promise<void> {
    if (this.scheduled !== undefined || this.mountTask !== undefined) {
      throw new Error('dsh-desktop: a native shell generation is already registered')
    }
    const payload = this.serializablePayload(spec)
    this.scheduled = payload
    this.scheduledSpec = 'requestModeChange' in spec ? spec : undefined
    let disposed = false
    return async () => {
      if (disposed) return
      disposed = true
      try {
        await this.mountTask
      } finally {
        if (this.generationId !== undefined) {
          await this.transport.request(NATIVE_METHODS.release, { generationId: this.generationId })
        }
        this.generationId = undefined
        this.mountTask = undefined
        if (this.scheduled === payload) {
          this.scheduled = undefined
          this.scheduledSpec = undefined
        }
      }
    }
  }

  mountScheduled(beforeInteractive?: () => void): Promise<void> {
    const spec = this.scheduled
    if (spec === undefined) {
      return Promise.reject(new Error('dsh-desktop: the Host did not register a window'))
    }
    if (this.mountTask === undefined) {
      const live = this.scheduledSpec
      if (live !== undefined) {
        this.setLocalePreference(live.readLocalePreference())
        this.setThemeSource(live.readThemeSource())
        this.registerTrayItem({
          group: 'status',
          order: 1_000,
          label: () => this.modeToggleLabel(live.mode),
          invoke: () => live.requestModeChange(this.nextMode(live.mode)),
        })
      }
      this.mountTask = this.transport.request<{ generationId: string }>(NATIVE_METHODS.schedule, spec)
        .then(async scheduled => {
          this.generationId = scheduled.generationId
          await this.transport.request(NATIVE_METHODS.mount, { generationId: scheduled.generationId })
          this.rendererHealthGate?.acceptNativeMount()
          beforeInteractive?.()
        })
        .catch(cause => {
          this.generationId = undefined
          throw cause
        })
    }
    return this.mountTask
  }

  private serializablePayload(spec: DesktopShellPayload | DesktopShellSpec): DesktopShellPayload {
    const themeSource = 'readThemeSource' in spec ? spec.readThemeSource() : spec.themeSource
    const locale = 'readLocalePreference' in spec ? spec.readLocalePreference() ?? this.locale : spec.locale
    return {
      mode: spec.mode,
      macosMaterial: spec.macosMaterial,
      windowsMaterial: spec.windowsMaterial,
      width: spec.width,
      height: spec.height,
      minWidth: spec.minWidth,
      minHeight: spec.minHeight,
      material: spec.material,
      ...(spec.windowsBuild === undefined ? {} : { windowsBuild: spec.windowsBuild }),
      url: spec.url,
      productName: spec.productName,
      windowTitle: spec.windowTitle,
      iconPath: spec.iconPath,
      trayIcons: spec.trayIcons,
      locale,
      themeSource,
    }
  }

  private nextMode(mode: DesktopShellMode): DesktopShellMode {
    if (mode === 'compatibility') return 'extended'
    if (mode === 'extended') return 'advanced'
    return 'compatibility'
  }

  private modeToggleLabel(mode: DesktopShellMode): string {
    if (mode === 'compatibility') return this.locale === 'zh' ? '切换到扩展窗口' : 'Switch to Extended'
    if (mode === 'extended') return this.locale === 'zh' ? '切换到增强模式' : 'Switch to Advanced'
    return this.locale === 'zh' ? '切换到兼容模式' : 'Switch to Compatibility'
  }

  private native(method: string, params?: unknown): void {
    void this.transport.request(method, params).then(
      () => undefined,
      cause => {
        process.stderr.write(
          `dsh-desktop: native ${method} failed: ${cause instanceof Error ? cause.message : String(cause)}\n`,
        )
      },
    )
  }

  show(): void {
    this.native(NATIVE_METHODS.show, { generationId: this.generationId })
  }

  notifyAttention(notification: DesktopNotification): void {
    this.native(NATIVE_METHODS.notifyAttention, notification)
  }

  registerTrayItem(item: DesktopTrayItem): DesktopTrayItemRegistration {
    const id = `tray-${String(this.traySeq)}`
    this.traySeq += 1
    const submenu = item.submenu?.() ?? []
    this.tray.set(id, { id, item, submenu: [...submenu] })
    this.pushTray(id)
    let active = true
    return {
      refresh: () => {
        if (!active) return
        const current = this.tray.get(id)
        if (current === undefined) return
        current.submenu.splice(0, current.submenu.length, ...(item.submenu?.() ?? []))
        this.pushTray(id)
      },
      dispose: () => {
        if (!active) return
        active = false
        this.tray.delete(id)
        this.native(NATIVE_METHODS.trayRemove, { id })
      },
    }
  }

  configureTerminal(spec: DesktopTerminalSpec): void {
    if (this.terminalSpec !== undefined) {
      throw new Error('dsh-desktop: terminal profile is already configured')
    }
    this.terminalSpec = { ...spec }
  }

  openTerminal(): void {
    if (this.terminalSpec === undefined) {
      throw new Error('dsh-desktop: terminal profile is not configured')
    }
    this.native(NATIVE_METHODS.openTerminal, this.terminalSpec)
  }

  reloadRenderer(): void {
    if (this.generationId === undefined) {
      throw new Error('dsh-desktop: renderer reload requires an active shell generation')
    }
    this.native(NATIVE_METHODS.reloadRenderer, { generationId: this.generationId })
  }

  toggleDeveloperTools(): void {
    if (this.generationId === undefined) {
      throw new Error('dsh-desktop: Developer Tools require an active shell generation')
    }
    this.native(NATIVE_METHODS.toggleDeveloperTools, { generationId: this.generationId })
  }

  async exportDiagnostics(): Promise<void> {
    await this.transport.request(NATIVE_METHODS.exportDiagnostics, { locale: this.locale })
  }

  async pickDirectory(): Promise<string | null> {
    const result = await this.transport.request<{ path: string | null }>(NATIVE_METHODS.pickDirectory)
    return result.path
  }

  async validateDirectory(path: string): Promise<boolean> {
    const result = await this.transport.request<{ ok: boolean }>(NATIVE_METHODS.validateDirectory, { path })
    return result.ok
  }

  reportRendererBoot(report: RendererBootReport): void {
    this.rendererHealthGate?.report(report)
    this.native(NATIVE_METHODS.reportRendererBoot, report)
  }

  openProfileCreateWindow(options: ProfileCreateWindowOptions): void {
    this.profileCreate = options
    this.native(NATIVE_METHODS.openProfileCreate, { locale: this.locale })
  }

  submitProfileCreate(name: string): void {
    void this.profileCreate?.onSubmit(name)
  }

  setLocalePreference(preference: DesktopLocale | undefined): void {
    this.locale = preference ?? this.locale
    this.native(NATIVE_METHODS.setLocale, { locale: this.locale })
  }

  setThemeSource(source: DesktopThemeSource): void {
    this.native(NATIVE_METHODS.setTheme, { source })
  }

  async requestRestart(): Promise<void> {
    await this.transport.request(NATIVE_METHODS.restart)
  }

  async requestRecoveryRestart(): Promise<void> {
    await this.transport.request(NATIVE_METHODS.restartRecovery)
  }

  prepareToQuit(): void {
    this.native(NATIVE_METHODS.prepareToQuit)
  }

  async openDialog(options: DesktopDialogOptions): Promise<DesktopDialogResult> {
    return await this.transport.request<DesktopDialogResult>(NATIVE_METHODS.openDialog, options)
  }

  async openRecovery(state: unknown): Promise<void> {
    await this.transport.request(NATIVE_METHODS.openRecovery, { state })
  }

  openProfileCreate(state: unknown): void {
    this.native(NATIVE_METHODS.openProfileCreate, { state })
  }

  openDevtools(options: { tab: 'logs' | 'network', origin: string }): void {
    this.native(NATIVE_METHODS.openDevtools, options)
  }

  async persistBootstrap(profileName: string, mode: DesktopShellMode): Promise<void> {
    await this.transport.request(NATIVE_METHODS.writeBootstrap, { profileName, mode })
  }

  dispose(): void {
    for (const unsubscribe of this.unsubscribers) unsubscribe()
    this.unsubscribers.length = 0
  }

  private snapshot(entry: RegisteredTray): DesktopTrayItemSnapshot {
    const submenu: DesktopTraySubmenuSnapshot[] = entry.submenu.map((item, index) => ({
      id: `${entry.id}:${String(index)}`,
      label: item.label(),
      ...(item.type === undefined ? {} : { type: item.type }),
      enabled: item.enabled?.() ?? true,
      checked: item.checked?.() ?? false,
    }))
    return {
      id: entry.id,
      group: entry.item.group,
      order: entry.item.order,
      label: entry.item.label(),
      enabled: entry.item.enabled?.() ?? true,
      ...(submenu.length === 0 ? {} : { submenu }),
    }
  }

  private pushTray(id: string): void {
    const entry = this.tray.get(id)
    if (entry === undefined) return
    this.native(NATIVE_METHODS.trayUpsert, this.snapshot(entry))
  }

  private async handleTrayInvoke(params: unknown): Promise<void> {
    if (params === null || typeof params !== 'object') return
    const record = params as { id?: unknown, submenuIndex?: unknown }
    if (typeof record.id !== 'string') return
    const entry = this.tray.get(record.id)
    if (entry === undefined) return
    if (typeof record.submenuIndex === 'number') {
      await entry.submenu[record.submenuIndex]?.invoke()
      return
    }
    await entry.item.invoke()
  }
}
