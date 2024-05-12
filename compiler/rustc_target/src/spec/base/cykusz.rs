use crate::spec::{cvs, LinkArgs, LinkerFlavor, RelroLevel, TargetOptions, Cc, Lld};

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();
    args.insert(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        vec![
            // We want to be able to strip as much executable code as possible
            // from the linker command line, and this flag indicates to the
            // linker that it can avoid linking in dynamic libraries that don't
            // actually satisfy any symbols up to that point (as with many other
            // resolutions the linker does). This option only applies to all
            // following libraries so we're sure to pass it as one of the first
            // arguments.
            "-Wl,--as-needed".into(),
            // Always enable NX protection when it is available
            "-Wl,-z,noexecstack".into(),
        ],
    );

    TargetOptions {
        os: "cykusz".into(),
        dynamic_linking: true,
        executables: true,
        families: cvs!["unix"],
        has_rpath: true,
        pre_link_args: args,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        has_thread_local: true,
        crt_static_respected: true,
		features: "-mmx,-sse,+soft-float".into(),
        ..Default::default()
    }
}
