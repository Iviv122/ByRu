# yet another bytepusher in rust

usage: bury [File.BytePusher]

example roms can be found in ./roms or wiki
```
    ./bury Sprites.BytePusher 
```

input:
- click q in window to exit app

If you don't know what BytePusher is i recommend read [wiki](https://esolangs.org/wiki/BytePusher), but generally speaking this is simple vm by itself features:

- CPU with single simple operation and big endian ordering (``` A B C ```, copy from address A to adress B and go to computing line C)
- Web safe display which consists of 256x256 matrix of 216 colors
- Keyboard Input
- Audio Output
- 60 FPS

This explicit implementation features:
- Audio (i am not sure if works properly)
- Display (256x256 216 colors)
- Rom loading
- Blazing fast
- Framerate is guaranteed via window.set_target_fps(FRAMERATE)
- Supports .BytePusher file type

dependecies:
- minifb for gui
- rodio for audio
