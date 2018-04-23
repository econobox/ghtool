# ghtool

`ghtool` is a command-line tool for interacting with all the extra
data associated with a GitHub repository that doesn't come from `git`
itself.

## Features

*Note*: This is a side project. I'm mainly driven to implement
features in `ghtool` by my own need for them in my workflow. That
said, though, if there's a feature that you'd like to see implemented,
feel free to file an issue and I'm likely to work with you to get it
working (or at least consider it seriously). I'll certainly accept
pull requests if you develop a feature yourself.

### Labels

- [x] List the labels in a repository
      ([#3](https://github.com/sorenmortensen/ghtool/issues/3)).
- [ ] Copy labels from one repository to another
      ([#4](https://github.com/sorenmortensen/ghtool/issues/4)).

## Usage

```
ghtool 0.1.0
Søren Mortensen <soren@sorenmortensen.com>
Tool for interacting with all the extra data associated with a GitHub repository that doesn't come from git itself

USAGE:
    ghtool [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity (up to -vvv)
    -V, --version    Prints version information

OPTIONS:
    -T, --token <TOKEN>    GitHub personal access token that provides access to the repositories specified by <FROM>
                           and <TO>. Overrides any existing value from ~/.config/ghtool/config.toml. Not required if a
                           configuration file is found.

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    label    Make modifications to the issue labels in a GitHub repository
```

## hubcaps

It should be noted that this is essentially a glorified command-line
interface for [`hubcaps`](https://github.com/softprops/hubcaps);
`hubcaps` does all the heavy lifting.

## License

`ghtool` is licensed under the Apache-2.0 license. Please see the
[LICENSE](LICENSE) file for more information.
