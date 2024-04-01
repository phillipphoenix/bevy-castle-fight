# Faction files

Faction files are files that setup a whole faction. Buildings and units included.
They are using the JSON format, but the extension needs to be `.faction.json`.

The faction folder may only hold faction files and should not include mods.

## Format

The faction files have the following format:

- The root is the faction itself. Give it a unique ID and a name.
- The faction contains an array of buildings:
  - Give each building a unique ID (for instance prepend with the faction ID and then the name of the building).
  - Buildings also have a name.
  - Each building has a sprite, which is the path to sprite. They are given directly to the asset loader, so they are relative to the asset folder.
  - Buildings can also have components. These are formatted in a special way as they are loaded in as enums in Rust.
    - See serde_json for formatting details.
    - See the code for the available components and their data.
    - Typical components are: `Health` and `UnitSpawner`.
- The faction contains an array of units:
  - Give each unit a unique ID (for instance prepend with the faction ID and then the name of the building).
  - Units also have a name.
  - Each unit has a sprite, which is the path to sprite. They are given directly to the asset loader, so they are relative to the asset folder.
  - Units can also have components. These are formatted in a special way as they are loaded in as enums in Rust.
      - See serde_json for formatting details.
      - See the code for the available components and their data.
      - Typical components are: `Health`, `AttackStats`, `MovementSpeed`, `OpponentFollower` and `Visible`.