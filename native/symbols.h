#pragma once

#include <string>

namespace cordial {
void register_runtime_symbols(const std::string& library_path);
const char* last_symbol_error();
}
