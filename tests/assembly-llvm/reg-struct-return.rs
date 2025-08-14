// Test the reg-struct-return ABI.
//@ add-core-stubs
//@ assembly-output: emit-asm
//@ compile-flags: -O --target=i686-unknown-linux-gnu -Crelocation-model=static
//@ revisions: WITH WITHOUT
//@[WITH] compile-flags: -Zreg-struct-return
//@ needs-llvm-components: x86

#![feature(no_core)]
#![no_std]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;
use minicore::*;

struct div_t {
    quot: i32,
    rem: i32,
}

unsafe extern "C" {
    fn div(numerator: i32, denominator: i32) -> div_t;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn direct_construction(numerator: i32, denominator: i32) -> div_t {
    // WITH-LABEL: direct_construction
    // WITH: movl $42, %eax
    // WITH: movl $42, %edx
    // WITH: retl

    // WITHOUT-LABEL: direct_construction
    // WITHOUT: movl 4(%esp), %eax
    // WITHOUT: movl $42, (%eax)
    // WITHOUT: movl $42, 4(%eax)
    // WITHOUT: retl $4
    div_t { quot: 42, rem: 42 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn builtin_call(numerator: i32, denominator: i32) -> div_t {
    // WITH-LABEL: builtin_call
    // WITH: jmp div

    // WITHOUT-LABEL: builtin_call
    // WITHOUT: pushl %esi
    // WITHOUT: subl $8, %esp
    // WITHOUT: movl 16(%esp), %esi
    // WITHOUT: subl $4, %esp
    // WITHOUT: pushl 28(%esp)
    // WITHOUT: pushl 28(%esp)
    // WITHOUT: pushl %esi
    // WITHOUT: calll div
    // WITHOUT: addl $12, %esp
    // WITHOUT: movl %esi, %eax
    // WITHOUT: addl $8, %esp
    // WITHOUT: popl %esi
    // WITHOUT: retl $4
    div(numerator, denominator)
}
