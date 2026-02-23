<template>
  <div class="app">
    <header class="app-header">
      <h1>🔧 DSP 蓝图升降级工具</h1>
      <p class="subtitle">戴森球计划 · Blueprint Upgrade / Downgrade</p>
      <button class="btn theme-toggle" @click="toggleTheme" :title="isDark ? '切换到明亮模式' : '切换到暗色模式'">
        {{ isDark ? '️🌙' : '☀️' }}
      </button>
    </header>

    <div v-if="!wasmReady" class="loading-banner">
      <span class="spinner" />
      <span>正在加载 WASM 模块…</span>
    </div>
    <div v-if="wasmError" class="error-banner">
      ⚠️ WASM 加载失败：{{ wasmError }}
      <br />请先运行 <code>npm run build:wasm</code> 生成 wasm-pkg 目录。
    </div>

    <main v-if="wasmReady" class="layout">
      <!-- ① 蓝图输入 -->
      <section class="card">
        <h2>① 粘贴蓝图</h2>
        <textarea
          v-model="inputBp"
          class="bp-textarea bp-textarea--input"
          placeholder="将蓝图字符串粘贴到这里…"
          spellcheck="false"
          @input="onInputChange"
        />
        <div class="row">
          <button class="btn btn-secondary" @click="pasteInput" title="从剪贴板粘贴蓝图">📋 粘贴</button>
          <button class="btn btn-secondary" @click="inputBp = ''; outputBp = ''; infoText = ''; inputBuildingCounts = []; outputBuildingCounts = []">清空</button>
          <label class="btn btn-secondary file-upload-label" title="从 .txt 文件加载蓝图">
            📂 从文件导入
            <input type="file" accept=".txt,.blueprint" class="file-input-hidden" @change="onFileUpload" />
          </label>
        </div>
        <BlueprintSummary :items="inputBuildingCounts" />
      </section>

      <!-- ② 升降级配置 -->
      <section class="card">
        <div class="card-header-row">
          <h2>② 配置升降级</h2>
          <button class="btn btn-secondary btn-sm" @click="clearConfig" title="重置所有选择为（不替换）">清空配置</button>
        </div>

        <div v-for="group in upgradeGroups" :key="group.id" class="group-row">
          <span class="group-label">{{ group.label }}</span>
          <div class="selects">
            <select v-model="selections[group.id].from">
              <option value="">（全部）</option>
              <option v-for="m in group.members" :key="m.id" :value="m.id">{{ m.label }}</option>
            </select>
            <span class="arrow">→</span>
            <select v-model="selections[group.id].to">
              <option value="">（不替换）</option>
              <option v-for="m in group.members" :key="m.id" :value="m.id">{{ m.label }}</option>
            </select>
          </div>
        </div>

        <!-- 快捷按钮 -->
        <div class="preset-section">
          <p class="preset-title">快捷预设</p>
          <div class="presets">
            <button
              v-for="p in presets"
              :key="p.label"
              class="btn btn-preset"
              @click="applyPreset(p.replacements)"
            >{{ p.label }}</button>
          </div>
        </div>

        <!-- 压缩等级 -->
        <div class="option-row">
          <label>压缩等级：</label>
          <input type="number" v-model.number="compressionLevel" min="1" max="9" class="compression-input" />
          <span class="tooltip-wrap">
            <span class="tooltip-icon">?</span>
            <span class="tooltip-text">控制蓝图字符串的 zlib 压缩强度（1–9）。<br>默认 6，与游戏原版一致。<br>等级越高压缩越彻底但耗时略长。<br>设为 9 相比默认约减小 5% 体积。</span>
          </span>
        </div>

        <!-- 蓝图图标编辑 -->
        <div class="option-row">
          <label style="display:flex;align-items:center;gap:8px;cursor:pointer;">
            <input type="checkbox" v-model="iconEditEnabled" />
            编辑蓝图图标（5个槽位）
          </label>
        </div>
        <div v-if="iconEditEnabled" class="icon-editor">
          <p class="hint" style="margin:0 0 8px;">
            搜索物品名称或直接输入 ID；配方图标用 <code>ID + 20000</code>（如 <code>20001</code>）；留空清除。
            <a href="https://dsp-wiki.com/Items" target="_blank" rel="noopener" class="wiki-link">🔗 DSP Wiki 物品列表</a>
          </p>
          <div class="icon-slots">
            <div v-for="(_, i) in iconSearches" :key="i" class="icon-slot-row">
              <span class="icon-slot-label">图标 {{ i + 1 }}</span>
              <input
                class="icon-slot-input"
                :list="`icon-list-${i}`"
                v-model="iconSearches[i]"
                placeholder="搜索物品名或 ID，留空清除"
                autocomplete="off"
              />
              <datalist :id="`icon-list-${i}`">
                <option v-for="item in iconList" :key="item.id" :value="`${item.name} (${item.id})`" />
              </datalist>
            </div>
          </div>
        </div>

        <button
          class="btn btn-primary run-btn"
          :disabled="!inputBp.trim() || running"
          @click="run"
        >
          <span v-if="running" class="spinner" />
          {{ running ? '处理中…' : '开始转换' }}
        </button>

        <div v-if="runError" class="error-banner mt">⚠️ {{ runError }}</div>
      </section>

      <!-- ③ 输出 -->
      <section class="card">
        <h2>③ 转换结果</h2>
        <textarea
          v-model="outputBp"
          class="bp-textarea bp-textarea--output"
          placeholder="转换后的蓝图字符串将显示在这里…"
          readonly
          spellcheck="false"
        />
        <div class="row">
          <button
            class="btn btn-primary"
            :disabled="!outputBp"
            @click="copyOutput"
          >{{ copied ? '✅ 已复制' : '复制蓝图' }}</button>
          <button
            class="btn btn-secondary"
            :disabled="!outputBp"
            @click="useOutputAsInput"
          >用作输入 (链式操作)</button>
          <button
            class="btn btn-secondary"
            :disabled="!outputBp"
            @click="saveAsFile"
            title="保存为 .txt 文件"
          >💾 保存为文件</button>
        </div>
        <BlueprintSummary :items="outputBuildingCounts" />
      </section>
    </main>

    <footer>
      <div class="footer-links">
        <a href="https://github.com/1ceshadow/dspbp-web" target="_blank" rel="noopener">📦 本项目 GitHub</a>
        <span class="footer-sep">·</span>
        <a href="https://dsp-wiki.com" target="_blank" rel="noopener">📖 DSP Wiki</a>
        <span class="footer-sep">·</span>
        <a href="https://huww98.github.io/dsp_blueprint_editor/" target="_blank" rel="noopener">🎨 蓝图预览</a>
      </div>
      <div class="footer-note">基于 Rust + WASM · 离线运行，蓝图不离开本机</div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { loadWasm, getUpgradeGroups } from './wasm'
