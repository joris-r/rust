// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-win32 Broken because of LLVM bug: http://llvm.org/bugs/show_bug.cgi?id=16249

// compile-flags:-Z extra-debug-info
// debugger:set print pretty off
// debugger:break _zzz
// debugger:run
// debugger:finish
// debugger:print a->boxed
// check:$1 = 1
// debugger:print b->boxed
// check:$2 = {2, 3.5}
// debugger:print c->boxed
// check:$3 = 4
// debugger:print d->boxed
// check:$4 = false

fn main() {
    let a = ~1;
    let b = ~(2, 3.5);
    let c = @4;
    let d = @false;
    _zzz();
}

fn _zzz() {()}