use crate::spec::TargetResult;

pub fn target() -> TargetResult {
    let mut base = super::arm_unknown_linux_musleabi::target()?;

    base.llvm_target = "arm-gentoo-linux-musleabi".to_string();
    base.target_vendor = "gentoo".to_string();
    base.options.crt_static_default = false;

    Ok(base)
}
