# nine-cli — Make CLI skills usable, discoverable, and extensible ✨

> Turn small CLI projects into discoverable, agent-callable "skills" that are easy to install and run. nine-cli focuses on usability, discoverability, and extensibility so teams and agents can share and call local capabilities reliably.

---

## 🚀 What this project does (from a user's POV)

- Let you install a small CLI project (a "skill") into a local registry and run it by name.
- Let agents or other programs invoke installed skills in a predictable way (same name, same entrypoint).
- Provide minimal validation so installed skills follow a predictable structure and metadata format.
- Keep things local and permissionable — you decide which third-party skills to install.

Example: once `weather` is installed as a skill, a user or agent runs:

```bash
nine-cli weather --city=HongKong
```

and nine-cli will spawn the `cli/run` inside the `weather` skill and forward stdin/stdout/stderr.

---

## ✨ Key features (quick list)

- Install / remove skills: `nine-cli skill add <path>` / `nine-cli skill remove <name>`
- List installed skills: `nine-cli skill list`
- Execute skills: `nine-cli <skill-name> [args...]` (IO forwarded, exit code propagated)
- Minimal verification of `SKILL.md` frontmatter and naming rules (agentskills-like)
- Intl-ready message templates and a developer guide to author new skills

---

## 🧠 How it works (high level)

1. Author a skill folder containing `SKILL.md` (YAML frontmatter) and `cli/run` (entrypoint).
2. Install with `nine-cli skill add /path/to/skill` — the folder is copied into `~/.nine-cli/skills/<name>` after validation.
3. Invoke with `nine-cli <skill-name> [args...]` — nine-cli spawns `cli/run` and forwards IO.

---

## 🛠️ Quick start (3 commands)

```bash
# build
cargo build --release

# install (local folder)
./target/release/nine-cli skill add /path/to/skill

# run
./target/release/nine-cli <skill-name> arg1 arg2
```

---

## ⚠️ Security & trust — read before installing

> nine-cli executes code from installed skills. Do **not** install skills from unknown or untrusted sources without inspecting their contents.

- Always review `cli/run` and `SKILL.md` before installing.
- Prefer skills from trusted authors or repositories, and run them in a safe environment if unsure.
- We accept no liability for third-party code. Fact-check outputs produced by skills before acting on them.

---

## 📋 Skill spec (what we validate on install)

- `SKILL.md` must exist and include YAML frontmatter with `name` and `description`.
- `name` rules: 1–64 chars, lowercase letters, digits, hyphens only; must not start/end with hyphen; no consecutive hyphens.
- `cli/run` must exist inside the skill folder (script or binary). We will also set executable permission on unix-like systems.

If you want stricter agentskills.io schema validation, provide the schema and we can extend the verifier.

---

## 👩‍💻 For developers — build a skill that works well with nine-cli

Minimum layout:

```text
my-skill/
  SKILL.md    # YAML frontmatter with name + description
  CLI.md      # optional user-facing docs
  TEST.md     # optional test instructions
  tests/      # optional test files
  cli/run     # executable entrypoint (script or binary)
  scripts/    # optional helper scripts
```

SKILL.md example:

```yaml
---
name: my-skill
description: "A one-line summary of what this skill does"
---
```

Developer checklist (before publishing/installing):

- [ ] Folder name matches `name` in SKILL.md and follows naming rules
- [ ] `cli/run` exists and is executable (`chmod +x cli/run`)
- [ ] `CLI.md` and `TEST.md` contain clear usage and test instructions
- [ ] `cli/run` respects exit codes and handles args correctly

Local test:

```bash
nine-cli skill add /path/to/my-skill
nine-cli my-skill --help
```

---

## 📚 Examples & docs

- Example skill: `examples/skills/hello` (install with `scripts/install_sample_skill.sh`).
- Detailed docs: see the `docs/` folder (English and Cantonese) for usage and developer guides.

---

## 🤝 Contributing & License

- Fork, create a branch from `main`, add tests if applicable, and open a PR. Code is MIT licensed.
