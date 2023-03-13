mod tinyvm;

fn main() {
    let mut x: u64 = 4;
    x = x * 6;
    assert_eq!(x, 4 * 6);

    let add_and_print: Vec<u8> = vec![
        5, 2, 9,            // ADD 2 2
        3,                  // PRINT
        99,                 // HALT
    ];

    let infinite_loop: Vec<u8> = vec![
        2, 27,              // PUSH 27
        0,                  // NOP
        0,                  // NOP
        4,                  // INC
        3,                  // PRINT
        2, 1,               // PUSH 1
        4,                  // INC
        3,                  // PRINT
        5, 10, 15,          // ADD 10 15 <---
        3,                  // PRINT        |
        7, 10, 1,           // XOR 10 1     |
        3,                  // PRINT        |
        10, 10,             // JMP 0 --------
        99                  // HALT
    ];

    let mut tiny_vm = tinyvm::vm::VM::new(add_and_print);
    tiny_vm.cpu();
}
