[![Build Status](https://travis-ci.org/simias/gb-rs.svg)]
(https://travis-ci.org/simias/gb-rs)

gb-rs
=====

Game Boy emulator written in rust. No unsafe code so far.

It's still quite buggy but it can run a few games well enough to be
playable. The CPU passes all the instruction tests as well as the
timing tests (see the "Ressources" section below for the links to the
tests).

Saving is implemented, it creates a file with the ".sav" extension in
the same directory as the ROM being executed if it supports saving.

Things that remain to be implemented:
* Sound
* Serial link
* Support for various types of cartridges
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

The `Escape` key exits the emulator.

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
* Super Mario Land

Games that are broken somehow:

* Pokemon Blue: the world map seems accurate but all the models used
  in battles are completely broken.
* Space Invaders: the menu works but the actual levels are completely
  messed up.
* Super Mario 4: the world map works but the levels are messed up.
* Bomberman: the intro and menu display correctly but the input
  doesn't seem to register.

Ressources
----------

The game boy CPU manual: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf

Opcode map: http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html

Infos about many GB quircks: http://www.devrs.com/gb/files/faqs.html

Some extremely useful accuracy tests made by Shay Green:
http://tasvideos.org/EmulatorResources/GBAccuracyTests.html

An introduction to GB emulation in Javascript:
http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU

There are many small errors or discrepancies in the docs listed above,
unfortunately I don't know if/how those docs are maintained so I'm
going to list the errors I've found here for now:

* `RLCA` instruction: various sources disagree on whether the `Z` flag
  should be modified by the operation. After some research I'm pretty
  sure it shouldn't be modified (the Z80 processor for instance does
  not modify `Z`).

* `HALT` instruction: when this instruction is executed while
  interrupts are disabled in the CPU (for instance by a `DI`
  instruction) the CPU will still be awoken by any interrupt enabled
  in the `IE` register, however the interrupt handler will *not* be
  run and the control will be given back to the code that called the
  `HALT`. Be careful that the instruction following an `HALT` is
  glitchy but that's well documented.

* `ADD SP,N` (opcode 0xe8) and `LDHL SP,N` (opcode 0xf8): the values
  of the carry and halfcarry are computed on the low 8bits.

* `RLCA`, `RLA`, `RRCA`, `RRA` instructions: the flags are described
   wrong in the CPU manual: after the execution `C` contains the
   rotated bit while `N`, `H` and `Z` are always 0. The equivalent
   instructions in the extended `CB` opcode space however set `Z` if
   the result is 0 and that's correctly documented.

* Instruction timings: The GameBoy CPU manual gets all the timings of
  conditional branches wrong: it doesn't say that the number of cycles
  taken by the instruction to execute depends on whether or not the
  branch is taken. More generally, both this manual and the opcode map
  linked above have some discrepencies when it comes to instruction
  timings. In the end it's probably safer to use the timings directly
  from the assembly source for the accuracy tests.

* The actual state machine for the configurable GPU LCD interrupt
  seems not well described anywhere. I tried to put a lot of comments
  in my code to describe my approach, however I'm not sure whether
  it's 100% accurate.
