set working-directory := "."
set shell := ["bash", "-uc"]

run *args="":
  @./scripts/run {{args}}

trace *args="":
  @./scripts/trace {{args}}
