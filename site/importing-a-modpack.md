---
title: Importing a modpack
layout: default
nav_order: 6
---

# Importing a modpack
With Packrinth, it is possible to import a Modrinth modpack (`.mrpack`). Use this command for importing modpacks:
```bash
$ packrinth import <MODRINTH_PACK>
```
This command will create a new branch based on the name of the Modrinth modpack. It will copy the overrides and
branch files. Additionally, the `branch.json` configuration will be filled out as much as possible based on the
information of the original modpack. You should, however, check this file because some fields may be inaccurate
or incomplete.

The command above won't change anything other than adding a branch. To add projects of the original modpack
to the `modpack.json` file (so that `packrinth update` works), use the `--add-projects` flag.

{: .tip }
If you already have a manager for your modpack, `packrinth import` can make migrating to Packrinth a lot easier
by doing the hard work for you!
