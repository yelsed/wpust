# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

wpust is a WordPress administration CLI utility that opens WordPress admin pages in the browser and performs network diagnostics (DNS, IP, SSL, HTTP response timing) for sites.

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo run -- <command>   # Run with arguments
```

## Architecture

Modular CLI application organized into:

```
src/
├── main.rs           # Entry point, command dispatch
├── cli.rs            # Clap argument/subcommand definitions
├── wordpress/        # WordPress admin page openers
│   ├── mod.rs        # perma, themes, plugins, site_health functions
│   └── utils.rs      # URL preparation (staging auth handling)
├── network/          # Network diagnostic commands
│   ├── dns.rs        # Full DNS lookup (A, AAAA, MX, TXT, NS, CNAME, SOA)
│   ├── ip.rs         # IP lookup with reverse DNS
│   ├── ssl.rs        # SSL certificate inspection
│   ├── response.rs   # HTTP response timing
│   └── page_load.rs  # Headless browser page load (WIP)
└── utils/
    └── url.rs        # Hostname/domain extraction helpers
```

### Key Dependencies

- **clap** (derive) - CLI argument parsing
- **color-eyre** - Error handling with `Result<()>`
- **open** - Launch URLs in default browser
- **hickory-resolver** - DNS record lookups
- **dns-lookup** - IP/reverse DNS lookups
- **openssl** - SSL certificate inspection
- **reqwest** (blocking) - HTTP response timing
- **headless_chrome** - Page load measurement (not yet implemented)

### CLI Commands

All commands take a `site` argument (hostname or URL):

- `perma`, `themes`, `plugins`, `siteinfo` - Open WordPress admin pages
- `dns` - Full DNS record lookup
- `ip` - IP address lookup with reverse DNS
- `ssl` - SSL certificate details and expiry
- `response` - HTTP response timing
- `page-load` - Browser page load time (WIP)

### URL Handling

`wordpress/utils.rs::prepare_wordpress_url()` handles acceptance/staging environments: sites containing "acc." get prefixed with basic auth credentials (`REDACTED:REDACTED@`).
