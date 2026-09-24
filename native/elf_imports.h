#pragma once

#include <string>
#include <vector>

namespace cordial {

struct ElfImport {
    std::string name;
    unsigned char binding;
};

std::vector<ElfImport> read_elf_imports(const std::string& path);

} // namespace cordial
