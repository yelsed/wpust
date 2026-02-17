# wpust

WordPress administration CLI utility that opens WordPress admin pages in the browser and performs network diagnostics (DNS, IP, SSL, HTTP response timing) for sites.

## Installation

### From GitHub Releases

1. Go to the [Releases page](https://github.com/yelsed/wpust/releases) and download the binary for your platform:
   - **macOS (Intel)**: `wpust-x86_64-apple-darwin.tar.gz`
   - **macOS (Apple Silicon)**: `wpust-aarch64-apple-darwin.tar.gz`
   - **Linux**: `wpust-x86_64-unknown-linux-gnu.tar.gz`
   - **Windows**: `wpust-x86_64-pc-windows-msvc.zip`

2. Extract the archive:
   - **macOS/Linux**: `tar -xzf wpust-*.tar.gz`
   - **Windows**: Extract the ZIP file

3. Add the binary to your PATH:

   **macOS/Linux:**
   ```bash
   # Option 1: Install to ~/.local/bin (recommended)
   mkdir -p ~/.local/bin
   mv wpust ~/.local/bin/

   # Option 2: Install to /usr/local/bin (requires sudo)
   sudo mv wpust /usr/local/bin/
   ```

   **Windows:**
   - Move `wpust.exe` to a folder (e.g., `C:\Program Files\wpust\`)
   - Add that folder to your system PATH:
     1. Open System Properties → Environment Variables
     2. Edit the `Path` variable
     3. Add the folder containing `wpust.exe`

4. Verify installation:
   ```bash
   wpust about
   ```

## Usage

All commands accept a `site` argument (hostname or URL):

### WordPress Admin Pages

```bash
# Open permalink settings
wpust perma example.com

# Open themes page
wpust themes example.com

# Open plugins page
wpust plugins example.com

# Open site health page
wpust siteinfo example.com
```

### Network Diagnostics

```bash
# DNS lookup (A, AAAA, MX, TXT, NS, CNAME, SOA)
wpust dns example.com

# IP address lookup with reverse DNS
wpust ip example.com

# SSL certificate details and expiry
wpust ssl example.com

# HTTP response timing
wpust response example.com

# Page load time in headless browser (WIP)
wpust page-load example.com
```

### Other Commands

```bash
# Show help
wpust about

# Render a goose
wpust goose
```

## Local Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yelsed/wpust.git
cd wpust

# Build release binary
cargo build --release

# The binary will be at target/release/wpust
```

### Adding to PATH for Development

**macOS/Linux:**
```bash
# Option 1: Create a symlink
ln -s $(pwd)/target/release/wpust ~/.local/bin/wpust

# Option 2: Copy the binary
cp target/release/wpust ~/.local/bin/
```

**Windows:**
- Add `target\release\` to your PATH, or copy `wpust.exe` to a folder already in your PATH

### Running Tests

```bash
cargo test
```

## Requirements

- **macOS/Linux**: No additional runtime dependencies
- **Windows**: No additional runtime dependencies

## License

[Add your license here]