import type { WasmModule, UpgradeGroup } from './wasm'
import BlueprintSummary from './BlueprintSummary.vue'
import type { BuildingCount } from './BlueprintSummary.vue'

// ── Theme ──────────────────────────────────────────────────────────────────
const isDark = ref(false)
function toggleTheme() {
  isDark.value = !isDark.value
  document.body.classList.toggle('light-mode', !isDark.value)
}

// Apply initial theme
document.body.classList.add('light-mode')

// ── WASM state ─────────────────────────────────────────────────────────────
const wasmReady = ref(false)
const wasmError = ref('')
let wasm: WasmModule | null = null

onMounted(async () => {
  try {
    wasm = await loadWasm()
    const groups = getUpgradeGroups(wasm)
    upgradeGroups.value = groups
    // Initialize selection state for each group
    for (const g of groups) {
      selections[g.id] = { from: '', to: '' }
    }
    // Load item list for icon editor
    try {
      iconList.value = JSON.parse(wasm.item_list()) as { id: number; name: string }[]
    } catch { /* ignore */ }
    wasmReady.value = true
  } catch (e: unknown) {
    wasmError.value = e instanceof Error ? e.message : String(e)
    wasmReady.value = true
  }
})

// ── Blueprint I/O ──────────────────────────────────────────────────────────
const inputBp = ref('')
const outputBp = ref('')
const infoText = ref('')
const running = ref(false)
const runError = ref('')
const copied = ref(false)

