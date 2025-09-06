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

---

# Client+ _by Thijzert ([https://github.com/Thijzert123](https://github.com/Thijzert123))_
Modpack focused on improving vanilla Minecraft on the client-side.
## What is included?

|Name|1.21.8|1.21.7|1.21.6|1.21.5|1.21.4|1.21.3|1.21.1|1.20.4|1.20.1|
|:--|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|[(Bee's) Fancy Crops](https://modrinth.com/project/UGEVQ6t9)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[(Bee's) Fancy Crops](https://modrinth.com/project/fancy-crops)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[3D Skin Layers](https://modrinth.com/project/zV5r3pPn)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[No Chat Reports](https://modrinth.com/project/qQyHxfxd)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[No Chat Reports](https://modrinth.com/project/no-chat-reports)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Sodium](https://modrinth.com/project/AANobbMI)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Sodium Extra](https://modrinth.com/project/PtjYWJkn)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Sodium Extra](https://modrinth.com/project/sodium-extra)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Solas Shader](https://modrinth.com/project/solas-shader)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Solas Shader](https://modrinth.com/project/EpQFjzrQ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Sound Controller](https://modrinth.com/project/sound-controller)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Sound Controller](https://modrinth.com/project/uY9zbflw)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Sounds](https://modrinth.com/project/sound)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Sounds](https://modrinth.com/project/ZouiUX7t)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Spawned Eggs](https://modrinth.com/project/spawned-eggs)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Spectral](https://modrinth.com/project/spectral)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Zoomify](https://modrinth.com/project/zoomify)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[[EMF] Entity Model Features](https://modrinth.com/project/entity-model-features)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[[EMF] Entity Model Features](https://modrinth.com/project/4I1XuqiY)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[[ETF] Entity Texture Features](https://modrinth.com/project/entitytexturefeatures)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[[ETF] Entity Texture Features](https://modrinth.com/project/BVzZfTc1)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[cat_jam](https://modrinth.com/project/cat_jam)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[e4mc](https://modrinth.com/project/qANg5Jrr)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[e4mc](https://modrinth.com/project/e4mc)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[qrafty's Capitalized Font](https://modrinth.com/project/qraftys-capitalized-font)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[qrafty's Capitalized Font](https://modrinth.com/project/FA4ebMMU)|❌|❌|❌|❌|❌|❌|❌|❌|✅|