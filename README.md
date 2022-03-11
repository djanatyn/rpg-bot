# rpg-bot

play NES rpgs with fceux, lua, and rust

## running server
```sh
# load dependencies
nix-shell -p lua51Packages.luasocket lua51Packages.rapidjson

# launch fceux through rust
cd sofia
cargo run
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/sofia`
Loading SDL sound with pulseaudio driver...
[
    MemoryAddress {
        value: 0,
        tags: [
            "menu selection",
        ],
    },
    MemoryAddress {
        value: 255,
        tags: [
            "character selection",
        ],
    },
    MemoryAddress {
        value: 0,
        tags: [
            "slot 1 class",
        ],
    },
    MemoryAddress {
        value: 0,
        tags: [
            "slot 2 class",
        ],
    },
    MemoryAddress {
        value: 0,
        tags: [
            "slot 3 class",
        ],
    },
    MemoryAddress {
        value: 0,
        tags: [
            "slot 4 class",
        ],
    },
    MemoryAddress {
        value: 255,
        tags: [
            "menu cursor x",
        ],
    },
    MemoryAddress {
        value: 255,
        tags: [
            "menu cursor y",
        ],
    },
    MemoryAddress {
        value: 255,
        tags: [
            "world map x",
        ],
    },
    MemoryAddress {
        value: 0,
        tags: [
            "world map y",
        ],
    },
    MemoryAddress {
        value: 0,
        tags: [
            "battle cursor x",
        ],
    },
    MemoryAddress {
        value: 0,
        tags: [
            "battle cursor y",
        ],
    },
]
rust: got 12 addresses (frame 0)
...
```

## open questions

* `nix develop` doesn't work for lua dependencies (`lua51Packages.luasocket`), but `nix-shell -p` does; how can we reproduce the effect of `nix-shell -p ...` using flakes?

## references
* https://fceux.com/web/help/LuaScripting.html
* https://github.com/spiiin/fceux_luaserver
