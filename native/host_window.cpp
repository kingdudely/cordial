#include "host_window.h"

#include <X11/Xlib.h>
#include <X11/keysym.h>

#include <chrono>
#include <cstdio>
#include <ctime>
#include <cstring>

extern "C" {
int cordial_game_activity_key(long,int,int,int,int,int,int,long long,long long,int*,char*,size_t);
int cordial_game_activity_surface_resized(long long,int,int,int,char*,size_t);
int cordial_input_mouse_move(void*,float,float,float,float,char*,size_t);
int cordial_input_mouse_button(void*,float,float,int,int,char*,size_t);
int cordial_input_mouse_wheel(void*,float,float,float,char*,size_t);
}

namespace cordial {
namespace {
Display* display = nullptr;
Window window = 0;
Atom wm_delete = 0;
int width = 0, height = 0, last_x = 0, last_y = 0;
bool closed = false;
void* mouse_move = nullptr;
void* mouse_button = nullptr;
void* mouse_wheel = nullptr;

long long now_ms() {
    timespec ts{};
    clock_gettime(CLOCK_MONOTONIC,&ts);
    return static_cast<long long>(ts.tv_sec)*1000 + ts.tv_nsec/1000000;
}

int keycode(KeySym k) {
    if (k >= XK_a && k <= XK_z) return 29 + int(k-XK_a);
    if (k >= XK_A && k <= XK_Z) return 29 + int(k-XK_A);
    switch (k) {
        case XK_0:return 7; case XK_1:return 8; case XK_2:return 9; case XK_3:return 10;
        case XK_4:return 11; case XK_5:return 12; case XK_6:return 13; case XK_7:return 14;
        case XK_8:return 15; case XK_9:return 16; case XK_Return:return 66; case XK_Escape:return 111;
        case XK_BackSpace:return 67; case XK_Tab:return 61; case XK_space:return 62;
        case XK_Left:return 21; case XK_Right:return 22; case XK_Up:return 19; case XK_Down:return 20;
        case XK_Shift_L:return 59; case XK_Shift_R:return 60;
        case XK_Control_L:return 113; case XK_Control_R:return 114;
        case XK_Alt_L:return 57; case XK_Alt_R:return 58;
        case XK_F1:return 131; case XK_F2:return 132; case XK_F3:return 133; case XK_F4:return 134;
        case XK_F5:return 135; case XK_F6:return 136; case XK_F7:return 137; case XK_F8:return 138;
        case XK_F9:return 139; case XK_F10:return 140; case XK_F11:return 141; case XK_F12:return 142;
        default:return 0;
    }
}

int meta(unsigned state) {
    int v=0;
    if(state&ShiftMask) v|=1;
    if(state&Mod1Mask) v|=2;
    if(state&ControlMask) v|=0x1000;
    return v;
}

} // namespace

bool open_host_window(int w,int h,const char* title) {
    display=XOpenDisplay(nullptr);
    if(!display) {
        std::fprintf(stderr,"[cordial] XOpenDisplay failed\n");
        return false;
    }
    const int screen=DefaultScreen(display);
    window=XCreateSimpleWindow(display,RootWindow(display,screen),0,0,w,h,0,
                               BlackPixel(display,screen),BlackPixel(display,screen));
    if(!window) return false;
    width=w;height=h;
    wm_delete=XInternAtom(display,"WM_DELETE_WINDOW",False);
    XSetWMProtocols(display,window,&wm_delete,1);
    XStoreName(display,window,title?title:"Roblox");
    XSelectInput(display,window,StructureNotifyMask|PointerMotionMask|
                 ButtonPressMask|ButtonReleaseMask|KeyPressMask|KeyReleaseMask);
    XMapWindow(display,window);
    XFlush(display);
    return true;
}

void pump_host_window(long long handle) {
    while(display && XPending(display)) {
        XEvent ev{};
        XNextEvent(display,&ev);
        if(ev.type==ClientMessage && Atom(ev.xclient.data.l[0])==wm_delete) {
            closed=true;
        } else if(ev.type==ConfigureNotify) {
            if(ev.xconfigure.width!=width || ev.xconfigure.height!=height) {
                width=ev.xconfigure.width;
                height=ev.xconfigure.height;
                char err[128]{};
                cordial_game_activity_surface_resized(handle,1,width,height,err,sizeof(err));
            }
        } else if(ev.type==MotionNotify) {
            if(mouse_move) {
                char err[128]{};
                cordial_input_mouse_move(mouse_move,ev.xmotion.x,ev.xmotion.y,
                    ev.xmotion.x-last_x,ev.xmotion.y-last_y,err,sizeof(err));
            }
            last_x=ev.xmotion.x; last_y=ev.xmotion.y;
        } else if(ev.type==ButtonPress || ev.type==ButtonRelease) {
            const bool down=ev.type==ButtonPress;
            char err[128]{};
            if((ev.xbutton.button==4 || ev.xbutton.button==5) && down) {
                if(mouse_wheel) cordial_input_mouse_wheel(mouse_wheel,ev.xbutton.x,ev.xbutton.y,
                    ev.xbutton.button==4?1.0f:-1.0f,err,sizeof(err));
            } else if(mouse_button) {
                cordial_input_mouse_button(mouse_button,ev.xbutton.x,ev.xbutton.y,
                    down?1:0,int(ev.xbutton.button),err,sizeof(err));
            }
        } else if(ev.type==KeyPress || ev.type==KeyRelease) {
            char text[64]{};
            KeySym ks=0;
            XLookupString(&ev.xkey,text,sizeof(text)-1,&ks,nullptr);
            const int android=keycode(ks);
            if(android) {
                int consumed=0;
                char err[128]{};
                const int unicode=(ev.type==KeyPress && text[0])?
                    static_cast<unsigned char>(text[0]):0;
                cordial_game_activity_key(handle,ev.type==KeyPress,android,ev.xkey.keycode,
                    meta(ev.xkey.state),0,unicode,now_ms(),now_ms(),&consumed,err,sizeof(err));
            }
        }
    }
}

bool host_window_closed(){ return closed; }
int host_window_width(){ return width; }
int host_window_height(){ return height; }
void* x11_native_window(){ return reinterpret_cast<void*>(static_cast<uintptr_t>(window)); }

extern "C" void* cordial_eglCreateWindowSurface(void* dpy,void* config,void*,const int* attrs) {
    using Fn=void*(*)(void*,void*,void*,const int*);
    static Fn real_fn=reinterpret_cast<Fn>(dlsym(RTLD_NEXT,"eglCreateWindowSurface"));
    return real_fn ? real_fn(dpy,config,x11_native_window(),attrs) : nullptr;
}

void* egl_create_window_surface_wrapper(){ return reinterpret_cast<void*>(&cordial_eglCreateWindowSurface); }

void* x11_symbol(const char* name) {
    if(std::strcmp(name,"ANativeWindow_fromSurface")==0) return x11_native_window;
    if(std::strcmp(name,"ANativeWindow_acquire")==0) return +[](void*){};
    if(std::strcmp(name,"ANativeWindow_release")==0) return +[](void*){};
    if(std::strcmp(name,"ANativeWindow_getWidth")==0) return +[](void*){return host_window_width();};
    if(std::strcmp(name,"ANativeWindow_getHeight")==0) return +[](void*){return host_window_height();};
    if(std::strcmp(name,"ANativeWindow_getFormat")==0) return +[](void*){return 1;};
    if(std::strcmp(name,"ANativeWindow_setBuffersGeometry")==0)
        return +[](void*,int,int,int){return 0;};
    if(std::strcmp(name,"ANativeWindow_lock")==0)
        return +[](void*,void*,const void*){return -1;};
    if(std::strcmp(name,"ANativeWindow_unlockAndPost")==0)
        return +[](void*){return 0;};
    if(std::strcmp(name,"eglCreateWindowSurface")==0) return egl_create_window_surface_wrapper();
    return nullptr;
}

} // namespace cordial
