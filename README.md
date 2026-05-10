# nine-cli 🛠️


nine-cli is a small Rust program that provides a simple filesystem-based skills registry, installation helpers, validation against a minimal agentskills spec, and a user-friendly command interface with internationalized messages.

Key features
- Install and remove skills into a per-user directory (~/.nine-cli/skills).
- Run installed skills by name; the skill's cli/run is executed with stdio forwarded.
- Verify SKILL.md frontmatter (name + description) and basic name rules to follow agentskills.io conventions.
- i18n-ready message templates (messages.toml) with simple variable substitution.
- Example skill shipped under examples/skills/hello to get started quickly.

Quick start
1. Build the CLI:

```sh
cargo build --release
```

2. Install a skill (local folder):

```sh
./target/release/nine-cli skill add /path/to/your/skill
```

3. List installed skills:

```sh
./target/release/nine-cli skill list
```

4. Run a skill:

```sh
./target/release/nine-cli hello arg1 arg2
```

Example
- The repository includes examples/skills/hello with a minimal SKILL.md, CLI docs and a cli/run script. Use the provided script scripts/install_sample_skill.sh to copy it into your ~/.nine-cli/skills for quick testing.

Agentskill spec notes
- verify_skill implements a minimal enforcement of agentskills.io-like rules:
  - SKILL.md must exist and include YAML frontmatter with `name` and `description`.
  - `name` must be 1-64 characters, lowercase letters, numbers and hyphens only; must not start/end with hyphen or contain consecutive hyphens.
  - `cli/run` must exist (can be a script or binary).

If you need stricter validation, provide the full agentskills.io schema and the project can be extended.

Internationalization
- Messages live in src/languages/<lang>/messages.toml and the default language is set in src/languages/default.toml. The CLI loads templates and performs {name}/{path}/{reason} substitutions.

Development notes
- Project layout:
  - src/cli/mod.rs: main CLI and skill management logic
  - src/cli/welcome.rs: welcome message
  - examples/skills/: sample skills
  - scripts/: helper scripts (e.g., install sample skill)
- The messages loader caches the parsed messages.toml to avoid repeated IO.

Contributing
- Create a branch from main, make changes, and open a PR. Tests are welcome.

License
- MIT
