# NEK ELF Tests

Small tests to compile ELF binaries for the ESP32-S3 (Xtensa) for use during
the development of NEK, the base for my OS for ESP32-based devices.

Since I only run these programs with NEK, I don't know if they work with ELF
loaders other than [Expel](https://git.black.observer/neos/expel), which is
my `no_std` Rust ELF Loader.

## Disclaimer about compilation

I'm compiling with a custom entry point `-Wl,-e,_start`. This `_start`
function is the same as any other normal `_start` function in ELF, however,
specifying the entry point is necessary to make it work with relocatable
(`-Wl,-r`) objects. Since this `_start` function is fully custom, it collides
with whatever implementation GCC wants to include by default, which means
that we also want to compile with `-nostartfiles`.

A custom `crt0.o` could also be used, but the point is that compilation 
always requires a bit of setup on this target.

---

# Legacy stuff - Research

At the time of writing, NEK can load standard C/Rust programs that can
interact with the kernel (host/firmware) with syscalls.

The first ever program I made in this repository was a simple C-like `main`
function that would return an arbitrary number (`67`). The entry point wasn't `_start` since I would just grab the `.text` section (`main`) and
directly copy it into a buffer in IRAM.

In the history of this repository we have some programs that I ran by
`objcopy`-ing `.text`, hacky strangely-compiled ELF files and normal ELF
files.

## About the `asm` directory

This was one of my first tests to create valid LX7 machine code that I could copy to IRAM. You can assemble the ASM code with the following commands:

```bash
xtensa-esp32s3-elf-as asm/return67.S -o asm/return67.o
xtensa-esp32s3-elf-objcopy -O binary asm/return67.o asm/return67.bin
```

You should now have a `.o` file, which is the entire code, and a `.bin` file.  
This `.bin` file is your executable, which you can copy into a raw buffer in
IRAM (like `.rwtext`) in your ESP32 and run it directly.

You can inspect the output file with objdump if you want:

```bash
xtensa-esp32s3-elf-objdump -d asm/return67.o
```

Which should return something like this:

```
asm/return67.o:     file format elf32-xtensa-le


Disassembly of section .text:

00000000 <return67>:
   0:	004136      entry	a1, 32
   3:	324c      	movi.n	a2, 67
   5:	f01d      	retw.n
```

---

Just so you know, though, running `cargo run --release` and extracting its
`.text` section with this command returns the exact same bytes:

```bash
# Remember to change `nek-elf-tests` to the actual path to the binary
xtensa-esp32s3-elf-objcopy -O binary -j .text nek-elf-tests return67.bin
```

So... yeah... it might be better to just write Rust lmao.
