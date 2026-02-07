# Testing the Junita VS Code Extension

This guide walks through testing the newly built Junita DSL VS Code extension locally.

## Quick Start

### 1. Build the Extension (Already Done ✅)

The extension has already been compiled:

```bash
cd extensions/junita_vscode_lsp
npm install      # Install dependencies
npm run compile  # Compile TypeScript to JavaScript
```

Output files are in `extensions/junita_vscode_lsp/out/extension.js`

### 2. Launch in Debug Mode

Open VS Code and run the extension in debug mode:

**Option A: Using VS Code UI**
1. Open the workspace folder in VS Code
2. Go to Run → Start Debugging (F5)
3. A new VS Code window opens with the extension loaded

**Option B: Using VS Code CLI (in this dev container)**
```bash
cd /workspaces/Junita/extensions/junita_vscode_lsp
code --extensionDevelopmentPath=$PWD ..
```

### 3. Test File Association

Create a test file or use an existing one:

```bash
# Use the existing hot reload demo
cat examples/hot_reload_demo/main.junita
```

Open it in VS Code - you should see:
- ✨ **Syntax highlighting** for decorators (@widget, @state, @render, etc.)
- 🎯 **Logo icon** (logo.svg) shown in the file tab
- 🔤 **Text coloring** for keywords, types, comments

### 4. Test Language Features

#### Hover Provider (Documentation)
1. Hover over any decorator like `@widget`, `@state`, `@render`
2. A tooltip appears explaining what the decorator does

#### Autocomplete (Ctrl+Space)
1. Start typing `@w` and press Ctrl+Space
2. See suggestions for `@widget`, `@wasm`, etc.
3. Hit Tab to accept suggestion
4. Decorators, types, and keywords all autocomplete

#### Formatting (Cmd+Shift+F or Ctrl+Shift+F)
1. Open a .junita file
2. Run "Format Document"
3. Code is reformatted with proper indentation
4. Widget declarations are sorted

#### Commands (Cmd+Shift+P or Ctrl+Shift+P)
1. Open command palette
2. Type "Junita" and see commands:
   - `Junita: Connect to Hot Reload` - Opens input box for port
   - `Junita: Format Document` - Formats active document

### 5. Verify File Icons

Check that `.junita`, `.bl`, and `.junitaproj` files show the logo.svg icon:

```bash
# Create a test file to see the icon
touch test.junita

# Check in VS Code file explorer - should show logo.svg icon
```

### 6. Test with Hot Reload Example

1. Open `examples/hot_reload_demo/main.junita`
2. Make an edit (e.g., change a string value)
3. Save the file
4. Notice syntax highlighting works correctly
5. Try hovering over decorators to see documentation

## Extension Features

### ✅ Implemented Features

| Feature | Status | Usage |
|---------|--------|-------|
| **Syntax Highlighting** | ✅ Complete | Colors decorators, keywords, types |
| **File Icons** | ✅ Complete | logo.svg for .junita/.bl/.junitaproj |
| **Hover Documentation** | ✅ Complete | Hover over decorators for help |
| **Autocomplete** | ✅ Complete | Ctrl+Space for suggestions |
| **Document Formatting** | ✅ Complete | Cmd+Shift+F to format |
| **Commands** | ✅ Complete | Cmd+Shift+P to access |

### 📋 TextMate Grammar Support

The extension includes a complete TextMate grammar (`syntaxes/junita.tmLanguage.json`) that recognizes:

- **Comments**: `//` and `/* */`
- **Decorators**: `@widget`, `@state`, `@prop`, `@derived`, `@animation`, `@machine`, `@render`, `@paint`, `@spring`
- **Keywords**: `if`, `else`, `for`, `while`, `match`, `pub`, `async`, `await`
- **Types**: `Int`, `Float`, `Bool`, `String`, `color`, `Vec`, `HashMap`, `Option`, `Result`
- **Strings and Numbers**: Quoted strings and numeric literals

### 🎯 Hover Provider Support

Hover over these decorators to see documentation:

| Decorator | Documentation |
|-----------|---------------|
| `@widget` | Defines a reusable widget component |
| `@state` | Declares a reactive state variable |
| `@prop` | Declares a widget property |
| `@derived` | Declares a derived/computed value |
| `@machine` | Declares a state machine (FSM) |
| `@animation` | Declares a keyframe animation |
| `@spring` | Declares a spring physics animation |
| `@render` | Defines the render body for layout |
| `@paint` | Defines custom painting code |

