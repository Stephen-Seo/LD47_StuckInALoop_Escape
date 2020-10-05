# Links

[ldjam page](https://ldjam.com/events/ludum-dare/47/escape)

[itch.io page](https://seodisparate.itch.io/ludumdare-47-entry-escape)

# Compiling

[Install rust](https://rust-lang.org)

`cargo build --release`

# Running

The `resources` directory needs to be in the same place as the executable.  
This may cause `cargo run` to fail, as it doesn't move the resources directory.

You will have to either move the executable to the root directory of this
project, or make a symlink/copy of `resources` to the directory that holds the
executable.

There are some issues running in debug mode, so be sure to use the executable
built with `cargo build --release`, or run with `cargo run --release`.

# About

Game programmed with [neovim](https://github.com/neovim/neovim) in the [Rust
programming language](https://rust-lang.org), and uses the
[ggez](https://ggez.rs) Rust library.

All audio created with [lmms](https://lmms.io).

All images created with [gimp](https://gimp.org).

Uses ClearSans font (licensed under the [Apache 2
license](resources/clearsans-LICENSE-2.0.txt)).
