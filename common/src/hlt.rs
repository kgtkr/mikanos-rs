pub fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}


pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}