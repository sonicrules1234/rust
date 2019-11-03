use crate::spec::{LinkerFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    let mut base = super::linux_base::opts();

    // At least when this was tested, the linker would not add the
    // `GNU_EH_FRAME` program header to executables generated, which is required
    // when unwinding to locate the unwinding information. I'm not sure why this
    // argument is *not* necessary for normal builds, but it can't hurt!
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-Wl,--eh-frame-hdr".to_string());

    // libssp_nonshared.a is needed for __stack_chk_fail_local when using libc.so
    base.post_link_args.insert(LinkerFlavor::Gcc, vec!["-lssp_nonshared".to_string()]);

    // These targets statically link libc by default
    base.crt_static_default = true;
    // These targets allow the user to choose between static and dynamic linking.
    base.crt_static_respected = true;

    base
}
