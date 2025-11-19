# DCM for Zed

DCM.dev extension for Zed IDE which provides static analysis for Dart and Flutter projects.

## Features

Read more about [DCM Feature](https://dcm.dev/features/) on their official website.

## Installation

### Prerequisites

1. **Install DCM** — Follow the [installation guide](https://dcm.dev/docs/getting-started/for-developers/installation) for your platform (macOS, Linux, or Windows).
2. Ensure `dcm` is available in your PATH and verify with `dcm version`.
3. Have Rust installed if building from source: [rustup](https://rustup.rs/)

### Building from Source

Follow the steps and build the extension: 

```bash
git clone https://github.com/nelsoncampos-cloudwalk/dcm-zed-extension.git
cd dcm-zed-extension
cargo build --release
```

This produces `extension.wasm` in the repository root, along with `extension.toml` and other build artifacts.

```bash
.
├── Cargo.lock
├── Cargo.toml
├── Justfile
├── LICENSE
├── README.md
├── extension.toml
├── extension.wasm
├── src
└── target
```

### Install Into Zed (Recommended: Dev Extension)

The recommended way to install a locally-built extension is via **Install Dev Extension**:

1. Open the Extensions panel in Zed: `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows)
2. Click the **Install Dev Extension** button (or run `zed: install dev extension` from the command palette)
3. Select the directory containing the cloned `dcm-zed-extension` folder
4. Zed will load the extension immediately

## Configuration

Add to your Zed settings.json:

```json
{
  "dcm": {
    "dart_sdk_path": "/path/to/dart-sdk",  // Optional: Path to Dart SDK
    "executable_path": "/path/to/dcm",     // Optional: Path to DCM executable
    "show_unused_code": false,             // Show unused code issues
    "show_unused_files": false,            // Show unused file issues
    "disable_baseline": false,             // Disable baseline filtering
    "enable_old_formatter": false          // Use pre-Dart 3.7 formatter
  }
}
```

## DCM Doc

DCM has added a documentation on their official website to support [DCM and Zed integration](https://dcm.dev/docs/ide-integrations/zed/). 

## Usage

The extension automatically activates when you open Dart or Flutter projects. It will:

1. Start the DCM language server
2. Analyze your code in real-time
3. Show diagnostics inline
4. Provide code actions for quick fixes

## Commands

Available commands in Zed command palette:

- **Restart Analysis Server**: Restart the DCM analysis server
- **Fix All Auto-fixable Problems**: Apply all available auto-fixes
- **Format Current File**: Format the current Dart file
- **Show/Hide Baseline Issues**: Toggle baseline issue visibility
- **Show/Hide Unused Code Issues**: Toggle unused code detection
- **Show/Hide Unused File Issues**: Toggle unused file detection

## License

See LICENSE file for details.

## Support
- [Discord](https://discord.gg/Vzjprgk4sb)
- [Email](mailto:info@dcm.dev)
- [Website](https://dcm.dev/)