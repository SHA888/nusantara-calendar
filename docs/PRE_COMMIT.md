# Pre-commit Hooks for Nusantara Calendar

This project uses [pre-commit](https://pre-commit.com/) to ensure code quality and consistency before commits.

## Installation

1. **Install pre-commit** (if not already installed):
   ```bash
   # On Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install pre-commit

   # On other systems, check package manager or use:
   # pip install pre-commit (if system package not available)
   ```

2. **Install the hooks**:
   ```bash
   pre-commit install
   ```

3. **Update hooks** (periodically):
   ```bash
   pre-commit autoupdate
   ```

## What the Hooks Do

### Rust Development
- **`cargo fmt`**: Formats all Rust code according to rustfmt standards
- **`cargo clippy`**: Runs Rust linter with strict warnings
- **`cargo check`**: Validates compilation without building
- **`cargo deny`**: Checks license compliance and security advisories
- **`cargo audit`**: Checks Rust dependencies for security vulnerabilities
- **`cargo doc`**: Builds documentation to check for broken links

### File Quality
- **Trailing whitespace removal**: Ensures no trailing spaces
- **End-of-file fixer**: Ensures files end with newline
- **YAML/TOML syntax checking**: Validates configuration files
- **Merge conflict detection**: Prevents committing conflict markers
- **Large file prevention**: Blocks files >1MB from being committed

### Project Structure
- **Crate name validation**: Ensures crate names match directory names
- **Case conflict checking**: Prevents case-insensitive filename conflicts

## Usage

### On Every Commit
The hooks run automatically when you run `git commit`. If any hook fails, the commit is aborted.

### Manual Testing
Run all hooks manually:
```bash
pre-commit run --all-files
```

Run specific hooks:
```bash
pre-commit run rustfmt --all-files
pre-commit run cargo-check --all-files
```

### Bypassing Hooks (Not Recommended)
If you need to bypass hooks temporarily:
```bash
git commit --no-verify -m "Commit message"
```

## Hook Configuration

The hooks are configured in `.pre-commit-config.yaml`:

- **Fail fast**: Disabled (runs all hooks even if one fails)
- **Exclude patterns**: Skips `target/`, `.git/`, `.cargo/`, and IDE directories
- **Language versions**: Uses system tools and Python 3 for system hooks

## Troubleshooting

### Hook Installation Fails
```bash
# Clean and reinstall
pre-commit uninstall
pre-commit install
```

### Rust Hook Issues
```bash
# Ensure Rust tools are up to date
rustup update
rustup component add rustfmt clippy

# Install cargo tools if missing
cargo install cargo-deny
cargo install cargo-audit
```

### Hook Runs Slowly
- Consider running hooks on specific files only: `pre-commit run --files src/main.rs`
- Use `pre-commit run --all-files` sparingly (it checks every file)

## Integration with CI

The same checks are performed in GitHub Actions (`.github/workflows/ci.yml`), ensuring consistency between local development and CI.

## Custom Hooks

The configuration includes custom hooks for:
- **License compliance** via `cargo deny`
- **Security vulnerability scanning** via `cargo audit`
- **Documentation validation** via `cargo doc`
- **Crate structure validation** via bash script

Add new hooks in the `local` section of `.pre-commit-config.yaml`.
