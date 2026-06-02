//! The build script also sets the linker flags to tell it which link script to use.

use bindgen::callbacks::ItemInfo;
use std::env;
use std::fs::{File, read_to_string, write};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

// const NEWLIB_INC_PATH: &str = "NEWLIB_INC_PATH";
const NEWLIB_INC_PATH: &str = "/usr/lib/arm-none-eabi/include";

const DRIVERLIB_ROOT: &str = "coresdk_cc13xx_cc26xx/source/ti/devices/cc26x0";

const LIB_ROM_ORIGINAL: &str = "rom/driverlib.elf";
const LIB_ROM_FILTERED: &str = "libROM_driverlib_filtered.elf";

const LIB_NOROM_ORIGINAL: &str = "driverlib/bin/gcc/driverlib.lib";
const LIB_NOROM_FINAL: &str = "libdriverlib.a";

const DRIVERLIB_SOURCES: &str = "driverlib";
const DRIVERLIB_INCLUDES: &str = "inc";
const BINDINGS_PATH: &str = "src/driverlib/bindings.rs";

const EXTERN_C_NAME: &str = "extern.c";
const EXTERN_O_NAME: &str = "extern.o";

const ENABLED_ROM_FNS_TXT: &str = "enabled_rom_fns.txt";

#[derive(Debug)]
struct NoromStripper;

// Since we decided to not use -DDOXYGEN, many sombols from
// driverlib have NOROM_ prefix. This callback ensures, that
// the rust binding still links to the symbol with NOROM_ prefix
// but rust function itself does not contain the prefix.
impl bindgen::callbacks::ParseCallbacks for NoromStripper {
    fn item_name(&self, original_item_name: ItemInfo<'_>) -> Option<String> {
        original_item_name.name.strip_prefix("NOROM_").map(|s| s.to_string())
    }
}

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let builder = DriverlibBuilder::new(out);
    builder.build();
}

struct DriverlibBuilder {
    out: PathBuf,
    newlib_inc_path: String,
    driverlib_sources: PathBuf,
    driverlib_includes: PathBuf,
    lib_norom_original_path: PathBuf,
    lib_rom_original_path: PathBuf,
    lib_rom_filtered_path: PathBuf,
    extern_c_path: PathBuf,
    extern_o_path: PathBuf,

    enabled_rom_fns_path: PathBuf,
}

impl DriverlibBuilder {
    fn new(out: PathBuf) -> Self {
        let newlib_inc_path = NEWLIB_INC_PATH.to_string();

        let cc2650_crate_root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let cc2650_crate_driverlib_root = cc2650_crate_root.join("src/driverlib").join(DRIVERLIB_ROOT);
        let cc2650_crate_driverlib_sources = cc2650_crate_driverlib_root.join(DRIVERLIB_SOURCES);
        let cc2650_crate_driverlib_includes = cc2650_crate_driverlib_root.join(DRIVERLIB_INCLUDES);

        let lib_norom_original_path = cc2650_crate_driverlib_root.join(LIB_NOROM_ORIGINAL);
        let lib_rom_original_path = cc2650_crate_driverlib_root.join(LIB_ROM_ORIGINAL);
        let lib_rom_filtered_path = out.join(LIB_ROM_FILTERED);
        let extern_c_path = out.join(EXTERN_C_NAME);
        let extern_o_path = out.join(EXTERN_O_NAME);
        let enabled_rom_fns_path = out.join(ENABLED_ROM_FNS_TXT);

        Self {
            out,
            newlib_inc_path,
            driverlib_sources: cc2650_crate_driverlib_sources,
            driverlib_includes: cc2650_crate_driverlib_includes,
            lib_norom_original_path,
            lib_rom_original_path,
            lib_rom_filtered_path,
            extern_c_path,
            extern_o_path,
            enabled_rom_fns_path,
        }
    }

    fn build(&self) {
        // Create driverlib_full.h, a single entrypoint to all driverlib headers.
        self.generate_driverlib_full_h();

        // Generate bindings from C driverlib to Rust code using bindgen.
        // Create a file containing the FFI code.
        self.generate_bindings();

        // Compile functions that are given in driverlib as `static inline` into another object file
        // to be able to call them.
        self.compile_static_inline_extern_fns();

        // Parse driverlib rom.h to determine which functions are allowed to be called from ROM.
        // The others are stripped from the ROM ELF.
        self.strip_disabled_rom_fns();

        // Remove "NOROM_" prefix from symbols in libdriverlib.a.
        //self.unprefix_norom_symbols();

        // Remove from libdriverlib.a symbols that are to be called from ROM,
        // in order to prevent multiple definitions linking errors.
        self.strip_rom_symbols_from_norom_lib();

        // Combine ROM symbols, outlined `static inline` NOROM functions and NOROM library
        // into one big library.
        self.merge_lib();

        // Instruct cargo to link against libdriverlib.a.
        self.link_driverlib();
    }

