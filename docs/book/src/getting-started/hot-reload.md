# Hot Reload Development

Hot reload is Junita's **sub-100ms iteration** feature that lets you edit your UI in real-time without stopping your app. This is a game-changer for developer productivity—edit a widget, see the change instantly, preserve all your app state.

## What is Hot Reload?

Hot reload watches your `.junita` files for changes. When you save:

1. **File detection** (5ms) — File watcher detects your change
2. **Batching** (300ms) — Changes are debounced (prevents 10 edits = 10 recompiles)
3. **Compilation** (20-50ms) — Junita compiler parses and validates your code
4. **Diffing** (5-20ms) — Fine-grained diff algorithm computes minimal updates
5. **Application** (5-10ms) — Changes applied to running widget tree
6. **Frame request** (2-3ms) — GPU signals a re-render

**Total: < 100ms from save to visual update**

```
┌─────────────────────────────────────────────────┐
│ Edit counter.junita (save)                      │
└──────────────────┬──────────────────────────────┘
                   ↓ [5ms]
            ┌─────────────────┐
            │ File Watcher    │
            └────────┬────────┘
                     ↓ [300ms batch]
            ┌─────────────────┐
            │ Compilation     │  (parse + validate)
            └────────┬────────┘
                     ↓ [20-50ms]
            ┌─────────────────┐
            │ Tree Diffing    │  (compute changes)
            └────────┬────────┘
                     ↓ [5-20ms]
            ┌─────────────────┐
            │ Widget Updates  │  (apply diffs)
            └────────┬────────┘
                     ↓ [5-10ms]
            ┌─────────────────┐
            │ Frame Request   │  (GPU render)
            └────────┬────────┘
                     ↓ [2-3ms]
        ┌────────────────────────┐
        │ UI Updated on Screen   │
        └────────────────────────┘
               < 100ms total
```

## Key Features

### State Preservation
Your app's state is automatically preserved during hot reload. If you have:

```Junita
@widget Counter {
  @state count: Int = 0
  
  @render {
    text: str(count)
    onclick: |_| {
      count = count + 1
    }
  }
}
```

When you click the button 5 times (`count = 5`), then edit the widget, your `count` value stays at 5. Zero state loss.

### Animation-Safe Updates
Spring animations and keyframe timelines resume smoothly. If a slide-in animation is mid-flight when you edit, it completes naturally—no jank, no restart.

### Tree-Aware Diffing
Junita's hot reload uses a **4-type diff algorithm**:

- **Updated**: Property changed (color, text, layout)
- **Added**: New widget inserted
- **Removed**: Widget deleted
- **Reordered**: Children rearranged

Only these specific nodes re-render. Everything else is untouched.

## Getting Started

### Prerequisites

1. **Junita CLI installed** (see [Installation](./installation.md))
2. **A .junita project** with `junita.toml` config file
3. **Your favorite editor** (VS Code, Vim, Neovim, etc.)

### Start Hot Reload Server

```bash
cd your-junita-project
junita dev
```

This command:
- Watches `src/` for `.junita` file changes
- Watches `assets/` and `animations/` for resource changes
- Compiles incrementally (only changed files)
- Sends diffs to your running app
- Preserves state across rebuilds

### Make Your First Edit

1. Open `src/main.junita` in your editor
2. Change a color or text property:

```diff
  @widget Button {
    @prop label: String = "Click me"
-   background: rgb(0, 100, 255)
+   background: rgb(255, 100, 0)  
  }
```

3. Save the file
4. Watch the UI update in < 100ms ✓

## Configuration

Create a `junita.toml` in your project root:

```toml
[hot_reload]
enabled = true
watch_extensions = ["junita", "bl"]
debounce_ms = 300
ignore_patterns = ["target/", ".git/", "node_modules/"]
```