// ── Building summaries ─────────────────────────────────────────────────────
const inputBuildingCounts = ref<BuildingCount[]>([])
const outputBuildingCounts = ref<BuildingCount[]>([])

function loadBuildingCounts(bp: string): BuildingCount[] {
  if (!wasm || !bp.trim()) return []
  try {
    return JSON.parse(wasm.blueprint_building_counts(bp.trim())) as BuildingCount[]
  } catch {
    return []
  }
}

// ── Icon editing ───────────────────────────────────────────────────────────
const iconEditEnabled = ref(false)
const iconList = ref<{ id: number; name: string }[]>([])
// text displayed per slot: "负熵小柜 (1127)" or raw number or empty
const iconSearches = ref<string[]>(['', '', '', '', ''])

function iconIdFromSearch(s: string): number {
  if (!s.trim()) return 0
  const m = s.match(/\((\d+)\)$/)
  if (m) return parseInt(m[1], 10)
  const n = parseInt(s.trim(), 10)
  return isNaN(n) ? 0 : n
}

async function pasteInput() {
  try {
    const text = await navigator.clipboard.readText()
    if (text) {
      inputBp.value = text.trim()
      outputBp.value = ''
      onInputChange()
    }
  } catch {
    // Permission denied or API unavailable – silently ignore
  }
}

function onInputChange() {
  outputBp.value = ''
  outputBuildingCounts.value = []
  runError.value = ''
  if (!wasm || !inputBp.value.trim()) {
    infoText.value = ''
    iconSearches.value = ['', '', '', '', '']
    inputBuildingCounts.value = []
    return
  }
  try {
    infoText.value = wasm.blueprint_info(inputBp.value.trim())
  } catch {
    infoText.value = ''
  }
  inputBuildingCounts.value = loadBuildingCounts(inputBp.value)
  // Load current icon values
  try {
    const icons: number[] = JSON.parse(wasm.get_blueprint_icons(inputBp.value.trim()))
    iconSearches.value = icons.map(v => {
      if (!v) return ''
      const item = iconList.value.find(it => it.id === v)
      if (item) return `${item.name} (${item.id})`
      // Icons can also be recipes (value >= 20000); show them with a clear label
      if (v >= 20000) return `Recipe ID: ${v}`
      return String(v)
    })
  } catch {
    iconSearches.value = ['', '', '', '', '']
  }
}

// ── Upgrade groups / selections ────────────────────────────────────────────
const upgradeGroups = ref<UpgradeGroup[]>([])
const selections = reactive<Record<string, { from: string; to: string }>>({})

