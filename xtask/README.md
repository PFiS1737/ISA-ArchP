# xtask

```
Usage: xtask <COMMAND>

Commands:
  run    Run a program in the simulator
  trace  Open a waveform file in the trace viewer
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
Run a program in the simulator

Usage: xtask run [OPTIONS] <FILE> [-- <SIM_ARGS>...]

Arguments:
  <FILE>         File path to run
  [SIM_ARGS]...  Arguments to the simulator

Options:
  -s, --asm      <FILE> is an assembly file, assemble it before running
  -t, --trace    Enable tracing and open the trace viewer after running
  -c, --console  Start simulation in a new tty, nessesary if you used the framebuffer
  -h, --help     Print help
```

```
Open a waveform file in the trace viewer

Usage: xtask trace <FILE>

Arguments:
  <FILE>  Waveform file to open in the trace viewer

Options:
  -h, --help  Print help
```
