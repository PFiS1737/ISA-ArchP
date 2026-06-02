set working-directory := "."
set shell := ["bash", "-uc"]

default:
  @just config debug
  @echo
  @just build

config mode:
  @veryl build --quiet
  @xmake config --clean --mode={{mode}}

[arg("flag", pattern="--verbose|")]
build flag="":
  @veryl build {{flag}}
  @xmake build {{flag}}

alias b := build

run:
  @xmake run --workdir=.

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
  rm *.min.sv || true
  rm *.f || true
