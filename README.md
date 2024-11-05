# kman

A very specific Kubeconfig MANager...

## What is it

It's simple; `kman` allows you to update Kubernetes oauth tokens in your kubeconfig.
This way you retain a minimal amount of contexts that you can properly name & identify.

## How do I install it

`todo!();`

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

## TODO's

- [ ] CI & releases
- [ ] Proper error handling
- [ ] Logging
- [ ] Tests
