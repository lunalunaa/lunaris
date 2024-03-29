use core::{marker::PhantomData, ops::Deref};

use aarch64_cpu::asm;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

#[cfg(feature = "lab")]
const MMIO_BASE: usize = 0xFE00_0000;
#[cfg(feature = "default")]
const MMIO_BASE: usize = 0x3F00_0000;

const UART0_BASE: usize = MMIO_BASE + 0x201000;
const UART3_BASE: usize = MMIO_BASE + 0x201600;
const CLK_BASE: usize = MMIO_BASE + 0x3000;

// PL011 UART registers.
//
// Descriptions taken from "PrimeCell UART (PL011) Technical Reference Manual" r1p5.
register_bitfields! {
    u32,

    /// Flag Register.
    FR [
        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, LCR_H.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        /// - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        /// - This bit does not indicate if there is data in the transmit shift register.
        TXFE OFFSET(7) NUMBITS(1) [],

        /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is full.
        /// - If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS(1) [],

        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the receive holding register is empty.
        /// - If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) [],

        /// UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains
        /// set until the complete byte, including all the stop bits, has been sent from the shift
        /// register.
        ///
        /// This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether
        /// the UART is enabled or not.
        BUSY OFFSET(3) NUMBITS(1) []
    ],

    /// Integer Baud Rate Divisor.
    IBRD [
        /// The integer baud rate divisor.
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Divisor.
    FBRD [
        ///  The fractional baud rate divisor.
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register.
    LCR_H [
        /// Word length. These bits indicate the number of data bits transmitted or received in a
        /// frame.
        #[allow(clippy::enum_variant_names)]
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],

        /// Enable FIFOs:
        ///
        /// 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding
        /// registers.
        ///
        /// 1 = Transmit and receive FIFO buffers are enabled (FIFO mode).
        FEN  OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ],

        STP2 OFFSET(3) NUMBITS(1) [
            TwoStopBits = 1
        ],

        EPS OFFSET(2) NUMBITS(1) [
            OddParity = 0,
            EvenParity = 1
        ],

        PEN OFFSET(1) NUMBITS(1) [
            ParityDisable = 0,
            ParityEnable = 1
        ],

        BRK OFFSET(0) NUMBITS(1) [
            Normal = 1,
            Break = 0
        ]
    ],

    /// Control Register.
    CR [
        /// Receive enable. If this bit is set to 1, the receive section of the UART is enabled.
        /// Data reception occurs for either UART signals or SIR signals depending on the setting of
        /// the SIREN bit. When the UART is disabled in the middle of reception, it completes the
        /// current character before stopping.
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled.
        /// Data transmission occurs for either UART signals, or SIR signals depending on the
        /// setting of the SIREN bit. When the UART is disabled in the middle of transmission, it
        /// completes the current character before stopping.
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// UART enable:
        ///
        /// 0 = UART is disabled. If the UART is disabled in the middle of transmission or
        /// reception, it completes the current character before stopping.
        ///
        /// 1 = The UART is enabled. Data transmission and reception occurs for either UART signals
        /// or SIR signals depending on the setting of the SIREN bit
        UARTEN OFFSET(0) NUMBITS(1) [
            /// If the UART is disabled in the middle of transmission or reception, it completes the
            /// current character before stopping.
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt Clear Register.
    ICR [
        /// Meta field for all pending interrupts.
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCR_H: WriteOnly<u32, LCR_H::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

pub struct MMIODeRefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<T>,
}

impl<T> MMIODeRefWrapper<T> {
    pub fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> Deref for MMIODeRefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}

type Registers = MMIODeRefWrapper<RegisterBlock>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum UARTLine {
    Console,
    Marklin,
}

impl UARTLine {
    pub fn addr(&self) -> usize {
        match self {
            UARTLine::Console => UART0_BASE,
            UARTLine::Marklin => UART3_BASE,
        }
    }
}

pub struct UART {
    registers: Registers,
    line: UARTLine,
}

impl UART {
    fn new(line: UARTLine) -> Self {
        Self {
            registers: Registers::new(line.addr()),
            line,
        }
    }

    pub fn console() -> Self {
        let mut uart = Self::new(UARTLine::Console);
        uart.init();
        uart
    }

    pub fn train() -> Self {
        let mut uart = Self::new(UARTLine::Marklin);
        uart.init();
        uart
    }

    fn init(&mut self) {
        let (baud_ival, baud_fval): (u32, u32);

        match self.line {
            UARTLine::Console => {
                baud_ival = 26;
                baud_fval = 2;
            }
            UARTLine::Marklin => {
                baud_ival = 1250;
                baud_fval = 1;
            }
        }

        self.registers.CR.set(0);

        self.registers.ICR.write(ICR::ALL::CLEAR);

        self.registers.IBRD.set(baud_ival);
        self.registers.FBRD.set(baud_fval);
        self.registers.LCR_H.write(
            LCR_H::WLEN::EightBit
                + LCR_H::FEN::FifosEnabled
                + LCR_H::PEN::ParityDisable
                + match self.line {
                    UARTLine::Console => LCR_H::STP2::TwoStopBits,
                    UARTLine::Marklin => LCR_H::STP2::CLEAR,
                },
        );

        self.registers
            .CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }

    pub fn getc_no_wait(&self) -> char {
        self.registers.DR.get() as u8 as char
    }

    pub fn rxwaiting(&self) -> bool {
        self.registers.FR.is_set(FR::RXFE)
    }

    pub fn getc(&self) -> char {
        while self.rxwaiting() {
            asm::nop()
        }
        self.getc_no_wait()
    }

    pub fn txwaiting(&self) -> bool {
        self.registers.FR.is_set(FR::TXFF)
    }

    pub fn putc_nowait(&mut self, ch: u8) {
        self.registers.DR.set(ch as u32)
    }

    pub fn putc(&mut self, ch: u8) {
        while self.txwaiting() {}
        self.putc_nowait(ch)
    }

    pub fn put_slice(&mut self, slice: &[u8]) {
        slice.iter().for_each(|ch| self.putc(*ch));
    }

    pub fn println(&mut self, str: &[u8]) {
        str.iter().for_each(|ch| self.putc(*ch));
        self.putc(b'\n');
    }
}
