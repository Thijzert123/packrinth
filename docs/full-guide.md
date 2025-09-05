---
title: Full guide
layout: default
nav_order: 2
---

# Full guide
{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

{: .note }
This page assumes you have already installed Packrinth.
Instructions can be found [here](https://thijzert123.github.io/packrinth/#installation).

{: .note }
Mods, resource packs, data packs, shaders and plugins will be called `projects` from now on.

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
`optional`.

{: .summary }
If you want to separate mods for client and server, keep this
setting to `false`. If you want that every user downloads all projects you have specified, set
this setting to `true`.

`auto_dependencies` decides if Packrinth should automatically add dependencies of added projects.
Most people should set this to `true` to prevent crashes caused by the lack of mod dependencies.
This field is set to `false` by default to prevent confusion when more projects are added than
you thought. It is important to know that only _required_ dependencies will be added.
If you want optional dependencies to be in your modpack, you have to add them manually.

{: .note }
We will ignore the `branches` and `projects` field, and we will come back to it later.

## Branches
A Packrinth modpack always consists of at lease one branch. A branch has its own project versions and is completely
separate from other branches. It also has its own _overrides_, files that get put in the `.minecraft` directory
upon installation of the modpack.

A common usage for branches is having a branch for every Minecraft version. That way, you can support multiple
Minecraft versions at once without having to overwrite the data of older versions and not being able to update
your modpack for older Minecraft versions.

To add a new branch, use this command:
```bash
$ packrinth branch add <BRANCHES>
```
You can also pass more branches to create multiple at once. If you add two branches (`1.21.5` and `1.21.8`)
for the respective Minecraft versions, the modpack directory tree will look like this:
```
root
├── 1.21.5
│   ├── branch.json
├── 1.21.8
│   ├── branch.json
└── modpack.json
```
As you can see, for every branch a config file with the name `branch.json` is created. It looks like this by default:
```json
{
        "version": "1.0.0-fabric",
        "minecraft_version": "1.21.8",
        "acceptable_minecraft_versions": [
                "1.21.6",
                "1.21.7"
        ],
        "mod_loader": "fabric",
        "loader_version": "0.17.2",
        "acceptable_loaders": [
                "minecraft",
                "vanilla"
        ]
}
```
_branch.json_

Obviously, assuming this is the configuration file for our branch `1.21.5`, it is not right for our purposes.
You can change the version to `1.0.0-fabric+1.21.5` to make it clearer to users what modpack branch this is.
The version will be included in the final `.mrpack` file. Additionally, change `minecraft_version` to the version
the branch is targeting. If you add Minecraft versions to `acceptable_minecraft_versions`, projects that
only target Minecraft 1.21.7, in this case, will still be added to this branch, even tough the branch originally targeted
Minecraft 1.21.8.

`mod_loader` specifies the mod loader that gets installed alongside Minecraft by the modpack installer. It is important
to set the correct mod loader version in `loader_version`. You can check possible `mod_loader` values [here](loaders.html#main-mod-loader).
Similar to `acceptable_minecraft_versions`, `acceptable_loaders` allows other loaders than the main mod loader
to be added to the branch. For a full list, check [here](loaders.html#other-loaders).

{: .note }
You can also choose to not provide a main mod loader. To do this, remove the `mod_loader` and `loader_version`
fields.

## Adding projects
Let's add some projects to our modpack with `packrinth project add <PROJECTS>`. To add Sodium and Lithium, run this:
```bash
$ packrinth project add sodium lithium
```
This adds both projects to the `projects` field in `modpack.json`. Note that you have to use Modrinth's project ID.
You can find it in the URL when you are on the front page of a project. You can also use the more obscure ID (`AANobbMI`),
but this would just make it more difficult to remove the project at a later time with `packrinth project remove`.

## Updating the branches
The projects are currently only in our modpack configuration, not in our branches. We can fix this by running:
```bash
$ packrinth update [BRANCHES]
```
If you don't specify any branches, all branches will be updated. Packrinth pulls data from Modrinth: it finds every
newest project suitable for your branch settings. It will put all this data in a new file `.branch_files.json`.
This file contains data that can directly be used for exporting your modpack. This way, export time is minimized
because no requests to the Modrinth API have to be made.

{: .important }
You should never edit this file: always tweak your settings
in `modpack.json` or `branch.json` and then run `packrinth update`. You should, however, add the file to your Git
repository, to make sure you can always export the exact same modpack. If you are familiar with Cargo, it works
about the same as the `Cargo.lock` file.

## Exporting a branch
It is time to export to a Modrinth modpack! To do so, run this command:
```bash
$ packrinth export [BRANCHES]
```
If no branches are provided, all branches will be exported. The final file will be in the directory of the branch.
The file name is produced like this:
```
name_version.mrpack
```
The `name` refers to the `name` field in `modpack.json`, and the `version` field refers to the `version` field
in the relevant `branch.json`.

{: .note }
Exporting a modpack doesn't take long, because all the web requests have already been made during `packrinth update`!

The final modpack file can be uploaded to Modrinth, or you can privately distribute it among your friends.

## Further reference
Please read one of the other guides for more specific documentation for other features.