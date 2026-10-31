# Better PVP for Horizon (native mod)

A Horizon native mod (`.hnmod`) with the feature set of Xaero's Better PVP, written from scratch in Rust.
It contains no code or assets from the Java mod (which is All Rights Reserved) and is not a port.

| Feature | How it works here |
|---|---|
| Potion Status | Box listing active effects, level and time left; blinks in the last 10 s |
| Armor Status | Armor, main hand and off hand with icons; durability as percent / number / both; arrow count with a bow |
| Notifications | Low health, hunger, oxygen, and a swelling creeper nearby |
| XP Drops | Floating "+N XP" lines |
| Entity Info | Name, health and armor of the mob under the crosshair |
| Toggle Sneak | Key (default Caps Lock) toggles sneaking |
| Better Sprint | Hold one key (default V) to sprint forward |
| Quick Use / Quick Eat | Keypad 1-9 and B use a hotbar item while the key is held, then your hand returns |

Settings: Options > Mods > Better PVP > Settings. Keys are rebindable there too.
It appears in Horizon Mod Menu (levzzz5154/horizon-mod-menu) as a Native mod, since it is an installed `.hnmod` with id `better_pvp`.

Not implemented: "being shot" notifications, entity model preview, potion icons, the HUD editor and presets,
and the minimap (Horizon has its own).

## Build
`cargo test` runs the logic tests. `python3 package.py` writes `better_pvp-1.0.0.hnmod`.
Install via the launcher's Mods page or `horizon-launcher native-build mixins/better_pvp --trust-native-code`.
Pinned to Horizon commit `ab92024` (it reads private game fields), game 26.3. Compile-checked on that commit.
