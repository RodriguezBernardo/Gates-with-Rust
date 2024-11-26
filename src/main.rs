mod logic; use logic::*;

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