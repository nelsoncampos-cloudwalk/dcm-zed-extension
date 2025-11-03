# DCM for Zed

DCM (Dart Code Metrics) extension for Zed IDE - provides static analysis for Dart and Flutter projects.

## Features

- **Static Analysis**: Quickly find quality and consistency problems in your Dart code
- **Code Metrics**: Analyze code complexity and maintainability
- **Unused Code Detection**: Find and remove unused code (beta)
- **Unused Files Detection**: Identify unused files in your project (beta)
- **Auto-fixes**: Apply quick fixes for common issues
- **Baseline Support**: Hide known issues using baseline

## Installation

### Prerequisites

1. Install DCM CLI tool:
   ```bash
   dart pub global activate dcm
   ```

2. Ensure DCM is in your PATH or configure the executable path in settings

### Building from Source

1. Install Rust via [rustup](https://rustup.rs/)
2. Clone this repository
3. Build the extension:
   ```bash
   cd dcm-zed-extension
   just compile
   ```
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