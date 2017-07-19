## Real (36)
- {and, or, tmul, tcmp, cmp, shf, add} $dest, $a, $b
- {mul, div} $a, $b
- {andi, ori, tmuli, tcmpi, shfi, addi} $dest, $src, immediate (12)
- lui $dest, immediate (12)
- lsr $dest, $sys
- ssr $sys, $src
- {lt, lh, lw} $dest, $src, offset (12)
- {st, sh, sw} $dest, $src, offset (12)
- {bT, b0, b1, bT0, bT1, b01} $src, index (3), hint (1), offset (12)
- {jmp, call} address (20)
- {jmpr, callr} $src
- syscall
- break

## Pseudo (6)
- sub $dest, $a, $b -> tcmp $b, $zero, $b; add $dest, $a, $b
- mov $dest, $src -> tcmp $dest, $src, $zero
- not $dest, $src -> tcmp $dest, $zero, $src
- li $dest, immediate -> addi $dest, $zero, immediate
- la $dest, address -> lui $dest, address[12:24]; ori $dest, address[0:12]
- nop -> and $zero, $zero, $zero
