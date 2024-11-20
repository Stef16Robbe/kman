# kman

A very specific Kubeconfig MANager...

## What is it

It's simple; `kman` helps you in updating (expired) Kubernetes oauth tokens in your kubeconfig.
This way you retain a minimal amount of contexts that you can properly name & identify.

A bit more information on the problem it solves can be found in [this issue](https://github.com/sunny0826/kubecm/issues/1022#issuecomment-2456709999).

## How do I install it

### Cargo

1. Make sure you have Rust & Cargo installed: https://www.rust-lang.org/tools/install
2. Run `cargo install kman`
3. You can have "automatic" updates by periodically running [`cargo install-update -a`](https://github.com/nabijaczleweli/cargo-update)

### Direct binary installation

> [!WARNING]
> `kman` does not have self-updating capabilities (yet), so you'll have to manually update with new releases

1. Download a binary from the [releases section](https://github.com/Stef16Robbe/kman/releases).
2. Add the binary to your `$PATH`

## How do I use it

(from `kman help`):

```plaintext
A Kubeconfig MANager that allows you to easily refresh oauth tokens

Usage: kman [OPTIONS] [COMMAND]

Commands:
  list     Lists all contexts
  select   Select context to use
  refresh  Refresh context token
  remove   Remove context(s)
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

You can specify a Kubeconfig file other than the default (`$HOME/.kube/config`) using the `KUBECONFIG` environment variable

## Releases

1. Update version number in `Cargo.toml`
2. Add a new tag & create a Github release
3. Update the change log with [git-cliff](https://git-cliff.org/) using `git cliff -o CHANGELOG.md`
4. Run `cargo publish`

## TODO's

- [x] CI & releases
- [ ] Proper error handling
- [ ] Logging
- [ ] Tests
- [ ] Find easier way to refresh token (less manual actions)
- [ ] Incorporate in `brew`
