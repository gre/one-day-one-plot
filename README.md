Similarly to "One day, One shader" ( https://github.com/gre/shaderday.com ), I'm going to "plot" every day.

This [Rust](https://www.rust-lang.org/) project uses `examples/` that generates a SVG file, then plotted with Inkscape + Axidraw.

(replace 000 with day number)

```
cargo run --example 000
```

How to do "hot reload":

Run this

```
cargo watch "run --example 000"
```

And then open the `image.svg` with a viewer that allows to update when the file changes. (E.g. vscode SVG Viewer)
