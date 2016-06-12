# comet

> local CI

## Configuration

To configure a new project, place a `.comet.json` in your project directory:

```json
{
  "script": [
    "cargo test"
  ],
  "watch": "src/"
}
```
Here you see an example configuration for a Rust project.

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
