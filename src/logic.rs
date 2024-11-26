pub fn nand(a: u8, b: u8) -> u8 {
    let x: u8 = a & b;
    !x
}

pub fn nor(a: u8, b: u8) -> u8 {
    let x: u8 = nand(!a, !b);
    !x
}

pub fn or(a: u8, b: u8) -> u8 {
    let x: u8 = nand(!a, !b);
    x
}

pub fn and(a: u8, b: u8) -> u8 {
    let x: u8 = nand(a, b);
    !x
}

pub fn xor(a: u8, b: u8) -> u8 {
    let in1: u8 = and(a, !b);
    let in2: u8 = and(!a, b);
    let out: u8 = or(in1, in2);
    out
}

pub fn multiplexer(a: u8, b:u8, sel: bool) -> u8{
    let out: u8;
    
    if sel {
        out = b;
    } else {
        out = a;
    }
    
    out
}

pub fn demultiplexer(input: bool, sel: u8) -> u8 {
    let mut out: u8 = 0b00000000;

    if input {
        match sel {
            0 => out = 0b00000001,
            1 => out = 0b00000010,
            2 => out = 0b00000100,
            3 => out = 0b00001000,
            4 => out = 0b00010000,
            5 => out = 0b00100000,
            6 => out = 0b01000000,
            7 => out = 0b10000000,
            _ => out = 0b00000000,
        }
    }

    out
}