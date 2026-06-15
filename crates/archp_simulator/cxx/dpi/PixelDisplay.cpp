#include "Vtop__Dpi.h"

#include "PixelDisplay.hpp"

bool PixelDisplay::init(uint32_t w, uint32_t h, uint32_t scale) {
  W = w;
  H = h;
  SCALE = scale;
  fb.resize(W * H, 0x00404040);

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    SDL_Log("SDL_Init failed: %s", SDL_GetError());
    return false;
  }

  if (!SDL_CreateWindowAndRenderer("PixelDisplay", W * SCALE, H * SCALE, 0, &win, &ren)) {
    SDL_Log("SDL_CreateWindowAndRenderer failed: %s", SDL_GetError());
    return false;
  }

  tex = SDL_CreateTexture(ren, SDL_PIXELFORMAT_XRGB8888, SDL_TEXTUREACCESS_STREAMING, W, H);
  if (!tex) {
    SDL_Log("SDL_CreateTexture failed: %s", SDL_GetError());
    return false;
  }

  SDL_SetTextureScaleMode(tex, SDL_SCALEMODE_NEAREST);

  return true;
}

void PixelDisplay::destroy() {
  if (tex) {
    SDL_DestroyTexture(tex);
  }
  if (ren) {
    SDL_DestroyRenderer(ren);
  }
  if (win) {
    SDL_DestroyWindow(win);
  }
  SDL_Quit();
}

void PixelDisplay::reset() {
  std::fill(fb.begin(), fb.end(), 0x00404040);
}

void PixelDisplay::set(uint32_t x, uint32_t y, uint32_t color) {
  if (x >= W || y >= H)
    return;

  fb[y * W + x] = color & 0x00FFFFFF;
}

void PixelDisplay::commit() {
  SDL_UpdateTexture(tex, nullptr, fb.data(), W * sizeof(uint32_t));

  SDL_RenderClear(ren);
  SDL_RenderTexture(ren, tex, nullptr, nullptr);
  SDL_RenderPresent(ren);
}

bool PixelDisplay::exit() {
  SDL_Event e;
  while (SDL_PollEvent(&e)) {
    if (e.type == SDL_EVENT_QUIT || e.type == SDL_EVENT_WINDOW_CLOSE_REQUESTED) {
      return true;
    }
  }
  return false;
}

PixelDisplay pd;

void pixel_display_reset() {
  pd.reset();
}

void pixel_display_set(unsigned int x, unsigned int y, unsigned int color) {
  pd.set(x, y, color);
  pd.commit();
}
