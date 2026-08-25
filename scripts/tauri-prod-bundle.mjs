import { spawn } from 'node:child_process'

process.env.DSH_DEV_TOOLS = '0'
process.env.DSH_PROFILE = 'production'
const child = spawn('npx', ['tauri', 'build', ...process.argv.slice(2)], {
  stdio: 'inherit',
  env: process.env,
  shell: process.platform === 'win32',
})
child.on('exit', code => {
  process.exit(code ?? 1)
})