// ── Presets ────────────────────────────────────────────────────────────────
const presets = [
  {
    label: '传送带全部升至 Mk.III',
    replacements: [
      { groupId: 'belt', from: 'ConveyorBeltMKI',  to: 'ConveyorBeltMKIII' },
      { groupId: 'belt', from: 'ConveyorBeltMKII', to: 'ConveyorBeltMKIII' },
    ],
  },
  {
    label: '传送带全部降至 Mk.I',
    replacements: [
      { groupId: 'belt', from: 'ConveyorBeltMKII',  to: 'ConveyorBeltMKI' },
      { groupId: 'belt', from: 'ConveyorBeltMKIII', to: 'ConveyorBeltMKI' },
    ],
  },
  {
    label: '分拣器全部升至 Mk.III',
    replacements: [
      { groupId: 'sorter', from: 'SorterMKI',  to: 'SorterMKIII' },
      { groupId: 'sorter', from: 'SorterMKII', to: 'SorterMKIII' },
    ],
  },
  {
    label: '制造台全部升至 Mk.III',
    replacements: [
      { groupId: 'assembler', from: 'AssemblingMachineMkI',  to: 'AssemblingMachineMkIII' },
      { groupId: 'assembler', from: 'AssemblingMachineMkII', to: 'AssemblingMachineMkIII' },
    ],
  },
  {
    label: '熔炉 电弧→位面',
    replacements: [
      { groupId: 'smelter', from: 'ArcSmelter', to: 'PlaneSmelter' },
    ],
  },
  {
    label: '熔炉 全部→负熵',
    replacements: [
      { groupId: 'smelter', from: 'ArcSmelter',   to: 'NegentropySmelter' },
      { groupId: 'smelter', from: 'PlaneSmelter', to: 'NegentropySmelter' },
    ],
  },
  {
    label: '制造台全部→重组式',
    replacements: [
      { groupId: 'assembler', from: 'AssemblingMachineMkI',  to: 'RecomposingAssembler' },
      { groupId: 'assembler', from: 'AssemblingMachineMkII', to: 'RecomposingAssembler' },
      { groupId: 'assembler', from: 'AssemblingMachineMkIII', to: 'RecomposingAssembler' },
    ],
  },
  {
    label: '化工厂→量子化工厂',
    replacements: [
      { groupId: 'chemplant', from: 'ChemicalPlant', to: 'QuantumChemicalPlant' },
    ],
  },
  {
    label: '研究站→自演化',
    replacements: [
      { groupId: 'lab', from: 'MatrixLab', to: 'SelfevolutionLab' },
    ],
  },
  {
    label: '全部升级 (带+拣+炉+台)',
    replacements: [
      { groupId: 'belt',      from: 'ConveyorBeltMKI',       to: 'ConveyorBeltMKIII' },
      { groupId: 'belt',      from: 'ConveyorBeltMKII',      to: 'ConveyorBeltMKIII' },
      { groupId: 'sorter',    from: 'SorterMKI',             to: 'SorterMKIII' },
      { groupId: 'sorter',    from: 'SorterMKII',            to: 'SorterMKIII' },
      { groupId: 'assembler', from: 'AssemblingMachineMkI',  to: 'AssemblingMachineMkIII' },
      { groupId: 'assembler', from: 'AssemblingMachineMkII', to: 'AssemblingMachineMkIII' },
      { groupId: 'smelter',   from: 'ArcSmelter',            to: 'PlaneSmelter' },
    ],
  },
  {
    label: '全部升级 (含负熵系)',
    replacements: [
      { groupId: 'belt',      from: 'ConveyorBeltMKI',        to: 'ConveyorBeltMKIII' },
      { groupId: 'belt',      from: 'ConveyorBeltMKII',       to: 'ConveyorBeltMKIII' },
      { groupId: 'sorter',    from: 'SorterMKI',              to: 'SorterMKIII' },
      { groupId: 'sorter',    from: 'SorterMKII',             to: 'SorterMKIII' },
      { groupId: 'assembler', from: 'AssemblingMachineMkI',   to: 'RecomposingAssembler' },
      { groupId: 'assembler', from: 'AssemblingMachineMkII',  to: 'RecomposingAssembler' },
      { groupId: 'assembler', from: 'AssemblingMachineMkIII', to: 'RecomposingAssembler' },
      { groupId: 'smelter',   from: 'ArcSmelter',             to: 'NegentropySmelter' },
      { groupId: 'smelter',   from: 'PlaneSmelter',           to: 'NegentropySmelter' },
      { groupId: 'chemplant', from: 'ChemicalPlant',          to: 'QuantumChemicalPlant' },
      { groupId: 'lab',       from: 'MatrixLab',              to: 'SelfevolutionLab' },
    ],
  },
]

