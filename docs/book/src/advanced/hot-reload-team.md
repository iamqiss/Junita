# Hot Reload Optimization for Teams

This guide covers how to structure your Junita project and development practices to maximize hot reload benefits across your team.

## Project Structure for Hot Reload

The optimal directory structure minimizes compilation time and maximizes parallelism:

```
my-junita-app/
├── src/
│   ├── main.junita           # app entry point
│   └── widgets/              # individual widget files
│       ├── button.junita     # one widget per file
│       ├── card.junita       # smaller = faster hot reload
│       ├── form.junita
│       └── dialog.junita
├── animations/               # separate animation files
│   ├── slide.junita
│   ├── fade.junita
│   └── bounce.junita
├── machines/                 # state machines
│   ├── auth_flow.junita
│   └── form_validation.junita
├── assets/
│   ├── colors.toml           # shared constants
│   ├── icons/
│   └── fonts/
├── junita.toml              # hot reload config
└── Cargo.toml
```

### Why Small Files?

Junita's compiler processes files incrementally:

- **large file** (1000 lines): 150ms to recompile if one widget changes
- **small file** (100 lines): 20ms to recompile

But with smaller files:
- Only the changed file recompiles
- Other widgets unaffected
- Parallelized compilation (if multiple files change)

**Result**: 100 lines change = 20ms recompile vs 150ms

## Configuration for Teams

Create `junita.toml` in your project root:

```toml
[hot_reload]
enabled = true

# Watch settings
watch_extensions = ["junita", "bl"]
watch_dirs = ["src", "animations", "machines", "assets"]
debounce_ms = 300

# Ignore common build artifacts
ignore_patterns = [
    "target/",
    ".git/",
    "node_modules/",
    "*.tmp",
    ".cache/",
]

# Performance tuning
max_file_size_for_hot_reload = "10MB"
parallel_compilation = true
incremental = true

# Team settings
warnings_as_errors = false  # don't break dev iteration on warnings
format_on_compile = false   # let rustfmt/prettier handle it

[compiler]
optimization_level = "dev"  # prioritize speed over binary size
```

## Development Workflow Patterns

### Pattern 1: Feature Branch Isolation

```bash
# Team member A works on auth flow
git checkout -b feature/auth-redesign
cd my-junita-app
junita dev

# Edits:
# - src/widgets/login_form.junita
# - src/widgets/signup_form.junita
# - animations/auth_flow.junita

# Hot reload applies each change in < 100ms
# No merge conflicts with other branches (different files)
```

### Pattern 2: Parallel Component Development

```bash
# Team Member 1: Working on button-based card
vim src/widgets/card_buttons.junita
# Hot reload: detects changes, recompiles, updates

# Team Member 2: Working on text-based dialog
vim src/widgets/dialog_text.junita
# Hot reload: detects changes, recompiles, updates

# Both apps running simultaneously
# Both seeing their changes in real-time
# No interference (different files)
```

### Pattern 3: Shared Asset Editing

```bash
# Shared color palette (referenced by all widgets)
vim assets/colors.toml

# Hot reload detects asset change
# All widgets using those colors update instantly
# Single source of truth for branding
```

## Performance Tips for Large Teams

### Tip 1: Use Separate Files for Each Widget

❌ **Bad** (monolithic file):
```
src/
└── all_widgets.junita  (5000 lines)
```

Change one button = 5000 line file recompile = 300ms

✅ **Good** (modular):
```
src/
├── button.junita       (100 lines)
├── card.junita         (150 lines)
├── dialog.junita       (200 lines)
└── form.junita         (180 lines)
```

Change one button = 100 line file recompile = 20ms

**15x faster iteration**

### Tip 2: Extract Constants to Separate Files

```
assets/
├── colors.toml
│   primary = rgb(0, 100, 255)
│   secondary = rgb(255, 100, 0)
├── typography.toml
│   heading_size = 24.0
│   body_size = 14.0
└── spacing.toml
   padding_sm = 8.0
   padding_md = 16.0
```

Then reference in widgets:

```Junita
@widget Button {
   background: colors.primary
   padding: spacing.padding_md
}
```

**Benefit**: Edit a color in one place, all widgets update.

### Tip 3: Use Composition Over Nesting

❌ **Inefficient** (deep nesting):
```Junita
@widget ComplexForm {
   @widget Section {
      @widget Field {
         @widget Label { ... }
         @widget Input { ... }
      }
      @widget Field { ... }
   }
}
```

Edit the innermost widget = all parents watch it = slow hot reload

✅ **Efficient** (composition):
```Junita
@widget FormField {
   @prop label: String
   @prop placeholder: String
}

@widget ComplexForm {
   FormField { label: "Email", ... }
   FormField { label: "Name", ... }
}
```

Change FormField = only that file recompiles = fast update

### Tip 4: Lazy Load Large Resources

```Junita
@widget Image {
   @prop path: String
   
   @derived image_data: ImageBuffer = {
      lazy_load(path)  // loaded on first render, not on property change
   }
}
```

This means changing other properties doesn't reload the image.

## CI/CD Integration

### Local Development (Hot Reload Enabled)

