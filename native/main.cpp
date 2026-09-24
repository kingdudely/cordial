#include "host_window.h"
#include "symbols.h"

#include <dlfcn.h>

#include <cstdio>
#include <filesystem>
#include <fstream>
#include <iterator>
#include <string>
#include <thread>
#include <cstdlib>
#include <chrono>

extern "C" {
void cordial_linker_init();
void cordial_linker_update_ld_library_path(const char*);
void* cordial_linker_dlopen(const char*,int);
void* cordial_linker_dlsym(void*,const char*);

void* cordial_jni_create_vm();
int cordial_jni_call_onload(void*,char*,size_t);

void cordial_set_bootstrap(void(*)());
long cordial_game_activity_init(void*,const char*,const char*,const char*,char*,size_t);
int cordial_game_activity_start(long,int,int,int,char*,size_t);

void cordial_set_ui_mode_night(int);
void cordial_set_display_size(int,int);
int cordial_set_init_params(void*,const char*,int,int,char*,size_t);
int cordial_asset_manager_init(void*,char*,size_t);

int cordial_init_client_settings(void*,const char*,const char*,const char*,char*,size_t);
int cordial_init_flags(void*,const char*,char*,size_t);
int cordial_post_client_settings_loaded(void*,char*,size_t);
}

namespace {
void* library_handle=nullptr;
std::string flag_names;
std::string files_dir;

void bootstrap() {
    if(!library_handle) return;
    char err[512]{};

    if(void* p=cordial_linker_dlsym(library_handle,
        "Java_com_roblox_engine_jni_NativeGLInterface_nativeInitClientSettings")) {
        cordial_init_client_settings(p,"","",files_dir.c_str(),err,sizeof(err));
        if(*err) std::fprintf(stderr,"[cordial] client settings: %s\n",err);
    }

    if(void* p=cordial_linker_dlsym(library_handle,
        "Java_com_roblox_client_flags_FlagJniInterface_nativeInitializeNativeFlags")) {
        cordial_init_flags(p,flag_names.c_str(),err,sizeof(err));
        if(*err) std::fprintf(stderr,"[cordial] flags: %s\n",err);
    }

    if(void* p=cordial_linker_dlsym(library_handle,
        "Java_com_roblox_engine_jni_NativeGLInterface_nativePostClientSettingsLoadedInitialization3")) {
        cordial_post_client_settings_loaded(p,err,sizeof(err));
        if(*err) std::fprintf(stderr,"[cordial] settings post: %s\n",err);
    }
}

void usage() {
    std::puts("Usage: roblox [--libroblox PATH] [--assets PATH] [--width N] [--height N]");
}
}

int main(int argc,char** argv) {
    std::string lib_path="./libroblox.so";
    std::string assets="./assets";
    int width=1280,height=720;

    for(int i=1;i<argc;i++) {
        const std::string arg=argv[i];
        if(arg=="--libroblox" && i+1<argc) lib_path=argv[++i];
        else if(arg=="--assets" && i+1<argc) assets=argv[++i];
        else if(arg=="--width" && i+1<argc) width=std::atoi(argv[++i]);
        else if(arg=="--height" && i+1<argc) height=std::atoi(argv[++i]);
        else if(arg=="--help" || arg=="-h") { usage(); return 0; }
        else { std::fprintf(stderr,"unknown argument: %s\n",arg.c_str()); usage(); return 2; }
    }

    lib_path=std::filesystem::absolute(lib_path).string();
    assets=std::filesystem::absolute(assets).string();
    const std::string lib_dir=std::filesystem::path(lib_path).parent_path().string();
    files_dir=lib_dir+"/appData/files";
    std::filesystem::create_directories(files_dir);
    std::filesystem::create_directories(lib_dir+"/appData/cache");
    std::filesystem::create_directories(lib_dir+"/appData/external");

    std::ifstream flags("native/flag-names.txt");
    if(flags) flag_names.assign(std::istreambuf_iterator<char>(flags),std::istreambuf_iterator<char>());

    std::printf("Cordial C++ runtime\n  libroblox: %s\n  assets: %s\n",
                lib_path.c_str(),assets.c_str());

    cordial_linker_init();
    cordial_linker_update_ld_library_path(lib_dir.c_str());
    cordial::register_runtime_symbols(lib_path);
    if(const char* e=cordial::last_symbol_error(); e && *e) return 1;

    library_handle=cordial_linker_dlopen(lib_path.c_str(),RTLD_NOW);
    if(!library_handle) {
        std::fprintf(stderr,"[cordial] failed to load %s\n",lib_path.c_str());
        return 1;
    }

    if(!cordial_jni_create_vm()) {
        std::fprintf(stderr,"[cordial] failed to create JavaVM\n");
        return 1;
    }

    if(void* onload=cordial_linker_dlsym(library_handle,"JNI_OnLoad")) {
        char err[1024]{};
        const int rc=cordial_jni_call_onload(onload,err,sizeof(err));
        if(rc<0) {
            std::fprintf(stderr,"[cordial] JNI_OnLoad failed: %s\n",*err?err:"unknown");
            return 1;
        }
        std::printf("  JNI_OnLoad -> 0x%x\n",rc);
    }

    cordial_set_bootstrap(bootstrap);

    // Android framework initialization that normally happens before the first
    // surface is created.
    if (void* p=cordial_linker_dlsym(
        library_handle,"Java_com_roblox_client_JNIAAssetManagerSetup_initNative")) {
        cordial_asset_manager_init(p,err,sizeof(err));
        if (*err) std::fprintf(stderr,"[cordial] asset manager: %s\\n",err);
    }

    void* init=cordial_linker_dlsym(
        library_handle,"Java_com_google_androidgamesdk_GameActivity_initializeNativeCode");
    if(!init) {
        std::fprintf(stderr,"[cordial] GameActivity.initializeNativeCode is missing\n");
        return 1;
    }

    cordial_set_ui_mode_night(0);

    char err[1024]{};
    const long handle=cordial_game_activity_init(
        init,files_dir.c_str(),files_dir.c_str(),(lib_dir+"/appData/external").c_str(),
        err,sizeof(err));
    if(!handle) {
        std::fprintf(stderr,"[cordial] GameActivity init failed: %s\n",*err?err:"unknown");
        return 1;
    }

    if(!cordial::open_host_window(width,height,"Roblox")) return 1;

    cordial_set_display_size(width,height);
    if (void* p=cordial_linker_dlsym(
        library_handle,"Java_com_roblox_client_startup_MainGameActivity_nativeAppBridgeSetInitParams")) {
        cordial_set_init_params(p,assets.c_str(),width,height,err,sizeof(err));
        if (*err) std::fprintf(stderr,"[cordial] init params: %s\\n",err);
    }
    if(cordial_game_activity_start(handle,width,height,1,err,sizeof(err))<0) {
        std::fprintf(stderr,"[cordial] GameActivity start failed: %s\n",*err?err:"unknown");
        return 1;
    }

    std::puts("  Roblox is running. Close the window to exit.");
    while(!cordial::host_window_closed()) {
        cordial::pump_host_window(handle);
        std::this_thread::sleep_for(std::chrono::milliseconds(4));
    }
    return 0;
}
