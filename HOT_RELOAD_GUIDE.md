# Junita Hot Reload Integration Guide

## 🔥 Hot Reload - The Killer Feature

Hot Reload is the core feature of Junita that makes development **incredibly fast**. Changes compile and update in real-time as you type, giving you instant visual feedback without manual compilation or page refreshes.

## 🚀 Getting Started

### 1. **Start Hot Reload**

Press **Ctrl+Shift+P** (or Cmd+Shift+P on Mac) and type:
```
Junita: Start Hot Reload
```

Or click the **Hot Reload** status bar item on the right (it will show `⚪ Hot Reload OFF`).

### 2. **Watch the Status Bar**

Once enabled, the status bar will show:
- **🔴 Hot Reload ON** - Hot reload is active and listening for changes
- **⚪ Hot Reload OFF** - Hot reload is inactive
- Click the status bar item to toggle on/off

### 3. **Edit Your Code**

Open any `.junita` file and start editing:

```junita
// Every change auto-compiles!
@widget MyButton {
  label: "Click Me"
  onClick: handleClick
}

@machine ButtonState {
  state: idle
  state: pressed
  state: released
}

@animation Bounce {
  duration: 500ms
  easing: ease-out
}
```

The LSP will show:
- **Green success messages** ✅ - Compilation succeeded
- **Red error messages** ❌ - Compilation failed
- **Detailed compile summaries** in the Hot Reload output panel

## 📊 What Information You Get

When you save a file with hot reload enabled, you'll see:

```
[14:32:15] ✅ Compilation successful!
Widgets: 2
Machines: 1
Animations: 1
Imports: 0
Target: browser
Size: 2048 bytes
```

This tells you:
- **Status** - Did it compile successfully?
- **Widget count** - How many UI components defined
- **Machine count** - How many state machines
- **Animation count** - How many animations
- **Imports** - Dependencies loaded
- **Target** - Which platform (browser, native, fuchsia)
- **Size** - Compiled bytecode size

## 🎨 Features

### Compile on Save
When hot reload is **enabled**, every file save triggers:
1. ✅ Syntax validation
2. ✅ Duplicate detection
3. ✅ Decorator validation
4. ✅ Bytecode compilation
5. ✅ Live update notification

### Real-Time Compilation Feedback
The **Hot Reload** output panel (View > Output > Junita Hot Reload) shows:
- Compilation timestamps
- Success/failure status
- Widget/machine/animation counts
- Size information
- Detailed error messages

### Status Bar Integration
The status bar item:
- Shows current hot reload state
- Clickable to toggle on/off
- Updates in real-time
- Color-coded (when on, it's prominently visible)

## 🔄 Workflow

### Optimal Development Flow

1. **Open your project**
   ```bash
   code /path/to/junita/project
   ```

2. **Start hot reload** (Ctrl+Shift+P > Junita: Start Hot Reload)

3. **Open Hot Reload panel** (View > Output > Junita Hot Reload)

4. **Edit your `.junita` files**
   - Write code
   - Save (Ctrl+S)
   - See instant compilation feedback
   - Fix errors immediately if shown

5. **Monitor the output panel** for:
   - ✅ Success notifications
   - ❌ Error messages
   - 📊 Compilation statistics

6. **Stop when done** (Ctrl+Shift+P > Junita: Stop Hot Reload)

## 🧪 Testing Hot Reload

### Try the Demo File

Open: `examples/hot_reload_demo/showcase.junita`

1. Start hot reload
2. Make edits:
   - Change a widget name
   - Add a new @animation
   - Modify @machine states
3. Watch the Hot Reload panel update in real-time
4. See error messages if you introduce issues

## 🔧 Advanced: How It Works

### Architecture

```
VS Code
  ↓
Junita Extension
  ├─ Status Bar (UI)
  ├─ Output Panel (Feedback)
  └─ Hot Reload Commands
       ↓
  LSP Client (sends commands)
       ↓
Junita LSP Server (Rust)
  ├─ Parser (syntax validation)
  ├─ Compiler (bytecode generation)
  └─ Hot Reload Handler
       ├─ Compile on didChange
       ├─ Publish results
       └─ Send notifications
```

### Commands Implemented

Three commands handle hot reload:

1. **`junita.startHotReload`**
   - Enables compile-on-save
   - Creates output channel
   - Starts server-side hot reload handler

2. **`junita.stopHotReload`**
   - Disables compile-on-save
   - Closes output channel
   - Stops server-side handler

3. **`junita.toggleHotReload`**
   - Toggles between on/off states
   - Connected to status bar click

### Notifications

The server sends real-time notifications for:
- **`junita/hotReloadUpdate`** - Compilation results with metadata

```json
{
  "action": "compiled",
  "message": "✅ Compilation successful!...",
  "widget_count": 2,
  "machine_count": 1,
  "animation_count": 1
}
```

## ⚡ Performance Tips

### Keep Hot Reload Enabled During Development
- No performance penalty when disabled
- Only compiles on save (not continuously)
- Minimal overhead per compile

### Large Files
If you have very large `.junita` files:
- Compilation is still fast (< 100ms)
- Uses efficient regex-based parser
- Suitable for file up to 10k+ lines

### Disable When Not Needed
- Stop hot reload when you're reviewing code
- Saves system resources
- No compile notifications in background

## 🐛 Troubleshooting

### "Hot reload won't start"
- Check if LSP server is running (check Extension host terminal)
- Verify binary exists at: `junita-analyzer/target/release/junita-lsp`
- Try restarting VS Code

### "Compilation failures not showing"
- Check Problems panel (Ctrl+Shift+M) for diagnostics
- Look at Hot Reload output panel (View > Output)
- Check LSP output if issues persist

### "Performance is slow"
- Close other VS Code windows
- Check disk I/O (fast SSDs recommended)
- Monitor CPU usage (should be < 10% during compile)

## 📚 Related Features

### Also Available
- **Diagnostics** - Real-time error detection (Ctrl+Shift+M)
- **IntelliSense** - Hover documentation
- **Completions** - Code suggestions (Ctrl+Space)
- **Go to Definition** - Jump to declarations (Ctrl+Click)

### Coming Soon
- Semantic highlighting (color-coded by token type)
- Document symbols (Ctrl+Shift+O)
- Code formatting (Shift+Alt+F)
- Document renaming support

## 🎉 Next Steps

1. ✅ **Got it working?** Try editing `examples/hot_reload_demo/showcase.junita`
2. ✅ **Create new file?** Make your own `.junita` project
3. ✅ **Want faster?** Check junita-analyzer/target/release/junita-lsp binary size
4. ✅ **Need more?** Check DIAGNOSTICS_GUIDE.md for error detection

---

**Enjoy blazingly fast Junita development!** 🚀⚡
