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

Below is the generated config output from `packrinth doc`.

---

# Client+ _by Thijzert ([https://github.com/Thijzert123](https://github.com/Thijzert123))_
Modpack focused on improving vanilla Minecraft on the client-side.
## What is included?

|Name|1.21.8|1.21.7|1.21.6|1.21.5|1.21.4|1.21.3|1.21.1|1.20.4|1.20.1|
|:--|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|[(Bee's) Fancy Crops](https://modrinth.com/project/UGEVQ6t9)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[(Bee's) Fancy Crops](https://modrinth.com/project/fancy-crops)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[3D Skin Layers](https://modrinth.com/project/zV5r3pPn)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[3D Skin Layers](https://modrinth.com/project/3dskinlayers)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Advancement Plaques](https://modrinth.com/project/advancement-plaques)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Animated Items](https://modrinth.com/project/uBBepXuH)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Animated Items](https://modrinth.com/project/animated-items)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Animatica](https://modrinth.com/project/PRN43VSY)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Animatica](https://modrinth.com/project/animatica)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[AppleSkin](https://modrinth.com/project/EsAfCjCV)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[AppleSkin](https://modrinth.com/project/appleskin)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Architectury API](https://modrinth.com/project/lhGA9TYQ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Armor Trim Item Fix](https://modrinth.com/project/armor-trim-item-fix)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Auth Me](https://modrinth.com/project/auth-me)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Auth Me](https://modrinth.com/project/yjgIrBjZ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Auto-Run](https://modrinth.com/project/autorun)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Auto-Run](https://modrinth.com/project/2i7tg1Wv)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[BSL Shaders](https://modrinth.com/project/Q1vvjJYV)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[BSL Shaders](https://modrinth.com/project/bsl-shaders)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[BadOptimizations](https://modrinth.com/project/badoptimizations)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[BadOptimizations](https://modrinth.com/project/g96Z4WVZ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Bare Bones](https://modrinth.com/project/rox3U8B6)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Bare Bones](https://modrinth.com/project/bare-bones)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better Advancements](https://modrinth.com/project/Q2OqKxDG)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Better Advancements](https://modrinth.com/project/better-advancements)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better Command Block UI](https://modrinth.com/project/8iQcgjQ2)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Better Command Block UI](https://modrinth.com/project/bettercommandblockui)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better F1 Reborn](https://modrinth.com/project/better-f1-reborn)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better F1 Reborn](https://modrinth.com/project/2JIeCmxb)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Better Flame Particles](https://modrinth.com/project/ivUZsvzp)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Better Flame Particles](https://modrinth.com/project/better-flame-particles)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better Lanterns](https://modrinth.com/project/PGGrfcvL)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Better Lanterns](https://modrinth.com/project/better-lanterns)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better Mount HUD](https://modrinth.com/project/kqJFAPU9)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Better Mount HUD](https://modrinth.com/project/better-mount-hud)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better Statistics Screen](https://modrinth.com/project/n6PXGAoM)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Better Statistics Screen](https://modrinth.com/project/better-stats)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Better Suggestions](https://modrinth.com/project/better-suggestions)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Better Suggestions](https://modrinth.com/project/HfZKWsjM)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[BetterF3](https://modrinth.com/project/8shC1gFX)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[BetterF3](https://modrinth.com/project/betterf3)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[BindCommands](https://modrinth.com/project/bindcommands)|❌|✅|✅|✅|✅|✅|✅|✅|❌|
|[BindCommands](https://modrinth.com/project/WeytAdLH)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Bliss Shaders](https://modrinth.com/project/bliss-shader)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Bobby](https://modrinth.com/project/bobby)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Bobby](https://modrinth.com/project/M08ruV16)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Bow Load Indicator](https://modrinth.com/project/dj5wVJsq)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Bow Load Indicator](https://modrinth.com/project/bow-load-indicator)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[CICADA](https://modrinth.com/project/IwCkru1D)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[CalcMod](https://modrinth.com/project/calcmod)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[CalcMod](https://modrinth.com/project/XoHTb2Ap)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Capes](https://modrinth.com/project/89Wsn8GD)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Capes](https://modrinth.com/project/capes)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Cave Dust](https://modrinth.com/project/cave-dust)|✅|❌|✅|✅|✅|✅|❌|❌|❌|
|[Chat Animation [Smooth Chat]](https://modrinth.com/project/DnNYdJsx)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Chat Animation [Smooth Chat]](https://modrinth.com/project/chatanimation)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Chat Heads](https://modrinth.com/project/Wb5oqrBJ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Chat Heads](https://modrinth.com/project/chat-heads)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Chat Reporting Helper](https://modrinth.com/project/chat-reporting-helper)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Chat Reporting Helper](https://modrinth.com/project/tN4E9NfV)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Cherished Worlds](https://modrinth.com/project/cherished-worlds)|✅|✅|✅|❌|✅|✅|✅|✅|❌|
|[Cherished Worlds](https://modrinth.com/project/3azQ6p0W)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Chunks fade in](https://modrinth.com/project/JaNmzvA8)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Chunks fade in](https://modrinth.com/project/chunks-fade-in)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Chunky](https://modrinth.com/project/chunky)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Chunky](https://modrinth.com/project/fALzjamp)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Client Sort](https://modrinth.com/project/K0AkAin6)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Client Sort](https://modrinth.com/project/clientsort)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Cloth Config API](https://modrinth.com/project/9s6osm5g)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Clumps](https://modrinth.com/project/Wnxd13zP)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Clumps](https://modrinth.com/project/clumps)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Collective](https://modrinth.com/project/e0M1UDsY)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Complementary Shaders - Reimagined](https://modrinth.com/project/HVnmMxH1)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Complementary Shaders - Reimagined](https://modrinth.com/project/complementary-reimagined)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Complementary Shaders - Unbound](https://modrinth.com/project/complementary-unbound)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Complementary Shaders - Unbound](https://modrinth.com/project/R6NEzAwj)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Continuity](https://modrinth.com/project/continuity)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Continuity](https://modrinth.com/project/1IjD5062)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Controlify (Controller support)](https://modrinth.com/project/controlify)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Controlify (Controller support)](https://modrinth.com/project/DOUdJVEm)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Controlling](https://modrinth.com/project/controlling)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Controlling](https://modrinth.com/project/xv94TkTM)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[CraftPresence](https://modrinth.com/project/craftpresence)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[CraftPresence](https://modrinth.com/project/DFqQfIBR)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Creative Fly](https://modrinth.com/project/XrD3Auyv)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Creative Fly](https://modrinth.com/project/creative-fly)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Cubes Without Borders](https://modrinth.com/project/ETlrkaYF)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Cubes Without Borders](https://modrinth.com/project/cubes-without-borders)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Cubic Sun & Moon](https://modrinth.com/project/cubic-sun-moon)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Cubic Sun & Moon](https://modrinth.com/project/g4bSYbrU)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Custom Credits](https://modrinth.com/project/GhWh8CAU)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Custom Credits](https://modrinth.com/project/custom-credits)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Debugify](https://modrinth.com/project/QwxR6Gcd)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Debugify](https://modrinth.com/project/debugify)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Default Dark Mode](https://modrinth.com/project/default-dark-mode)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Default Dark Mode](https://modrinth.com/project/6SLU7tS5)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Default Splashes](https://modrinth.com/project/RMESe7qr)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Default Splashes](https://modrinth.com/project/default-splashes)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Detail Armor Bar](https://modrinth.com/project/detail-armor-bar)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Dramatic Skys](https://modrinth.com/project/2YyNMled)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Dramatic Skys](https://modrinth.com/project/dramatic-skys)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Durability Plus](https://modrinth.com/project/na1dL51S)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Durability Plus](https://modrinth.com/project/durability-plus)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Dynamic Crosshair](https://modrinth.com/project/ZcR9weSm)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Dynamic Crosshair](https://modrinth.com/project/dynamiccrosshair)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Dynamic FPS](https://modrinth.com/project/dynamic-fps)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Dynamic FPS](https://modrinth.com/project/LQ3K71Q1)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Eating Animation](https://modrinth.com/project/eating-animation)|✅|✅|✅|✅|❌|❌|❌|❌|❌|
|[Emotecraft](https://modrinth.com/project/emotecraft)|✅|✅|✅|❌|✅|✅|❌|✅|❌|
|[Emotecraft](https://modrinth.com/project/pZ2wrerK)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Enchant Icons](https://modrinth.com/project/enchant-icons-countxd)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Enhanced Block Entities](https://modrinth.com/project/ebe)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Entity Culling](https://modrinth.com/project/entityculling)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Entity Culling](https://modrinth.com/project/NNAgCjsB)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Entity View Distance](https://modrinth.com/project/entity-view-distance)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Entity View Distance](https://modrinth.com/project/ihnBJ6on)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Even Better Enchants](https://modrinth.com/project/even-better-enchants)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Even Better Enchants](https://modrinth.com/project/6udpuGCH)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Explosive Enhancement](https://modrinth.com/project/OSQ8mw2r)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Explosive Enhancement](https://modrinth.com/project/explosive-enhancement)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Fabric API](https://modrinth.com/project/P7dR8mSH)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Fabric Language Kotlin](https://modrinth.com/project/Ha28R6CL)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Fabrishot](https://modrinth.com/project/fabrishot)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Fabrishot](https://modrinth.com/project/3qsfQtE9)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Falling Leaves](https://modrinth.com/project/WhbRG4iK)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Falling Leaves](https://modrinth.com/project/fallingleaves)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[FancyMenu](https://modrinth.com/project/Wq5SjeWM)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[FancyMenu](https://modrinth.com/project/fancymenu)|✅|✅|✅|❌|✅|✅|✅|✅|❌|
|[Fast IP Ping](https://modrinth.com/project/9mtu0sUO)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Fast IP Ping](https://modrinth.com/project/fast-ip-ping)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Fast Trading](https://modrinth.com/project/Ht0RRAt0)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Fast Trading](https://modrinth.com/project/fast-trading)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[FastQuit](https://modrinth.com/project/fastquit)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[FastQuit](https://modrinth.com/project/x1hIzbuY)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[FerriteCore](https://modrinth.com/project/ferrite-core)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[FerriteCore](https://modrinth.com/project/uXXizFIs)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Flashback](https://modrinth.com/project/4das1Fjq)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Flashback](https://modrinth.com/project/flashback)|❌|❌|✅|✅|✅|✅|✅|✅|❌|
|[Forge Config API Port](https://modrinth.com/project/ohNO6lps)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Fresh Animations](https://modrinth.com/project/fresh-animations)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Fresh Animations](https://modrinth.com/project/50dA9Sha)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Fresh Moves](https://modrinth.com/project/tras-fresh-player)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Fullbright UB](https://modrinth.com/project/ItHr72Fy)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Fullbright UB](https://modrinth.com/project/fullbright-ub)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Fzzy Config](https://modrinth.com/project/hYykXjDp)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Gamma Utils (Fullbright)](https://modrinth.com/project/wdLuzzEP)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Gamma Utils (Fullbright)](https://modrinth.com/project/gamma-utils)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Glowing Torchflower](https://modrinth.com/project/glowing-torchflower)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Glowing Torchflower](https://modrinth.com/project/1S4LxcvL)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Grimdark Sky Pack](https://modrinth.com/project/grimdark-sky)|✅|❌|✅|❌|❌|❌|❌|❌|❌|
|[Hey Wiki](https://modrinth.com/project/hey-wiki)|❌|✅|✅|✅|✅|✅|✅|✅|❌|
|[Hey Wiki](https://modrinth.com/project/6DnswkCZ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Hyper Realistic Sky](https://modrinth.com/project/hyper-realistic-sky)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Hyper Realistic Sky](https://modrinth.com/project/PsMUgCo5)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Icons](https://modrinth.com/project/O7z3QKAG)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Icons](https://modrinth.com/project/icons)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Icons Advertisement Removal [1.8 - 1.21.5]](https://modrinth.com/project/7Rq0ipFz)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Icons Advertisement Removal [1.8 - 1.21.5]](https://modrinth.com/project/icons-advertisement-removal)|✅|❌|✅|✅|✅|✅|✅|✅|❌|
|[ImmediatelyFast](https://modrinth.com/project/5ZwdcRci)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[ImmediatelyFast](https://modrinth.com/project/immediatelyfast)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Improved Sign Editing](https://modrinth.com/project/improved-sign-editing)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Improved Sign Editing](https://modrinth.com/project/EWQifKYI)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Indium](https://modrinth.com/project/indium)|✅|✅|✅|❌|❌|❌|❌|❌|❌|
|[Inventory Blur](https://modrinth.com/project/lTS6nyFs)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Inventory Blur](https://modrinth.com/project/inventory-blur)|❌|❌|✅|✅|✅|✅|✅|✅|❌|
|[Iris Shaders](https://modrinth.com/project/YL57xq9U)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Iris Shaders](https://modrinth.com/project/iris)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Item Highlighter](https://modrinth.com/project/item-highlighter)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Jade 🔍](https://modrinth.com/project/nvQzSEkH)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Jade 🔍](https://modrinth.com/project/jade)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Kinecraft Serialization](https://modrinth.com/project/kinecraft-serialization)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Konkrete](https://modrinth.com/project/J81TRJWm)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Krypton](https://modrinth.com/project/krypton)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Krypton](https://modrinth.com/project/fQEb0iXm)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[LambDynamicLights](https://modrinth.com/project/lambdynamiclights)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[LambDynamicLights](https://modrinth.com/project/yBW8D80W)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Language Reload](https://modrinth.com/project/language-reload)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Language Reload](https://modrinth.com/project/uLbm7CG6)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Legendary Tooltips](https://modrinth.com/project/legendary-tooltips)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[LibJF](https://modrinth.com/project/WKwQAwke)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Lighty](https://modrinth.com/project/lighty)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Lighty](https://modrinth.com/project/yjvKidNM)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Litematica](https://modrinth.com/project/litematica)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Litematica](https://modrinth.com/project/bEpr0Arc)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Lithium](https://modrinth.com/project/gvQqBUqZ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Lithium](https://modrinth.com/project/lithium)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Low On Fire](https://modrinth.com/project/low-on-fire)|✅|❌|✅|✅|✅|✅|✅|✅|❌|
|[Low On Fire](https://modrinth.com/project/RRxvWKNC)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[M.R.U](https://modrinth.com/project/SNVQ2c0g)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[MaLiLib](https://modrinth.com/project/GcWjdA9I)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Mace but 3D](https://modrinth.com/project/mace-but-3d-resourcepack)|❌|❌|❌|❌|✅|✅|✅|✅|❌|
|[Mace but 3D](https://modrinth.com/project/r9aFsDLk)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Mace but 3D](https://modrinth.com/project/mace-but-3d)|❌|❌|✅|✅|❌|❌|❌|❌|❌|
|[Make Bubbles Pop](https://modrinth.com/project/make_bubbles_pop)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Make Bubbles Pop](https://modrinth.com/project/gPCdW0Wr)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[MakeUp - Ultra Fast](https://modrinth.com/project/makeup-ultra-fast-shaders)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[MakeUp - Ultra Fast](https://modrinth.com/project/izsIPI7a)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Melody](https://modrinth.com/project/CVT4pFB2)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Meme Soundboard](https://modrinth.com/project/meme-soundboard)|❌|❌|✅|✅|✅|✅|❌|❌|❌|
|[Miniature Shader](https://modrinth.com/project/miniature-shader)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Miniature Shader](https://modrinth.com/project/UaS8ROxa)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[MixinTrace](https://modrinth.com/project/mixintrace)|✅|✅|✅|❌|❌|❌|❌|❌|❌|
|[Mob Crates](https://modrinth.com/project/mob-crates)|✅|❌|✅|✅|✅|✅|❌|❌|❌|
|[Mod Loading Screen](https://modrinth.com/project/mod-loading-screen)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Mod Loading Screen](https://modrinth.com/project/xAGJ6rQS)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Mod Menu](https://modrinth.com/project/modmenu)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Mod Menu](https://modrinth.com/project/mOgUt4GM)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Mod Sets](https://modrinth.com/project/mod-sets)|✅|✅|✅|❌|✅|❌|❌|❌|❌|
|[Model Gap Fix](https://modrinth.com/project/QdG47OkI)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Model Gap Fix](https://modrinth.com/project/modelfix)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[ModernFix](https://modrinth.com/project/modernfix)|✅|✅|✅|❌|✅|❌|❌|❌|❌|
|[More Chat History](https://modrinth.com/project/morechathistory)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[More Chat History](https://modrinth.com/project/8qkXwOnk)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[More Culling](https://modrinth.com/project/moreculling)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[More Culling](https://modrinth.com/project/51shyZVL)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Motschen's Better Leaves](https://modrinth.com/project/better-leaves)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Motschen's Better Leaves](https://modrinth.com/project/uvpymuxq)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Mouse Tweaks](https://modrinth.com/project/mouse-tweaks)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Mouse Tweaks](https://modrinth.com/project/aC3cM3Vq)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Music Delay Remover (Infinite Music)](https://modrinth.com/project/infinite-music)|✅|✅|✅|✅|❌|❌|❌|❌|❌|
|[Music Notification](https://modrinth.com/project/music-notification)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Music Notification](https://modrinth.com/project/A4YQgwzz)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[My Resource Pack, My Choice](https://modrinth.com/project/PTj85Anz)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[My Resource Pack, My Choice](https://modrinth.com/project/my-resource-pack)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[New Glowing Ores](https://modrinth.com/project/new-glowing-ores)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[New Glowing Ores](https://modrinth.com/project/oL18adaQ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[No Chat Reports](https://modrinth.com/project/qQyHxfxd)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[No Chat Reports](https://modrinth.com/project/no-chat-reports)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[No Fortune Chest](https://modrinth.com/project/nofortunechest)|❌|✅|✅|✅|✅|✅|✅|✅|❌|
|[No Resource Pack Warnings](https://modrinth.com/project/no-resource-pack-warnings)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[No Resource Pack Warnings](https://modrinth.com/project/6xKUDQcB)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Noisium](https://modrinth.com/project/noisium)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Not Enough Animations](https://modrinth.com/project/MPCX6s5C)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Not Enough Animations](https://modrinth.com/project/not-enough-animations)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Nuit (formerly FabricSkyboxes)](https://modrinth.com/project/nuit)|✅|✅|✅|❌|✅|❌|❌|❌|❌|
|[Nuit Interop (formerly FabricSkyBoxes Interop)](https://modrinth.com/project/nuit-interop)|✅|✅|✅|❌|✅|❌|❌|❌|❌|
|[Online Emotes](https://modrinth.com/project/online-emotes)|✅|✅|✅|❌|✅|✅|❌|✅|❌|
|[Online Emotes](https://modrinth.com/project/Dc4g4seU)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[OptiGUI](https://modrinth.com/project/JuksLGBQ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[OptiGUI](https://modrinth.com/project/optigui)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Optiboxes](https://modrinth.com/project/DWuwk8aA)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Options Profiles](https://modrinth.com/project/DnyS3EEW)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Options Profiles](https://modrinth.com/project/options-profiles)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Panorama](https://modrinth.com/project/swd-panorama)|❌|❌|❌|❌|✅|❌|❌|❌|❌|
|[Particular ✨](https://modrinth.com/project/particular)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Pick Up Notifier](https://modrinth.com/project/ZX66K16c)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Pick Up Notifier](https://modrinth.com/project/pick-up-notifier)|✅|✅|✅|✅|✅|✅|❌|✅|❌|
|[Player Animation Library](https://modrinth.com/project/ha1mEyJS)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Polytone](https://modrinth.com/project/polytone)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Puzzles Lib](https://modrinth.com/project/QAGBst4M)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Raise Sound Limit Simplified](https://modrinth.com/project/rsls)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Raise Sound Limit Simplified](https://modrinth.com/project/SKW62Pht)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Recolourful Containers GUI + HUD](https://modrinth.com/project/lewweaHO)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Recolourful Containers GUI + HUD](https://modrinth.com/project/recolourful-containers-gui)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Recolourful Containers GUI + HUD (DARK)](https://modrinth.com/project/recolourful-containers-gui-hud-dark)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Recolourful Containers GUI + HUD (DARK)](https://modrinth.com/project/sQCUH0Mr)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Redstone Tweaks](https://modrinth.com/project/RvfAlf4Z)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Redstone Tweaks](https://modrinth.com/project/redstone-tweaks)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Reimagined](https://modrinth.com/project/reimagined)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Reimagined](https://modrinth.com/project/ta5dy0aA)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Resourcify](https://modrinth.com/project/resourcify)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Resourcify](https://modrinth.com/project/RLzHAoZe)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Respackopts](https://modrinth.com/project/TiF5QWZY)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Respackopts](https://modrinth.com/project/respackopts)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Rethinking Voxels](https://modrinth.com/project/rethinking-voxels)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Rethinking Voxels](https://modrinth.com/project/kmwfVOoi)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Roughly Enough Items (REI)](https://modrinth.com/project/rei)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Roughly Enough Items (REI)](https://modrinth.com/project/nfn13YXA)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Screenshot Viewer](https://modrinth.com/project/screenshot-viewer)|✅|✅|✅|✅|✅|❌|❌|❌|❌|
|[Screenshot to Clipboard](https://modrinth.com/project/screenshot-to-clipboard)|✅|✅|✅|❌|❌|❌|❌|❌|❌|
|[Scribble](https://modrinth.com/project/yXAvIk0x)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Scribble](https://modrinth.com/project/scribble)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Seamless Loading Screen ](https://modrinth.com/project/seamless-loading-screen)|✅|✅|✅|❌|❌|❌|❌|❌|❌|
|[Searchables](https://modrinth.com/project/fuuu3xnx)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Show Me Your Skin!](https://modrinth.com/project/bD7YqcA3)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Show Me Your Skin!](https://modrinth.com/project/show-me-your-skin)|✅|✅|✅|❌|❌|❌|✅|✅|❌|
|[Shulker Box Tooltip](https://modrinth.com/project/2M01OLQq)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Shulker Box Tooltip](https://modrinth.com/project/shulkerboxtooltip)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Simple Fog Control](https://modrinth.com/project/Glp1bwYc)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Simple Fog Control](https://modrinth.com/project/simplefog)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Simple Grass Flowers](https://modrinth.com/project/simple-grass-flowers)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Simple Voice Chat](https://modrinth.com/project/9eGKb6K1)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Simple Voice Chat](https://modrinth.com/project/simple-voice-chat)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Skin Grabber](https://modrinth.com/project/TtybOAsL)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Skin Grabber](https://modrinth.com/project/skin-grabber)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Skin Shuffle](https://modrinth.com/project/3s19I5jr)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Skin Shuffle](https://modrinth.com/project/skinshuffle)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Sodium](https://modrinth.com/project/sodium)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
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
|[Spryzeen's Healthbars](https://modrinth.com/project/spryzeens-healthbars)|❌|❌|✅|❌|❌|❌|❌|❌|❌|
|[Spryzeen's Knight Armor](https://modrinth.com/project/spryzeens-knight-armor)|❌|❌|✅|✅|✅|✅|❌|❌|❌|
|[Status Effect Bars](https://modrinth.com/project/x02cBj9Y)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Status Effect Bars](https://modrinth.com/project/status-effect-bars)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Subtle Effects](https://modrinth.com/project/4q8UOK1d)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Subtle Effects](https://modrinth.com/project/subtle-effects)|✅|❌|✅|✅|✅|✅|✅|✅|❌|
|[Super Duper Vanilla](https://modrinth.com/project/LMIZZNxZ)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Super Duper Vanilla](https://modrinth.com/project/super-duper-vanilla)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Symbol Chat](https://modrinth.com/project/NKvLVQMc)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Symbol Chat](https://modrinth.com/project/symbol-chat)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[TCDCommons API](https://modrinth.com/project/Eldc1g37)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Text Placeholder API](https://modrinth.com/project/eXts2L7r)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Theone's Eating Animation Pack](https://modrinth.com/project/OhzX8kDf)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Theone's Eating Animation Pack](https://modrinth.com/project/theones-eating-animation-pack)|❌|❌|❌|❌|✅|✅|✅|✅|❌|
|[Torturable Healthbars](https://modrinth.com/project/torturable-healthbars)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Torturable Healthbars, but with FA](https://modrinth.com/project/thfa)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[Translations for Sodium](https://modrinth.com/project/translations-for-sodium)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Translations for Sodium](https://modrinth.com/project/yfDziwn1)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[UniLib](https://modrinth.com/project/nT86WUER)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[VTDownloader](https://modrinth.com/project/1E2sq1cp)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[VTDownloader](https://modrinth.com/project/vtdownloader)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Vervada's enhanced plants](https://modrinth.com/project/3d-plants)|❌|❌|✅|✅|✅|✅|❌|❌|❌|
|[View Bobbing Options](https://modrinth.com/project/Yr9J16k6)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[View Bobbing Options](https://modrinth.com/project/viewboboptions)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Visuality](https://modrinth.com/project/visuality)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Visuality](https://modrinth.com/project/rI0hvYcd)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Visuals](https://modrinth.com/project/visuals)|❌|❌|❌|❌|❌|✅|✅|✅|❌|
|[Visuals](https://modrinth.com/project/pWBAsHgt)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Voice Chat Soundboard](https://modrinth.com/project/voicechat-soundboard)|✅|✅|✅|❌|✅|❌|❌|❌|❌|
|[WaxedIcons](https://modrinth.com/project/pC9ELBuh)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[WaxedIcons](https://modrinth.com/project/waxedicons)|❌|❌|✅|✅|✅|✅|✅|✅|❌|
|[Wider Tab](https://modrinth.com/project/IA3kkkhV)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Wider Tab](https://modrinth.com/project/wider-tab)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Withered Hearts](https://modrinth.com/project/LQI4ZTHY)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Withered Hearts](https://modrinth.com/project/withered-hearts)|❌|❌|✅|✅|✅|✅|✅|✅|❌|
|[World Play Time](https://modrinth.com/project/world-play-time)|✅|✅|✅|✅|✅|✅|❌|❌|❌|
|[WorldEdit](https://modrinth.com/project/worldedit)|❌|✅|✅|✅|✅|✅|✅|✅|❌|
|[WorldEdit](https://modrinth.com/project/1u6JkXh5)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Xaero's Minimap](https://modrinth.com/project/xaeros-minimap)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Xaero's Minimap](https://modrinth.com/project/1bokaNcj)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Xaero's World Map](https://modrinth.com/project/NcUtCpym)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Xaero's World Map](https://modrinth.com/project/xaeros-world-map)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[XaeroPlus](https://modrinth.com/project/xaeroplus)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[XaeroPlus](https://modrinth.com/project/EnPUzSTg)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Xander's Sodium Options](https://modrinth.com/project/xanders-sodium-options)|✅|❌|✅|✅|✅|✅|❌|❌|❌|
|[YetAnotherConfigLib (YACL)](https://modrinth.com/project/1eAoo2KR)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Your Options Shall Be Respected (YOSBR)](https://modrinth.com/project/WwbubTsV)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
|[Your Options Shall Be Respected (YOSBR)](https://modrinth.com/project/yosbr)|✅|✅|✅|✅|✅|✅|✅|✅|❌|
|[Zoomify](https://modrinth.com/project/w7ThoJFB)|❌|❌|❌|❌|❌|❌|❌|❌|✅|
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