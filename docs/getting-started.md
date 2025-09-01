---
title: Getting Started
layout: default
nav_order: 2
---

# Getting started
_This page assumes you have already installed Packrinth.
Instructions can be found [here](/#installation)._

__Mods, resource packs, data packs, shaders and plugins will be called `projects` from now on.__

## Initializing the modpack
To get started, initialize a Packrinth modpack instance with `packrinth init`.
This will create a new modpack in the current directory.
If you want to specify another directory, use the `--directory` or `-d` flag like so:
```bash
$ packrinth init -d /path/for/modpack
```
If the specified directory doesn't exist, it will be made, including all of its parents.
It is important to know that by default, Packrinth always uses the current directory as the
modpack location. It is always possible to use the `--directory` or `-d` flag like above.

While initializing, Packrinth has made a Git repository with a `.gitignore` file
(if those didn't already exist). The `.gitignore` ignores all `*.mrpack` files. These files are
Modrinth modpacks that you can eventually export.

Additionally, Packrinth has created a `modpack.json` file: the place for all global configuration,
including modpack name, author, branches and even all the projects. This is how it looks:
```json
{
        "pack_format": 1,
        "name": "My Modrinth modpack",
        "summary": "Short summary for this modpack",
        "author": "John Doe",
        "require_all": false,
        "auto_dependencies": false,
        "branches": [],
        "projects": {}
}
```
_modpack.json_

The pack format indicates what version the modpack was made in. This is important, because
this allows Packrinth to update to newer versions automatically.
The name, summary and author speak for themselves; you can edit them accordingly.

The field `require_all` is a little bit more obscure: if it is set to `true`, every environment
for all projects will be set to `required`. This means that when the user installs the modpack,
they will always install all mods, regardless of whether they are on a dedicated server or not.
Even more, some modpack installers may prompt the user if one of the environments is set to
`optional`. __A rule of thumbs is: if you want to separate mods for client and server, keep this
setting to `false`. If you want that every user downloads all projects you have specified, set
this setting to `true`.__

`auto_dependencies` decides if Packrinth should automatically add dependencies of added projects.
Most people should set this to `true` to prevent crashes caused by the lack of mod dependencies.
This field is set to `false` by default to prevent confusion when more projects are added than
you thought. It is important to know that only _required_ dependencies will be added.
If you want optional dependencies to be in your modpack, you have to add them manually.

We will ignore the `branches` and `projects` field, and we will come back later.