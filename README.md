# ShitTD

A simple CLI tool for managing the shit you've got to do.

```
$ shittd --help
A manager for your shit to do list

Usage: shittd <COMMAND>

Commands:
  add        Adds one or more tasks to the list
  list       Lists incomplete and completed tasks
  finish     Finishes one or many tasks by ID
  push       Pushes task(s) off to the following day
  clean      Removes completed tasks from the list
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Better README and examples forthcoming.

For information about individual commands, run:

```
$ shittd <COMMAND> --help
```

To install the project on your machine, clone the repo and run `cargo install
--path <path-to-clone>`.
