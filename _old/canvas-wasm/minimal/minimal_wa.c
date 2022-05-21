#define HEIGHT 400
#define WIDTH 800
#include <emscripten.h>

int data[WIDTH * HEIGHT];
int red = (255 << 24) | 255;

int* EMSCRIPTEN_KEEPALIVE render() {
   for (int y = 0; y < HEIGHT; y++) {
     int yw = y * WIDTH;
     for (int x = 0; x < WIDTH; x++) {
       data[yw + x] = red;
     }
   }
   return &data[0];
}

