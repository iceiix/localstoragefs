localstoragefs example

To run natively:

```sh
cargo run
```

To run on the web, install [cargo-web](https://github.com/koute/cargo-web) and run:

```sh
cargo web start
```

On the first run, the file `hello.txt` will be created and written to:

```
opening hello.txt
creating hello.txt
writing hello.txt
```

You can examine this file on your native filesystem or in the browser's JavaScript console
with `localStorage.getItem("hello.txt")`. On subsequent runs (rerun `cargo run` or reload
the web page), the contents of the file will be read:

```
opening hello.txt
reading hello.txt
read data = "Hello, world!"
```

To reset, run `rm hello.txt` or `localStorage.clear()`.
