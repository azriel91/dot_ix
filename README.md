# ✒️ dot_ix

[![Crates.io](https://img.shields.io/crates/v/dot_ix.svg)](https://crates.io/crates/dot_ix)
[![docs.rs](https://img.shields.io/docsrs/dot_ix)](https://docs.rs/dot_ix)
[![CI](https://github.com/azriel91/dot_ix/workflows/CI/badge.svg)](https://github.com/azriel91/dot_ix/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/dot_ix/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/dot_ix)

> 🚧 This is very much a work in progress

Try it yourself: ([demo_1][demo_1], [demo_2][demo_2])

<details open><summary>Example</summary>

![](./doc/example.svg)

</details>

<details><summary>Original Concept</summary>

https://user-images.githubusercontent.com/2993230/253878816-0729970f-651f-45ef-a986-470f383b8018.mp4

</details>


# Usage

Add the following to `Cargo.toml`

```toml
dot_ix = "0.7.0"

# Enables the `FlexDiag` web component.
dot_ix = { version = "0.7.0", features = ["flex_diag"] }

# Enables server side dot generation.
# Requires graphviz `dot` to be installed server side.
dot_ix = { version = "0.7.0", features = ["server_side_graphviz"] }
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
* [x] Get rid of `main.scss` and replace with tailwind classes.
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
[demo_2]: https://azriel.im/dot_ix/#src=LQhQBMEsCcFMGMAukD2A7AXAAgG62svAIYA2oAFpPkdPOQJ4ahZZEDuAzkyy+xwPoBzEigBGpbj16chI8SX6QiAW37QUJWP1GwiaDolIBrbAG8AvsymsZwsaUUq1GrZH2G08WGctS+a2EFUTCs-GTgg9H4cAAd4SWsbAQjg6Lj+IgTEpIDItDT4DP4OchpYcGKAV1E0WEQfUOz-FKjYwqJ+cFgcKpq6huyeZsDUtqKY9QqOatr6rAtGsOSR1vTRLMThvILtTu7e2YHBnJb8sdF+CZQpmf753ybwlbP0+MXrLdHXvZ7pvrmFscTs8doUrjd-g1QGhrlo0CpYFwrHxsAAiACCyiIAC90FgAOqwURYADK+BwkC8HFRyNscgkWFRAB5RAA+ADi9JIWAASoiUJVaIimQB6NmM2kCOzyRyqdSabS6dzGNEASXRAFleS4adIpVzZc4FW4DHovGrNdrNLrgXk0SzWXy8qRefzBVTReKbZ9VvFGQA1AAKAGFvU9tmNMoyncEsOiw8sI+kOiUyhDDoyABQk0pwcAASlJt0QCdyX3aPwOd1RmYAIgBRf2FknF0unHYdcFVuY1wM8gDytebrclZd92jRMbxACE2yDzpW-hmaw2m0X-nOk4ULl2l9XM33B8ON6P22M-aip2gsKHT-Pvl1fsW0XXG8fZpvy-wwZNuy-D0O64fqA0KwnsHDwEiLAGLAMT8AAjGiABibjgFgoiVIIpbSg4Shyi4ip6KaJAmIyfDALAJBEAYlI6ERhgkcAHDkpSsDAPKsDYQaeFGq4yqeN4ZGcBRVE0fAdHKoxCAAEzsTqd5bmi8EAOzSQAdAAzPBakAAy6SK8EAGyfuOKa5uUf6Mip6labp+nSdJJkvBWj6Wai1madpAAs9kaU5OwXK5e49h5tmGb5-nnouz5WapnlqQAHBFClfp2v7BUpcW2fBek6SK0l+Sl447ulMXuVl2nwepeUFZF3y7mVoWVT5NWFaBXT8LAygoAAVpAUFJNggCAZIA8H+jjhJDYIAPBuAPD743cU4HGEZJpGALwbgAjO-N9gKDxS0mh45pYBtRWYEdgB-uydBTTYACPuXZGxTmemdyrYAHrtjXqY7OUUQUxa9722l+gX7BlR1vXdD7A79YMfWeyaXKVkKg-9PpfSV1yWX94OFD+6Mg5joDlIIiKSCjHYPWm3b8Bk4apdFiMANqk-dqZ5t2AA0AOmXTswALpY0ULMWcFVNM2s3N3IzNNc4LT2IBzovbuLiB8zD94VjLlPU4mX6FD9DMKwLj3s5zX265D-x8-zHR67MItS19aW4zFkva1zNt1PL9tkw1Fv80DT7-Hbruo-DTv617C7u3LJsBaHssqzHUVR0Hn2gnHlku6nSfm7MnvB2nPu8yBH0TYaS0ScRRgp6Xu0EftZqCZnNeLQRFcMUYeeyNtZd1-xXgJ-4zf4cafdaFrWfJtgTcLcPfGmgJnew+0luGIIA2r138jYFIphYPCyiCaiE2olgDwb202+JLv++H20J9n0QgjFKvWiPtv18ImippE1gr5NvfVgN7fy0OCd+e9P6MmAb-AC+YAHtS0KvAag8Z68WWpXbe9MN4TQHnSbutcR7z0OiwTBj9N6kBwfnC+QJsgkKfm0ChE91ZGxBrQgoDCl7fRzncHgrDgE-HYWrXYUdL5YF4S-fh-MzYBwzDwoB4jHwCK3OMBGMjRFyMfiAyYijAbpxBsQ9RRM47aPHDjWWl8xEaKMSBQQ0AiAxHIBSbEGRECIGgANGIRB4BV26l0bANBbH0H4IgaSIFEDkC6t4KwBh6CaAGiwGEHUugADMiCVBIIgOJPBYAAA9XFRgAOTwEFBwFA0BgAxBQG4RA+AsDqEqGgLo4BgAkEEPkxohNX6wBSWkjJGwsA5LydgIpbjSnlMqWgap0BGiUHAF0U6AAyGZcz5j9NybY7ASzYDXgeKrJRZkKbC3Hhw62XC5gACpNloEaAbfZrNDkG39pZC5kBZlbOuRHOGGt7kfOxkrbAzzXlXKWGQnaLcFRt2MNXFBe1R7-Mue8yhcNk5HMEY7MxWAAVzIRYwoRpyU4cLRuizFbzgUcKkZrA2pinnwuBUPVB9cBL4tRXCl5WKpAkEgIIcgcx5kcq5Ygd4STIAkAUKmDqMJoBYkmlgfJOUdJtMSEKkV5MOrkBQHgaA2B8kAFZ5WCuFaK0oHUkkoCKVwGVcqFXWCVYaogHVPHIDwFqy17wDDqCMFoMVcJSlSq1V5HSerEhupQB6lVWg1Uaq1RpANVqpDBtDV6jISBIBOplbqwN1h42esQDEwS4AUAuPKO8Dxsy3CCCjbGwa7wmRMn+Xy7lrrSgxC0PADQpTsD0EoiINg7wBSIA5bUb8bbNVYE7SKlAPapCNDgCU90WgbEChiNgeZM6BRCiEHUmI1ba0YvrQKoNTaW3DuwAYdAsBe2VH7W4I9IgR2ntqNiia-zV1zo3Yu7Fpxn1unXQuyoW7SVqz6TWutnKG1BtcSG7NuaT0aBedO54y7TjbpA-yxtdib3tqwBwKi1SL1XsHa229J6cPnoA3sr9eQP2CPWBipDZGdYUeCMXHgIN5nBWQ7u0D+7M2HqHUR9CJBKikesH2gdGGR2iEE8JuNEGE05s0DBjl4AqN7PJncmKZz2P0bdqc-5Wmhg-OUWHDMmniwqZ0cIjF+ndk6MLncUz-xzMmL+VZsz2nTa6I0+xoAA
