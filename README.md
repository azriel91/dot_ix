# ✒️ dot_ix

> 🚧 This is very much a work in progress

Try it yourself: ([demo_1][demo_1], [demo_2][demo_2])

Original concept:

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

* [ ] Split crate into multiple subcrates.
* [ ] Split `app::info_graph::InfoGraph` into smaller components.
* [ ] Probably get rid of `main.scss` and replace with tailwind classes.
* [ ] Inline styles in SVG.
* [ ] Inline font in SVG styles.
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

[demo_1]: https://azriel.im/dot_ix/
[demo_2]: https://azriel.im/dot_ix/?src=BYSwpgTghhDGwE8BcAoABGqS0G8C%2B6aARtvobKQRgCaUpjUDmYAzqhlCWgNpQA0xALqEiFHkQGxhGWLR6wB1YQDsA9tTAB9EMoBmqtoSy40yqAFsw2AIJoqxUqYtW0AITvlHZy9gDCHmi9nbAARDwAXKBAAGwB3HWpNWGioFhZWdkxsAD4AWkIMbgAybIAHKHDgQSRdGOjcxggwMGVcgGYABg6CnhLyyuqWcIhVAGswXIBGHuKyiqqkIZHxhqaW3IA2Lpm%2B%2BergVQA3SBq61ebWgCZtjELdgaQD44hF4bGJxovNm9vZ-oX9LAAK5sVRA8LRHQTaa3XpzB6AkFIMEQqG5SGWb7dWF-PY1VTA0HgyHKCbUVLABg7eEAglIkZA5Qaai5AAe0R6wIgLFUEFypVUOnCkBEOXyOPuALORGiQImnWxv0lgzeK0u1P%2B1W4S3euXJLGAMGgyAALNIlTStTqVvqDrpdOlwkhJua7pakFBlCBzBUJixSjoNXjeSBGDpctw2pcAKSaXKXDbR11wzWvZYTGVyrFBh5PE61aL1TMTa6Kt2pvMva0Z2UTLZllN4xFE1GkqY52mE5HEtEYus-ctNuktklkilUiXu5tIBlMhhsjmwrk8vkCoUijhcPIdlXp9uT1PV87resa1TRBCMVTKaoF%2BqfE8DlPny-X3e6mEWgUXq83tO6h9WlPA8hy7FFR33L9QKRcDe29fsG1xBFh27VsxwNCcoOQrtZ2ZBdOSBbleX5QVlGFCAUCAA
