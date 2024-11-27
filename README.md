<img width="350" src="/doc/logo.png" />

<div>
  
  [![Lint CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Lint)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Code Style CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Code%20style)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Crates.io Version](https://img.shields.io/crates/v/CodeSnap?logo=rust&color=%232ecc71)](https://crates.io/crates/codesnap)
  [![Lint convention](https://img.shields.io/badge/wizardoc--commit--convention-%233498db?style=flat&logo=lintcode&logoColor=white&link=https%3A%2F%2Fgithub.com%2Fwizardoc%2Fcommitlint-wizardoc)](https://github.com/wizardoc/commitlint-wizardoc)
  
</div>


## CodeSnap
> [!WARNING]  
> This project is still WIP, please do not use CodeSnap in production env.

CodeSnap is a pure Rust tool for generate beautiful code snapshots, it directly use graphic engine to generate snapshots, which means the entire process is just matter of computation and rendering, without need for network or something like browser-based rendering solution.

Generally, you can directly use CLI tool provide by CodeSnap to generate code snapshots you want. Or CodeSnap also provide a library for you to integrate it into your own project, so you can generate code snapshots in your own way (See [Related projects](#) for more information).

<img src="https://github.com/user-attachments/assets/b8c9490f-ce17-4881-9d36-72e9c17bf34b" width="580px" />


## Getting started
CodeSnap provide two ways to use it, you can use it as a CLI tool or as a library in your own project.

### CLI
For CLI tool, you can install it via `cargo`:

**Cargo**
```bash
cargo install codesnap-cli
```

**Homebrew**
```bash
brew install mistricky/tap/CodeSnap
```

Use `codesnap` command to generate code snapshot:

```bash
# Run codesnap to generate code snapshot by providing code file
codesnap -f ./code_snippet.hs -o "./output.png"

# Run codesnap --help to see more information
codesnap -h
```

Read more about [codesnap cli]()

### Library
For library, add `CodeSnap` in your project using Cargo

```bash
cargo add codesnap
```

Use `CodeSnap` builder to generate code snapshot:

```rust
let snapshot = CodeSnap::default()
        .code("fn main() { println!(\"Hello, world!\"); }")
        .watermark(WatermarkBuilder::default().content("CodeSnap").build()?)
        .build()?
        .create_snapshot()?;

// Save snapshot to file
snapshot.save("output.png")?;

// Copy snapshot to clipboard 
snapshot.copy_to_clipboard()?;
```

Read more about [codesnap library]()

## Related projects
- [codesnap](https://github.com/mistricky/CodeSnap/tree/main/core)
- [codesnap-cli](https://github.com/mistricky/CodeSnap/tree/main/cli)
- [codesnap.nvim](https://github.com/mistricky/codesnap.nvim)
- [codesnap.idea](https://github.com/RAOE/CodeSnap.idea)
- codesnap.vscode (Planning)
- codesnap.zed (Planning)


## License
MIT.
