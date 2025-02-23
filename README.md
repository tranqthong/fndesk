<!-- [![Linux build]()] -->

# fndesk
Basic terminal file browser.

Note: Currently in development, things will change without any notice

## Dependencies
- [cargo](https://github.com/rust-lang/cargo/) >= 1.85.0
- [rustc](https://www.rust-lang.org/) >= 1.85.0

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
- Move to parent directory: `backspace` or `Esc` or `arrow_left`
- Open file or directory: `space` or `enter` or `arrow_right`
- Quit App: `q`

#### File Operations
- Toggle hidden files: `h`
- Delete a file or directory: `del`
- Rename file or directory: WIP
- Copy file or directory: WIP
- Move file or directory: WIP

## Roadmap
- Implement basic file IO: WIP
- differentiate between file and dir
- async IO
- set up github workflow
### Nice to haves
- Windows support
- Mac support
- themes