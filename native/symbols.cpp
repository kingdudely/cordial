#include "symbols.h"
#include "elf_imports.h"

#include <dlfcn.h>
#include <cctype>
#include <cstdio>
#include <string>
#include <unordered_map>
#include <vector>

extern "C" {
void cordial_linker_init();
void* cordial_linker_load_library(const char*, const char* const*, void* const*, size_t);
}

namespace cordial {
namespace {
std::string g_error;

extern "C" long long cordial_generic_stub(...) { return 0; }

bool prefix(const std::string& s, const char* p) {
    return s.compare(0, std::char_traits<char>::length(p), p) == 0;
}

const char* android_library(const std::string& s) {
    static const std::pair<const char*, const char*> table[] = {
        {"AMedia","libmediandk.so"},{"AMEDIA","libmediandk.so"},{"AImage","libmediandk.so"},
        {"AndroidBitmap","libjnigraphics.so"},{"__android_log","liblog.so"},
        {"android_set_abort_message","liblog.so"},{"android_get_device_api_level","liblog.so"},
        {"ANative","libandroid.so"},{"AAsset","libandroid.so"},{"AInput","libandroid.so"},
        {"AKey","libandroid.so"},{"AMotion","libandroid.so"},{"ALooper","libandroid.so"},
        {"ASensor","libandroid.so"},{"AChoreographer","libandroid.so"},
        {"AConfiguration","libandroid.so"},{"ATrace","libandroid.so"},
        {"AHardwareBuffer","libandroid.so"},{"ASharedMemory","libandroid.so"},
        {"APerformanceHint","libandroid.so"},{"AObb","libandroid.so"},
        {"AStorageManager","libandroid.so"},{"ASurface","libandroid.so"},
        {"AFont","libandroid.so"},{"ASystemFont","libandroid.so"},
        {"SL_IID_","libOpenSLES.so"},{"slCreateEngine","libOpenSLES.so"}
    };
    for (const auto& e : table) if (prefix(s,e.first)) return e.second;
    return nullptr;
}

const char* library_for(const std::string& s) {
    if (prefix(s,"egl") && s.size()>3 &&
        std::isupper(static_cast<unsigned char>(s[3]))) return "libEGL.so";
    if (prefix(s,"gl") && s.size()>2 &&
        std::isupper(static_cast<unsigned char>(s[2]))) return "libGLESv2.so";
    if (const char* a = android_library(s)) return a;
    return "libc.so";
}

bool linker_symbol(const std::string& s) {
    static const char* names[] = {
        "dlopen","dlsym","dlclose","dlerror","dladdr","dl_iterate_phdr","dlvsym"
    };
    for (const char* n : names) if (s == n) return true;
    return false;
}

void register_group(const std::string& name,
                    const std::vector<std::pair<std::string,void*>>& group) {
    if (group.empty()) return;
    std::vector<const char*> names;
    std::vector<void*> addrs;
    names.reserve(group.size());
    addrs.reserve(group.size());
    for (const auto& e : group) {
        names.push_back(e.first.c_str());
        addrs.push_back(e.second);
    }
    if (!cordial_linker_load_library(name.c_str(), names.data(), addrs.data(), names.size()))
        g_error = "failed to register " + name;
}
}

void register_runtime_symbols(const std::string& path) {
    g_error.clear();
    const auto imports = read_elf_imports(path);

    struct Lib { const char* soname; const char* guest; void* h; };
    Lib candidates[] = {
        {"libm.so.6","libm.so",nullptr},{"libz.so.1","libz.so",nullptr},
        {"libGLESv2.so.2","libGLESv2.so",nullptr},{"libEGL.so.1","libEGL.so",nullptr},
        {"libstdc++.so.6","libc.so",nullptr},{"libgcc_s.so.1","libc.so",nullptr},
        {"libc.so.6","libc.so",nullptr}
    };
    std::vector<Lib> libs;
    for (auto l : candidates) {
        l.h = dlopen(l.soname, RTLD_NOW | RTLD_LOCAL);
        if (l.h) libs.push_back(l);
    }

    std::unordered_map<std::string,std::vector<std::pair<std::string,void*>>> groups;

    for (const auto& imp : imports) {
        if (linker_symbol(imp.name)) continue;

        void* addr = nullptr;
        if (imp.name == "ANativeWindow_fromSurface" ||
            imp.name == "ANativeWindow_acquire" ||
            imp.name == "ANativeWindow_release" ||
            imp.name == "ANativeWindow_getWidth" ||
            imp.name == "ANativeWindow_getHeight" ||
            imp.name == "ANativeWindow_getFormat" ||
            imp.name == "ANativeWindow_setBuffersGeometry" ||
            imp.name == "ANativeWindow_lock" ||
            imp.name == "ANativeWindow_unlockAndPost" ||
            imp.name == "eglCreateWindowSurface") {
            std::string wrapper = "cordial_" + imp.name;
            addr = dlsym(RTLD_DEFAULT, wrapper.c_str());
        }

        if (!addr) {
            for (const auto& l : libs) {
                if ((addr = dlsym(l.h, imp.name.c_str()))) break;
            }
        }

        if (!addr) addr = reinterpret_cast<void*>(cordial_generic_stub);
        groups[library_for(imp.name)].push_back({imp.name,addr});
    }

    groups["libOpenMAXAL.so"];

    if (void* vh = dlopen("libvulkan.so.1", RTLD_NOW | RTLD_LOCAL)) {
        if (void* vk = dlsym(vh,"vkGetInstanceProcAddr")) {
            groups["libvulkan.so"].push_back({"vkGetInstanceProcAddr",vk});
            groups["libvulkan.so.1"].push_back({"vkGetInstanceProcAddr",vk});
        }
        dlclose(vh);
    }

    for (auto& e : groups) register_group(e.first,e.second);
    for (auto& l : libs) dlclose(l.h);
}

const char* last_symbol_error() { return g_error.c_str(); }
}
