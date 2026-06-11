set working-directory := "."
set shell := ["bash", "-uc"]

default:
  @just config debug
  @echo
  @just build

config mode:
  @veryl build --quiet
  @xmake config --clean --mode={{mode}}

build *args="":
  @veryl build {{args}}
  @xmake build {{args}}

alias b := build

run *args="":
  @xmake run --workdir=. Vtop {{args}}

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
