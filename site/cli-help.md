---
title: CLI Help
layout: default
---

# Command-Line Help for `packrinth`

This document contains the help content for the `packrinth` command-line program.

**Command Overview:**

* [`packrinth`↴](#packrinth)
* [`packrinth init`↴](#packrinth-init)
* [`packrinth import`↴](#packrinth-import)
* [`packrinth project`↴](#packrinth-project)
* [`packrinth project list`↴](#packrinth-project-list)
* [`packrinth project add`↴](#packrinth-project-add)
* [`packrinth project version-override`↴](#packrinth-project-version-override)
* [`packrinth project version-override add`↴](#packrinth-project-version-override-add)
* [`packrinth project version-override remove`↴](#packrinth-project-version-override-remove)
* [`packrinth project inclusions`↴](#packrinth-project-inclusions)
* [`packrinth project inclusions add`↴](#packrinth-project-inclusions-add)
* [`packrinth project inclusions remove`↴](#packrinth-project-inclusions-remove)
* [`packrinth project exclusions`↴](#packrinth-project-exclusions)
* [`packrinth project exclusions add`↴](#packrinth-project-exclusions-add)
* [`packrinth project exclusions remove`↴](#packrinth-project-exclusions-remove)
* [`packrinth project remove`↴](#packrinth-project-remove)
* [`packrinth branch`↴](#packrinth-branch)
* [`packrinth branch list`↴](#packrinth-branch-list)
* [`packrinth branch add`↴](#packrinth-branch-add)
* [`packrinth branch remove`↴](#packrinth-branch-remove)
* [`packrinth update`↴](#packrinth-update)
* [`packrinth export`↴](#packrinth-export)
* [`packrinth clean`↴](#packrinth-clean)
* [`packrinth doc`↴](#packrinth-doc)
* [`packrinth completions`↴](#packrinth-completions)
* [`packrinth version`↴](#packrinth-version)

## `packrinth`

**Usage:** `packrinth [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `init` — Initialize a new modpack project
* `import` — Import data from a Modrinth modpack to the existing Packrinth modpack
* `project` — Add or remove Modrinth projects and tweak them for your branches
* `branch` — Create and remove branches that separate your Modpack for various versions
* `update` — Update branches with the newest project versions
* `export` — Export a branch to a Modrinth modpack
* `clean` — Removes the target directory
* `doc` — Generate Markdown documentation
* `completions` — Generate shell completion for Packrinth
* `version` — Show information about the current Packrinth installation

###### **Options:**

* `-d`, `--directory <DIRECTORY>` — Set the root directory of the modpack (directory of modpack.json)
* `-v`, `--verbose` — Output more information about the current process



## `packrinth init`

Initialize a new modpack project

**Usage:** `packrinth init [OPTIONS] [MODPACK_NAME]`

###### **Arguments:**

* `<MODPACK_NAME>` — The name of the modpack to create.

   A directory with this name will be made in the working directory.

###### **Options:**

* `-G`, `--no-git-repo` — Don't initialize a Git repository
* `-f`, `--force` — Force initializing a new modpack even if one already exists



## `packrinth import`

Import data from a Modrinth modpack to the existing Packrinth modpack

**Usage:** `packrinth import [OPTIONS] <MODRINTH_PACK>`

###### **Arguments:**

* `<MODRINTH_PACK>` — Location of the Modrinth modpack to import

###### **Options:**

* `-p`, `--add-projects` — Add projects to the modpack configuration file if they aren't in there yet
* `-f`, `--force` — Force importing a modpack even if the branch already exists (the branch will be overwritten)
* `-D`, `--allow-dirty` — If the modpack is in a Git repository, allow importing even if there are uncommitted changes



## `packrinth project`

Add or remove Modrinth projects and tweak them for your branches

**Usage:** `packrinth project [PROJECTS]... [COMMAND]`

###### **Subcommands:**

* `list` — List all projects that are currently added to this modpack
* `add` — Add projects to this modpack
* `version-override` — Add a version override to a project in this modpack
* `inclusions` — Add inclusions to a project in this modpack
* `exclusions` — Add exclusions to a project in this modpack
* `remove` — Remove projects from this modpack

###### **Arguments:**

* `<PROJECTS>` — List information about added projects. If none are specified, all projects will be listed



## `packrinth project list`

List all projects that are currently added to this modpack

**Usage:** `packrinth project list`

**Command Alias:** `ls`



## `packrinth project add`

Add projects to this modpack

**Usage:** `packrinth project add [OPTIONS] <PROJECTS>...`

###### **Arguments:**

* `<PROJECTS>` — Projects to add

   The projects must be from Modrinth. You have to specify either the human-readable slug that appears in the URL (fabric-api) or the slug (P7dR8mSH).

###### **Options:**

* `-i`, `--inclusions <INCLUSIONS>` — Add branch inclusions for the projects that you are adding

   The added projects will only be updated for the branches you specify. For a project, you can only have inclusions OR exclusions.
* `-e`, `--exclusions <EXCLUSIONS>` — Add branch exclusions for the projects that you are adding

   The added projects will not be updated for the branches you specify, but the unspecified branches will be updated with this project. For a project, you can only have inclusions OR exclusions.



## `packrinth project version-override`

Add a version override to a project in this modpack

**Usage:** `packrinth project version-override <COMMAND>`

###### **Subcommands:**

* `add` — Add a version override to a project
* `remove` — Remove a version override from a project



## `packrinth project version-override add`

Add a version override to a project

**Usage:** `packrinth project version-override add <PROJECT> <BRANCH> <PROJECT_VERSION_ID>`

###### **Arguments:**

* `<PROJECT>` — Project to add the version override to
* `<BRANCH>` — Branch that you want to be overridden
* `<PROJECT_VERSION_ID>` — The version ID of the override

   This must be a Modrinth version ID. You can find this by going to a project on the Modrinth website, navigating to the version that you want to override and copying the version ID that looks something like this: Q8ssLFZp



## `packrinth project version-override remove`

Remove a version override from a project

**Usage:** `packrinth project version-override remove [OPTIONS] <PROJECT> [BRANCH]`

**Command Alias:** `rm`

###### **Arguments:**

* `<PROJECT>` — Project to remove the override from
* `<BRANCH>` — Branch to remove the override from

###### **Options:**

* `-a`, `--all` — Remove all overrides from a project



## `packrinth project inclusions`

Add inclusions to a project in this modpack

**Usage:** `packrinth project inclusions <COMMAND>`

###### **Subcommands:**

* `add` — Add inclusions to a project
* `remove` — Remove inclusions from a project



## `packrinth project inclusions add`

Add inclusions to a project

**Usage:** `packrinth project inclusions add <PROJECT> <INCLUSIONS>...`

###### **Arguments:**

* `<PROJECT>` — Project to add inclusions to
* `<INCLUSIONS>` — Branches to include



## `packrinth project inclusions remove`

Remove inclusions from a project

**Usage:** `packrinth project inclusions remove [OPTIONS] <PROJECT> [INCLUSIONS]...`

**Command Alias:** `rm`

###### **Arguments:**

* `<PROJECT>` — Project to remove inclusions from
* `<INCLUSIONS>` — Inclusions to remove

###### **Options:**

* `-a`, `--all` — Remove all inclusions from the project



## `packrinth project exclusions`

Add exclusions to a project in this modpack

**Usage:** `packrinth project exclusions <COMMAND>`

###### **Subcommands:**

* `add` — Add exclusions to a project
* `remove` — Remove exclusions from a project



## `packrinth project exclusions add`

Add exclusions to a project

**Usage:** `packrinth project exclusions add <PROJECT> <EXCLUSIONS>...`

###### **Arguments:**

* `<PROJECT>` — Project to add exclusions to
* `<EXCLUSIONS>` — Branches to exclude



## `packrinth project exclusions remove`

Remove exclusions from a project

**Usage:** `packrinth project exclusions remove [OPTIONS] <PROJECT> [EXCLUSIONS]...`

**Command Alias:** `rm`

###### **Arguments:**

* `<PROJECT>` — Project to remove exclusions from
* `<EXCLUSIONS>` — Exclusions to remove

###### **Options:**

* `-a`, `--all` — Remove all exclusions from the project



## `packrinth project remove`

Remove projects from this modpack

**Usage:** `packrinth project remove <PROJECTS>...`

**Command Alias:** `rm`

###### **Arguments:**

* `<PROJECTS>` — Projects to remove from the modpack



## `packrinth branch`

Create and remove branches that separate your Modpack for various versions

**Usage:** `packrinth branch [BRANCHES]... [COMMAND]`

###### **Subcommands:**

* `list` — List information about all branches
* `add` — Add new branches
* `remove` — Remove branches

###### **Arguments:**

* `<BRANCHES>` — Branches to list. If none are specified, you must use a subcommand



## `packrinth branch list`

List information about all branches

**Usage:** `packrinth branch list`

**Command Alias:** `ls`



## `packrinth branch add`

Add new branches

**Usage:** `packrinth branch add <BRANCHES>...`

**Command Alias:** `new`

###### **Arguments:**

* `<BRANCHES>` — Names of new branches to add



## `packrinth branch remove`

Remove branches

**Usage:** `packrinth branch remove <BRANCHES>...`

**Command Alias:** `rm`

###### **Arguments:**

* `<BRANCHES>` — Names of branches to remove



## `packrinth update`

Update branches with the newest project versions

**Usage:** `packrinth update [OPTIONS] [BRANCHES]...`

###### **Arguments:**

* `<BRANCHES>` — Branches to update. If no branches are specified, all branches will be updated

###### **Options:**

* `--no-alpha` — Don't allow alpha releases to be added to branch files
* `--no-beta` — Don't allow beta releases to be added to branch files
* `-r`, `--require-all` — For every environment (server and client), set all projects as required
* `-a`, `--auto-dependencies` — Automatically add any dependencies required by the projects in the modpack
* `-D`, `--allow-dirty` — If the modpack is in a Git repository, allow updating even if there are uncommitted changes



## `packrinth export`

Export a branch to a Modrinth modpack

**Usage:** `packrinth export [BRANCHES]...`

###### **Arguments:**

* `<BRANCHES>` — Branches to export. If no branches are specified, all branches will be exported



## `packrinth clean`

Removes the target directory

**Usage:** `packrinth clean`



## `packrinth doc`

Generate Markdown documentation

**Usage:** `packrinth doc [OPTIONS]`

###### **Options:**

* `-t`, `--table-only` — Only generate the project table
* `-C`, `--no-compatibility-icons`



## `packrinth completions`

Generate shell completion for Packrinth

**Usage:** `packrinth completions <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell to generate the completion for

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `packrinth version`

Show information about the current Packrinth installation

**Usage:** `packrinth version`




