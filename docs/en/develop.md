# Developing skills for nine-cli

Minimum structure

```
my-skill/
  SKILL.md
  cli/run
```

SKILL.md requirements

- YAML frontmatter at the top delimited by `---` containing at least `name` and `description`.
- `name` must match naming rules (1-64 chars, lowercase, digits, hyphens only).

cli/run

- The executable invoked by nine-cli. Can be a shell script or a compiled binary. Ensure it is executable (`chmod +x cli/run`).

Testing locally

1. Install to your local registry:

```sh
nine-cli skill add /path/to/my-skill
```

2. Run and test:

```sh
nine-cli my-skill --help
```

Security

- Do not run untrusted code in production. Validate the skill source and tests before installing.
