# nine-cli — 將 CLI skills 打造成可用、可發現、可擴充的本地工具 ✨

> nine-cli 讓你（或自動化 agent）將細小 CLI 專案變成可發現、可安裝、可呼叫嘅 "skills"。重點係：易用（直接呼叫）、可發現（有 metadata）、可擴充（方便開發者加入新 skill）。

---

## 🚀 用家角度：nine-cli 幫你做啲乜

- 以名稱安裝並執行小型 CLI（skill），唔駛調 PATH 或包裝啲複雜設定。
- 讓 agent 或其他程式可以用一致方法呼叫本地能力。
- 提供基本驗證，確保技能具備可預期嘅 metadata 同結構。

示例：安裝 `weather` 之後，可以打：

```bash
nine-cli weather --city=HongKong
```

nine-cli 會執行 `weather` 裡嘅 `cli/run`，並把 stdin/stdout/stderr 轉發返你或呼叫方。

---

## ✨ 主要功能一覽

- 安裝 / 移除：`nine-cli skill add <path>` / `nine-cli skill remove <name>`
- 列表：`nine-cli skill list`
- 執行：`nine-cli <skill-name> [args...]`（會轉發 IO）
- 基本驗證：檢查 `SKILL.md` frontmatter 同命名規則
- 支援多語系訊息模板，同埋開發者檔案範例

---

## 🧭 運作原理（簡述）

1. 作者建立 skill folder，包含 `SKILL.md`（YAML frontmatter）與 `cli/run`（entrypoint）。
2. 安裝：`nine-cli skill add /path/to/skill` → 經驗證後複製到 `~/.nine-cli/skills/<name>`。
3. 執行：`nine-cli <skill-name> [args...]` → nine-cli spawn `cli/run`，並轉發 IO。

---

## 🛠️ 快速上手（三步）

```bash
# build
cargo build --release

# install (local folder)
./target/release/nine-cli skill add /path/to/skill

# run
./target/release/nine-cli <skill-name> arg1 arg2
```

---

## ⚠️ 信任與安全（必讀）

> nine-cli 會執行已安裝技能嘅程式碼。勿安裝或執行來歷不明嘅技能。安裝前請審查 `cli/run` 與 `SKILL.md`。

- 優先使用可信作者或 repo 嘅技能；唔確定就先喺隔離環境測試。
- 我哋唔對第三方程式碼負責，請自行驗證技能輸出。

---

## 📋 Skill 規格（安裝時會驗證）

- `SKILL.md` 必須存在，且有 YAML frontmatter 包含 `name` 及 `description`。
- `name` 規則：1–64 字元，只能用小寫字母、數字同 hyphen；不可以 hyphen 開頭/結尾；不可有連續 hyphen。
- `cli/run` 必須存在（script 或 binary），Unix-like 系統會自動設定可執行權限。

本專案遵循 agentskills.io 規範；詳細規格請參閱：https://agentskills.io/specification

---

## 👩‍💻 開發者指南（簡潔版）

最小目錄結構：

```text
my-skill/
  SKILL.md    # YAML frontmatter
  CLI.md      # optional
  TEST.md     # optional
  tests/      # optional
  cli/run     # executable
  scripts/    # optional
```

SKILL.md 範例：

```yaml
---
name: my-skill
description: "A one-line summary"
---
```

檢查清單：

- 資料夾名與 SKILL.md 的 `name` 一致
- `cli/run` 可執行（`chmod +x cli/run`）
- `CLI.md` / `TEST.md` 提供使用及測試說明

本地測試：

```bash
nine-cli skill add /path/to/my-skill
nine-cli my-skill --help
```

---

## 📚 範例與更多文檔

- 範例：`examples/skills/hello`（使用 `scripts/install_sample_skill.sh` 安裝）
- 詳細教學請參考 `docs/` 目錄（英文與廣東話）

---

## 🤝 貢獻與授權

- Fork → 建分支 → 加測試 → PR
- 授權：MIT
