# goto-ally
[![Build Status](https://travis-ci.org/mcoffin/goto-ally.svg?branch=master)](https://travis-ci.org/mcoffin/goto-ally)
[![Stories in Ready](https://badge.waffle.io/mcoffin/goto-ally.png?label=ready&title=Ready)](https://waffle.io/mcoffin/goto-ally)

Goto ally is a program that helps to maintain a list of directories that you commonly hop to

## Installation
1. Build the crate with `cargo`, and install the `goto-ally` executable somewhere on your path.
2. In your `.bashrc` or something similar, add the `goto()` function (see below).
3. Copy `goto.sh` from the root of this project in to your home directory

## bashrc

```bash
goto() {
	LOCATION=$1 . ~/goto.sh
}
```

## Configuration
Goto-ally will look in the following places for `.goto.yml` config files.

1. Your home directory
2. Your current directory, and every directory under it.

### Configuration file format
The configuration file format is a simple YAML format, with the top-level element being a map between alias names, and absolute locations.

```yaml
some-alias: /home/user/something
bar: /home/user/Documents/bar
```

If a user had the above configuration saved somewhere accessible, then the command `goto bar` would take him/her to `/home/user/Documents/bar` and the command `goto some-alias` would take him/her to `/home/user/something`.

## Example Usage

Lets say a user has two projects, `foo`, and `bar`. Both projects are in the user's home directory. The user wants to be able to jump to these projects easily so the following config file is used at `~/.goto.yml`.

```yaml
foo: /home/user/foo
bar: /home/user/bar
```

Unfortunately, the user works on the `foo` project quite a bit, and wants to be able to jump around between subsystems within the project. The `foo` project has two subsystems, `baz`, and `bar`. The user can then use the following config at `~/foo/.goto.yml` to do their jumping without any naming conflicts.

```yaml
bar: ./bar
baz: Some/Dumb/SubDirectoryChain/baz
root: './'
```

With the above configuration, the user can also, from anywhere on the system, jump **directly** to a sub-component of the `foo` project by using *path jumping*. A path-jumping command would look like the following:

```bash
$ goto foo/baz
```

This will take the user to `/home/user/foo/Some/Dumb/SubDirectoryChain/baz`. This is most useful for managing paths that need to skip down deep directory trees.
