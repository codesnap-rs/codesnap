<img width="350" src="/doc/logo.png" />

<div>
  
  [![Lint CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Lint)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Code Style CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Code%20style)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Crates.io Version](https://img.shields.io/crates/v/CodeSnap?logo=rust&color=%232ecc71)](https://crates.io/crates/codesnap)
  [![Lint convention](https://img.shields.io/badge/wizardoc--commit--convention-%233498db?style=flat&logo=lintcode&logoColor=white&link=https%3A%2F%2Fgithub.com%2Fwizardoc%2Fcommitlint-wizardoc)](https://github.com/wizardoc/commitlint-wizardoc)
  
</div>


## CodeSnap

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

CodeSnap is available in the [extra repository](https://archlinux.org/packages/extra/x86_64/codesnap/):

```bash
pacman -S codesnap
```

</details>

<details>
<summary>Nix/NixOS</summary>

CodeSnap is available in the [nixpkgs](https://github.com/NixOS/nixpkgs):

```bash
nix-env -i codesnap
```

</details>

<details>
<summary>Cargo</summary>

```bash
cargo install codesnap-cli
```

Or install via precompiled binary:
```bash
cargo binstall codesnap-cli
```

</details>

<details>
<summary>Homebrew</summary>

```bash
brew install codesnap
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
let code_content = Content::Code(
  CodeBuilder::from_t
  .content(r#"print "Hello, World!""#)
  .language("python")
  .build()?,
);

let snapshot = CodeSnap::from_default_theme()
  .content(code_content)
  .build()?
  .create_snapshot()?;

// Copy the snapshot data to the clipboard
snapshot.raw_data()?.copy()
```

## 🌰 Examples
All examples can be found in [examples](https://github.com/mistricky/CodeSnap/tree/main/cli/examples).

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
  "theme": "candy",
  "window": {
    "mac_window_bar": true,
    "shadow": {
      "radius": 20,
      "color": "#00000040"
    },
    "margin": {
      "x": 82,
      "y": 82
    },
    "border": {
      "width": 1,
      "color": "#ffffff30"
    }
  },
  "code_config": {
    "font_family": "CaskaydiaCove Nerd Font",
    "breadcrumbs": {
      "separator": "/",
      "color": "#80848b",
      "font_family": "CaskaydiaCove Nerd Font"
    }
  },
  "watermark": {
    "content": "CodeSnap",
    "font_family": "Pacifico",
    "color": "#ffffff"
  },
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


## Linux Wayland
Copy screenshots directly into the clipboard is cool, however, it doesn't work well on wl-clipboard, because the wl-clipboard can't paste the content which come from exited processes. As Hyprland document say:


> When we copy something on Wayland (using wl-clipboard) and close the application we copied from, the copied data disappears from the clipboard and we cannot paste it anymore. So to fix this problem we can use a program called as wl-clip-persist which will preserve the data in the clipboard after the application is closed. 


If you are using CodeSnap on wl-clipboard, you can refer [wl-clip-persist](https://github.com/Linus789/wl-clip-persist), it reads all the clipboard data into memory and then overwrites the clipboard with the data from our memory to persist copied data.


## ❤️ Related projects
- [codesnap](https://github.com/mistricky/CodeSnap/tree/main/core)
- [codesnap-cli](https://github.com/mistricky/CodeSnap/tree/main/cli)
- [codesnap.nvim](https://github.com/mistricky/codesnap.nvim)
- codesnap.idea (Planning)
- codesnap.vscode (Planning)
- codesnap.zed (Planning)


## 📑 License
MIT.
