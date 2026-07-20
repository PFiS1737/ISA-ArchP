#pragma once

#include <SDL3/SDL.h>
#include <cstdint>
#include <vector>

class PixelDisplay {
public:
  PixelDisplay() = default;
  PixelDisplay(const PixelDisplay &) = delete;
  PixelDisplay &operator=(const PixelDisplay &) = delete;

  bool init(uint32_t w = 128, uint32_t h = 96, uint32_t scale = 6) const;
  void destroy() const;

  void reset();
  void set(uint32_t x, uint32_t y, uint32_t color);

  void commit();
  bool handle_event() const;

  mutable SDL_Scancode scancode = SDL_SCANCODE_UNKNOWN;

private:
  mutable uint32_t W = 0, H = 0, SCALE = 1;

  mutable SDL_Window *win = nullptr;
  mutable SDL_Renderer *ren = nullptr;
  mutable SDL_Texture *tex = nullptr;

  mutable std::vector<uint32_t> fb;
};

extern "C" PixelDisplay pd;
