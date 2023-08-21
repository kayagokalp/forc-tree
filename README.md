# forc-tree

`forc-tree` is a simple forc plugin which prints dependency tree of a given sway project. Currently it does not support workspaces.


Example usage:

```console
# Inside a forc project.
forc-tree

# Also possible to use in any arbitrary location with `--path` argument.
forc-tree --path ./test-project

# It is possible to force dependency graph generation to work in offline mode.
forc-tree --offline
```