function applyPreset(replacements: { groupId: string; from: string; to: string }[]) {
  // Reset all
  for (const g of upgradeGroups.value) {
    selections[g.id] = { from: '', to: '' }
  }
  // Group by groupId: if a group has multiple "from" entries, show "（全部）" in UI
  const grouped = new Map<string, { froms: string[]; to: string }>()
  for (const r of replacements) {
    if (!grouped.has(r.groupId)) {
      grouped.set(r.groupId, { froms: [], to: r.to })
    }
    grouped.get(r.groupId)!.froms.push(r.from)
  }
  for (const [groupId, { froms, to }] of grouped.entries()) {
    if (selections[groupId]) {
      // Multiple froms → show "（全部）" so UI accurately reflects the replacement
      selections[groupId].from = froms.length > 1 ? '' : froms[0]
      selections[groupId].to = to
    }
  }
  // Store full preset for run():
  pendingPreset.value = replacements
}

function clearConfig() {
  for (const g of upgradeGroups.value) {
    selections[g.id] = { from: '', to: '' }
  }
  pendingPreset.value = null
}

const pendingPreset = ref<{ groupId: string; from: string; to: string }[] | null>(null)

// ── Run ────────────────────────────────────────────────────────────────────
const compressionLevel = ref(6)

function buildReplaceBuildingString(): string {
  if (!wasm) return ''

  // Combine: from pendingPreset or from UI selections
  const pairs: string[] = []

  if (pendingPreset.value) {
    for (const r of pendingPreset.value) {
      if (r.from && r.to && r.from !== r.to) {
        pairs.push(`${r.from}:${r.to}`)
      }
    }
    pendingPreset.value = null
  } else {
    for (const g of upgradeGroups.value) {
      const sel = selections[g.id]
      if (!sel || !sel.to || sel.from === sel.to) continue

      if (sel.from) {
        // Single from→to
        pairs.push(`${sel.from}:${sel.to}`)
      } else {
        // "全部" → replace all members except target
        for (const m of g.members) {
          if (m.id !== sel.to) {
            pairs.push(`${m.id}:${sel.to}`)
          }
        }
      }
    }
  }

  return pairs.join(',')
}

function run() {
  if (!wasm) return
  const bp = inputBp.value.trim()
  if (!bp) return

  running.value = true
  runError.value = ''
  outputBp.value = ''

  // Run in next tick so Vue can update the spinner
  setTimeout(() => {
    try {
      const replaceBuilding = buildReplaceBuildingString()
      let result = wasm!.edit_blueprint(
        bp,
        replaceBuilding,
        '', // replace_item
        '', // replace_recipe
        '', // replace_both
        compressionLevel.value
      )
      // Apply icon changes if enabled
      if (iconEditEnabled.value) {
        const updates = iconSearches.value
          .map((s, i) => ({ slot: i, value: iconIdFromSearch(s) }))
        result = wasm!.set_blueprint_icons(
          result,
          JSON.stringify(updates),
          compressionLevel.value
        )
      }
      outputBp.value = result
      outputBuildingCounts.value = loadBuildingCounts(result)
    } catch (e: unknown) {
      runError.value = e instanceof Error ? e.message : String(e)
      outputBuildingCounts.value = []
    } finally {
      running.value = false
    }
  }, 20)
}

// ── Helpers ────────────────────────────────────────────────────────────────
async function copyOutput() {
  if (!outputBp.value) return
  await navigator.clipboard.writeText(outputBp.value)
  copied.value = true
  setTimeout(() => { copied.value = false }, 2000)
}

function useOutputAsInput() {
  if (!outputBp.value) return
  inputBp.value = outputBp.value
  inputBuildingCounts.value = outputBuildingCounts.value
  outputBp.value = ''
  outputBuildingCounts.value = []
  onInputChange()
}

