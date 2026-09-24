#include "elf_imports.h"

#include <elf.h>

#include <cstdio>
#include <cstring>
#include <fstream>
#include <stdexcept>
#include <unordered_set>

namespace cordial {

std::vector<ElfImport> read_elf_imports(const std::string& path) {
    std::ifstream file(path, std::ios::binary);
    if (!file) throw std::runtime_error("could not open ELF: " + path);

    Elf64_Ehdr eh{};
    file.read(reinterpret_cast<char*>(&eh), sizeof(eh));
    if (!file || std::memcmp(eh.e_ident, ELFMAG, SELFMAG) != 0)
        throw std::runtime_error("not an ELF file: " + path);
    if (eh.e_ident[EI_CLASS] != ELFCLASS64 || eh.e_ident[EI_DATA] != ELFDATA2LSB)
        throw std::runtime_error("only little-endian ELF64 is supported");

    if (eh.e_shoff == 0 || eh.e_shentsize != sizeof(Elf64_Shdr))
        throw std::runtime_error("ELF has no usable section table");

    file.seekg(static_cast<std::streamoff>(eh.e_shoff));
    std::vector<Elf64_Shdr> sections(eh.e_shnum);
    file.read(reinterpret_cast<char*>(sections.data()),
              static_cast<std::streamsize>(sections.size() * sizeof(Elf64_Shdr)));
    if (!file) throw std::runtime_error("could not read ELF sections");

    int dynsym_index = -1;
    for (size_t i = 0; i < sections.size(); ++i) {
        if (sections[i].sh_type == SHT_DYNSYM) {
            dynsym_index = static_cast<int>(i);
            break;
        }
    }
    if (dynsym_index < 0) return {};

    const Elf64_Shdr& dynsym = sections[dynsym_index];
    if (dynsym.sh_link >= sections.size()) throw std::runtime_error("bad .dynsym link");
    const Elf64_Shdr& strtab = sections[dynsym.sh_link];

    std::string strings(strtab.sh_size, '\0');
    file.seekg(static_cast<std::streamoff>(strtab.sh_offset));
    file.read(strings.data(), static_cast<std::streamsize>(strings.size()));
    if (!file) throw std::runtime_error("could not read .dynstr");

    const size_t count = dynsym.sh_size / sizeof(Elf64_Sym);
    file.seekg(static_cast<std::streamoff>(dynsym.sh_offset));

    std::vector<ElfImport> result;
    std::unordered_set<std::string> seen;
    for (size_t i = 0; i < count; ++i) {
        Elf64_Sym sym{};
        file.read(reinterpret_cast<char*>(&sym), sizeof(sym));
        if (!file) throw std::runtime_error("could not read .dynsym");

        if (sym.st_shndx != SHN_UNDEF || sym.st_name >= strings.size())
            continue;
        const unsigned char binding = ELF64_ST_BIND(sym.st_info);
        if (binding != STB_GLOBAL && binding != STB_WEAK)
            continue;

        const char* name = strings.data() + sym.st_name;
        if (!*name || !seen.insert(name).second)
            continue;

        result.push_back({name, binding});
    }

    return result;
}

} // namespace cordial
