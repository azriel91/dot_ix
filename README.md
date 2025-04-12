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
dot_ix = "0.9.2"

# Enables the `FlexDiag` web component.
dot_ix = { version = "0.9.2", features = ["flex_diag"] }
```


## Development

```bash
cargo install cargo-leptos

# Then, one of:
# * client side rendering -- uses WASM compiled graphviz to generate the graph.
cargo leptos watch
```


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
[demo_2]: https://azriel.im/dot_ix/#src=LQhQBMEsCcFMGMAukD2A7AXAAgG62svAIYA2oAFpPkdPOQJ4ahZZEDuAzkyy+xwPoBzEigBGpbj16chI8SX6QiAW37QUJWP1GwiaDolIBrbAG8AvsymsZwsaUUq1GrZH2G08WGctS+a2EFUTCs-GTgg9H4cAAd4SWsbAQjg6Lj+IgTEpIDItDT4DP4OchpYcGKAV1E0WEQfUOz-FKjYwqJ+cFgcKpq6huyeZsDUtqKY9QqOatr6rAtGsOSR1vTRLMThvILtTu7e2YHBnJb8sdF+CZQpmf753ybwlbP0+MXrLdHXvZ7pvrmFscTs8doUrjd-g1QGhrlo0CpYFwrHxsAAiACCyiIAC90FgAOqwURYADK+BwkC8HFRyNscgkWFRAB5RAA+ADi9JIWAASoiUJVaIimQB6NmM2kCOzyRyqdSabS6dzGNEASXRAFleS4adIpVzZc4FW4DHovGrNdrNLrgXk0SzWXy8qRefzBVTReKbZ9VvFGQA1AAKAGFvU9tmNMoyncEsOiw8sI+kOiUyhDDoyABQk0pwcAASlJt0QCdyX3aPwOd1RmYAIgBRf2FknF0unHYdcFVuY1wM8gDytebrclZd92jRMbxACE2yDzpW-hmaw2m0X-nOk4ULl2l9XM33B8ON6P22M-aip2gsKHT-Pvl1fsW0XXG8fZpvy-wwZNuy-D0O64fqA0KwnsHDwEiLAGLAMT8AAjGiABibjgFgoiVIIpbSg4Shyi4ip6KaJAmIyfDALAJBEAYlI6ERhgkcAHDkpSsDAPKsDYQaeFGq4yqeN4ZGcBRVE0fAdHKoxCAAEzsTqd5bmi8EAOzSQAdAAzPBakAAy6SK8EAGyfuOKa5uUf6Mip6labp+nSdJJkvBWj6Wai1madpAAs9kaU5OwXK5e49h5tmGb5-nnouz5WapnlqQAHBFClfp2v7BUpcW2fBek6SK0l+Sl447ulMXuVl2nwepeUFZF3y7mVoWVT5NWFaBXT8LAygoAAVpAUFJNggCAZIA8H+jjhJDYIAPBuAPD743cU4HGEZJpGALwbgAjO-N9gKDxS0mh45pYBtRWYEdgB-uydBTTYACPuXZGxTmemdyrYAHrtjXqY7OUUQUxa9722l+gX7BlR1vXdD7A79YMfWeyaXKVkKg-9PpfSV1yWX94OFD+6Mg5joDlIIiKSCjHYPWm3b8Bk4apdFiMANqk-dqZ5t2AA0AOmXTswALpY0ULMWcFVNM2s3N3IzNNc4LT2IBzovbuLiB8zD94VjLlPU4mX6FD9DMKwLj3s5zX265D-x8-zHR67MItS19aW4zFkva1zNt1PL9tkw1Fv80DT7-Hbruo-DTv617C7u3LJsBaHssqzHUVR0Hn2gnHlku6nSfm7MnvB2nPu8yBH0TYaS0ScRRgp6Xu0EftZqCZnNeLQRFcMUYeeyNtZd1-xXgJ-4zf4cafdaFrWfJtgTcLcPfGmgJnew+0luGIIA2r138hohNNob20aJtLvRCCMUq9aI+aKmkTWCvk2R8n1fWjgpfZ83wB+Y0qAG+QIgXXr8fm8GRSGnt3WuCo27GE7kPXiihR4Jz3nEbA2RM5L0XmrQ2FNgrwIAY-H4SCUHoOtjnD2icxZRzQVub8StsEPzPnHKeBtHaywoYDdOwUWHjhxvHECG8DD0E0AIAAZigeAlR-4nwmtgAAZBvLogiiCVBIIgDgjQYQdTkQopRA0pCphiFoeAGgUDQGwCQSAyhYDvAMOoIwWg2CQHAIgcg2AADk9N4IxAAB482cZYxA1itB8M0C4jgGh7E+MSFYlANjyZdBcQAVh0jpcJ1hBGQBIAoVMsSsDOOkok5JCC-QACpZGwHkYo5RVheF0IvlgYpACNHlJUSwKpx8n6TGwHUk+DStEgUENAIgMRyAUmxBkRAfiBoxCIPAKu3Usk0H6fQfgiBpI8PIF1bwVhAnE1UWBbpyiNhYFgB4vxUZnGiOgCE6AwAYgoDcL-aAWB1CVDQF0cAwASCCGSSwQm59SmaP2e8I5JzsDnMudc25aB7mNEoOALop0pEwrhfMQ5xz+nYERbAa8DxVaULMpg4sKcl7fWIXMQpGK0CNEYeTVmwtx5Ev9pZMl9i4WUojnDDWtKDZmwDhmJlsLMWsv1KAlu4ClSV2rjPGB9cBIdPJYKieLkSWEsIWwmKfKWVLAVbsZOdL0Fo1lrK5lArNVEu5ZrLlqrEbquNUMOkwrZ6wPnl4ZVuLDX8opVIUxghyBzCkV6n17xUnpJiXCIxWJJrZJykkwNaSMmlA6uQFAeBjHZISV8ngQa41EA6sI0RXBI15JjcGzJWgpnIDwC4qN6boJ+KiQE+NoboDhpcV5QtETa3RJLfwRNyaXEaTbdYSJnaG0ZCQJACtqaB06I7QExA-DBLgBQGM8o7xJmwrcIIPt6aUTvCZEyDp-rECWNKHo78hiU30EoiINg7wBSIFMbUM9IgL1XpQDeqQjQ4AhPdFoPpAoYjSK-QKIUQgnkxF3fu2ph7j0DP0ee7ABh0AWMSHeh9cHn0IcQEh+VkjalAZ-aB-98rTgdPwyBv9lRwMmrVgcvdB7IDeqPe2-xp950IdCeAT9zxAPPAg-RxjMHT0GIw1gDgVFf63sqPetw6GjEIfE8h21+dIykd49Ryh6w8NqaU1qoppxi48BBlI4KfGoMMYDREk9smU2iBIJURTUhUMyafXJ9CdmHOGZnaxoJomOPEZVRytVJn1O0yjh04LOmiVMMZRFnFrCwu1Ni6QxWhc7iFKSxahL6Xiz+coVwmLOWgA
