set working-directory := "."
set shell := ["bash", "-uc"]

run *args="":
  @./scripts/run {{args}}
