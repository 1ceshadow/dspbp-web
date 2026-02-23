# DSP 蓝图工具 / DSP Blueprint Tool

戴森球计划蓝图编辑工具，支持建筑升降级、物品/配方替换等功能。  
提供**网页版**（无需安装，浏览器直接使用）与**命令行版**两种使用方式。

**🌐 在线体验：[https://1ceshadow.github.io/dspdp-web/](https://1ceshadow.github.io/dspdp-web/)**

---

## 功能

- **建筑升降级**：传送带 Mk.I/II/III、分拣器 Mk.I/II/III、制造台 Mk.I/II/III、熔炉升级等
- **物品替换**：将蓝图中的物品替换为其他物品
- **配方替换**：替换制造厂的生产配方
- **同步替换**：替换物品时自动匹配对应配方
- **蓝图信息**：查看蓝图内建筑统计
- **JSON 导出/导入**：将蓝图转为 JSON 格式便于查看和编辑

---

## 网页版使用方法

访问 **[https://1ceshadow.github.io/dspdp-web/](https://1ceshadow.github.io/dspdp-web/)**，无需安装任何软件：

1. **粘贴蓝图** — 将游戏中复制的蓝图字符串粘贴到输入框
2. **配置升降级** — 从下拉菜单选择各建筑类型的替换方案，或点击快捷预设按钮
3. **开始转换** — 点击按钮，处理完成后复制输出的新蓝图字符串，粘贴回游戏即可

> 所有处理在本地浏览器内完成，蓝图数据不会上传到任何服务器。

---

## 命令行版

### 下载

从 [Releases](../../releases) 页面下载对应平台的可执行文件：
- Windows：`dspbp.exe`
- Linux：`dspbp`

### 使用示例

**查看帮助**
```
dspbp help
```

**升级传送带和熔炉**
```
dspbp -i "old.txt" -o "new.txt" edit -b ConveyorBeltMKII:ConveyorBeltMKIII,ArcSmelter:PlaneSmelter
```

**替换物品并自动匹配配方**
```
dspbp -i "iron.txt" -o "titanium.txt" edit -B IronOre:TitaniumOre,IronIngot:TitaniumIngot -t "36"
```

**查看蓝图信息**
```
dspbp -i blueprint.txt info
```

**导出为 JSON**
```
dspbp -i blueprint.txt dump
```

**查看所有可用的物品/配方名称**
```
dspbp items
dspbp recipes
```

替换字符串格式：`"来源:目标,来源2:目标2,..."`

| 参数 | 说明 |
|------|------|
| `-b` | 替换建筑（升降级） |
| `-r` | 仅替换物品 |
| `-R` | 仅替换配方 |
| `-B` | 同时替换物品和配方 |
| `-t` | 修改蓝图图标文字 |
| `-c` | 压缩等级 1-9（默认 6，设为 9 可减小约 5% 体积） |

---

## 本地开发 / 构建

### 环境要求

- [Rust](https://rustup.rs/) 工具链
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Node.js 18+

### 命令行版构建

```bash
cargo build --release
# 产物：target/release/dspbp.exe (Windows) 或 target/release/dspbp (Linux)
```

### 网页版本地开发

```bash
# 1. 编译 WASM
wasm-pack build --target web --out-dir web/src/wasm-pkg --features wasm

# 2. 启动前端开发服务器
cd web
npm install
npm run dev
# 访问 http://localhost:5173
```

### 网页版生产打包

```bash
cd web
npm run build
# 产物在 web/dist/，可直接静态托管
```

---

## 部署

推送到 `main` 分支时，GitHub Actions 会自动：
1. 编译 Rust → WebAssembly
2. 构建 Vue 3 前端
3. 发布到 GitHub Pages

若需手动触发，在仓库 **Actions** 标签页点击 `Build & Deploy to GitHub Pages` → `Run workflow`。

---

## 致谢

- [johndoe31415/dspbptk](https://github.com/johndoe31415/dspbptk) — 逆向了 DSP 自定义哈希和蓝图格式，本项目大量参考其实现
- [huww98/dsp_blueprint_editor](https://github.com/huww98/dsp_blueprint_editor) — 蓝图可视化工具，提供了宝贵参考
- [cying314/edit-dspblue-print](https://github.com/cying314/edit-dspblue-print) — 蓝图变换工具，前端实现参考
- DSP Wiki（fandom）提供配方 ID 数据

---

## Python 绑定

参见 [PyPI: dspbp](https://pypi.org/project/dspbp/)。
* More blueprint edit actions, maybe?
