use command_prelude::*;

use cargo::ops;
use cargo::ops::FetchOptions;

pub fn cli() -> App {
    subcommand("fetch")
        .about("Fetch dependencies of a package from the network")
        .arg_manifest_path()
        .arg_target_triple("Fetch dependencies for the target triple")
        .after_help(
            "\
If a lockfile is available, this command will ensure that all of the git
dependencies and/or registries dependencies are downloaded and locally
available. The network is never touched after a `cargo fetch` unless
the lockfile changes.

If the lockfile is not available, then this is the equivalent of
`cargo generate-lockfile`. A lockfile is generated and dependencies are also
all updated.
",
        )
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    let ws = args.workspace(config)?;

    let opts = FetchOptions {
        config,
        target: args.target(),
    };
    ops::fetch(&ws, &opts)?;
    Ok(())
}
