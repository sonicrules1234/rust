use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::powerpc64_unknown_linux_musl::target();

    base.llvm_target = "powerpc64-foxkit-linux-musl".to_string();
    base.options.vendor = "foxkit".to_string();
    base.options.crt_static_default = false;

    base
}
