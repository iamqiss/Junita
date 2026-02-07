# Junita LSP Diagnostics Guide

## 🎯 What's New

The Junita Language Server now provides **real-time error detection** with visual feedback in VS Code!

## 📋 Diagnostic Types

### 1. **Duplicate Declaration Errors**
When you define two widgets, machines, or animations with the same name, the LSP reports an error:

```junita
@widget Button { }
@widget Button { }  // ❌ ERROR: Duplicate widget name 'Button'
```

**Error Message**: `Duplicate widget name: 'Button'`  
**Severity**: Error (red squiggle)  
**Impact**: Can cause naming conflicts at runtime

### 2. **Invalid Decorator Errors**
Using unknown or misspelled decorators:

```junita
@widgetx MyComponent { }  // ❌ ERROR: Unknown decorator
@invalid MyMachine { }     // ❌ ERROR: Unknown decorator
```

**Error Message**: `Unknown decorator. Valid decorators: @widget, @machine, @animation`  
**Severity**: Error (red squiggle)  
**Valid Decorators**: 
- `@widget` - Define UI components
- `@machine` - Define state machines
- `@animation` - Define animations

## 🎨 Visual Indicators

In VS Code, errors appear as:
- **Red squiggly underlines** under the problematic code
- **Error count badge** in the file tab and status bar
- **Problems panel** (Ctrl+Shift+M) showing all issues

## 📍 Error Information

Each diagnostic includes:
- **Line number** - Where the error occurred
- **Column** - Starting position of the error
- **Message** - Human-readable explanation
- **Severity** - Error, Warning, or Information level
- **Source** - Marked as "junita" for filtering

## 🧪 Testing Diagnostics

Open [examples/diagnostics_demo/demo.junita](../examples/diagnostics_demo/demo.junita) to see:
- ✅ Valid definitions (no errors)
- ❌ Duplicate names
- ❌ Invalid decorators
- Live real-time error updates as you type

## 🔧 Implementation Details

### Parser Changes
The `junita-parser` crate now:
- Tracks declared names in a HashSet
- Validates decorator syntax
- Collects all errors during parsing
- Returns `JunitaDocument` with `diagnostics` field

### LSP Server Changes
The `junita-lsp` server now:
- Implements `textDocument/publishDiagnostics` notification
- Publishes diagnostics on document open
- Publishes diagnostics on document change (real-time)
- Clears diagnostics when document closes

### LSP Capability
The extension advertises:
```json
"diagnosticProvider": {
  "interFileDependencies": false,
  "workspaceDiagnostics": false
}
```

## 📚 Future Enhancements

Planned additions:
1. **More error types**:
   - Invalid syntax in properties
   - Missing required decorators
   - Circular dependencies in imports
   
2. **Warnings and hints**:
   - Unused declarations
   - Naming conventions (camelCase, PascalCase)
   - Performance warnings
   
3. **Quick fixes**:
   - Auto-rename duplicates
   - Suggest correct decorator names
   - Auto-import modules

## 🚀 Usage in Development

The diagnostics help you:
1. **Catch errors early** - Real-time feedback as you type
2. **Understand issues** - Clear, actionable error messages
3. **Fix quickly** - Line and column information for quick navigation
4. **Prevent bugs** - Duplicate names could cause runtime issues

## 💡 Pro Tips

- Use the Problems panel (Ctrl+Shift+M) to see all errors at once
- Search in Problems panel to filter by error type
- File icons show a warning indicator when the file has errors
- Hover over the error to see the full message
