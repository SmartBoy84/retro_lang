# Retro-lang
`./retro-lang {program.rasm} -o {output.mem} > program.asm`

Very simplistic toy language for a very simplistic computer architecture (built in [Retro](https://roblab.org/retro/)) (`UWA-ELEC3020-Lab2`).  

3 representations:
- Retro-lang syntax - highest leven
- Assembly
- Raw machine codes  

All 3 are technically independent from each other.   

Have support for sub-routines`
- E.g., will need a sub-routine to negate a number due to not NOT facilities

## Example program - Fibonnaci
```
var a = 1
var b = 1
var c

var n = 13              # count to 13th fibonnaci number (8 bit storage - max)

reg = a ;loop_start     # show current number

# compute next fibonacci number
c = a
a = a + b
b = c

n = n - 1

goto break n             # branch to exit if expression (n) is 0
goto loop_start          # otherwise go back to loop_start

reg = a   ;break         # display final result
goto exit ;exit          # loop forever
```
Things not shown above:
- `noop`
- Expressions can be arbitrarily long
- Not strict about spacing wherever logical to ignore spacing
