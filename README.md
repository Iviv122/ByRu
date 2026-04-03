# yet another bytepusher in rust

https://github.com/user-attachments/assets/f0c82d03-31e9-4210-880d-114d3bb9f1f6

usage: bury [File.BytePusher]

- currently stable for linux (uknown_gnu_linux)
- windows build was tested via wine, worked fine
- no mac build

input:
- click q in window to exit app

example roms can be found in ./roms or [wiki](https://esolangs.org/wiki/BytePusher)
```
    ./bury Sprites.BytePusher 
```
---


tl;dr 

Bytepusher is perfect vm for you if you was tasked to make simpliest **usable** cpu


If you don't know what BytePusher is i recommend read [wiki](https://esolangs.org/wiki/BytePusher), but generally speaking this is simple vm by itself features:

- CPU with single simple operation and big endian ordering (``` A B C ```, copy from address A to adress B and go to computing line C)
- 16 MiB RAM 
- Web safe display which consists of 256x256 matrix of 216 colors
- Keyboard Input
- Audio Output
- 60 FPS
- 100% sandboxed
- persistent

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
