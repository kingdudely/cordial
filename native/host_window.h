#pragma once
namespace cordial {
bool open_host_window(int width, int height, const char* title);
void pump_host_window(long long handle);
bool host_window_closed();
int host_window_width();
int host_window_height();
void* x11_native_window();
void* egl_create_window_surface_wrapper();
void* x11_symbol(const char* name);
}
