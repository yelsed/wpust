# wpust

WordPress administration CLI utility. Open admin pages in the browser and run network diagnostics (DNS, IP, SSL, HTTP response timing) for any site.

## Install

### Quick install (Linux / macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/yelsed/wpust/master/install.sh | sh
```

### Download from GitHub Releases

Grab the latest binary for your platform from the [Releases page](https://github.com/yelsed/wpust/releases):

| Platform | Asset |
|---|---|
| Linux (x86_64) | `wpust-x86_64-unknown-linux-gnu.tar.gz` |
| macOS (Apple Silicon) | `wpust-aarch64-apple-darwin.tar.gz` |
| Windows | `wpust-x86_64-pc-windows-msvc.zip` |

Extract and move the binary somewhere on your PATH (e.g. `~/.local/bin`).

### From source

```sh
cargo install --git https://github.com/yelsed/wpust.git
```

## Usage

All commands take a site argument (hostname or URL).

### WordPress admin pages

```sh
wpust perma example.com      # Permalink settings
wpust themes example.com     # Themes page
wpust plugins example.com    # Plugins page
wpust siteinfo example.com   # Site health
```

### Network diagnostics

```sh
wpust dns example.com        # DNS records (A, AAAA, MX, TXT, NS, CNAME, SOA)
wpust ip example.com         # IP lookup with reverse DNS
wpust ssl example.com        # SSL certificate details and expiry
wpust response example.com   # HTTP response timing
```

### Configuration

```sh
wpust config                 # Interactive setup
```

Run `wpust config` to launch an interactive wizard that lets you configure:

- **Browser** — choose which browser opens admin pages (system default, Firefox, Chrome, or Chromium)
- **Admin path** — set a custom `wp-admin` path if your site uses a non-standard location
- **Basic auth rules** — add username/password credentials for staging or acceptance environments that require HTTP basic auth. When a site URL matches a configured pattern, credentials are automatically included.

Settings are stored in `~/.config/wpust/config.toml`.

### Other

```sh
wpust about                  # About info
wpust goose                  # Render a goose
```

## Development

```sh
cargo build              # Debug build
cargo build --release    # Release build
cargo test               # Run tests
cargo run -- dns example.com  # Run directly
```

## License

[MIT](LICENSE)
