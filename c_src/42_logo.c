#include <42_logo.h>
#include <bridge.h>

pixel_data_42 get_next_pixel_42() {
    unsigned int pixel [3];
    pixel_data_42 ret;

    HEADER_PIXEL(header_data, pixel);
    ret.r = pixel[0];
    ret.g = pixel[1];
    ret.b = pixel[2];
    return ret;
}