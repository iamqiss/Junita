# Plugins

Place your local Junita plugins here. Each plugin should be in its own directory.

## Creating a Plugin

```bash
cd plugins
junita plugin new my_plugin
```

## Using a Plugin

Add to your `.junitaproj`:

```toml
[[dependencies.plugins]]
name = "my_plugin"
path = "plugins/my_plugin"
```