| Setting | Default | Description |
|---------|---------|-------------|
| `enabled` | `true` | Enable/disable hot reload |
| `watch_extensions` | `["junita", "bl"]` | File extensions to watch |
| `debounce_ms` | `300` | Batch file changes within this window (prevents flicker) |
| `ignore_patterns` | See above | Paths to ignore during watching |

## Workflow: Edit → See → Test

Here's the optimal hot reload workflow:

### 1. Start Your App

```bash
cd my-app
junita dev
```

Your app runs and watches for changes.

### 2. Edit in Your Editor

Make changes to `.junita` files—change a layout, add a widget, tweak colors. Save.

### 3. See Instant Feedback

< 100ms later, the UI updates. No restart needed.

### 4. Test Interactions

- Click buttons, test state changes
- Verify animations work
- Check responsive layouts

### 5. Iterate

Edit again, save, see changes. Repeat.

**Result**: Iterate 30 times in 1 hour instead of 5 times (because 25 minutes isn't spent waiting for recompilation).

## State Preservation Deep Dive

When you edit a widget, Junita:

1. **Snapshots the current state** before recompilation
   - All `@state` variables
   - All derived values
   - Animation frames (springs, timelines)

2. **Compiles the new code** with your changes

3. **Diffs the widget trees** — old tree vs new tree

4. **Applies minimal updates** to the scene graph

5. **Restores preserved state** to the updated widgets

This means:

- ✅ Form inputs keep their text
- ✅ Counters keep their count
- ✅ Animations continue smoothly
- ✅ Scroll position preserved
- ✅ Modal dialogs stay open
- ✅ Timers continue running

## Advanced: What Triggers Recompilation?

Hot reload recompiles when you change:

### Always Recompile:
- Widget definitions (`@widget Name { ... }`)
- State variables (`@state x: Type = default`)
- Properties (`@prop name: Type`)
- Render bodies (`@render { ... }`)
- Paint code (`@paint { ... }`)
- Animation definitions (`@animation ...`)
- State machines (`@machine ...`)
- Spring configs (`@spring ...`)

### Don't Trigger Recompile:
- Comments
- Whitespace/formatting
- Import order (same set of imports)

This optimization means editing a comment doesn't trigger a rebuild (future enhancement).

## Performance Metrics

On a typical machine:

| Stage | Time | Notes |
|-------|------|-------|
| File detection | 5-10ms | Re-scan watched dirs |
| Batching/debounce | 300ms | Waits for file changes to settle |
| Compilation | 20-50ms | Depends on file size and complexity |
| Tree diffing | 5-20ms | O(n) algorithm, scales linearly |
| Widget updates | 5-10ms | Scene graph mutation |
| GPU frame request | 2-3ms | Signal to render |
| **Total (P50)** | **50ms** | Typical case |
| **Total (P95)** | **120ms** | Larger files or slow disk |

Even on a slow machine, hot reload beats manual restart:
- Manual restart: 2-5 seconds
- Hot reload: 50-120ms
- **Speedup: 20-100x**

## Debugging Hot Reload

### Check if hot reload is running

```bash
junita dev --verbose
```

Look for log lines like:
```
[hot_reload] Watching src/
[hot_reload] Debounce window: 300ms
[hot_reload] Detected change: src/widgets/button.junita
[compile] Compiling button.junita
[compile] Tree diff computed: 2 updated, 0 added, 0 removed
[hot_reload] Applied 1 update in 18ms
```

### File changes not detected?

1. Check file extension is `.junita` or `.bl`
2. Verify file is in watched directory (default: `src/`)
3. Check `ignore_patterns` in `junita.toml` doesn't exclude the file
4. Try a larger `debounce_ms` value (some file systems are slow)

### Changes not applying?

1. Check compilation succeeded (no error in logs)
2. Verify syntax is correct (parser validates braces, parens)
3. Check widget name export is correct
4. Try restarting with `Ctrl+C` then `junita dev` again

### State not preserved?

Hot reload preserves state automatically. If state is lost:

1. Check you're using `@state` (not local variables)
2. Verify state variable names didn't change
3. Check types are compatible (changing `count: Int` to `count: String` will lose the value)
4. Large state objects (100MB+) might not serialize—use smaller state

## Tips & Tricks

### Tip 1: Use Small State Objects

Hot reload snapshots your entire state. Smaller objects = faster serialization:

```Junita
// ✅ Good: ~10KB state, snaps in 2ms
@state ui_open: Bool = false

// ⚠️ Risky: 10MB state, snaps in 200ms (overshadows gains)
@state large_image_buffer: Vec<u8> = vec![...]
```

Move large data to constants or resources.

### Tip 2: Leverage the Debounce Window

The 300ms debounce means multiple saves in quick succession = single recompile. Great for auto-saving editors (VSCode, Vim with auto-save):

```bash
# Each keystroke in VS Code triggers auto-save
# But hot reload batches all changes within 300ms window
# Result: One recompile for 5 edits instead of 5 recompiles
```

### Tip 3: Test Layout Changes Live

Hot reload excels at layout iteration. Edit padding, margins, flexbox:

```Junita
@widget Card {
  @prop padding: f32 = 16.0
  
  @render {
    // Edit padding in real-time!
  }
}
```

No restart needed. See spacing changes instantly.

### Tip 4: Combine with Responsive Breakpoints

Use hot reload + window resize to test responsive layouts:

1. Edit `@prop screen_width: f32` and responsive conditions
2. Drag window to test breakpoints
3. Hot reload updates as you resize
4. Perfect for mobile-first design

### Tip 5: Disable Hot Reload for Testing

Before shipping, disable hot reload in CI:

```bash
# Production build (no hot reload overhead)
junita build --release
```

## Common Patterns

### Pattern 1: Real-Time Color Tweaking

```Junita
@widget Themed {
  @prop accent: color = rgb(0, 100, 255)
  
  @render {
    background: accent
    border_color: accent.darken(0.3)
  }
}
```

Edit the accent color and watch the whole theme update in real-time.

### Pattern 2: Animation Tuning

```Junita
@animation slide {
  duration: 300ms
  easing: cubic-bezier(0.25, 0.46, 0.45, 0.94)
  // Edit easing mid-animation, see curve change
}
```

### Pattern 3: Dynamic Copy Testing

```Junita
@widget Form {
  @render {
    title: "Sign Up"
    subtitle: "Enter your details"
    // Change copy and see layout reflow instantly
  }
}
```

## Limitations & Future

### Current Limitations

1. **File size**: Very large files (> 10K lines) compile slower (~500ms)
2. **Complex machines**: Large state machines recompile slower
3. **No network hot reload**: Changes only apply locally (remote device support coming)
4. **Breaking type changes**: Renaming a state var of different type loses data

### Future Enhancements

- 🔮 **Browser device sync**: Wireless hot reload to phones
- 🔮 **Snapshot/restore UI**: Save and restore full UI state between sessions
- 🔮 **Collaborative editing**: Multiple devs editing same project, all see updates
- 🔮 **Undo/redo during hot reload**: Revert code changes with one keystroke
- 🔮 **Visual inspector**: Click UI elements to jump to code

## Summary

Hot reload is a **developer experience multiplier**. It removes the compile-restart-test cycle, letting you focus on building beautiful UIs.

**Key takeaways:**

✅ Edit `.junita` files, see changes in < 100ms  
✅ State preserved automatically  
✅ Fine-grained diffing (only changed nodes re-render)  
✅ Animations continue smoothly  
✅ Configurable via `junita.toml`  
✅ Works with REPL, debugger, and other dev tools  

**Next steps:**

- Start `junita dev` in your project
- Edit a widget property
- Watch it update in real-time
- Build faster 🚀

---

**Questions?** See [Event Handling](../core/events.md) for state manipulation patterns or [State Management](../core/state.md) for advanced reactive concepts.