function onFileUpload(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = (e) => {
    inputBp.value = (e.target?.result as string).trim()
    outputBp.value = ''
    onInputChange()
  }
  reader.readAsText(file)
  // Reset input so same file can be re-uploaded
  ;(event.target as HTMLInputElement).value = ''
}

function saveAsFile() {
  if (!outputBp.value) return
  const blob = new Blob([outputBp.value], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'blueprint.txt'
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<style>
*, *::before, *::after { box-sizing: border-box; }

:root {
  --bg-body: #0d1117;
  --bg-card: #161b22;
  --bg-input: #0d1117;
  --bg-select: #21262d;
  --border-color: #30363d;
  --text-primary: #e6edf3;
  --text-secondary: #8b949e;
  --text-muted: #c9d1d9;
  --accent: #58a6ff;
  --btn-secondary-bg: #21262d;
  --preset-bg: #1f2e47;
  --preset-color: #79c0ff;
  --preset-border: #1f6feb;
  --preset-hover: #1f6feb33;
  --loading-bg: #1f2e47;
  --loading-border: #1f6feb;
  --loading-color: #79c0ff;
  --error-bg: #3d1f1f;
  --error-border: #f85149;
  --error-color: #ffa198;
  --error-code-bg: #21262d;
  --count-color: #22d3ee;
  --count-shadow: 0 0 8px rgba(34,211,238,0.7);
  --card-shadow: 0 2px 12px rgba(0,0,0,0.35);
  --card-shadow-hover: 0 4px 20px rgba(0,0,0,0.5);
}

body.light-mode {
  --bg-body: #f6f8fa;
  --bg-card: #ffffff;
  --bg-input: #f0f2f5;
  --bg-select: #f0f2f5;
  --border-color: #d0d7de;
  --text-primary: #1f2328;
  --text-secondary: #57606a;
  --text-muted: #24292f;
  --accent: #0969da;
  --btn-secondary-bg: #f6f8fa;
  --preset-bg: #dbeafe;
  --preset-color: #1d4ed8;
  --preset-border: #3b82f6;
  --preset-hover: #3b82f633;
  --loading-bg: #dbeafe;
  --loading-border: #3b82f6;
  --loading-color: #1d4ed8;
  --error-bg: #fff0f0;
  --error-border: #cf222e;
  --error-color: #cf222e;
  --error-code-bg: #f0f2f5;
  --count-color: #0550ae;
  --count-shadow: none;
  --card-shadow: 0 1px 6px rgba(0,0,0,0.1);
  --card-shadow-hover: 0 3px 14px rgba(0,0,0,0.15);
}

body {
  margin: 0;
  font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
  background: var(--bg-body);
  color: var(--text-primary);
  min-height: 100vh;
  transition: background 0.25s, color 0.25s;
}

.app {
  max-width: 1200px;
  margin: 0 auto;
  padding: 24px 16px 48px;
}

.app-header {
  text-align: center;
  margin-bottom: 32px;
  position: relative;
}
.app-header h1 {
  margin: 0 0 6px;
  font-size: 2rem;
  font-weight: 700;
  color: var(--accent);
  letter-spacing: -0.01em;
  text-shadow: 0 0 32px rgba(88,166,255,0.25);
}
.subtitle {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.9rem;
}
.theme-toggle {
  position: absolute;
  top: 0;
  right: 0;
  background: var(--bg-select);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  font-size: 0.85rem;
  padding: 6px 14px;
  border-radius: 20px;
  cursor: pointer;
  transition: background 0.2s, border-color 0.2s;
}
.theme-toggle:hover {
  border-color: var(--accent);
  color: var(--accent);
}

/* Layout */
.layout {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 20px;
}
@media (max-width: 900px) {
  .layout { grid-template-columns: 1fr; }
}

.card {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  padding: 22px 20px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  transition: background 0.25s, border-color 0.25s, box-shadow 0.25s;
  box-shadow: var(--card-shadow);
}
.card:hover {
  box-shadow: var(--card-shadow-hover);
}
.card h2 {
  margin: 0;
  font-size: 0.78rem;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}
.card h2::before {
  content: '';
  display: inline-block;
  width: 3px;
  height: 14px;
  background: var(--accent);
  border-radius: 2px;
  opacity: 0.7;
  flex-shrink: 0;
}

/* Textarea */
.bp-textarea {
  width: 100%;
  flex: 1;
  background: var(--bg-input);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-primary);
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 0.78rem;
  padding: 10px 12px;
  resize: vertical;
  outline: none;
  transition: border-color 0.2s, box-shadow 0.2s, background 0.25s, color 0.25s;
  box-sizing: border-box;
  box-shadow: inset 0 1px 4px rgba(0,0,0,0.15);
}
.bp-textarea:focus {
  border-color: var(--accent);
  box-shadow: inset 0 1px 4px rgba(0,0,0,0.15), 0 0 0 3px rgba(88,166,255,0.12);
}
.bp-textarea--input { min-height: 110px; }
.bp-textarea--output {
  min-height: 72px;
  color: var(--text-secondary);
  font-size: 0.73rem;
  cursor: default;
}
.bp-textarea--output:not(:placeholder-shown) {
  color: var(--text-primary);
  opacity: 0.85;
}

/* Buttons */
.btn {
  padding: 7px 15px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: background 0.15s, opacity 0.15s, transform 0.1s, box-shadow 0.15s;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  white-space: nowrap;
}
.btn:disabled { opacity: 0.38; cursor: not-allowed; }
.btn:not(:disabled):active { transform: translateY(1px); }
.btn-primary {
  background: #238636;
  color: #fff;
  box-shadow: 0 1px 3px rgba(0,0,0,0.3);
}
.btn-primary:not(:disabled):hover {
  background: #2ea043;
  box-shadow: 0 2px 8px rgba(35,134,54,0.45);
}
.btn-secondary {
  background: var(--btn-secondary-bg);
  color: var(--text-muted);
  border: 1px solid var(--border-color);
}
.btn-secondary:not(:disabled):hover {
  background: var(--border-color);
  color: var(--text-primary);
}
.btn-preset {
  background: var(--preset-bg);
  color: var(--preset-color);
  border: 1px solid var(--preset-border);
  font-size: 0.78rem;
  padding: 5px 10px;
  border-radius: 6px;
}
.btn-preset:hover { background: var(--preset-hover); }

.run-btn {
  width: 100%;
  justify-content: center;
  padding: 13px;
  font-size: 1rem;
  border-radius: 10px;
  letter-spacing: 0.02em;
  box-shadow: 0 2px 8px rgba(35,134,54,0.3);
}
.run-btn:not(:disabled):hover {
  box-shadow: 0 4px 16px rgba(35,134,54,0.45);
}

/* Group rows */
.group-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.group-label {
  font-size: 0.82rem;
  color: var(--text-secondary);
  font-weight: 500;
}
.selects {
  display: flex;
  align-items: center;
  gap: 8px;
}
.selects select {
  flex: 1;
  background: var(--bg-select);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
  padding: 6px 10px;
  border-radius: 7px;
  font-size: 0.84rem;
  outline: none;
  cursor: pointer;
  transition: background 0.25s, color 0.25s, border-color 0.2s, box-shadow 0.2s;
}
.selects select:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px rgba(88,166,255,0.12);
}
.arrow { color: var(--accent); font-weight: bold; }

