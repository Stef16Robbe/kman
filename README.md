# kman

A very specific Kubeconfig MANager...

## What is it

It's simple; `kman` helps you in updating (expired) Kubernetes oauth tokens in your kubeconfig.
This way you retain a minimal amount of contexts that you can properly name & identify.

## How do I install it

1. Download a binary from the [releases section](https://github.com/Stef16Robbe/kman/releases).
2. Add the binary to your `$PATH`

## How do I use it

```
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
- Add a new tag & release (I do it via the Github CLI)
- Run `cargo publish`

## TODO's

- [x] CI & releases
- [ ] Proper error handling
- [ ] Logging
- [ ] Tests
- [ ] Find easier way to refresh token (less manual actions)
- [ ] Incorporate in `brew`
