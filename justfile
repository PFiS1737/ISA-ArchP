set working-directory := "."
set shell := ["bash", "-uc"]

default:
  @just config
  @echo
  @just build

[arg("mode", pattern="trace|")]
config mode="":
  @veryl build --quiet
  @xmake config --clean --mode={{mode}}

alias f := config

build *args="":
  @veryl build {{args}}
  @xmake build {{args}}

alias b := build

run *args="":
  @./scripts/run {{args}}

trace *args="":
  @./scripts/trace {{args}}

clean:
  @# Xmake
  rm -rf ./.xmake
  rm -rf ./build
  @# Clangd
  rm -rf ./.cache
  @# Veryl
  rm -rf ./.build
  rm -rf ./dependencies
  rm -rf ./sourcemaps
  rm -rf ./waves
  rm bundled.sv || true
  rm *.f || true
