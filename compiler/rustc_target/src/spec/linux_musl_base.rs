use crate::spec::{LinkerFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    let mut base = super::linux_base::opts();

    // libssp_nonshared.a is needed for __stack_chk_fail_local when using libc.so
    base.post_link_args.insert(LinkerFlavor::Gcc, vec!["-lssp_nonshared".to_string()]);

    // These targets statically link libc by default
    base.crt_static_default = true;

    base
}
