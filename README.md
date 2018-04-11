# Fate/Extra Solver
Combat solver for Fate/Extra

## Building
You need Rust & Cargo (or just Rust with the crates specified in Cargo.toml).

To get, build, and run just: 

```
git clone github.com/RaineForest/FateExtraSolver.git
cd FateExtraSolver
cargo run
```

For those Windows folks, I've included a binary in the root.

## Problem description (for those not aquainted)
Fate/Extra is an RPG developed by Type-Moon for the PSP.
The combat is very rng-based and requires quite a bit of memorization later in the game.

The combat is structured like this:
- Both sides choose 6 actions to perform per round
- Actions include: attack, guard, break, specials
- Guard beats attack, break beats guard, attack beats break (rock-paper-scissors style)
- Specials don't have a weakness *per se*, and they beat everything else at a mana cost, but can be beaten by rpg stats in the event of competing specials
- Based on the number of enemies you've beaten of a certain type, a few actions in the string are given to you.

Since enemy attack patterns are fixed, although numerous, we can do some statistics and pattern matching to determine the "best" response.

![Fate/Extra Combat Screenshot found through Google Images](https://i0.wp.com/i201.photobucket.com/albums/aa288/reversethieves/show%20images/Type%20Moon/Fate%20Extra/FateExtraGamePlay.jpg)

## Solution
Taking known enemy patterns and a given string of actions, it can determine the best course of action based on how risky you'd like to play the game.
I made a shell type entry format for solving, considering that someone would be playing the game for long stretches and wouldn't want to enter a long command for each round.
The commands supported are as follows:
- `select` - selects the enemy to solve against
- `solve` - determines the best pattern to respond with
- `threshold` - set the riskiness factor (lower values are safer)
- `list` - lists the loaded enemies
- `help` - lists commands in the shell
- `exit` - exits the program

Example:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11 secs
     Running `target\debug\fate_extra_solver.exe`
> select Inspire
> solve _ G _ _ _ _
Guard   Break   Break   Break   Guard   Break
> threshold 40
> solve _ G _ _ _ _
Break   Break   Break   Break   Break   Break
>
```

## File details
- `src/main.rs`: Command dispatch and shell logic
- `src/enemy.rs`: Enemy structure and solving utility functions
- `src/enemy_builder.rs`: Reads `enemies.txt` and generates a map of `Enemy`s
- `src/action_count.rs`: Utility structure for keeping track of statistics
- `src/action.rs`: Action enum and simple trump/weak definitions
- `enemies.txt`: Enemy action tables

## A bit more about `enemies.txt`
It's got a simple, human-readable format for easy editing.
Enemy names start at the start of a line and end in a colon (`:`).
Following that are the action sequences for the enemy.
The action sequences consist of four different letters: A, G, B, or S for Attack, Guard, Break, or Special, respectively.

Example (or just look at enemies.txt for yourself, but y'know, spoilers):

```
<Name>:
<Sequence of 6 action initials [A, G, B, S]>
<more sequences>

<Another Name>:
<more sequences>
```

## Todos
A lot of the action data is still incomplete or inaccurate...
I could use some help here, especially from a more authoritative source than just playing the game and writing down patterns that you run into.
I tried extracting from the game's files and I couldn't find anything helpful.

**If you find something wrong, please make a pull! Don't just abide, man!**

`enemies.txt` was designed to be easy to edit!

Also, I haven't played Fate/Extra CCC yet, but if the combat is the same, I'd like to extend this to that.

A GUI would be nice, but right now I'm not willing to put up with Rust to make it happen.

## Thanks

http://www.aegisgames.net/guide-fate-extra-enemy-patterns/

https://www35.atwiki.jp/fate_extra/pages/1.html (Google Translate is helpful here as well)
