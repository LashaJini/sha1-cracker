```bash
cargo run -- wordlists.txt (echo -n "qwerty" | sha1sum | awk '{ print $1 }')
```