    fn generate_driverlib_full_h(&self) {
        let driverlib_full_h_path = self.driverlib_sources.join("driverlib_full.h");
        let mut driverlib_full_h =
            std::fs::File::create(&driverlib_full_h_path).expect("Failed to create driverlib_full.h");

        let mut driverlib_headers = std::fs::read_dir(&self.driverlib_sources)
            .expect("Failed to iterate through driverlib directory")
            .filter_map(|driverlib_file_res| {
                driverlib_file_res
                    .map(|driverlib_file| {
                        let driverlib_file_name = driverlib_file.file_name();
                        // For all *.h files...
                        (driverlib_file_name.as_encoded_bytes().ends_with(b".h")
                            && driverlib_file_name.as_encoded_bytes() != b"driverlib_full.h")
                            .then_some(driverlib_file_name)
                    })
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|err| panic!("Failed to read file in driverlib_directory: {}", err,));

        driverlib_headers.sort_unstable();

        // sw_poly1305-donna-32.h needs to be included before sw_poly1305-donna-32.h,
        // even though the latter comes first in the alphanumeric order.
        let sw_poly1305_donna_32_h_idx = driverlib_headers
            .iter()
            .enumerate()
            .find_map(|(idx, s)| (s.as_encoded_bytes() == b"sw_poly1305-donna-32.h").then_some(idx));
        let sw_poly1305_donna_h_idx = driverlib_headers
            .iter()
            .enumerate()
            .find_map(|(idx, s)| (s.as_encoded_bytes() == b"sw_poly1305-donna.h").then_some(idx));
        if let (Some(sw_poly1305_donna_32_h_idx), Some(sw_poly1305_donna_h_idx)) =
            (sw_poly1305_donna_32_h_idx, sw_poly1305_donna_h_idx)
        {
            driverlib_headers.swap(sw_poly1305_donna_32_h_idx, sw_poly1305_donna_h_idx);
        }

        for driverlib_header in driverlib_headers {
            // Put #include "$file" into driverlib_full.h.
            driverlib_full_h
                .write_all(
                    [b"#include \"", driverlib_header.as_encoded_bytes(), b"\"\n"]
                        .join(&b""[..])
                        .as_slice(),
                )
                .expect("Failed to write into driverlib_full.h");
        }
    }

    fn generate_bindings(&self) {
        println!(
            "cargo:rerun-if-changed={}/driverlib_full.h",
            self.driverlib_sources.display()
        );

        // Create driverlib bindings
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header(format!("{}/driverlib_full.h", self.driverlib_sources.display()))
            // This creates wrapper functions around "static inline" fns to make them available...
            .wrap_static_fns(true)
            // ...and this stores them in the provided path.
            .wrap_static_fns_path(&self.extern_c_path)
            // Instead of ::str::... qualification, use ::core::...
            .use_core()
            // Don't look for standard C types in ::std; instead, use cty crate.
            .ctypes_prefix("cty")
            // Required to get reasonable function signatures in driverlib headers.
            .clang_arg("-D__GNUC__")
            // Required in rust-analyzer to succeed in building.
            .clang_arg("-D__GLIBC_USE(...)")
            // Add driverlib headers. E.g. "inc/hw_types.h" is required.
            .clang_arg(format!("-I{}", self.driverlib_includes.display()))
            // Add newlib headers. E.g. <string.h> is required.
            .clang_arg(format!("-I{}", self.newlib_inc_path))
            // Don't extract doc comments.
            .generate_comments(false)
            // Don't create layout tests - trust bindgen.
            .layout_tests(false)
            // So that bitfields are more convenient to handle.
            .derive_default(true)
            // So that RFC CMDs are not forgot to be actually run.
            .must_use_type(".*rfc_CMD.*")
            // Strip NOROM from the names of the rust bindings.
            .parse_callbacks(Box::new(NoromStripper))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .unwrap_or_else(|err| panic!("Unable to generate bindings: {}", err));

        bindings.write_to_file(BINDINGS_PATH).expect("Couldn't write bindings!");
    }

    fn compile_static_inline_extern_fns(&self) {
        let extern_o_path = cc::Build::new()
            .compiler("clang")
            .file(&self.extern_c_path)
            .warnings(false)
            .define("__GNUC__", None)
            .include(self.newlib_inc_path.as_str())
            .include(&self.driverlib_includes)
            .include(".")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .cargo_metadata(false)
            .compile_intermediates()
            .into_iter()
            .next()
            .unwrap();

        std::fs::copy(&extern_o_path, &self.extern_o_path).expect("Failed to copy extern.o");
    }

    fn merge_lib(&self) {
        // Create empty C file
        let empty_c_path = self.out.join("empty.c");
        {
            File::create(&empty_c_path).unwrap();
            // close file here
        }

        let empty_o_path = self.out.join("empty.o");
        let rom_symbols_o_path = self.out.join("rom_symbols.o");

        // Create empty REL ELF
        // arm-none-eabi-gcc -c empty.c -o empty.o
        let status = Command::new("arm-none-eabi-gcc")
            .arg("-c")
            .arg(&empty_c_path)
            .arg("-o")
            .arg(&empty_o_path)
            .status()
            .unwrap();
        assert!(status.success(), "gcc compiling empty.c failed");

        // Extract ROM symbols to the empty REL ELF
        // arm-none-eabi-ld --relocatable --just-symbols libROM_driverlib_global.elf empty.o -o rom_symbols.o
        let status = Command::new("arm-none-eabi-ld")
            .arg("--relocatable")
            .arg("--just-symbols")
            .arg(&self.lib_rom_filtered_path)
            .arg(&empty_o_path)
            .arg("-o")
            .arg(&rom_symbols_o_path)
            .status()
            .unwrap();
        assert!(status.success(), "ld extracting symbols to rom_symbols.o failed");

        let status = Command::new("ar")
            .arg("rb")
            .arg("adi.o")
            .arg(&self.lib_norom_original_path)
            .arg(&rom_symbols_o_path)
            .arg(&self.extern_o_path)
            .status()
            .unwrap();
        assert!(status.success(), "merge driverlib ar failed");

        // Copy lib to the path expected by the linker.
        std::fs::copy(&self.lib_norom_original_path, self.out.join(LIB_NOROM_FINAL))
            .expect("Falied to copy library to th efinal location.");
    }

    // Strips those functions from ROM symbols ELF, which are disabled in rom.h.
    fn strip_disabled_rom_fns(&self) {
        get_enabled_rom_fns(&self.driverlib_sources, &self.enabled_rom_fns_path);

        let status = Command::new("arm-none-eabi-objcopy")
            .arg(format!(
                "--keep-global-symbols={}",
                self.enabled_rom_fns_path.to_str().unwrap()
            ))
            .arg(&self.lib_rom_original_path) // source file
            .arg(&self.lib_rom_filtered_path) // target file
            .status()
            .unwrap();
        assert!(status.success(), "objcopy strip disabled ROM symbols failed");

        // Writes ROM symbols enabled in rom.h to a file with the given name.
        fn get_enabled_rom_fns(sources: &Path, enabled_rom_fns: &Path) {
            let rom_h_path = sources.join("rom.h");
            let content = read_to_string(&rom_h_path).expect("Failed to read rom.h");

            let parsed_data: String = content
                .lines()
                .filter_map(|line| {
                    line.strip_prefix("#define ROM_")
                        .map(|rest| rest.trim_end_matches([' ', '\\']))
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n";

            write(enabled_rom_fns, parsed_data).expect("Failed to write to output file");
        }
    }

    fn strip_rom_symbols_from_norom_lib(&self) {
        const EXCLUDED: &[&str] = &[
            // Not stripped, because these are used in relocations.
            "FlashProtectionGet",
            "UARTDisable",
            "VIMSModeSet",
        ];

        let symbols = std::fs::read_to_string(&self.enabled_rom_fns_path).unwrap();
        for symbol in symbols
            .split('\n')
            .map(str::trim)
            .filter(|symbol| !EXCLUDED.contains(symbol))
        {
            let status = Command::new("arm-none-eabi-objcopy")
                .arg("--strip-symbol")
                .arg(symbol)
                .arg(&self.lib_norom_original_path)
                .status()
                .unwrap();
            assert_eq!(status.code(), Some(0));
        }
    }

    fn link_driverlib(&self) {
        println!("cargo:rustc-link-lib=static=driverlib");
        println!("cargo:rustc-link-search=native={}", self.out.to_str().unwrap());
    }
}
