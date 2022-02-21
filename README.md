# rpg-bot

play NES rpgs with fceux, lua, and rust

## running server
```sh
nix-shell -p lua51Packages.luasocket lua51Packages.rapidjson
fceux --loadlua final.lua final-fantasy.zip
```

## connecting client
```sh
yes | nc localhost 8080 -v
```

## open questions

* `nix develop` doesn't work for lua dependencies (`lua51Packages.luasocket`), but `nix-shell -p` does; how can we reproduce the effect of `nix-shell -p ...` using flakes?

## references
* https://fceux.com/web/help/LuaScripting.html
* https://github.com/spiiin/fceux_luaserver
