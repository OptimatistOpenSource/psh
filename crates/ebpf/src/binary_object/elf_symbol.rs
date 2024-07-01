// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use crate::binary_object::Symbol;
use std::fs;
use std::path::Path;

use object::{
    elf::{self, STT_OBJECT},
    read::elf::{FileHeader, Sym},
};

#[allow(dead_code)]
enum SymbolType {
    /// The symbol's value is absolute, and will not be changed by further linking.
    AbsoluteGlobal,
    /// The symbol is in the BSS data section.  This section typically contains
    /// zero-initialized or uninitialized data, although the exact behavior is system dependent.
    BssGlobal,
    BssLocal,
    /// The symbol is common.  Common symbols are uninitialized data.  
    /// When linking, multiple common symbols may appear with the same name. If the symbol is defined anywhere,
    /// the common symbols are treated as undefined references.  The lower case c character
    /// is used when the symbol is in a special section for small commons.
    CommonGlobal,
    CommonLocal,
    /// The symbol is in the initialized data section.
    DataGlobal,
    DataLocal,
    /// The symbol is in an initialized data section for small objects.  Some object file formats
    /// permit more efficient access to small data objects, such as a global int variable as
    /// opposed to a large global array.
    InitializedSmallObjGlobal,
    InitializedSmallObjLocal,
    /// For PE format files this indicates that the symbol is in a section specific to the implementation of DLLs.
    DllSpecificSymbol,
    /// For ELF format files this indicates that the symbol is an indirect function.  
    /// This is a GNU extension to the standard set of ELF symbol types.  It indicates a symbol which if
    /// referenced by a relocation does not evaluate to its address, but instead must be invoked at runtime.
    /// The runtime execution will then return the value to be used in the
    /// relocation.
    ElfIndirectFunction,
    /// The symbol is an indirect reference to another symbol.
    ElfIndirectReference,
    /// The symbol is a debugging symbol.
    DebugSymbol,
    /// The symbol is in a non-data, non-code, non-debug read-only section.
    SymbolInRdonlySection,
    /// The symbol is in a stack unwind section.
    SymbolInStackUnwindSection,
    /// The symbol is in a read only data section.
    SymbolInRdonlyDataSectionGlobal,
    SymbolInRdonlyDataSectionLocal,
    /// The symbol is in an uninitialized or zero-initialized data section for small objects.
    UninitializedOrZeorSmallObjGlobal,
    UninitializedOrZeorSmallObjLocal,

    /// The symbol is in the text (code) section.
    SymbolTextGlobal,
    SymbolTextLocal,

    /// The symbol is undefined.
    Undefined,

    /// The symbol is a unique global symbol.  This is a GNU
    /// extension to the standard set of ELF symbol bindings.
    /// For such a symbol the dynamic linker will make sure that
    /// in the entire process there is just one symbol with this
    /// name and type in use.
    UniqueSymbolGlobalGnu,

    /// The symbol is a weak object.  When a weak defined symbol
    /// is linked with a normal defined symbol, the normal
    /// defined symbol is used with no error.  When a weak
    /// undefined symbol is linked and the symbol is not defined,
    /// the value of the weak symbol becomes zero with no error.
    /// On some systems, uppercase indicates that a default value
    /// has been specified.
    WeakObj,
    WeakObjDefault,

    /// The symbol is a weak symbol that has not been
    /// specifically tagged as a weak object symbol.  When a weak
    /// defined symbol is linked with a normal defined symbol,
    /// the normal defined symbol is used with no error.  When a
    /// weak undefined symbol is linked and the symbol is not
    /// defined, the value of the symbol is determined in a
    /// system-specific manner without error.  On some systems,
    /// uppercase indicates that a default value has been
    /// specified.
    WeakSymbol,
    WeakSymbolDefault,
}

#[allow(dead_code)]
pub fn get_elf_symbols(path: &Path) -> anyhow::Result<Vec<Symbol>> {
    let data = fs::read(path)?;
    let elf = elf::FileHeader64::<object::Endianness>::parse(&*data)?;
    let endian = elf.endian()?;
    let sections = elf.sections(endian, &*data)?;
    let symbols = sections.symbols(endian, &*data, elf::SHT_SYMTAB)?;

    let mut symbol_vec = Vec::<Symbol>::new();

    // FIXME (speed up section look up)
    //let mut last_section_index: Option<u16> = None;
    //let mut last_section: Option<object::elf::SectionHeader64<_>> = None;
    for symbol in symbols.iter() {
        if (symbol.st_type() != elf::STT_NOTYPE
            && symbol.st_type() != elf::STT_FUNC
            && symbol.st_type() != elf::STT_GNU_IFUNC
            && symbol.st_type() != STT_OBJECT)
            || symbol.st_name(endian) == 0
            || symbol.st_size(endian) == 0
            || symbol.st_info() == elf::STT_SECTION
        {
            continue;
        }

        let section_index = symbol.st_shndx(endian);
        let _section = match section_index {
            elf::SHN_UNDEF | elf::SHN_ABS | elf::SHN_COMMON | elf::SHN_XINDEX => continue,
            _ => sections.section(object::SectionIndex(section_index.into()))?,
        };

        // If the attribute bit SHF_ALLOC is not set, the section
        // doesn't occupy memory during process exectuion.
        //
        // I'm not sure if I need to do this, it filters out 99% symbols after
        // uncommenting below if statement.
        //
        // - Chengdong Li
        // if section.sh_type(endian) & elf::SHF_ALLOC == 0 {
        //     continue;
        // }

        let name = String::from_utf8_lossy(symbol.name(endian, symbols.strings())?);
        //let start = section.sh_addr(endian) + symbol.st_value(endian);
        //let start = ((section_index as u64) << 40) | symbol.st_value(endian);
        let start = symbol.st_value(endian);
        let end = start + symbol.st_size(endian);
        symbol_vec.push(Symbol {
            start,
            end,
            symbol: name.to_string(),
        })
    }

    Ok(symbol_vec)
}

#[test]
fn test_get_elf_symbols() {
    use std::path::PathBuf;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../../target/debug/psh");
    match get_elf_symbols(&d) {
        Ok(symbol_vec) => {
            assert_ne!(symbol_vec.len(), 0);
            println!("total number of symbols: {}", symbol_vec.len());
            for symbol in symbol_vec {
                println!("{}", symbol)
            }
        }
        Err(e) => {
            assert!(false, "{}", e.to_string())
        }
    }
}
