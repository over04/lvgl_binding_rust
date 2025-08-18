#ifndef LVGL_API_H
#define LVGL_API_H

#ifdef __cplusplus
extern "C" {
#endif

#include "lvgl/lvgl.h"
#ifdef INCLUDE_SDL
#if INCLUDE_SDL == 1
#include <SDL2/SDL.h>
#endif
#endif

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /*LVGL_API*/