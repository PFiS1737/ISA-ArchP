# ArchP Simulator

## CLI Usage

```
The ArchP Simulator

Usage: archp [OPTIONS] <FILE>

Arguments:
  <FILE>  File path to the binary machine code file to be simulated

Options:
  -T, --trace <TRACE_FILE>           Output file path for the simulation trace, requires 'trace' feature to be enabled [default: dump.fst]
  -F, --hz <HZ>                      Max simulation frequency in Hz, optional unit is supported
      --ram-size <SIZE>              RAM size in byte, optional unit is supported [default: 64M]
  -f, --framebuffer                  Add a XRGB8888 framebuffer to the simulation
      --framebuffer-start <START>    Start address of the framebuffer device [default: 0x80000000]
      --framebuffer-size <SIZE>      Framebuffer size in WIDTHxHEIGHT format, your device must support the specified size [default: 640x480]
      --framebuffer-device <DEVICE>  Specify the framebuffer device path [default: /dev/dri/card1]
  -k, --keyboard                     Add a keyboard device to the simulation
      --keyboard-start <START>       Start address of the keyboard device [default: 0x90000000]
      --keyboard-grab                Whether to grab the keyboard input
  -h, --help                         Print help
  -V, --version                      Print version
```

### Generate Completion Script

Where `<shell>` can be `bash`, `zsh`, `fish`, `powershell`, or `elvish`.

```bash
ARCHP_COMPLETE=<shell> archp
```
