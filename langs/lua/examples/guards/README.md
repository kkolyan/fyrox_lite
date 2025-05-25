# About
Rudimental Game made with Fyrox Engine and https://github.com/kkolyan/fyrox_lite.

Lua port of https://github.com/kkolyan/fyrox_guards

Explored Fyrox APIs:
* Node graph.
* RigidBody (3D)
* Ray Casting
* Prefabs & Editor. Everything dynamic in game created in Editor and instantiated via prefabs, except UI, which is coded.

# How to run a game
```sh
git clone --recursive https://github.com/kkolyan/fyrox_lite_lua
chmod +x langs/lua/examples/guards/play.sh
./langs/lua/examples/guards/play.sh
```

# How to play
Use WASD and mouse to shoot enemies and optionally avoid their attacks.

# How to edit scenes
```sh
chmod +x langs/lua/examples/guards/edit.sh
./langs/lua/examples/guards/edit.sh
```

# How to edit scripts
Use any text editor to edit existing files under [scripts](scripts) directory. If you use VSCode, make sure that the [annotations](lua/annotations) are in the scope of the VSCode project, because it will provide some code insight (autocompletion, type checking, Fyrox Lite API reference). Though, it's optional and doesn't impact code execution. Hot reload of scripts is enabled by default in editor. To allow hot-reload in game mode, replace `LuaPlugin::with_hot_reload(false)` with `LuaPlugin::with_hot_reload(true)` in [executor-lua/src/main.rs](../../executor-lua/src/main.rs), though, hot-reload of game code can be tricky and requires a skill for good use.

# Screenshots
![gameplay.png](gameplay.png)