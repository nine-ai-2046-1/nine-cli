# nine-cli — Run & share CLI skills locally ⚡️

nine-cli is a tiny, focused Rust CLI that lets humans and automated agents discover, install, and invoke "skills" — small CLI packages written by anyone. Use it to modularize tools, let agents call local commands reliably, or share handy CLI utilities across your team.

Why you'll like it
- Run skills by name from your terminal or from an agent/process: `nine-cli weather`.
- Install third-party CLI tools (skills) into your local registry (`~/.nine-cli/skills`).
- Each skill adheres to a simple spec (SKILL.md + folder structure) so agents can safely discover and execute them.
- Lightweight, no central server — you control what gets installed.

Core capabilities
- Install/remove skills: `nine-cli skill add <path>` / `nine-cli skill remove <name>`
- List installed skills: `nine-cli skill list`
- Run skills: `nine-cli <skill-name> [args...]` (forwards stdin/stdout/stderr)
- Validation: basic agentskills-like checks on SKILL.md and naming conventions
- Internationalized messages with templates and simple variable substitution

Quick Start
1) Build the CLI:

```sh
cargo build --release
```

2) Install a skill (local folder):

```sh
./target/release/nine-cli skill add /path/to/skill
```

3) Run the skill:

```sh
./target/release/nine-cli <skill-name> arg1 arg2
```

Trust & safety — read before you install ⚠️
- Skills run arbitrary code. Do NOT install skills from unknown/untrusted sources without auditing their code. Review `cli/run` and SKILL.md before installing.
- Always fact-check outputs from third-party skills; they may be incorrect or malicious.
- We accept no responsibility for third-party code installed or executed via nine-cli.

Agentskills rules we enforce
- `SKILL.md` with YAML frontmatter including `name` and `description`.
- `name`: 1–64 characters; lowercase letters, digits, and hyphens only; no leading/trailing hyphen; no consecutive hyphens.
- `cli/run` must exist in the skill folder (script or binary).

Developer guide (how to build a skill for nine-cli) 🚀
Minimal folder layout:

```
my-skill/
  SKILL.md         # YAML frontmatter with name + description
  CLI.md           # optional user docs
  TEST.md          # optional test instructions
  tests/           # optional test files
  cli/run          # entrypoint executed by nine-cli (script or binary)
  scripts/         # optional helper scripts
```

SKILL.md frontmatter example:

```yaml
---
name: my-skill
description: A helpful tool for X
---
```

Make `cli/run` executable (`chmod +x cli/run`) and ensure the folder name matches the `name` in SKILL.md.

Try the included example
- `examples/skills/hello` is a working skill. Use `scripts/install_sample_skill.sh` to install it into `~/.nine-cli/skills` quickly.

Where to find docs
- See the `docs/` folder for detailed guides (English and Cantonese) covering usage, structure, and how to develop skills for nine-cli.

Contributing & License
- Create a branch from `main`, send a PR with clear intent and tests if possible. Code is MIT licensed.
