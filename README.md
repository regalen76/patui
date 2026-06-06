# PATUI

A terminal UI for managing Pangolin VPN accounts and connections.
<img width="2555" height="1073" alt="screenshot-20260606-201654" src="https://github.com/user-attachments/assets/13a0193b-3b1c-4e87-ab2b-1747076b8310" />

## Requirements

- [Rust](https://rustup.rs/) 1.85+ (2024 edition)
- [Pangolin CLI](https://docs.pangolin.network/) installed and on `PATH`

## Install

```bash
cargo install patui
```

## Run

```bash
patui
```

## Features

- View Pangolin auth and service status
- List saved Pangolin accounts from `~/.config/pangolin/accounts.json`
- Select and switch account
  <img width="2000" height="1125" alt="Changing Account Patui" src="https://github.com/user-attachments/assets/d2e0f6da-780a-4851-80bc-daef7da80253" />
- Interactive login chooser (cloud or self-hosted)
  <img width="1100" height="619" alt="output" src="https://github.com/user-attachments/assets/2e747868-3c60-4716-aba9-1bf0bb0abd2f" />
- Logout
  <img width="2000" height="1125" alt="Logout Patui" src="https://github.com/user-attachments/assets/e99d8823-2822-4ffd-966d-2fe7f21f3ab5" />
- Start and stop Pangolin VPN connection
  <img width="2000" height="1125" alt="Patui Up" src="https://github.com/user-attachments/assets/33c3b1ba-9a17-4088-b007-be324a03a015" />
  <img width="2000" height="1125" alt="Patui Down" src="https://github.com/user-attachments/assets/4f7351d5-c3cb-4738-b0dd-a2c9b32aa7ad" />
- Slash command input with suggestions

## Commands

| Key / Command          | Action                              |
| ---------------------- | ----------------------------------- |
| `j` / `k` or `↓` / `↑` | Move selection in account list      |
| `Enter` or `l`         | Select highlighted account          |
| `r`                    | Refresh statuses and accounts       |
| `/`                    | Open command input with suggestions |
| `Tab` or `Enter`       | Accept command suggestion           |
| `Esc`                  | Clear command input                 |
| `Ctrl-c`               | Quit                                |

### Slash commands

| Command    | Description                               |
| ---------- | ----------------------------------------- |
| `/quit`    | Exit the application                      |
| `/refetch` | Refresh Pangolin statuses and accounts    |
| `/login`   | Open login chooser (cloud or self-hosted) |
| `/logout`  | Logout active Pangolin account            |
| `/up`      | Start Pangolin connection                 |
| `/down`    | Stop Pangolin connection                  |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). PRs welcome. Open an issue first for big changes.

```bash
git clone https://github.com/regalen76/patui
cd patui
cargo run
```

## License

MIT
