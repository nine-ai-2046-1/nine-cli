# nine-cli 🛠️


nine-cli 用 Rust 寫成，透過檔案系統作為 skills registry，提供安裝/移除、基本驗證（依 agentskills 規則），同埋國際化嘅訊息模板。

主要功能
- 安裝/移除 skills 到用戶目錄 (~/.nine-cli/skills)。
- 以技能名運行已安裝嘅 skill，會執行 skills/<name>/cli/run，並轉發 stdin/stdout/stderr。
- 驗證 SKILL.md frontmatter（name + description）同基本名稱規則，配合 agentskills.io 慣例。
- 支援多語系嘅 messages.toml 模板，可以做簡單嘅 {name}/{path}/{reason} 參數替換。
- repo 包含 examples/skills/hello 範例，方便快速試用。

快速上手
1. Build CLI：

```sh
cargo build --release
```

2. 安裝本地 skill：

```sh
./target/release/nine-cli skill add /path/to/your/skill
```

3. 列出已安裝 skills：

```sh
./target/release/nine-cli skill list
```

4. 運行 skill：

```sh
./target/release/nine-cli hello arg1 arg2
```

示例
- repository 有個 examples/skills/hello，包含最少量嘅 SKILL.md、CLI.md 同 cli/run script。可以用 scripts/install_sample_skill.sh 快速安裝到 ~/.nine-cli/skills。

關於 agentskill 規格
- verify_skill 做咗最基本嘅驗證：
  - 必須有 SKILL.md，同包含 YAML frontmatter，有 `name` 同 `description`。
  - `name` 要 1-64 字元，只可以係小寫英文字母、數字同連字符（hyphen），唔能夠以 hyphen 開頭/結尾，亦唔能夠有連續 hyphen。
  - 必須有 `cli/run` 檔案（可以係 script 或 binary）。

如果需要更嚴格嘅驗證，可以提供完整 agentskills.io schema，我會協助擴充。

國際化
- 訊息檔案放喺 src/languages/<lang>/messages.toml，預設語言設定喺 src/languages/default.toml。CLI 會讀取模板並做 {name}/{path}/{reason} 替換。

開發者筆記
- 檔案結構：
  - src/cli/mod.rs：主要 CLI 與 skill 管理邏輯
  - src/cli/welcome.rs：歡迎訊息
  - examples/skills/：範例 skills
  - scripts/：輔助腳本（例如安裝範例 skill）
- messages loader 會快取已解析嘅 messages.toml，避免重複 IO。

貢獻方式
- 從 main 建新分支改 code，發 PR 即可。歡迎加測試。

授權
- MIT
