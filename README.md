# riot_rust

# Compilation
```
$ cargo build --release
$ arm-none-eabi-nm -D target/thumbv7m-none-eabi/release/libpwrm_ffi.a
$ arm-none-eabi-objdump --disassemble  target/thumbv7m-none-eabi/release/libpwrm_ffi.a |less
```
If the last two commands executes with error it is possible that choosen MCU architucture is wrong.
In that case commands down below will executes without any errors.
```
$ arm-none-eabi-nm -D target/thumbv7m-none-eabi/release/libpwrm_ffi.a
$ arm-none-eabi-objdump --disassemble  target/thumbv7m-none-eabi/release/libpwrm_ffi.a |less
```

# Adding to RIOT OS project

## 1. drivers/rtest
add to your drivers/

drivers/rtest/libpwrm_ffi.a

drivers/rtest/Makefile


## 2. drivers/rtest/Makefile

``` Makefile
CARGO_LIB = target/libpwrm_ffi.a

$(CARGO_LIB):
	echo `pwd`
	cp libpwrm_ffi.a /mnt/c/Work/pwr_meter_workspace/pwr_meter/apps/pwrmeter/bin/unwd-power-meter/rtest.a

.PHONY: $(CARGO_LIB)

include $(RIOTBASE)/Makefile.base
```

## 3. apps/application-name/Makefile
Add to your apps/application-name/Makefile

``` Makefile
#...
USE_MODULE += rtest

#...
ARCHIVES += $(CARGO_LIB)

include $(RIOTBASE)/Makefile.include
```


## 4. user code

Add to your code:

``` C
extern char print_hello_from_rust(void);

int main {
    printf("%d\n",print_hello_from_rust());
    return 0;
}
```    

# Links:
1) Rust on STM32 (habr). [link](https://habr.com/ru/post/495948/)
2) Exposing a Rust library to C. [link](https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/)
3) https://gitlab.com/etonomy/riot-examples/-/blob/master/bottles/Makefile
