; Factorial Computation

; Registers:
; Ra -> holds the current number
; Rb -> running result of the factorial
; Rc -> loop control, counting down to 0
; Rd -> memory storage for factorial result
; Re -> temporary register for operations

SET Ra, 5         ; Compute factorial for 5 (5!)
SET Rb, 1         ; Start result with 1
SET Rd, 0x2000    ; Memory storage address

factorial_loop:
; Check if Ra reached 1
SET Re, 1
CMP Ra, Re    ; Rf = 1 if Ra == 1, else 0
IFN store_result 

; Compute factorial
MUL Rb, Rb, Ra   ; Rb = Rb * Ra

; Decrement Ra for next iteration
SUB Ra, Ra, 1   ; Ra = Ra - 1

JMP factorial_loop  ; Jump back for next iteration

store_result:
SD Rb, Rd       ; Store factorial result in memory

HLT                 ; Halt
