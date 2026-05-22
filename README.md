# ShitTD

A simple CLI tool for managing the shit you've got to do.


## Usage

```
$ shittd --help
A manager for your shit to do

Usage: shittd <COMMAND>

Commands:
  add       Adds one or more tasks to the list
  list      Lists incomplete and completed tasks
  finish    Finishes one or many tasks by ID
  push      Pushes task(s) off to the following day, or optionally a specific calendar date
  renumber  Renumbers task IDs
  clean     Removes completed tasks from the list
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

For information about individual commands, run:

```
$ shittd <COMMAND> --help
```

## Task Model



## Installation

To install the project on your machine, run `cargo install shittd`
or clone the repo and run `cargo install --path <path-to-clone>`.
