use ringbuf::{ring_buffer::RbBase, Rb};

use crate::setup::UART;

const BUFFER_SIZE: usize = 2048;
const BUFFER_FLUSH_SIZE: usize = 256;

struct Cursor {
    pub x: usize,
    pub y: usize,
    pub on: bool,
}

struct Term {
    uart: UART,
    buffer: ringbuf::StaticRb<u8, BUFFER_SIZE>,
    width: usize,
    height: usize,
    cursor: Cursor,
}

fn u_to_str(i: usize, buf: &mut [u8; 10]) -> usize {
    let mut n = i;
    let mut cnt = 0;

    if n == 0 {
        buf[cnt] = b'0';
        cnt += 1;
    } else {
        while n > 0 {
            buf[cnt] = (n % 10) as u8 + b'0';
            cnt += 1;
            n /= 10;
        }
    }
    return cnt;
}

#[inline(always)]
fn to_str(i: i32, buf: &mut [u8; 10]) {
    if i < 0 {
        u_to_str(-i as usize, buf);
    }
}

#[inline(always)]
fn create_slice() -> [u8; 10] {
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

impl Term {
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
        let mut buf = create_slice();
        let len = u_to_str(x, &mut buf);

        buf.iter().take(len).for_each(|u| self.put_ch(*u));
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

    fn put_slice(&mut self, str: &[u8]) {
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
}
