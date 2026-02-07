# Component Library Overview

`junita_cn` is a comprehensive component library for Junita UI, inspired by [shadcn/ui](https://ui.shadcn.com/). It provides 40+ production-ready, themeable components built on top of `junita_layout`.

## Installation

Add `junita_cn` to your `Cargo.toml`:

```toml
[dependencies]
junita_cn = { path = "path/to/junita_cn" }
```

## Quick Start

```rust
use junita_cn::prelude::*;

fn build_ui() -> impl ElementBuilder {
    div()
        .flex_col()
        .gap(16.0)
        .p(24.0)
        .child(
            card()
                .child(card_header()
                    .child(card_title("Welcome"))
                    .child(card_description("Get started with junita_cn")))
                .child(card_content()
                    .child(text("Beautiful, accessible components.")))
                .child(card_footer()
                    .child(button("Get Started")))
        )
}
```

## Design Principles

### Composable

Components are built from smaller primitives that can be combined:

```rust
// Compose dialog from parts
dialog()
    .child(dialog_trigger().child(button("Open")))
    .child(dialog_content()
        .child(dialog_header().child(dialog_title("Title")))
        .child(/* content */)
        .child(dialog_footer().child(button("Close"))))
```

### Themeable

All components use theme tokens and automatically support dark mode:

```rust
// Components adapt to theme automatically
button("Click me") // Uses theme.colors.primary

// Override theme
ThemeState::set_color_scheme(ColorScheme::Dark);
```

### Accessible

Components include keyboard navigation and proper semantics:

- Focus management
- Keyboard shortcuts
- Screen reader support (planned)

## Component Categories

| Category | Components |
|----------|------------|
| **Buttons** | Button |
| **Cards** | Card, CardHeader, CardContent, CardFooter |
| **Dialogs** | Dialog, AlertDialog, Sheet, Drawer |
| **Forms** | Input, Textarea, Checkbox, Switch, Radio, Select, Slider |
| **Navigation** | Tabs, DropdownMenu, ContextMenu, Breadcrumb, Sidebar |
| **Feedback** | Alert, Badge, Progress, Spinner, Skeleton, Toast |
| **Layout** | Avatar, Separator, AspectRatio, ScrollArea, Accordion |
| **Data** | Tooltip, HoverCard, Popover, Chart |

## Prelude

Import common components with the prelude:

```rust
use junita_cn::prelude::*;

// Includes:
// - All component builders (button, card, dialog, etc.)
// - Variant enums (ButtonVariant, AlertVariant, etc.)
// - Size enums (ButtonSize, AvatarSize, etc.)
// - Common types and traits
```

## Next Steps

- [Button](./button.md) - Learn about button variants and usage
- [Card](./card.md) - Build card-based layouts
- [Dialog](./dialog.md) - Create modal dialogs
- [Form Components](./form.md) - Build forms with inputs
