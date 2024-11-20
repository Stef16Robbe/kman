# kman

A very specific Kubeconfig MANager...

## What is it

It's simple; `kman` helps you in updating (expired) Kubernetes oauth tokens in your kubeconfig.
This way you retain a minimal amount of contexts that you can properly name & identify.

## How do I install it

### Cargo

1. Make sure you have Rust & Cargo installed: https://www.rust-lang.org/tools/install
2. Run `cargo install kman`
3. You can have "automatic" updates by periodically running [`cargo update`](https://github.com/nabijaczleweli/cargo-update)

### Direct binary installation

> [!WARNING]
> The binary does not have self-updating capabilities (yet), so you'll have to manually update with new releases

1. Download a binary from the [releases section](https://github.com/Stef16Robbe/kman/releases).
2. Add the binary to your `$PATH`

## How do I use it

```
A Kubeconfig MANager that allows you to easily refresh oauth tokens

Usage: kman [OPTIONS] [COMMAND]

Commands:
  list     Lists all contexts
  select   Select context to use
  refresh  Refresh context token
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

You can specify a Kubeconfig file other than the default (`$HOME/.kube/config`) using the `KUBECONFIG` environment variable

## Releases

- Update the change log with [git-cliff](https://git-cliff.org/) using `git cliff -o CHANGELOG.md`
- Add a new tag & create a Github release
- Run `cargo publish`

## TODO's

- [x] CI & releases
- [ ] Proper error handling
- [ ] Logging
- [ ] Tests
- [ ] Find easier way to refresh token (less manual actions)
- [ ] Incorporate in `brew`