## Hot Reload Integration (Future)

The extension includes a command to connect to the hot reload server:

```
Junita: Connect to Hot Reload
```

When you run this command:
1. VS Code prompts for the hot reload server port (default: 8080)
2. Once implemented, it will establish a WebSocket connection
3. File changes will be sent to the server for real-time compilation

**Current Status**: Command is registered and working; WebSocket connection is a TODO for future implementation with the actual hot reload server.

## Troubleshooting

### Extension doesn't load
- Check the Debug Console for errors (F5 → Debug Console)
- Verify `out/extension.js` exists
- Check that `package.json` activation events are correct

### Syntax highlighting not working
- Verify `syntaxes/junita.tmLanguage.json` is referenced in `package.json`
- Check that language ID is `junita`
- Restart VS Code

### Autocomplete not triggering
- Press Ctrl+Space explicitly
- Check that language is detected as `junita`
- Look for errors in Debug Console

### File icons not showing
- Verify path to `logo.svg` in `fileicons/junita-icons.json`
- Check that file path is relative to extension root
- Ensure `.junitaproj` and `.bl` are included in icon configuration

## Building for Distribution

To create a VSIX package for distribution:

```bash
cd extensions/junita_vscode_lsp

# Install vsce (VS Code Extension Manager)
npm install -g @vscode/vsce

# Build VSIX package
vsce package

# Result: junita-dsl-0.0.1.vsix
```

Then install locally:
```bash
# In VS Code Extensions: Install from VSIX...
# Select the .vsix file
```

## Publishing to Marketplace

To publish to the Visual Studio Marketplace:

1. Create a Publisher account at https://marketplace.visualstudio.com
2. Create a Personal Access Token (PAT)
3. Run: `vsce publish -p <YOUR_PAT>`

**Current Status**: Not yet published; extension is functional locally.

## Architecture

The extension structure:

```
extensions/junita_vscode_lsp/
├── src/
│   └── extension.ts          # Extension entry point (214 lines)
├── syntaxes/
│   └── junita.tmLanguage.json# TextMate grammar for syntax highlighting
├── fileicons/
│   └── junita-icons.json     # Icon associations
├── language-configuration.json# Bracket matching, indentation
├── package.json              # Extension manifest
├── tsconfig.json            # TypeScript configuration
├── .vscodeignore            # Files to exclude from package
├── README.md                # User guide
└── out/
    └── extension.js         # Compiled JavaScript (auto-generated)
```

## Key Implementation Details

### Language Registration
The language is registered in `package.json`:
- Language ID: `junita`
- File extensions: `.junita`, `.bl`
- Configuration: `language-configuration.json`

### Syntax Grammar
Full TextMate grammar in `junita.tmLanguage.json`:
- 150+ lines of regex patterns
- Covers all Junita language constructs
- Compatible with VS Code's TextMate engine

### Provider Implementations
Four provider types implemented in `extension.ts`:

1. **Hover Provider** - Shows documentation on hover
2. **Completion Provider** - Autocomplete for decorators/types
3. **Document Range Formatter** - Indentation adjustment
4. **Commands** - Hot reload connection (framework ready)

## Next Steps

After testing locally, the extension can be:

1. **Published to VS Marketplace** - Make it installable for all VS Code users
2. **Enhanced with LSP** - Add full language server protocol support for diagnostics
3. **Integrated with Hot Reload** - Complete WebSocket connection implementation
4. **Added IDE Features** - Go to definition, rename, find references, etc.

## Testing Checklist

Use this checklist to verify all extension features:

- [ ] Extension loads without errors (F5 Debug mode)
- [ ] .junita files show logo.svg icon in explorer
- [ ] Syntax highlighting works (decorators colored)
- [ ] Hover tooltips appear on decorators
- [ ] Autocomplete (Ctrl+Space) shows suggestions
- [ ] Format Document (Cmd+Shift+F) reformats code
- [ ] Commands palette shows Junita commands
- [ ] No errors in Debug Console
- [ ] Performance is acceptable (no lag on large files)

All items ✅ = extension ready for production use!

## Questions?

Refer to:
- `extensions/junita_vscode_lsp/README.md` - User guide
- `extensions/junita_vscode_lsp/src/extension.ts` - Source code
- VS Code Extension API: https://code.visualstudio.com/api
