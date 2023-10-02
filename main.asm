; Fibonacci Sequence Program using the provided ISA
; We'll compute Fibonacci numbers and store each number in memory.
; r_a and r_b will store consecutive Fibonacci numbers.
; r_c will be our loop counter.
; r_d will be our memory pointer.

; Initialize registers
SET r_a, 0          ; First Fibonacci number
SET r_b, 1          ; Second Fibonacci number
SET r_c, 10         ; We want to calculate 10 Fibonacci numbers
SET r_d, 0x1000     ; Starting memory address to store the numbers
SEX 0x1, overflow_handler ; Set exception handler for overflow

start_loop:
; Check if we have computed the required number of Fibonacci numbers
SET r_f, 0
EQ r_e, r_c, r_f      ; r_e = 1 if r_c == 0, else 0
JNZ r_e, end_loop   ; If r_e is not zero, jump to end_loop

; Store current Fibonacci number in memory
SDR r_d, r_a         ; Store r_a at address r_d

; Calculate next Fibonacci number
ADD r_f, r_a, r_b   ; r_f = r_a + r_b

; Shift Fibonacci numbers for next iteration
MOV r_b, r_a        ; r_a = r_b
MOV r_f, r_b        ; r_b = r_f

; Increment memory pointer and decrement loop counter
ADDI r_d, r_d, 1    ; r_d = r_d + 1
SUBI r_c, r_c, 1    ; r_c = r_c - 1
JMP start_loop      ; Jump back to start of loop

end_loop:
HLT                 ; End of program

overflow_handler:
; Handle overflow by halting the program with an error code in r_g
SET f_r_f, 0xDEAD
HLT
