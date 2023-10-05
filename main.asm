; Square and Bitwise Operations

; Initialization
SET Ra, 6        ; Number to be squared
SET Rb, 0        ; Store result of the square operation
SET Rc, 2        ; Constant 2 for squaring
SET Rd, 0x4000   ; Memory storage address for squared number

square_operation:
MUL Rb, Ra, Ra   ; Square the number

; Check if squared number is even or odd
MOD Re, Rb, Rc   ; Modulus operation to check for even/odd
CMP Re, 0        ; Compare remainder with 0
IFE even_part    ; If even, jump to even_part
JMP odd_part     ; If odd, jump to odd_part

even_part:
SL Rb, 1         ; Shift left if even
SD Rd, Rb        ; Store result in memory
JMP end_program  ; Jump to end of program

odd_part:
SR Rb, 1         ; Shift right if odd
XOR Rb, 0xAAAA   ; Bitwise XOR operation
SD Rd, Rb        ; Store result in memory

end_program:
HLT              ; Halt the program
