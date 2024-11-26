fn nand(a: u8, b: u8) -> u8 {
    let x: u8 = a & b;
    !x
}

fn nor(a: u8, b: u8) -> u8 {
    let x: u8 = nand(!a, !b);
    !x
}

fn or(a: u8, b: u8) -> u8 {
    let x: u8 = nand(!a, !b);
    x
}

fn and(a: u8, b: u8) -> u8 {
    let x: u8 = nand(a, b);
    !x
}

fn xor(a: u8, b: u8) -> u8 {
    let in1: u8 = and(a, !b);
    let in2: u8 = and(!a, b);
    let out: u8 = or(in1, in2);
    out
}

fn multiplexer(a: u8, b:u8, sel: bool) -> u8{
    let out: u8;
    
    if sel {
        out = b;
    } else {
        out = a;
    }
    
    out
}

fn demultiplexer(input: bool, sel: u8) -> u8 {
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

fn main() {
    let a: u8 = 0b10001001;
    let b: u8 = 0b10101010;
    let sel: bool = true;
    
    let nand_out: u8 = nand(a, b);
    println!("Result of NAND of {:b} and {:b}: {:b}", a, b, nand_out);
    println!("In Decimal: {} NAND {} = {}", a, b, nand_out);
    
    let nor_out: u8 = nor(a, b);
    println!("\nResult of NOR of {:b} and {:b}: {:b}", a, b, nor_out);
    println!("In Decimal: {} NOR {} = {}", a, b, nor_out);
    
    let xor_out: u8 = xor(a, b);
    println!("\nResult of XOR of {:b} and {:b}: {:b}", a, b, xor_out);
    println!("In Decimal: {} XOR {} = {}", a, b, xor_out);
    
    let mut mux_out: u8 = multiplexer(a, b, sel);
    println!("\nWhen mux select is 1, Output is: {:b}", mux_out);
    mux_out = multiplexer(a, b, !sel);
    println!("When mux select is 0, Output is: {:b}\n", mux_out);

    let input: bool = true;
    for demux_sel in 0..8 {
        println!("For Demux select value {}, Demux output is {:b}", demux_sel, demultiplexer(input, demux_sel));
    }
}