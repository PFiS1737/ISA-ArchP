#pragma once

#include <SDL3/SDL.h>
#include <cstdint>
#include <vector>

class PixelDisplay {
public:
  PixelDisplay() = default;
  PixelDisplay(const PixelDisplay &) = delete;
  PixelDisplay &operator=(const PixelDisplay &) = delete;

  bool init(uint32_t w = 128, uint32_t h = 96, uint32_t scale = 6);
  void destroy();

  void reset();
  void set(uint32_t x, uint32_t y, uint32_t color);

  void commit();
  bool exit();

private:
  uint32_t W, H, SCALE;

  SDL_Window *win;
  SDL_Renderer *ren;
  SDL_Texture *tex;

  std::vector<uint32_t> fb;
};
