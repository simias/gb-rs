[![Build Status](https://travis-ci.org/simias/gb-rs.svg)]
(https://travis-ci.org/simias/gb-rs)

gb-rs
=====

Game Boy emulator written in rust. No unsafe code so far.

It's still quite buggy but it can run a few games well enough to be playable.

Things that remain to be implemented:
* Saving (right now you lose all progress everytime you restart the emulator)
* Sound
* Serial link
* Support for various types of cartridges
* Bugfixes galore...
* Maybe GameBoy Color support?

The emulator is not optimized at all but thanks to the GB's measly
4Mhz system clock it should run at speed on any half-descent desktop
CPU. Don't forget to build with ```cargo --release``` do enable the
optimizations however.

The display and input are handled through SDL2. That code is modular
and abstracted away from the emulator core so it shouldn't be
difficult to add support for alternative backends if need be.

The keybindings are hardcoded in `src/ui/sdl2.rs` at the moment:
you'll have to edit the ```update_key``` function if you want to
rebind them.

The defaults are:

| GameBoy button  | Key           |
| --------------- | ------------- |
| A               | Left Control  |
| B               | Left Alt      |
| Start           | Return        |
| Select          | Backspace     |
| Up              | Up            |
| Down            | Down          |
| Left            | Left          |
| Right           | Right         |

Game Support
------------

Games that are playable:

* Tetris
* Bombjack
* DrMario
* Kirby's Dream Land
* The Legend of Zelda - Link's Awakening
* Motocross Maniacs
* Wario Land
* Bubble Ghost
* Castelian
* Earthworm Jim

Games that are broken somehow:

* Super Mario Land: the game is playable for a while (a few levels)
  but it'll always crash at some point due to a badly handled ROM
  access.
* Pokemon Blue: the world map seems accurate but all the models used
  in battles are completely broken.
* Space Invaders: the menu works but the actual levels are completely
  messed up.
* Super Mario 4: the world map works but the levels are messed up.
* Bomberman: the intro and menu display correctly but the input
  doesn't seem to register.
