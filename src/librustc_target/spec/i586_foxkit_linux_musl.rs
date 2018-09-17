// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use spec::TargetResult;

pub fn target() -> TargetResult {
    let mut base = super::i586_unknown_linux_musl::target()?;

    base.llvm_target = "i586-foxkit-linux-musl".to_string();
    base.target_vendor = "foxkit".to_string();
    base.options.crt_static_default = false;

    Ok(base)
}
