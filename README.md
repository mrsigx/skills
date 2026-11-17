# Null Limit skills

Agent skills from Null Limit, free to use. Each skill is a folder you can copy, and its tools are built from source.

| Skill | What it does | Tools |
|---|---|---|
| [number-play](skills/number-play) | Plays with any numbers you hand it and tags every finding as structure, convention or made, with a count of the readings it tried. It also checks prime days and writes posts that count themselves. | `numplay` |
| [aim-profile](skills/aim-profile) | Finds the mouse sensitivity your hands already trust and carries it into another game, with every assumption written down. | `aimcalc`, `aimcount` |

The tools are small Rust programs with no dependencies. No executable is committed to this repository.

## Install one skill

Copy its folder into your skills folder, then build its tools once. Building needs Windows and a Rust toolchain.

- **Claude Code:** `~/.claude/skills/<skill>`
- **Codex:** `~/.codex/skills/<skill>`. Each skill's `agents/openai.yaml` holds Codex's display text.

```powershell
git clone https://github.com/mrsigx/skills
Copy-Item -Recurse skills\skills\number-play ~\.claude\skills\number-play
pwsh -File ~\.claude\skills\number-play\build.ps1
```

Each skill's own README has its details.

## Install all of them as a Claude Code plugin

```powershell
claude plugin marketplace add mrsigx/skills
claude plugin install null-limit@null-limit-skills
```

A skill's tools are built inside its own folder, so for a skill with tools the folder copy above is the simpler install.

## Build everything

```powershell
pwsh -File build.ps1
```

This runs each skill's `build.ps1`, which builds that skill's tools and runs their tests.

## Also from Null Limit

- [xsession](https://github.com/mrsigx/xsession): read-only search across local Claude Code, Codex and Antigravity transcripts, with provenance on every hit.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option. Each skill folder carries both license files, so a copied folder keeps its terms.
