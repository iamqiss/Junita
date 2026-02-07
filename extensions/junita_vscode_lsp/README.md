# Junita DSL for VS Code

Add syntax highlighting, language support, and hot reload integration for Junita UI Framework (`.junita`, `.bl` files) in VS Code.

## Features

- ✨ **Syntax Highlighting** - Colored keywords, decorators, types, and strings
- 📝 **Language Configuration** - Auto-indent, bracket matching, fold regions
- 🎨 **File Icons** - Beautiful Junita logo icon for `.junita` and `.bl` files
- 🔄 **Hot Reload Commands** - Connect to local hot reload server
- 💡 **Autocomplete** - Smart completions for decorators and types
- 🎯 **Hover Info** - Documentation on hover for decorators
- 🔧 **Formatting** - Format Junita code with one command

## Installation

1. Download this extension from the VS Code Marketplace (search "junita-dsl")
2. Or build locally: 
   ```bash
   npm install
   npm run compile
   # Then install from Extensions: Install from VSIX...
   ```

## Quick Start

### File Association
VS Code automatically recognizes `.junita` and `.bl` files with Junita language mode.

### Autocomplete
Start typing `@` to see decorator suggestions:
- `@widget` - Define a widget component
- `@state` - Declare a reactive state variable
- `@prop` - Declare a property
- `@animation` - Declare animations
- `@machine` - Declare state machines
- `@render` - Define layout
- And more...

### Formatting
Command palette (Ctrl+Shift+P):
- **Junita: Format Document** - Auto-format `.junita` code

### Hot Reload
Command palette:
- **Junita: Connect to Hot Reload** - Connect to running `junita dev` server

## Syntax

The extension supports Junita DSL syntax:

```junita
@widget Counter {
  @prop initial_count: Int = 0
  
  @state count: Int = initial_count
  
  @derived display: String = str(count)
  
  @animation increment {
    duration: 300ms
    easing: ease-out
  }
  
  @render {
    text: display
    onclick: |_| { count = count + 1 }
  }
}
```

## Configuration

No additional configuration needed! The extension works out-of-the-box.

## Troubleshooting

**Junita files not recognized?**
- Reload VS Code window (Cmd+R on Mac, Ctrl+Shift+F5 on Windows/Linux)
- Check file extension is `.junita` or `.bl`

**Icons not showing?**
- Select file icon theme: File → Preferences → File Icon Theme → "Junita Icons"

**Autocomplete not working?**
- Make sure you have the extension installed and active
- Restart VS Code

## Development

### Build
```bash
npm install
npm run compile
```

### Watch Mode
```bash
npm run watch
```

### Test
```bash
npm test
```

### Package for Distribution
```bash
npm install -g @vscode/vsce
vsce package
```

## Contributing

Contributions welcome! Please open issues or PRs on [GitHub](https://github.com/iamqiss/Junita).

## License

Same as Junita framework - see LICENSE file in main repo.

## Related

- [Junita Framework](https://github.com/iamqiss/Junita) - GPU-accelerated UI framework
- [Zyntax Grammar](https://github.com/darmie/zyntax) - Parser framework used by Junita
