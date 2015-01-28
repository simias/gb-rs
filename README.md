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

Sound is implemented with adaptative resampling to match the sound
card sample rate. Note that it might take a few seconds for the
algorithm to settle on the correct sample rate so you might get a few
dropped sound packets when you start the emulator. Normally once this
training time has elapsed there should be no glitch.

Things that remain to be implemented:
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

The keybindings are hardcoded in `src/ui/sdl2/controller.rs` at the
moment: you'll have to edit the ```update_key``` function if you want
to rebind them.

The defaults are:

| GameBoy button  | Key           |
| --------------- | ------------- |
| A               | Left Alt      |
| B               | Left Control  |
| Start           | Return        |
| Select          | Right Shift   |
| Up              | Up            |
| Down            | Down          |
| Left            | Left          |
| Right           | Right         |

The `Escape` key exits the emulator.

By default the emulator is built with the original Gameboy bootrom
which scrolls the logo down the screen before actually jumping into
the game. By building with the `--features sgb_bootrom` option you can
opt to use the Super Game Boy bootrom instead which is much faster to
boot up.

Game Support
------------

Games that are known to work:

* Tetris
* Bombjack
* DrMario (However the sound is pretty bogus, I need to investigate that)
* Kirby's Dream Land
* The Legend of Zelda - Link's Awakening
* Motocross Maniacs
* Wario Land
* Bubble Ghost
* Castelian
* Earthworm Jim
* Super Mario Land 1, 2 and 3
* Pokemon Red/Blue
* Space Invaders
* Bomberman
* Trip World

Ressources
----------

The Game Boy CPU manual: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf

The Game Boy Programming manual: http://www.romhacking.net/documents/544/

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

* When there are more than 10 sprites on a line only the 10 first in
  OAM order are displayed. The sprite's X coordinates and priority
  don't matter, only the position in OAM.
