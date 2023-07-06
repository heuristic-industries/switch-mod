# Switch mod

## What is it?

The "switch mod" retrofits more modern switching functionality to existing pedals that utilize "soft switching" systems based on SPST switches.

Many implementations of these systems lack some features that would be fun to add back in, namely:

### Persistent state between power cycles

One of the only advantages we lose moving away from mechanical switches is the state is no longer independent from the power state.
Many pedals will simply default to being in an "off" state when powered on, which can be problematic for those who have a handful of "always on" effects.
Even worse, some pedals default to being on, forcing users to switch it off at the start of each session.

This mod leverages the microcontroller's onboard EEPROM and will persist the state between settings.

### Hold for momentary operation

A feature becoming more and more popular across many brands, "hold for momentary" allows you to use the pedal as if it had a momentary switch when held down for ~1 second.
It comes in handy when you temporarily want the effect engaged, say, during a solo, without having to hunt for the switch again to turn it off.

## What pedals is it compatible with?

This should work with pedals that use an SPST switch for bypass, and don't already implement features like tap tempo and other hold operations on that switch.
I designed it with Boss dirt pedals in mind, but it can work with many others.

## What does it require?

Installation is simple!

That said, as with any mod there is still a very real risk of ruining a pedal you care about.
I will do my best to explain how to install it, but take no responsibility for anything bad that happens along the way.
You've been warned.

Tools:

- Wire cutters
- Wire strippers
- Soldering iron + solder
- Voltmeter
- A single wire

## Installation

_Pictures coming at some point, sorry._

### 1. Take measurements

Before we start hacking things up, we need to take a few measurements that will impact _how_ we wire the mod up.

First, with the pedal powered on, locate the power jack and determine which leads (or pads, if it's PCB-mounted) are 9V and ground using your voltmeter.
Make a note of these.

Next, locate the pads that the footswitch you want to modify connects to on the board.
Touch the negative lead from your voltmeter to the ground you found on the power jack and the other to each pad on the PCB that the switch connects to.
One will give a positive voltage reading (most likely in the 5 to 9V range) and the other will be ground. Make a note of which is which.

### 2. (maybe) Solder the jumper

If the pedal you're modifying defaults to being "off" when powered on initially, skip this step. If it does power on by default, do the following:

Locate the jumper labeled `J2` on the back side of the mod PCB and place a blob of solder on it, connecting the two pads.

### 3. Sever the switch connections

Grab your wire cutters and cut the leads to the switch at roughly the point where you want the mod PCB to sit inside the enclosure.
Then, strip the ends of the resulting 4 leads. Maybe consider tinning them if they're stranded.

If you're installing this on a more modern pedal that makes use of JST headers, you can skip this and go grab some pre-wired connectors.
So long as the pitch on the connectors is 2.54mm you can solder it onto the mod board and non-destructively connect everything!

### 4. Connect the leads

Start with the leads that go to the switch itself; you can solder those onto the mod board next to the "switch" label.
Polarity doesn't matter, they're both the same as far as we're concerned.

Using the earlier measurements, take the lead connected to the PCB with a positive voltage and hook it into the pad with the `+` sign on the mod board.
The other one will go into the one labeled `-`.

### 5. Connect to power

With the switch spliced into the mod board, we need to power it.
Take some wire and hook it from the 9V terminal on the DC jack to the pad labeled `9V` on the mod board.

### 6. `\m/`

That's it!
Enjoy your freshly-modded pedal.

## Open hardware/software

This repo includes everything you need to build the mod, licensed permissively (see `LICENSE` files in the `firmware` and `pcb` directories).

Hardware hackers can dive right in, but those who don't feel like setting everything up can get pre-built files for PCB fabrication and firmware under [releases](https://github.com/heuristic-industries/switch-mod).

### PCB

The PCB is designed entirely in [KiCad](https://www.kicad.org) and was intended for manufacture through [JLCPCB](https://jlcpcb.com) utilizing their assembly services.
In the release archive you'll find all the files necessary to get the board assembled (with the exception of the `attiny85`, which they don't stock).
You can find all the part numbers in the `LCSC` annotations for each symbol.

If you're making changes and want I highly recommend the wonderful [`kicad-jlcpcb-tools`](https://github.com/Bouni/kicad-jlcpcb-tools) plugin, assuming you want to use JLC for manufacture.

### Firmware

The `README` file in that directory has more information.
Everything is in Rust, because in this house we love modern toolchains and compiler-level safety guarantees.

Boards purchased through me will come pre-flashed, but for those going the DIY route the PCB is designed with a [Tag-Connect header](https://www.tag-connect.com/product/tc2030-idc-nl) to save space (and because it's convenient for me).
It'll work with any AVR programmer; I'm partial to the [SparkFun pocket programmer](https://www.sparkfun.com/products/9825) myself.
