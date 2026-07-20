# ArchP Assembler

## CLI Usage

```
The ArchP Assembler CLI

Usage: archp-as [OPTIONS] <SRC_FILE>

Arguments:
  <SRC_FILE>  File path to the source assembly file

Options:
  -o, --out-file <OUT_FILE>  The output file path [default: a.o]
      --stdout               Output to stdout
      --hex                  Output formatted hex instead of binary machine code
      --disable-macro        Disable the macro-instructions
  -h, --help                 Print help
  -V, --version              Print version
```

## Generate Completion Script

Where `<shell>` can be `bash`, `zsh`, `fish`, `powershell`, or `elvish`.

```bash
ARCHP_AS_COMPLETE=<shell> archp-as
```

## Assembly Syntax

See also [ArchP Assembler](../archp_assembler/README.md).
