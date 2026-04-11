# Naming Conventions

This document holds some conventions used in localization strings.

> **NOTE:** This document is incomplete.

If you find some replaced text which does not follow these conventions, please
open an issue on this repository.

> ## Table of Contents
>
> - [Naming Conventions](#naming-conventions)
>   - [Colors](#colors)
>   - [Stat Names](#stat-names)

## Colors

For text entries that support markup, this is how certain types of information
are colored.

> _Terraria_'s markup allows for text to be colored using the following syntax:
> `[c/RRGGBB:<text>]`, where `RRGGBB` is a hex color code.

| Context                          | Color Code               |
|----------------------------------|--------------------------|
| Generic Highlight                | `ffff00` (pure yellow)   |
| Duration                         | `4782c9` (mid sky)       |
| Stat Change (positive)           | `ff0000` (pure red)      |
| Resource Regen Change (positive) | `00ff7f` (bright teal)   |
| Resource Change (positive)       | `00ff00` (pure green)    |
| Resource Cost                    | `ff7f00` (bright orange) |

## Stat Names

These are how the names of stats and damage classes are formatted.

| Stat/Class        | Name       |
|-------------------|:----------:|
| Health            | `HP`       |
| Mana              | `TP`       |
| Attack            | `AT`       |
| Defense           | `DF`       |
| Armor Penetration | `-sans DF` |
| Melee             | `MLE`      |
| Ranged            | `RNG`      |
| Mage              | `MGC`      |
| Summoner          | `SMN`      |
