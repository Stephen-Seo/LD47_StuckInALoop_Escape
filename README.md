# Compiling

`cargo build --release`

# Running

The `resources` directory needs to be in the same place as the executable.  
This may cause `cargo run` to fail, as it doesn't move the resources directory.

You will have to either move the executable to the root directory of this
project, or make a symlink/copy of `resources` to the directory that holds the
executable.
