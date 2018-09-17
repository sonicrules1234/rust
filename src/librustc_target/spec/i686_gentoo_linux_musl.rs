use crate::spec::{LinkerFlavor, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::i686_unknown_linux_musl::target()?;

    base.llvm_target = "i686-gentoo-linux-musl".to_string();
    base.target_vendor = "gentoo".to_string();
    base.options.crt_static_default = false;
    base.options.post_link_args.insert(LinkerFlavor::Gcc,
        vec!["-Wl,--as-needed".to_string(), "-lssp_nonshared".to_string()]);

    Ok(base)
}
