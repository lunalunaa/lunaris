use core::{arch::asm, fmt, panic::PanicInfo};

use super::setup::UART;
use heapless::String;
use numtoa::NumToA;
use once_cell::unsync::Lazy;
use ringbuf::{ring_buffer::RbBase, Rb, StaticRb};

const BUFFER_SIZE: usize = 2048;
const BUFFER_FLUSH_SIZE: usize = 256;
const WINDOW_WIDTH: usize = 140;
const WINDOW_HEIGHT: usize = 90;

struct Cursor {
    pub x: usize,
    pub y: usize,
    pub on: bool,
}

impl Cursor {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, on: false }
    }
}

pub struct Term {
    uart: UART,
    buffer: ringbuf::StaticRb<u8, BUFFER_SIZE>,
    width: usize,
    height: usize,
    cursor: Cursor,
}

impl Term {
    pub fn init() -> Self {
        Self {
            uart: UART::console(),
            buffer: StaticRb::default(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            cursor: Cursor::new(0, 0),
        }
    }

    pub fn flush_all(&mut self) {
        self.buffer.pop_iter().for_each(|c| self.uart.putc(c));
    }

    fn flush(&mut self, len: usize) {
        self.buffer
            .pop_iter()
            .take(len)
            .for_each(|c| self.uart.putc(c));

        // flush as much as possible
        while !self.uart.rxwaiting() {
            self.uart.putc_nowait(self.buffer.pop().unwrap());
        }
    }

    fn move_cursor(&mut self, x: usize, y: usize) {
        self.put_escape();
        let buf: String<10> = (x as u64).try_into().unwrap();
    }

    fn cursor_pos(&mut self) -> Cursor {
        todo!()
    }

    fn draw_at(&mut self, x: usize, y: usize) {
        todo!()
    }

    #[inline(always)]
    fn put_escape(&mut self) {
        self.put_ch(b'\x1b');
    }

    #[inline(always)]
    fn put_unchecked(&mut self, ch: u8) {
        self.buffer.push(ch).unwrap();
    }

    fn put_ch(&mut self, ch: u8) {
        if self.buffer.is_full() {
            self.flush(BUFFER_FLUSH_SIZE);
            self.put_unchecked(ch);
        } else if self.buffer.free_len() == 1 {
            self.put_unchecked(ch);
            self.flush(BUFFER_FLUSH_SIZE);
        } else {
            self.put_unchecked(ch);
        }
    }

    pub fn put_slice(&mut self, str: &[u8]) {
        if self.buffer.free_len() == str.len() {
            self.buffer.push_slice(str);
            self.flush(BUFFER_FLUSH_SIZE);
        } else {
            while self.buffer.free_len() < str.len() {
                self.flush(BUFFER_FLUSH_SIZE);
            }
            self.buffer.push_slice(str)
        }
    }

    pub fn put_u_hex(&mut self, u: usize) {
        let mut buffer = [0u8; 20];

        self.put_slice(u.numtoa(16, &mut buffer));
    }

    pub fn put_u_dec(&mut self, u: usize) {
        let mut buffer = [0u8; 20];

        self.put_slice(u.numtoa(10, &mut buffer));
    }

    pub fn put_int(&mut self, i: i8) {
        let mut buffer = [0u8; 20];

        self.put_slice(i.numtoa(10, &mut buffer));
    }

    pub fn put_u_dec_flush(&mut self, u: usize) {
        self.put_u_dec(u);
        self.flush_all();
    }

    pub fn put_int_flush(&mut self, i: i8) {
        self.put_int(i);
        self.flush_all();
    }

    pub fn put_slice_flush(&mut self, str: &[u8]) {
        self.put_slice(str);
        self.flush_all();
    }
}

pub static mut TERM_GLOBAL: Lazy<Term> = Lazy::new(Term::init);

impl fmt::Write for Term {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { TERM_GLOBAL.put_slice_flush(s.as_bytes()) }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        unsafe {
            use core::fmt::Write;
            let _ = write!($crate::kernel::term::TERM_GLOBAL, $($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        unsafe {
            use core::fmt::Write;
            let _ = writeln!($crate::kernel::term::TERM_GLOBAL, $($arg)*);
        }
    }};
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panicked");
    println!(
        "file = {}, line = {}",
        info.location().unwrap().file(),
        info.location().unwrap().line()
    );

    loop {
        aarch64_cpu::asm::wfe();
    }
}
