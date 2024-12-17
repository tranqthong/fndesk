<!-- [![Linux build]()] -->

# drawer_fm
Basic terminal file browser

## Dependencies
- [cargo](https://github.com/rust-lang/cargo/) >= 1.83.0
- [rustc](https://www.rust-lang.org/) >= 1.83.0

See [Cargo.toml](Cargo.toml)

## Building

```
~$ cargo build
```

## Running

```
~$ cargo run
```

## Usage
#### Navigation
- Move up: `arrow_up`
- Move down: `arrow_down`
- Move to parent directory: `backspace` or `Esc`
- Open file or directory: `space` or `enter`

#### File Operations
- Toggle hidden files: `h`
- Rename file or directory: WIP
- Copy file or directory: WIP
- Move file or directory: WIP

## Roadmap
- Implement basic file IO
- deleting a file sends it to trash/recycle bin
- set up github workflow
### Nice to haves
- Windows support
- Mac support
- themes