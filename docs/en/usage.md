# Usage — nine-cli

Basic commands

- Install a skill from local folder:

```sh
nine-cli skill add /path/to/skill
```

- List installed skills:

```sh
nine-cli skill list
```

- Remove a skill:

```sh
nine-cli skill remove <name>
```

- Run a skill:

```sh
nine-cli <skill-name> [args...]
```

Where skills are stored

Skills are installed to `~/.nine-cli/skills/<skill-name>`.

Example

1. Install examples/skills/hello:

```sh
scripts/install_sample_skill.sh
```

2. Run it:

```sh
nine-cli hello
```
