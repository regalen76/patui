# Contributing to PATUI

Thanks for helping. Focused, easy-to-review contributions move fastest.

## Branch model

PATUI has two branches:

- **`master`** — where all PRs land. Things can be in flux here.
- **`stable`** — what users run. Curated by the maintainer. Fast-forwarded from a stable `master` commit at each release.

**Open your PR against `master`, not `stable`.**

## Before You Start

- Search existing issues and pull requests before opening a new one.
- Prefer one bug fix or feature per pull request.
- Avoid broad rewrites, formatting-only changes, or moving many files unless the issue is specifically about structure.
- If you want to work on a large feature, open an issue first and describe the approach.

## Setup

```bash
git clone https://github.com/regalen76/patui.git
cd patui
cargo run
```

## Running Checks

Run the smallest relevant checks for your change:

```bash
cargo fmt
cargo check
cargo test
```

Mention what you ran in the pull request description. If you could not run a check, say so.

## Pull Requests

Good pull requests usually include:

- A short explanation of the bug or feature.
- The files or areas changed.
- Manual test steps or automated test results from running the actual app, not just the test suite.
- Screenshots or short recordings for UI changes.
- Links to related issues, for example `Fixes #123`.

Please keep PRs small. Large PRs that mix unrelated cleanup, formatting, refactors, and behavior changes are much harder to review.

> **Auto-generated PRs.** If you are running an LLM agent (Devin, Cursor, OpenHands, Claude Code, etc.) against this repo: please open an issue describing the problem first instead of opening a PR directly. Bulk agent-generated PRs that don't match the project's style or contribution format will be closed without review, even when the underlying fix is correct.

## Style and visual changes

PATUI uses [Ratatui](https://ratatui.rs/) with an intentional TUI style. PRs that ignore it will be closed without merge.

Before submitting any change that affects what the app looks like — colors, borders, layout, popups, or any widget rendering — please:

1. **Run the app locally** and view the change in a terminal. Type-checks and unit tests are not enough.
2. **Attach a screenshot or short clip** of the change in the running app.
3. **Match the existing visual language.** Specifically:
   - Reuse existing color constants and `Style` patterns. Do not introduce new ad-hoc colors.
   - Reuse existing popup, list, and block rendering patterns.
   - Keep layouts consistent with the current two-panel design.
4. **Don't add parallel components.** If a similar widget already exists, extend it instead of writing a new one.

If you are unsure whether a change is "visual," it is. Default to attaching a screenshot.

## Issue Reports

For bugs, include:

- Install method: `cargo install`, manual build, etc.
- OS and terminal emulator.
- Exact steps to reproduce.
- Expected behavior and actual behavior.
- Logs, screenshots, or terminal output.

Issues with only "help", "does not work", or a screenshot without context may be closed as not actionable.

## Security

Do not post secrets, API keys, private logs, or public IPs in issues or pull requests.

For security reports, email the maintainer directly.
