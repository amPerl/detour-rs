use std::mem;
use generic_array::{GenericArray, typenum};
use pic::{Thunkable, StaticThunk};

#[repr(packed)]
pub struct CallAbs {
    // call [rip+8]
    opcode0: u8,
    opcode1: u8,
    dummy0: u32,
    // jmp +10
    dummy1: u8,
    dummy2: u8,
    // destination
    address: usize,
}

pub fn call_abs(destination: usize) -> Box<Thunkable> {
    Box::new(StaticThunk::<typenum::U16>::new(move |_| {
        let code = CallAbs {
            opcode0: 0xFF,
            opcode1: 0x15,
            dummy0: 0x000000002,
            dummy1: 0xEB,
            dummy2: 0x08,
            address: destination,
        };

        let slice: [u8; 16] = unsafe { mem::transmute(code) };
        GenericArray::clone_from_slice(&slice)
    }))
}

#[repr(packed)]
pub struct JumpAbs {
    // jmp +6
    opcode0: u8,
    opcode1: u8,
    dummy0: u32,
    // destination
    address: usize,
}

pub fn jmp_abs(destination: usize) -> Box<Thunkable> {
    Box::new(StaticThunk::<typenum::U14>::new(move |_| {
        let code = JumpAbs {
            opcode0: 0xFF,
            opcode1: 0x25,
            dummy0: 0x000000000,
            address: destination,
        };

        let slice: [u8; 14] = unsafe { mem::transmute(code) };
        GenericArray::clone_from_slice(&slice)
    }))
}

#[repr(packed)]
pub struct JccAbs {
    // jxx + 16
    opcode: u8,
    dummy0: u8,
    dummy1: u8,
    dummy2: u8,
    dummy3: u32,
    // destination
    address: usize,
}

pub fn jcc_abs(destination: usize, condition: u8) -> Box<Thunkable> {
    Box::new(StaticThunk::<typenum::U16>::new(move |_| {
        let code = JccAbs {
            // Invert the condition in x64 mode to simplify the conditional jump logic
            opcode: 0x71 ^ condition,
            dummy0: 0x0E,
            dummy1: 0xFF,
            dummy2: 0x25,
            dummy3: 0x00000000,
            address: destination,
        };

        let slice: [u8; 16] = unsafe { mem::transmute(code) };
        GenericArray::clone_from_slice(&slice)
    }))
}