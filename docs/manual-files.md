---
title: Manual files
layout: default
nav_order: 6
---

# Manual files

Everything we have seen so far only supports projects hosted on Modrinth. However, you might want to add other
projects that are hosted on GitHub, or even CurseForge. To do this, you can add the `manual_files` key to the branch
configuration (`branch.json`). You have to put a list of files.
To see what information you need to provide, look at the [configuration reference](configuration-reference.md#file-object).

{: .important }
> You can add any manual files and export it to a Modrinth modpack, but to host a Modrinth modpack on the Modrinth
> website, the download links can only be from these websites:
> - `cdn.modrinth.com`
> - `github.com`
> - `raw.githubusercontent.com`
> - `gitlab.com`