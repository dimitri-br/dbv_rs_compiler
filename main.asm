; Fibonacci Sequence with Even Sum and Prime Check

; Initialization
SET Ra, 1        ; Current Fibonacci number
SET Rb, 0        ; Previous Fibonacci number
SET Rc, 10       ; Compute Fibonacci for 10 terms (for this example)
SET Rd, 0x2000   ; Memory storage address for Fibonacci numbers
SET Rf, 0        ; Initialize sum of even Fibonacci numbers

fibonacci_loop:
; Store current Fibonacci term in memory
SD Rd, Ra      ; Store current term in memory
ADD Rd, Rd, 4    ; Move to next memory location

; Compute next Fibonacci number
ADD Re, Ra, Rb   ; Re = Ra + Rb
MOV Rb, Ra       ; Shift Ra value to Rb
MOV Ra, Re       ; Move computed value to Ra

; Check if current Fibonacci number is even and add to sum
AND Re, Ra, 1    ; Last bit will determine odd/even (0 for even)
IFN skip_even_sum
ADD Rf, Rf, Ra   ; Add even Fibonacci number to sum
skip_even_sum:

; Decrement counter and check if we've reached the desired terms
SUB Rc, Rc, 1    ; Decrement counter
SET Re, 0        ; Set comparison value
CMP Rc, Re       ; Check if counter has reached 0
IFN fibonacci_loop ; If not 0, continue the loop

; Check if the sum of even Fibonacci numbers is prime
SET Rg, 2        ; Initialize loop control for prime checking
prime_check_loop:
CMP Rg, Rf       ; Check if loop control has reached the number
IFE end_prime_check ; If equal or greater, number is prime

MOD Rh, Rf, Rg   ; Get remainder of Rf/Rg
SET Re, 0        ; Set comparison value
CMP Rh, Re       ; Check if remainder is 0
IF end_not_prime ; If remainder is 0, number is not prime

ADD Rg, Rg, 1    ; Increment loop control for prime check
JMP prime_check_loop ; Jump back to start of prime check loop

end_not_prime:
; Load address of 0x2100 into Rp
SET Rp, 0x2100
SD Rp, 0   ; Store 0 at address 0x2100 indicating sum is not prime
JMP end_program

end_prime_check:
; Load address of 0x2100 into Rp
SD Rp, 1   ; Store 1 at address 0x2100 indicating sum is prime

end_program:
HLT             ; Halt the program
