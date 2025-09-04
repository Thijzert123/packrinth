---
title: Loaders
layout: default
nav_order: 3
---

# Loaders
{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

In your `branch.json` configuration files, you can set a main mod loader and add other acceptable loaders.
Below is every loader explained.

## Main mod loader
These loaders are possible values for `mod_loader` in `branch.json`.

| Name     | Configuration value |
|----------|---------------------|
| Fabric   | `fabric`            |
| Forge    | `forge`             |
| NeoForge | `neoforge`          |
| Quilt    | `quilt`             |

## Other loaders
Every project on Modrinth has a loader (including shader packs). Some notable loaders are `minecraft` and
`vanilla`. `minecraft` is the loader for resource packs and `vanilla` is used for vanilla shaders. These are both
included in your `branch.json`s `acceptable_loaders` by default.

| Name                 | Configuration value |
|----------------------|---------------------|
| Minecraft            | `minecraft`         |
| Fabric               | `fabric`            |
| Forge                | `forge`             |
| NeoForge             | `neoforge`          |
| Quilt                | `quilt`             |
| Babric               | `babric`            |
| BTA (Babric)         | `bta-babric`        |
| Java Agent           | `java-agent`        |
| Legacy Fabric        | `legacy-fabric`     |
| LiteLoader           | `liteloader`        |
| Risugami's ModLoader | `modloader`         |
| NilLoader            | `nilloader`         |
| Ornithe              | `ornithe`           |
| Rift                 | `rift`              |
| Canvas               | `canvas`            |
| Iris                 | `iris`              |
| OptiFine             | `optifine`          |
| Vanilla Shader       | `vanilla`           |
| Bukkit               | `bukkit`            |
| Folia                | `folia`             |
| Paper                | `paper`             |
| Purpur               | `purpur`            |
| Spigot               | `spigot`            |
| Sponge               | `sponge`            |
| BungeeCord           | `bungeecord`        |
| Velocity             | `velocity`          |
| Waterfall            | `waterfall`         |