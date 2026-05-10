
# nine-cli — 本地運行與分享 CLI skills ⚡️

nine-cli 係一個細小而實用嘅 Rust CLI，讓人同自動 agent 可以尋找、安裝、同埋呼叫「skills」— 即係其他人寫嘅小型 CLI 工具。佢係一個本地嘅技能倉庫，方便團隊或 agent 呼叫。

點解用 nine-cli？
- 你／agent 可以直接用技能名呼叫命令，例如 `nine-cli weather`。
- 可以安裝其他人寫嘅 CLI 到本地註冊表（`~/.nine-cli/skills`），變成可重用嘅工具。
- 每個 skill 需要有結構清晰嘅 metadata（SKILL.md frontmatter）同命名規則，方便 agent 自動發現同呼叫。

主要功能
- 安裝/移除：`nine-cli skill add <path>` / `nine-cli skill remove <name>`
- 列表：`nine-cli skill list`
- 執行：`nine-cli <skill-name> [args...]`（會轉發 stdin/stdout/stderr）
- 驗證：針對 SKILL.md 同命名做最少量驗證
- 支援多語系訊息模板

快速上手（三步）
1) Build CLI：

```sh
cargo build --release
```

2) 安裝本地 skill：

```sh
./target/release/nine-cli skill add /path/to/skill
```

3) 運行 skill：

```sh
./target/release/nine-cli <skill-name> arg1 arg2
```

示例
- repo 已包含 `examples/skills/hello`，係一個可即刻運行嘅範例。用 `scripts/install_sample_skill.sh` 可以快速安裝到 `~/.nine-cli/skills`。

agentskills 規格（我哋 enforce 嘅重點）
- `SKILL.md` 必須有 YAML frontmatter，並包含 `name` 同 `description`。
- `name` 規則：1–64 字元；只使用小寫英文字母、數字同 hyphen；唔可以以 hyphen 開頭/結尾；唔可以有連續 hyphen。
- `cli/run` 必須存在於 skill folder（script 或 binary 都得）。

信任與安全提醒 ⚠️
- nine-cli 會執行任意第三方程式碼。唔好安裝來歷不明或未經審查嘅 skills。安裝前請審核 `cli/run` 同 SKILL.md。
- 第三方 skill 嘅輸出可能有誤或有惡意行為，請自行核實資訊。我哋唔對用戶安裝或執行嘅第三方程式碼負責。

開發者指南 — 點樣為 nine-cli 開發 skill 🚀
最基本嘅目錄結構：

```
my-skill/
  SKILL.md
  CLI.md
  TEST.md
  tests/
  cli/run
  scripts/
```

SKILL.md 範例 frontmatter：

```yaml
---
name: my-skill
description: A helpful tool for X
---
```

確保 `cli/run` 可執行 (`chmod +x cli/run`)，同文件夾名要同 SKILL.md 裡面嘅 `name` 一致。

更多文件與教學請見 `docs/` 目錄（有英文同中文版本），包含使用說明、技能結構同開發指南。

貢獻同授權
- 從 `main` 開分支，發 PR，通過審查後會合併。代碼採 MIT 授權。
