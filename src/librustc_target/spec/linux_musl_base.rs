// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use spec::{LinkerFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    let mut base = super::linux_base::opts();

    // At least when this was tested, the linker would not add the
    // `GNU_EH_FRAME` program header to executables generated, which is required
    // when unwinding to locate the unwinding information. I'm not sure why this
    // argument is *not* necessary for normal builds, but it can't hurt!
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-Wl,--eh-frame-hdr".to_string());

    // These targets statically link libc by default
    base.crt_static_default = true;
    // These targets allow the user to choose between static and dynamic linking.
    base.crt_static_respected = true;

    base
}
