---
title: Tweaking projects
layout: default
nav_order: 3
---

# Tweaking projects
{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

{: .note }
This page assumes you know the basics of Packrinth.
To acquire this knowledge, read the [full guide](full-guide.html).

To tweak which projects get added to which branches, you can use version overrides and inclusions or exclusions.

## Version overrides
If a project on Modrinth doesn't _say_ it works on a Minecraft version, but if it in reality does work on the Minecraft
version, you can still add it with version overrides. You have to provide a Modrinth version ID for every branch.

To add version overrides, add the `version_overrides` key with a map of `branch: Modrinth version ID` to the project
map like so:
```json
{
        -- snip --
  
        "projects": {
                "yosbr": {
                        "version_overrides": {
                                "1.21.5": "KMOzdYko",
                                "1.21.6": "KMOzdYko",
                                "1.21.7": "KMOzdYko",
                                "1.21.8": "KMOzdYko"
                        }
                }
        }
}
```
[Your Options Shall Be Respected](https://modrinth.com/mod/yosbr) is a Minecraft mod that works on all recent Minecraft
versions, but it doesn't say that on its Modrinth page. By adding the version overrides, the project gets added anyway.

### CLI commands
Using the `packrinth project version-override`s `add` and `remove` commands, you can add version overrides
via Packrinth's CLI.

## Inclusions and exclusions
For each project, you can specify either inclusions __OR__ exclusions. When adding inclusions, you specify the branches
that the project should be added to. Branches that aren't in the inclusions list, don't get the project added to them.
This also works for exclusions, but it is the other way around: branches in the exclusions list __DON'T__ get the project,
while all the other branches do.

To add inclusions or exclusions, add the `include` or `exclude` key with a string list of branch names to the project
map like so:
```json
{
        -- snip --
  
        "projects": {
                "sodium": {
                        "include": [
                                "1.20.1",
                                "1.20.4"
                        ]
                },
                "indium": {
                        "exclude": [
                                "1.21.3",
                                "1.20.4"
                        ]
                }
        }
}
```
_modpack.json_

Imagine if this is the configuration file for a modpack with four branches: `1.20.1`, `1.20.4` and `1.21.3`.
With these inclusions and exclusions, `sodium` will be added to `1.20.1` and `1.20.4`, but not to branch `1.21.3`.
`indium` will only be added to `1.20.1`.

{: .note }
You can't add both inclusions and exclusions, but you can always add version overrides to a project.

### CLI commands
When adding a project with `packrinth project add`, you can use the flags `--inclusions` or `--exclusions` to
add them to all specified projects. To add or remove inclusions or exclusions at a later time, you can use the `add`
and `remove` subcommands related to `packrinth project inclusions` and `packrinth project exclusions`.