# ✒️ dot_ix

> 🚧 This is very much a work in progress

[Try it yourself](https://azriel.im/dot_ix/) *(sorry not mobile friendly)*

https://user-images.githubusercontent.com/2993230/253878816-0729970f-651f-45ef-a986-470f383b8018.mp4


```bash
cargo install cargo-leptos

# Then, one of:
# * server side rendering -- runs `dot` on the server to generate the graph.
cargo leptos watch --bin-features "ssr" --features "server_side_graphviz" -v
# * client side rendering -- uses WASM compiled graphviz to generate the graph.
trunk serve
```

For server side rendering, the `"server_side_graphviz"` feature needs to be passed in separately because that feature still needs to be enabled for the lib compilation, i.e.

* server side rendering:
    - lib features: `"server_side_graphviz"`
    - bin features: `"ssr,server_side_graphviz"`
* client side rendering:
    - lib features: `""`
    - bin features: `""`


## To Do

> 🦜 Feel free to do any of these -- this project isn't my main focus, but I should be responsive in reviewing / merging things.

* [ ] Instead of building using `cargo leptos` in `pages`, use `trunk`.

    This means not having a `styles/main.scss` -- cargo leptos merges styles with tailwind, that's why we use `cargo leptos`. See `Trunk.toml` if we were to use `trunk` and tailwind on its own.

* [ ] Change `rt/into_graphviz_dot_src/info_graph.rs` to write to a buffer, instead of individual strings.
* [ ] Take each node with a "type".
* [ ] Take in tailwindcss classes to attach to node types.
* [ ] Take in theme / merge with default theme.
* [ ] Split `dot_ix` library from the web app, so it can be embedded in other apps.
* [ ] GitHub Actions / automated testing / releasing / publishing.
* [ ] Playground: Render pre-written graphs
    - Local storage
    - Graph in URL
    - Link to gist
* [ ] Mobile friendly web page.
