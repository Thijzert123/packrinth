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
|[Searchables](https://modrinth.com/project/fuuu3xnx)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Show Me Your Skin!](https://modrinth.com/project/bD7YqcA3)|✅|✅|✅|❌|❌|❌|✅|✅|✅|
|[Shulker Box Tooltip](https://modrinth.com/project/2M01OLQq)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Simple Fog Control](https://modrinth.com/project/Glp1bwYc)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Simple Grass Flowers](https://modrinth.com/project/ti9KkMHm)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Simple Voice Chat](https://modrinth.com/project/9eGKb6K1)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Skin Grabber](https://modrinth.com/project/TtybOAsL)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Skin Shuffle](https://modrinth.com/project/3s19I5jr)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Sodium](https://modrinth.com/project/AANobbMI)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Sodium Extra](https://modrinth.com/project/PtjYWJkn)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Solas Shader](https://modrinth.com/project/EpQFjzrQ)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Sound Controller](https://modrinth.com/project/uY9zbflw)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Sounds](https://modrinth.com/project/ZouiUX7t)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Spawned Eggs](https://modrinth.com/project/yPBwDzHA)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Spectral](https://modrinth.com/project/vaaOMowT)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Spryzeen's Healthbars](https://modrinth.com/project/ZMcqgmIV)|❌|❌|✅|❌|❌|❌|❌|❌|❌|
|[Spryzeen's Knight Armor](https://modrinth.com/project/EwJHG2NA)|❌|❌|✅|✅|✅|✅|❌|❌|❌|
|[Status Effect Bars](https://modrinth.com/project/x02cBj9Y)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Subtle Effects](https://modrinth.com/project/4q8UOK1d)|✅|❌|✅|✅|✅|✅|✅|✅|✅|
|[Super Duper Vanilla](https://modrinth.com/project/LMIZZNxZ)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Sushi Bar](https://modrinth.com/project/tr2Mv6ke)|✅|✅|❌|❌|❌|❌|❌|❌|❌|
|[Symbol Chat](https://modrinth.com/project/NKvLVQMc)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[TCDCommons API](https://modrinth.com/project/Eldc1g37)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Text Placeholder API](https://modrinth.com/project/eXts2L7r)|❌|✅|✅|✅|✅|✅|✅|✅|✅|
|[Theone's Eating Animation Pack](https://modrinth.com/project/OhzX8kDf)|❌|❌|❌|❌|✅|✅|✅|✅|✅|
|[Torturable Healthbars](https://modrinth.com/project/WPuyL1eO)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Torturable Healthbars, but with FA](https://modrinth.com/project/mQpUi57Q)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Translations for Sodium](https://modrinth.com/project/yfDziwn1)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[UniLib](https://modrinth.com/project/nT86WUER)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[VTDownloader](https://modrinth.com/project/1E2sq1cp)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Vervada's enhanced plants](https://modrinth.com/project/ghc0v6DT)|❌|❌|✅|✅|✅|✅|❌|❌|❌|
|[View Bobbing Options](https://modrinth.com/project/Yr9J16k6)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Visuality](https://modrinth.com/project/rI0hvYcd)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Visuals](https://modrinth.com/project/pWBAsHgt)|❌|❌|❌|❌|❌|✅|✅|✅|✅|
|[Voice Chat Soundboard](https://modrinth.com/project/N8s60DWW)|✅|✅|✅|❌|✅|❌|❌|❌|❌|
|[WaxedIcons](https://modrinth.com/project/pC9ELBuh)|❌|❌|✅|✅|✅|✅|✅|✅|✅|
|[Wider Tab](https://modrinth.com/project/IA3kkkhV)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Withered Hearts](https://modrinth.com/project/LQI4ZTHY)|❌|❌|✅|✅|✅|✅|✅|✅|✅|
|[World Play Time](https://modrinth.com/project/YkKeggdl)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[WorldEdit](https://modrinth.com/project/1u6JkXh5)|❌|✅|✅|✅|✅|✅|✅|✅|✅|
|[Xaero's Minimap](https://modrinth.com/project/1bokaNcj)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Xaero's World Map](https://modrinth.com/project/NcUtCpym)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[XaeroPlus](https://modrinth.com/project/EnPUzSTg)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Xander's Sodium Options](https://modrinth.com/project/sTkQBVyo)|✅|❌|✅|✅|✅|✅|❌|❌|❌|
|[YetAnotherConfigLib (YACL)](https://modrinth.com/project/1eAoo2KR)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Your Options Shall Be Respected (YOSBR)](https://modrinth.com/project/WwbubTsV)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[Zoomify](https://modrinth.com/project/w7ThoJFB)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[[EMF] Entity Model Features](https://modrinth.com/project/4I1XuqiY)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[[ETF] Entity Texture Features](https://modrinth.com/project/BVzZfTc1)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[cat_jam](https://modrinth.com/project/x3s69afN)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[e4mc](https://modrinth.com/project/qANg5Jrr)|✅|✅|✅|✅|✅|✅|✅|✅|✅|
|[oωo (owo-lib)](https://modrinth.com/project/ccKDOlHs)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[qrafty's Capitalized Font](https://modrinth.com/project/FA4ebMMU)|✅|✅|✅|✅|✅|✅|✅|✅|✅|