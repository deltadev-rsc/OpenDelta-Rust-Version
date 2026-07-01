#![no_std]
#![no_main]

mod vga;

#[panic_handler]
fn kpanic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

///---Main-Function---///
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let print = vga::Writer {
        columnPos: 0,
        colorCode: vga::ColorCode::new(vga::Color::Cyan, vga::Color::Black),
        buffer: unsafe {
            &mut *(0xB8000 as *mut vga::Buffer)
        },
    };

    vga::Writer::kprint(/* &mut Writer */, "Welcome to the OpenDelta Rust Ver!\n");
    vga::Writer::kprint(/* &mut Writer */, "UwU\n");
    loop {}
}
