pub enum Instruction {
    NOP,                // No operation
    POP,                // Pop from stack and throw away
    PUSH,               // Push to stack
    PRINT,              // Print o
    INC,                // INC
    ADD,
    SUB,
    XOR,
    JMPF,
    HALT,
    PANIC
}