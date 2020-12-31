Similarly to "One day, One shader" ( https://github.com/gre/shaderday.com ),

this is about doing a "plot" every day.

This project is a rust project that have an example every day and that generates a SVG file that is plotted with Inkscape + Axidraw.

(replace 000 with day number)

```
cargo run --example 000
```

How to do "hot reload":

Run this

```
cargo watch "run --example 000"
```

And then open the `image.svg` with a viewer that allows to update when the file changes. With Mac, you simply press SPACE on the file.
