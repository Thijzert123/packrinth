---
title: Generating documentation
layout: default
nav_order: 4
---

# Generating documentation

With Packrinth, it is possible to automatically generate documentation based on the branch files.
To do this, run this:
```bash
$ packrinth doc
```
The documentation will be printed, and you can use it for your modpack frontpage. It includes the name, author,
summary and all the branch files.

This is a `modpack.json` configuration example for [Client+](https://github.com/Thijzert123/client-plus),
a modpack that uses Packrinth:
```json
{
        "pack_format": 1,
        "name": "Client+",
        "summary": "Modpack focused on improving vanilla Minecraft on the client-side.",
        "author": "Thijzert (https://github.com/Thijzert123)",
        "require_all": true,
        "auto_dependencies": true,
        "branches": [
                "1.21.8",
                "1.21.7",
                "1.21.6",
                "1.21.5",
                "1.21.4",
                "1.21.3",
                "1.21.1",
                "1.20.4",
                "1.20.1"
        ],
        "projects": {}
}
```
_modpack.json (all projects were omitted)_

Below is the generated config output from `packrinth doc` (to keep the page concise, some projects were omitted).

{: .note }
If you only want the project table to be printed, you can use the `--table-only` flag.

---

# Client+ _by Thijzert ([https://github.com/Thijzert123](https://github.com/Thijzert123))_
Modpack focused on improving vanilla Minecraft on the client-side.
## What is included?

|Name|1.21.8|1.21.7|1.21.6|1.21.5|1.21.4|1.21.3|1.21.1|1.20.4|1.20.1|
|:--|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|[Searchables](https://modrinth.com/project/fuuu3xnx)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Show Me Your Skin!](https://modrinth.com/project/bD7YqcA3)|✅|✅|✅|❌|❌|❌|✅|✅|✅|
|[Simple Grass Flowers](https://modrinth.com/project/ti9KkMHm)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Spryzeen's Healthbars](https://modrinth.com/project/ZMcqgmIV)|❌|❌|✅|❌|❌|❌|❌|❌|❌|
|[Spryzeen's Knight Armor](https://modrinth.com/project/EwJHG2NA)|❌|❌|✅|✅|✅|✅|❌|❌|❌|
|[Status Effect Bars](https://modrinth.com/project/x02cBj9Y)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Vervada's enhanced plants](https://modrinth.com/project/ghc0v6DT)|❌|❌|✅|✅|✅|✅|❌|❌|❌|
|[View Bobbing Options](https://modrinth.com/project/Yr9J16k6)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
