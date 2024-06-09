# ✒️ dot_ix

[![Crates.io](https://img.shields.io/crates/v/dot_ix.svg)](https://crates.io/crates/dot_ix)
[![docs.rs](https://img.shields.io/docsrs/dot_ix)](https://docs.rs/dot_ix)
[![CI](https://github.com/azriel91/dot_ix/workflows/CI/badge.svg)](https://github.com/azriel91/dot_ix/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/dot_ix/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/dot_ix)

> 🚧 This is very much a work in progress

Try it yourself: ([demo_1][demo_1], [demo_2][demo_2])

Original concept:

https://user-images.githubusercontent.com/2993230/253878816-0729970f-651f-45ef-a986-470f383b8018.mp4


# Usage

Add the following to `Cargo.toml`

```toml
dot_ix = "0.6.0"

# Enables the `FlexDiag` web component.
dot_ix = { version = "0.6.0", features = ["flex_diag"] }

# Enables server side dot generation.
# Requires graphviz `dot` to be installed server side.
dot_ix = { version = "0.6.0", features = ["server_side_graphviz"] }
```


## Development

```bash
cargo install cargo-leptos

# Then, one of:
# * client side rendering -- uses WASM compiled graphviz to generate the graph.
cargo leptos watch
# * server side rendering -- runs `dot` on the server to generate the graph.
#   Requires `graphviz` to be installed.
cargo leptos watch --features "server_side_graphviz" -v
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

* [x] Split crate into multiple subcrates.
* [ ] Split `app::info_graph::InfoGraph` into smaller components.
* [ ] Probably get rid of `main.scss` and replace with tailwind classes.
* [x] Inline styles in SVG.
* [x] Inline font in SVG styles.
* [ ] Change `rt/into_graphviz_dot_src/info_graph.rs` to write to a buffer, instead of individual strings.
* [ ] Take each node with a "type".
* [x] Take in tailwindcss classes to attach to node types.
* [x] Take in theme / merge with default theme.
* [x] Split `dot_ix` library from the web app, so it can be embedded in other apps.
* [x] GitHub Actions / automated testing / releasing / publishing.
* [x] Playground: Render pre-written graphs
    - [x] Graph in URL
    - [ ] Link to gist

[demo_1]: https://azriel.im/dot_ix/
[demo_2]: https://azriel.im/dot_ix/?src=BYSwpgTghhDGwE8BcAoABGqS0G8C%2B6aARtvobKQRgCaUopjUDmYAzqhlCWgNpQA0xALqEiFXkUGwRGWLV6xB1ESgB2Ae2pgA%2BqqgBbNh0zYAgqOwAhctgDCheQBF6AF2BhDxwxBbaA7iBu2kRQrGDYLhAArmCErC4IADZGhBgaWtpaAGZQUYku7KkYaGAAHpFYaLBREKzqEAC0AA7qIKoukGgQ6lGqWtQNiUxFGPowTG3apdgALEWMvtm5%2BYXFxWUV2NW19c2t7ZBFlThorMBQTTqw6on12EwQYGCqaFTFJCOn55fa17cQ2CIiRin3i3QA1joAtQ3NgAEygyLqSHaeJJcJoaihdzUT4baDYAB8DU%2BGB4ADJCU0oG4hEgoKoQGMOg0eGDkWAGlizuoslkwi4Gvp1AA3HQAZlY2kSbTAMG0bSybUCYBka2K2zqjRabQ6EFJXR6fUYg2Gay4xjWZwuVxudzQDyeqkREJ01oyGggY0S2AAbAAGf2fJWJRKo84e%2Bre7AAVkD9FgrHYaGJhAAApCEFloIZWKckZCudjefywILhWLcEV-QBSYondmF7nAEsC7D%2BgDcryKAEZA3WGwXOc3W2XZgAOLtvAhAA
