# 為 nine-cli 開發 skill

最基本目錄結構

```
my-skill/
  SKILL.md
  cli/run
```

SKILL.md 要求

- 檔案頂部有 YAML frontmatter（`---`）且包含 `name` 同 `description`。
- `name` 要符合命名規則（1-64 字元，小寫字母、數字、hyphen）。

cli/run

- nine-cli 會執行呢個 entrypoint，可以係 script 或 binary。記得 `chmod +x cli/run`。

本地測試

1. 安裝到本地 registry：

```sh
nine-cli skill add /path/to/my-skill
```

2. 運行測試：

```sh
nine-cli my-skill --help
```

安全提示

- 盡量唔好喺生產環境直接執行未知來源嘅 skill，先審核原始碼同測試。

---

參考與規格

本項目遵循 agentskills.io 規範： https://agentskills.io/specification
