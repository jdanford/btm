## Real

- `{and, or, tmul, tcmp, cmp, shf, add} $dest, $a, $b`
- `{mul, div} $a, $b`
- `{andi, ori, tmuli, tcmpi, shfi, addi} $dest, $src, immediate (12)`
- `lui $dest, immediate (12)`
- `{lt, lh, lw} $dest, $src, offset (12)`
- `{st, sh, sw} $dest, $src, offset (12)`
- `{bT, b0, b1, bT0, bT1, b01} $src, offset (16)`
- `{bal} offset (16)`
- `{j, jal} address (20)`
- `{jr, jalr} $src`
- `syscall`
- `break`

## Pseudo

- `nop` -> `and $zero, $zero, $zero`
- `mov $dest, $src` -> `tcmp $dest, $src, $zero`
- `not $dest, $src` -> `tcmp $dest, $zero, $src`
- `sub $dest, $a, $b` -> `tcmp $dest, $zero, $b; add $dest, $a, $zero`
- `li $dest, immediate` -> `addi $dest, $zero, immediate` or `lui $dest, immediate[12:24]; ori $dest, $zero, immediate[0:12]`
- `la $dest, address` -> `lui $dest, address[12:24]; ori $dest, $zero, address[0:12]`
- `b offset (16)` -> `b0 $zero, offset (16)`
