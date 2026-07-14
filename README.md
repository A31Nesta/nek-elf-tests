# NEK ELF Tests

Small tests to compile ELF binaries for the ESP32-S3 (Xtensa) for use during
the development of NEK, the base for a custom OS for embedded systems.

## Disclaimer about compilation

Currently, I'm compiling with a custom entry point `-Wl,-e,main` instead of
the default. Usually, in ELF the entry point is `_start`, however, I'm compiling
C and Rust with this custom entry point to skip setting up the runtime and
system calls when my ELF Loader (Expel) was in early development.

Proper entry point handling for C and Rust is in development.

## About the `asm` directory

This is for some tests to create minimal executables with Assembly. You can
compile the code with the following commands:

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
