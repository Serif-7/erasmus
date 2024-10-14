mod tokenizer;
mod assembler;

fn main() -> std::io::Result<()> {
    // let mut assembler = Assembler::new();
    // assembler.read_input("input.asm")?;
    // assembler.first_pass();
    // assembler.second_pass();
    // assembler.encode();
    // assembler.write_output("output.bin")?;

    let line = "label: mov eax, [ebx + 4*ecx + 0x1234]";
    // let line = "label: mov eax, [ebx + 4*ecx + 5]";
    let tokens = tokenizer::tokenize_line(line);
    println!("{:?}", tokens);
    Ok(())
}
