# DustyLang

Dustylang is a self bootstraped, powerful, widely, safety and high speed program language.

## Modes

You can compile DustyLang into many platform or used as script.

- Compile into C (GNU or others)
- Compile into EVM to support smart contract
- Compile into Wasm
- Compile into LLVM IR (LLVM toolchain)
- Run in interpreter without compile

## bootstrap

### bootstrap 0

Only support DustyLIR: Direct `convert` to backend.

- Yul
- C-lite

Using rust to write this compiler.

### bootstrap 1

Rewrite b0-ir by DustyLIR

