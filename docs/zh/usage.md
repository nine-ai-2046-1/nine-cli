# 使用說明 — nine-cli

基本指令

- 安裝本地 skill：

```sh
nine-cli skill add /path/to/skill
```

- 列出已安裝 skills：

```sh
nine-cli skill list
```

- 移除 skill：

```sh
nine-cli skill remove <name>
```

- 運行 skill：

```sh
nine-cli <skill-name> [args...]
```

技能存放位置

安裝後會放喺 `~/.nine-cli/skills/<skill-name>`。

範例

1. 安裝 `examples/skills/hello`：

```sh
scripts/install_sample_skill.sh
```

2. 運行：

```sh
nine-cli hello
```

參考：本項目遵循 agentskills.io 規範 — https://agentskills.io/specification
