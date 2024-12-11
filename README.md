<img width="350" src="/doc/logo.png" />

<div>
  
  [![Lint CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Lint)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Code Style CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Code%20style)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Crates.io Version](https://img.shields.io/crates/v/CodeSnap?logo=rust&color=%232ecc71)](https://crates.io/crates/codesnap)
  [![Lint convention](https://img.shields.io/badge/wizardoc--commit--convention-%233498db?style=flat&logo=lintcode&logoColor=white&link=https%3A%2F%2Fgithub.com%2Fwizardoc%2Fcommitlint-wizardoc)](https://github.com/wizardoc/commitlint-wizardoc)
  
</div>


## CodeSnap
> [!WARNING]  
> This project is still in early stage and may have some bugs

CodeSnap is a pure Rust tool for generate beautiful code snapshots, it directly use graphic engine to generate snapshots, which means the entire process is just matter of computation and rendering, without need for network or something like browser-based rendering solution.

Generally, you can directly use CLI tool provide by CodeSnap to generate code snapshots you want. Or CodeSnap also provide a library for you to integrate it into your own project, so you can generate code snapshots in your own way (See [Related projects](#) for more information).


## 📷 Screenshots

<img src="https://github.com/user-attachments/assets/b8c9490f-ce17-4881-9d36-72e9c17bf34b" width="580px" />


## ✨ Features
- **Fast**: Pure Rust tool, generate code snapshot from graphic engine directly.
- **CLI tool**: CodeSnap provide a CLI tool for you to generate code snapshot directly from command line.
- **Library**: CodeSnap also provide a library for you to integrate it into your own project.
- **Line number**: Generate code snapshot with line number, it's really helpful if someone want to know the position of the code snippet.
- **Watermark**: Watermark can help make your code snapshot more personalized and interesting.
- **More beautiful themes**: The [Syntect](https://github.com/trishume/syntect) be treated as the syntax highlighterin CodeSnap, and it using [Sublime Text syntax definitions](https://www.sublimetext.com/docs/syntax.html#include-syntax) to highlight code, so basically you can use any theme that Sublime Text support.
- **Scale**: You can scale your code snapshot with a specific scale factor, CodeSnap will generate treble size snapshot by default to ensure the quality of the snapshot.
- **Beautiful background**: CodeSnap provide a beautiful background for your code snapshot, you can also customize the background color with solid color or gradient color.
- **Multiple snapshot format**: CodeSnap support multiple snapshot format, you can save snapshot as PNG, SVG and even HTML, or you want try ASCII code snapshot :)
- **Clipboard**: CodeSnap can copy snapshot to clipboard directly, or read code snippet from clipboard to generate snapshots.
- **Breadcrumb**: CodeSnap provide a breadcrumb for you to share your code snapshot with code path, it's really helpful if others want to know where the code snippet comes from.


## 💻 Getting started
CodeSnap provide two ways to use it, you can use it as a CLI tool or as a library in your own project.

### CLI
For CLI tool, you can install it for different platforms:

<details>
<summary>Arch Linux</summary>

[codesnap](https://aur.archlinux.org/packages/codesnap) is available on [AUR](https://aur.archlinux.org), you can install it using your preferred AUR helper. Example:

```bash
paru -S codesnap
```

</details>

<details>
<summary>Cargo</summary>

```bash
cargo install codesnap-cli
```

</details>

<details>
<summary>Homebrew</summary>

```bash
brew install mistricky/tap/CodeSnap
```

</details>

Use `codesnap` command to generate code snapshot:

```bash
# Run codesnap to generate code snapshot by providing code file
codesnap -f ./code_snippet.hs -o "./output.png"

# Run codesnap --help to see more information
codesnap -h
```

### Library
For library, add `CodeSnap` in your project using Cargo

```bash
cargo add codesnap
```

Use `CodeSnap` builder to generate code snapshot:

```rust
CodeSnap::default()
        .code("fn main() { println!(\"Hello, world!\"); }")
        .watermark(WatermarkBuilder::default().content("CodeSnap").build()?)
        .build()?
        .create_snapshot()?.raw_data()?.copy()?;
```

## 🌰 Examples
All examples can be found in [examples](https://github.com/mistricky/CodeSnap/tree/main/examples).

![hello](https://github.com/user-attachments/assets/99df51ff-0957-40bd-91d0-facbd46a0bec)



## ⚙️ Configuration
Codesnap can receive a JSON config as input, the config can be used to customize the snapshot, such as theme, background, watermark, etc.

If you are using Library, you can mount config to `CodeSnap` builder:

```rust
CodeSnap::from_config("Your config")?;
```

Or if you are using CLI tool, CodeSnap will generate a default config file for you under `~/.config/CodeSnap`, you can modify the config file to customize the snapshot:

```jsonc
// Both "CaskaydiaCove Nerd Font" and "Pacifico" is pre-installed in CodeSnap, you can use them out of the box
{
  "window": {
    "mac_window_bar": true,
    "shadow": 20,
    "margin": {
      "x": 82,
      "y": 82
    }
  },
  "code": {
    "font_family": "CaskaydiaCove Nerd Font",
    // CodeSnap use candy theme by default, if you want to use other theme, please refer https://github.com/trishume/syntect
    "theme": "candy"
  },
  "watermark": {
    "content": "CodeSnap",
    "font_family": "Pacifico",
    "color": "#ffffff"
  },
  // If you want to use gradient color, you can provide stops like the following config
  // But if you want to use solid color, you can just provide a color string like: 
  // "background": "#6bcba5"
  "background": {
    "start": {
      "x": 0,
      "y": 0
    },
    "end": {
      "x": "max",
      "y": 0
    },
    "stops": [
      {
        "position": 0,
        "color": "#6bcba5"
      },
      {
        "position": 1,
        "color": "#caf4c2"
      }
    ]
  }
}
```

All configuration items can be found in [config.rs](https://github.com/mistricky/CodeSnap/blob/main/core/src/config.rs)






## ❤️ Related projects
- [codesnap](https://github.com/mistricky/CodeSnap/tree/main/core)
- [codesnap-cli](https://github.com/mistricky/CodeSnap/tree/main/cli)
- [codesnap.nvim](https://github.com/mistricky/codesnap.nvim)
- [codesnap.idea](https://github.com/RAOE/CodeSnap.idea)
- codesnap.vscode (Planning)
- codesnap.zed (Planning)


## 📑 License
MIT.
