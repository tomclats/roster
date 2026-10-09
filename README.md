# roster

List every regular file under a directory. Each line is the size in bytes, the SHA-256 of the file, and the path relative to that directory. Lines are sorted by path. Hidden names and symbolic links are skipped.

```
cargo run --quiet -- fixtures/pack
```

```
5  389ed6887e49a315f706f6c2b931b1dcf0d797c91437124f32eb98555c669758  docs/note.txt
5  2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  hello.txt
```

`fixtures/pack/hello.txt` is the five bytes `hello`. `fixtures/pack/docs/note.txt` is the five bytes `note` plus a newline. The same tree always prints the same text. The hash is computed in this crate. There is no network and no extra crate.

## Will not do

- Follow a symbolic link
- Hash a hidden file
- Upload the manifest
- Change the files it reads

## Tests

```
cargo test
```

## License

MIT.