/* Card header row */
.card-header-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.card-header-row h2 { margin: 0; }
.btn-sm {
  font-size: 0.75rem;
  padding: 4px 10px;
}

/* Tooltip icon */
.tooltip-wrap {
  position: relative;
  display: inline-flex;
  align-items: center;
}
.tooltip-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--text-secondary);
  color: var(--bg-card);
  font-size: 0.7rem;
  font-weight: bold;
  cursor: help;
  flex-shrink: 0;
  user-select: none;
}
.tooltip-text {
  visibility: hidden;
  opacity: 0;
  pointer-events: none;
  position: absolute;
  left: 50%;
  bottom: calc(100% + 6px);
  transform: translateX(-50%);
  background: #1c2128;
  color: #e6edf3;
  border: 1px solid #444c56;
  border-radius: 6px;
  padding: 7px 10px;
  font-size: 0.78rem;
  line-height: 1.6;
  white-space: nowrap;
  box-shadow: 0 4px 12px rgba(0,0,0,0.4);
  transition: opacity 0.15s;
  z-index: 100;
}
.tooltip-wrap:hover .tooltip-text {
  visibility: visible;
  opacity: 1;
}

/* Presets */
.preset-section {
  border-top: 1px solid var(--border-color);
  padding-top: 12px;
}
.preset-title {
  margin: 0 0 8px;
  font-size: 0.8rem;
  color: var(--text-secondary);
}
.presets {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* Options */
.option-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.85rem;
}
.compression-input {
  width: 56px;
  background: var(--bg-select);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
  padding: 4px 8px;
  border-radius: 6px;
  text-align: center;
  transition: background 0.25s, color 0.25s;
}
.hint { color: var(--text-secondary); font-size: 0.75rem; }

