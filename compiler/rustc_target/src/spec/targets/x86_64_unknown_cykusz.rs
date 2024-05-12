use crate::spec::{base, LinkerFlavor, StackProbeType, Target, Cc, Lld};

pub fn target() -> Target {
    let mut base = base::cykusz::opts();
    base.cpu = "x86-64".into();
    base.max_atomic_width = Some(64);
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]);
    // don't use probe-stack=inline-asm until rust-lang/rust#83139 is resolved.
    base.stack_probes = StackProbeType::Call;

    Target {
        llvm_target: "x86_64-cykusz".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .into(),
        arch: "x86_64".into(),
        options: base,
    }
}
