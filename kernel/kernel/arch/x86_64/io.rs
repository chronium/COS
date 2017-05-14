pub use volatile::Volatile;
pub use core::ptr::Unique;
pub use spin::Mutex;
pub use core::fmt;

use self::terminal::ColorCode;
use self::terminal::Buffer;
use self::terminal::Color;

pub struct Writer {
   column: usize,
   row: usize,
   color: ColorCode,
   buffer: Unique<Buffer>
}

pub static WRITER: Mutex<Writer> = Mutex::new (Writer {
   column: 0,
   row: 0,
   color: ColorCode::new (Color::LightGray, Color::Black),
   buffer: unsafe { Unique::new (0xB8000 as *mut _) },
});

pub mod terminal {
   pub use volatile::Volatile;
   pub use core::ptr::Unique;
   use cpuio;

   use super::Writer;

   #[allow(dead_code)]
   #[derive(Debug, Clone, Copy)]
   #[repr(u8)]
   pub enum Color {
      Black      = 0,
      Blue       = 1,
      Green      = 2,
      Cyan       = 3,
      Red        = 4,
      Magenta    = 5,
      Brown      = 6,
      LightGray  = 7,
      DarkGray   = 8,
      LightBlue  = 9,
      LightGreen = 10,
      LightCyan  = 11,
      LightRed   = 12,
      Pink       = 13,
      Yellow     = 14,
      White      = 15,
   }

   #[derive(Debug, Clone, Copy)]
   pub struct ColorCode(u8);

   impl ColorCode {
      pub const fn new(foreground: Color, background: Color) -> ColorCode {
         ColorCode((background as u8) << 4 | (foreground as u8))
      }
   }

   #[derive(Debug, Clone, Copy)]
   #[repr(C)]
   struct TermChar {
      ascii_char: u8,
      color: ColorCode,
   }

   pub const BUFFER_WIDTH: usize = 80;
   pub const BUFFER_HEIGHT: usize = 25;

   pub struct Buffer {
      chars: [[Volatile<TermChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
   }

   impl Writer {
      #[inline(always)]
      pub fn write_byte (&mut self, byte: u8) {
         match byte {
            b'\n' => self.new_line (),
            byte => {
               let row = self.row;
               let col = self.column;

               let color = self.color;

               self.buffer ().chars[row][col].write (TermChar {
                  ascii_char: byte,
                  color: color,
               });

               self.column += 1;

               if self.column >= BUFFER_WIDTH {
                  self.row += 1;
                  self.column = 0;
               }

               if row >= BUFFER_HEIGHT {
                  self.scroll ();
               }

               self.update_cursor ();
            }
         }
      }

      #[inline(always)]
      pub fn update_cursor (&self) {
         let offset: usize = self.row * 80 + self.column;
         let off_low = offset & 0xFF;
         let off_high = (offset >> 8) & 0xFF;
         
         unsafe {
            cpuio::outb (0x0Fu8, 0x3D4);
            cpuio::outb (off_low as u8, 0x3D5);
            cpuio::outb (0x0Eu8, 0x3D4);
            cpuio::outb (off_high as u8, 0x3D5);
         }
      }

      #[inline(always)]
      #[allow(dead_code)]
      pub fn write_str (&mut self, s: &str) {
         for byte in s.bytes () {
            self.write_byte (byte)
         }
      }

      #[inline(always)]
      fn buffer (&mut self) -> &mut Buffer {
         unsafe { self.buffer.as_mut () }
      }

      #[inline(always)]
      fn new_line (&mut self) {
         self.column = 0;
         self.row += 1;

         if self.row >= 25 {
            self.scroll ();
         }

         self.update_cursor ();
      }

      #[inline(always)]
      fn scroll (&mut self) {
         for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
               let buffer = self.buffer();
               let character = buffer.chars[row][col].read();
               buffer.chars[row - 1][col].write(character);
            }
         }

         self.clear_row (BUFFER_HEIGHT - 1);

         self.column = 0;
         self.row -= 1;

         self.update_cursor ();
      }

      #[inline(always)]
      fn clear_row (&mut self, row: usize) {
         let blank = TermChar {
            ascii_char: b' ',
            color: self.color,
         };
         for col in 0..BUFFER_WIDTH {
            self.buffer ().chars[row][col].write (blank);
         }
      }
   }
}

impl fmt::Write for Writer {
   fn write_str (&mut self, s: &str) -> fmt::Result {
      for byte in s.bytes () {
         self.write_byte (byte)
      }

      Ok (())
   }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
       $crate::arch::io::print (format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

use self::terminal::BUFFER_HEIGHT;

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
        WRITER.lock ().row = 0;
        WRITER.lock ().column = 0;
    }
}
