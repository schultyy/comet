# comet

> local CI

## Configuration

Configure a new Rust project:

```json
{
  "language": "rust",
  "script": [
    "cargo test"
  ]
}
```

Currently supported languages:

- Rust

Then run comet:

```
$ comet
```

If you want to run a project from another directory, run:

```
$ comet -p /Users/jane/other_project
```

To run comet in watch mode, use `-w`:

```
$ comet -p /Users/jane/other_project -w
```
