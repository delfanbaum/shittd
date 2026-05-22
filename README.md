# Shittd

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

**Pro tip**: I like to alias it to `std` to save some keystrokes. That said, `std`
wasn't really what I wanted to have the package listed as, so.

## Task Model

A task is made up of a few things:

* An `id` that you use for saying something's finished or updating it
* An optional `project` if you like to organize things like that (listing by
project is a feature forthcoming)
* An optional `due_date` that will determine what shows up in the default `list`
(as well as `soon`)
* A yet-to-be-utilized optional `invalid_date` after which an item will no
longer show up on the list
* And, finally, a completion Boolean. 

## Database Model

Right now the database is just a json file located at `.shittd.json` in your
home directory. The database is currently modeled off of
[tinydb](https://tinydb.readthedocs.io/en/latest/). Something... different?
better? will be available eventually.


## Installation

To install the project on your machine, run `cargo install shittd`
or clone the repo and run `cargo install --path <path-to-clone>`.
