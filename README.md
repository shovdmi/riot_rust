# riot_rust

# Compilation
```
$ cargo build --release
$ arm-none-eabi-nm -D target/thumbv7m-none-eabi/release/libpwrm_ffi.a
$ arm-none-eabi-objdump --disassemble  target/thumbv7m-none-eabi/release/libpwrm_ffi.a |less
```
If the last two commands executes with error it is possible that choosed MCU architucture is wrong.
In that case commands down below will executes without any errors.
```
$ arm-none-eabi-nm -D target/thumbv7m-none-eabi/release/libpwrm_ffi.a
$ arm-none-eabi-objdump --disassemble  target/thumbv7m-none-eabi/release/libpwrm_ffi.a |less
```

# Adding to RIOT OS
``` C
extern char print_hello_from_rust(void);

int main {
    printf("%d\n",print_hello_from_rust());
    return 0;
}
```    

# Building RIOT OS

Links:
1) Rust on STM32 (habr). [link](https://habr.com/ru/post/495948/)
2) Exposing a Rust library to C. [link](https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/)
3) https://gitlab.com/etonomy/riot-examples/-/blob/master/bottles/Makefile
