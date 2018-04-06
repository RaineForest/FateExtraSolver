# Fate/Extra Solver
Combat solver for Fate/Extra

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
