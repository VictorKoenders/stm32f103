use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Pass our linker script to the top crate
    let mut ld = String::new();

    let interrupts = env::var_os("CARGO_FEATURE_INTERRUPTS").is_some();
    let static_ram = env::var_os("CARGO_FEATURE_STATIC_RAM").is_some();

    ld.push_str("
MEMORY
{
  /*
  CCRAM : ORIGIN = 0x60000000, LENGTH = 16K
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 96K
  */
  
  FLASH (rx)      : ORIGIN = 0x08000000, LENGTH = 64K
  RAM (xrw)       : ORIGIN = 0x20000000, LENGTH = 20K
}

ENTRY(main)

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    LONG(0x20005000);
    LONG(main + 1);
    KEEP(*(.rodata._EXCEPTIONS));
    _eexceptions = .;");

    if interrupts {
        ld.push_str("
    KEEP(*(.rodata._INTERRUPTS));
    _einterrupts = .;");
    }

    ld.push_str("
    /* Entry point: reset handler */
    main = .;
    *(.text.main);

    *(.text.*);
    *(.rodata.*);
  } > FLASH");

    if static_ram {
        ld.push_str("
  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);
  _heap_start = .;
  _heap_end = ORIGIN(RAM) + LENGTH(RAM);
");
    }

    ld.push_str("
  /DISCARD/ :
  {
    *(.ARM.exidx.*)
    *(.ARM.extab.*)
    *(.note.gnu.build-id.*)");

    if !static_ram {
        ld.push_str("
    *(.bss.*)
    *(.data.*)");
    }

    ld.push_str("
  }
}

");

    ld.push_str("
ASSERT(_eexceptions - ORIGIN(FLASH) == 0x40, \"exceptions not linked where expected\");");

    if interrupts {
        ld.push_str("
ASSERT(_einterrupts - ORIGIN(FLASH) == 0x130, \"interrupts not linked where expected\");");
    }

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("stm32f103.ld"))
        .unwrap()
        .write_all(ld.as_bytes())
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
}