# DSP 蓝图升降级工具（网页版）

> **本项目是 [Wesmania/dspbp](https://github.com/Wesmania/dspbp)（MIT License）的派生项目。**  
> Rust 核心库来自上游，本项目新增了 **WebAssembly 绑定** 和 **Vue 3 网页前端**，使任何人无需安装软件即可在浏览器中使用蓝图升降级功能。

**🌐 在线体验：[https://1ceshadow.github.io/dspdp-web/](https://1ceshadow.github.io/dspdp-web/)**

---

## 与上游的关系

| 部分 | 来自 |
|------|------|
| `src/`（除 `wasm.rs`） | [Wesmania/dspbp](https://github.com/Wesmania/dspbp) 原始代码（MIT） |
| `src/wasm.rs` | 本项目新增，WASM 对外暴露的 API 入口 |
| `web/` | 本项目新增，Vue 3 + TypeScript 前端 |
| `.github/workflows/deploy.yml` | 本项目新增，自动编译并发布到 GitHub Pages |

---

## 功能

- **建筑升降级**：传送带 Mk.I/II/III、分拣器 Mk.I/II/III、制造台 Mk.I/II/III、熔炉升级
- **物品替换**：将蓝图中的物品替换为其他物品
- **配方替换**：替换制造厂的生产配方
- **同步替换**：替换物品时自动匹配对应配方
- **蓝图信息**：查看蓝图内建筑统计
- **JSON 导出/导入**：将蓝图转为 JSON 格式（命令行版）

---

## 网页版使用方法

访问 **[https://1ceshadow.github.io/dspdp-web/](https://1ceshadow.github.io/dspdp-web/)**，无需安装任何软件：

1. **粘贴蓝图** — 将游戏中复制的蓝图字符串粘贴到输入框
2. **配置升降级** — 从下拉菜单选择各建筑类型的替换方案，或点击快捷预设按钮
3. **开始转换** — 点击按钮，处理完成后复制输出的新蓝图字符串，粘贴回游戏即可

> 所有处理在本地浏览器内完成（基于 WebAssembly），蓝图数据不会上传到任何服务器。

---

## 命令行版

命令行版由上游 [Wesmania/dspbp](https://github.com/Wesmania/dspbp) 提供，详细文档请参阅原项目。

快速示例：

```bash
# 升级传送带和熔炉
dspbp -i "old.txt" -o "new.txt" edit -b ConveyorBeltMKII:ConveyorBeltMKIII,ArcSmelter:PlaneSmelter

# 替换物品并自动匹配配方
dspbp -i "iron.txt" -o "titanium.txt" edit -B IronOre:TitaniumOre,IronIngot:TitaniumIngot

# 查看蓝图信息
dspbp -i blueprint.txt info

# 列出所有可用物品/配方名称
dspbp items
dspbp recipes
```

| 参数 | 说明 |
|------|------|
| `-b` | 替换建筑（升降级） |
| `-r` | 仅替换物品 |
| `-R` | 仅替换配方 |
| `-B` | 同时替换物品和配方 |
| `-t` | 修改蓝图图标文字 |
| `-c` | 压缩等级 1–9（默认 6） |

---

## 本地开发

### 环境要求

- [Rust](https://rustup.rs/) 工具链
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Node.js 18+

### 编译 WASM 并启动前端

```bash
# 1. 在项目根目录编译 WASM
wasm-pack build --target web --out-dir web/src/wasm-pkg --features wasm

# 2. 启动前端开发服务器
cd web
npm install
npm run dev
# 访问 http://localhost:5173
```

### 生产打包

```bash
cd web
npm run build
# 产物在 web/dist/，可直接静态托管
```

---

## 自动部署

推送到 `main` 分支时，GitHub Actions 自动完成：
1. 编译 Rust → WebAssembly（`wasm-pack build`）
2. 构建 Vue 3 前端（`npm run build`）
3. 发布到 GitHub Pages

---

## 许可证

本项目遵循 **MIT License**，与上游保持一致。  
原始版权归 Igor Kotrasinski（[Wesmania/dspbp](https://github.com/Wesmania/dspbp)）所有。

---

## 致谢

- **[Wesmania/dspbp](https://github.com/Wesmania/dspbp)** — 本项目的核心 Rust 库，蓝图解析与编辑逻辑全部来自此项目
- [johndoe31415/dspbptk](https://github.com/johndoe31415/dspbptk) — 逆向了 DSP 自定义哈希和蓝图二进制格式
- [huww98/dsp_blueprint_editor](https://github.com/huww98/dsp_blueprint_editor) — 蓝图可视化工具，前端参考
- [cying314/edit-dspblue-print](https://github.com/cying314/edit-dspblue-print) — 蓝图变换工具，前端参考

