# Stardew Valley Mod Manager

There are already a couple of mod managers for Stardew Valley but they fail in two areas. Cross platform support and modlists. This project aims to solve both of those issues.

## How it works

Currently I'm just working on making a CLI but it matures I hope to build a cross paltform app with the same feature set.

Normally you just add your mod folders to `./stardewvalley/Mods/`. When using the mod manager mods are saved in `./stardewvalley/mod-manager/mods`. When a mod is activated a symbolic link is created in `./stardewvalley/Mods/` that points to the mod. This allows you to only need to store the mod once and keep your settings through enables and disables.

Mod lists are saves in `./stardewvalley/mod-manager/config.json`. Modlists are arrays of mod's `UniqueID`s.
```json
config.json
{
    "active": "multiplayer",
    "lists": {
        "default": [
            "CJBok.CheatsMenu"
        ],
        "multiplayer": [
            "Pathoschild.ContentPatcher",
            "Elle.NewCoopAnimals
        ],
    }
}
```
