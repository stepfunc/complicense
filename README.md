# complicense
License report generator and whitelisting tool

Example invocation:

```
cargo run -- --import dependencies.json --config config.json  --token "<GITHUB OAUTH TOKEN>" > licenses.txt
```

The import file is the JSON output from [`cargo-license`](https://crates.io/crates/cargo-license), e.g.:

```
> cargo-license --avoid-dev-deps --avoid-build-deps -j > dependencies.json
```

Note: Run the `cargo-license` command on Linux, because windows doesn't properly encode the file as UTF-8 by default and then
the serde parsing fails when importing into `complicense`.

Configuration is specified in the format:

```
{
  "ignore": [<list of ignored crates>],
  "allowed_licenses": [<list of allowed license names>],
  "crates": {
    "<crate name>" : {
      "license_name": "<license name>",
      "file_content": "<base64 license text>"
    }
  }
}
```

The `crates` section allows you to manually specify the license name and content. If not found there, the license content will be retrieved from
the Github v3 API using the provided OAUTH token.
