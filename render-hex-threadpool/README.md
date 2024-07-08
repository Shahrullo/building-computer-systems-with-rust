# Implementation of render-hex project

## Getting Started
Run 
```bash
cargo run -- $(
    echo 'multi-threaded render-hex' |
    sha1sum |
    cut -f1 -d' '
)
``` 
in the project's root. You should get the result in the main repo.

Inspect with `cat ${*.svg}` 

Go ahead and modify the code according to your preferences.