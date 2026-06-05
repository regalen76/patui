# PTUI

A terminal UI for managing Pangolin VPN accounts and connections.

## Requirements

- [Rust](https://rustup.rs/) 1.85+ (2024 edition)
- [Pangolin CLI](https://docs.pangolin.network/) installed and on `PATH`

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```

## Features

- View Pangolin auth and service status
- List saved Pangolin accounts from `~/.config/pangolin/accounts.json`
- Select and switch accounts
- Interactive login chooser (cloud or self-hosted)
- Start and stop Pangolin VPN connection
- Slash command input with suggestions

## Commands

| Key / Command | Action |
|---------------|--------|
| `j` / `k` or `↓` / `↑` | Move selection in account list |
| `Enter` or `l` | Select highlighted account |
| `r` | Refresh statuses and accounts |
| `/` | Open command input with suggestions |
| `Tab` or `Enter` | Accept command suggestion |
| `Esc` | Clear command input |
| `Ctrl-c` | Quit |

### Slash commands

| Command | Description |
|---------|-------------|
| `/quit` | Exit the application |
| `/refetch` | Refresh Pangolin statuses and accounts |
| `/login` | Open login chooser (cloud or self-hosted) |
| `/logout` | Logout active Pangolin account |
| `/up` | Start Pangolin connection |
| `/down` | Stop Pangolin connection |
| `/help` | Show help |
| `/clear` | Clear output |

## License

MIT