/* Row */
.row {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}
.info-text {
  font-size: 0.8rem;
  color: var(--text-secondary);
  font-family: monospace;
  white-space: pre;
}

/* Banners */
.loading-banner, .error-banner {
  border-radius: 10px;
  padding: 12px 16px;
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 0.875rem;
}
.loading-banner {
  background: var(--loading-bg);
  border: 1px solid var(--loading-border);
  color: var(--loading-color);
}
.error-banner {
  background: var(--error-bg);
  border: 1px solid var(--error-border);
  color: var(--error-color);
}
.error-banner code {
  background: var(--error-code-bg);
  padding: 2px 6px;
  border-radius: 4px;
}
.mt { margin-top: 4px; }

/* Spinner */
.spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid currentColor;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}
@keyframes spin { to { transform: rotate(360deg); } }

/* Footer */
footer {
  text-align: center;
  margin-top: 40px;
  font-size: 0.8rem;
  color: var(--text-secondary);
}
.footer-links {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px 0;
  margin-bottom: 6px;
}
.footer-sep {
  margin: 0 8px;
  opacity: 0.4;
}
.footer-note {
  opacity: 0.6;
}
footer a { color: var(--accent); text-decoration: none; }
footer a:hover { text-decoration: underline; }

/* File upload */
.file-upload-label {
  cursor: pointer;
  position: relative;
}
.file-input-hidden {
  position: absolute;
  inset: 0;
  opacity: 0;
  cursor: pointer;
  width: 100%;
}

/* Icon editor */
.icon-editor {
  background: var(--bg-input);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 12px 14px;
  box-shadow: inset 0 1px 4px rgba(0,0,0,0.1);
}
.icon-slots {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.icon-slot-row {
  display: flex;
  align-items: center;
  gap: 10px;
}
.icon-slot-label {
  width: 52px;
  font-size: 0.82rem;
  color: var(--text-secondary);
  flex-shrink: 0;
}
.icon-slot-input {
  flex: 1;
  background: var(--bg-select);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.9rem;
  padding: 4px 8px;
}
.icon-slot-input:focus {
  outline: none;
  border-color: var(--accent);
}
.wiki-link {
  color: var(--accent);
  font-size: 0.82rem;
  text-decoration: none;
  margin-left: 6px;
}
.wiki-link:hover {
  text-decoration: underline;
}
</style>