```bash
# In your .gitignore
Cargo.lock          # let cargo manage
target/
.hot_reload_cache/

# Start dev server
junita dev --watch --fast

# Formatter runs on save (optional)
# Type checking runs in background (optional)
```

### CI Pipeline (Hot Reload Disabled)

```yaml
# .github/workflows/ci.yml
name: CI

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      # Disable hot reload for CI (unnecessary overhead)
      - run: junita build --release --no-hot-reload
      
      # Run tests (hot reload doesn't apply here)
      - run: cargo test
      
      # Type check
      - run: cargo check --all
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Verify syntax before commit
junita check src/
if [ $? -ne 0 ]; then
   echo "Syntax errors. Fix and try again."
   exit 1
fi

# Format widget files
junita fmt src/

git add src/
```

## Debugging Hot Reload Issues

### Problem: Hot reload applied, but UI looks wrong

```bash
junita dev --verbose
# Look for these logs:

# Check compilation succeeded
[compile] ✓ Compiled src/widgets/button.junita
# Check tree diff computed
[hot_reload] Tree diff: 2 updated, 0 added, 0 removed
# Check update applied
[hot_reload] Applied 1 update in 18ms
```

If compilation failed:
```
[compile] ✗ Error in src/widgets/button.junita:
   Expected '@prop' or '@state', found 'invalid'
```

Fix the syntax error and save.

### Problem: State is lost after hot reload

Check that state variables use `@state`:

```Junita
// ❌ This loses value on hot reload (local variable)
@widget Counter {
   count = 0
   
   @render {
      text: str(count)
   }
}

// ✅ This preserves value (state variable)
@widget Counter {
   @state count: Int = 0
   
   @render {
      text: str(count)
   }
}
```

### Problem: Hot reload is slow (taking > 200ms)

1. Check file size:
   ```bash
   wc -l src/widgets/*.junita
   # If any file > 500 lines, break it into smaller files
   ```

2. Check for expensive operations in state:
   ```Junita
   @state expensive: String = {
      // This runs on every hot reload!
      compute_large_json()
   }
   ```

   Move to:
   ```Junita
   @derived expensive: String = {
      // Only recomputes when dependencies change
      compute_large_json()
   }
   ```

3. Profile the compilation:
   ```bash
   junita dev --profile-compiler
   # Shows where time is spent (parsing, type-check, codegen)
   ```

## Team Guidelines

### Git Strategy

```bash
# Feature branches work well with hot reload:
# - Each dev own branch
# - Own files (no merge conflicts)
# - Hot reload applies changes instantly
# - Merge when feature done

git checkout -b feature/new-button
# Edit src/widgets/button.junita
# Hot reload: instant feedback
# Make PR when done
```

### Code Review for Hot Reload

When reviewing widget changes:

```diff
+ @widget ButtonNew {
+   @prop size: f32 = 16.0
+   @state hover: Bool = false
+
+   @animation hover_animation {
+     duration: 200ms
+     easing: ease-out
+   }
+ }
```

Checklist:
- ✅ File is small enough (< 500 lines)?
- ✅ Uses `@state` not local variables?
- ✅ Animations clear? Springs tuned?
- ✅ Props have sensible defaults?
- ✅ Names are descriptive?

### Merge Strategy

```bash
# Feature complete, tests pass
git checkout main
git merge feature/new-button --ff-only

# Hot reload continues on main branch
# Other devs see your changes when they pull and restart dev server
```

## Best Practices Summary

| Practice | Benefit |
|----------|---------|
| **Small files** (100-200 lines per file) | 20-50ms hot reload instead of 300ms+ |
| **One widget per file** | Clear ownership, easier to find code |
| **Separate animations** | Tweak animations without recompiling widgets |
| **Shared assets** (colors, fonts, spacing) | Single source of truth, DRY |
| **Use @state** | Preserve state across hot reloads |
| **Use @derived** | Efficient computed values |
| **Composition** | Reuse components, faster updates |
| **Feature branches** | Team parallel development |
| **Local hot reload** | 20-100x faster than restart cycles |

## Checklists

### Before Shipping

- [ ] Hot reload works locally (`junita dev`)
- [ ] State preserved correctly (click button, edit code, count still there)
- [ ] Large files broken into smaller modules
- [ ] Animations tuned and smooth
- [ ] All widgets tested with hot reload
- [ ] CI/CD disables hot reload for builds

### During Code Review

- [ ] Changes are isolated to few files?
- [ ] File size reasonable (< 500 lines)?
- [ ] State variables use `@state`?
- [ ] Animations defined separately?
- [ ] No hardcoded values (use assets/)?

### While Developing

- [ ] Terminal running `junita dev`?
- [ ] Editor open next to simulator/app?
- [ ] Save file → see change in < 100ms?
- [ ] State still correct after edit?

## Next Steps

- Set up team's `junita.toml` with optimal settings
- Split large widget files into modules
- Create shared asset files for colors/fonts
- Document team's hot reload workflow
- Train team on hot reload best practices

**Result**: Your team goes from compile-restart-test (2-5s) to edit-save-see (50-100ms) cycles. **20-100x productivity boost**.

---

**See Also**: [Hot Reload Development](./hot-reload.md) for individual developer guide.
