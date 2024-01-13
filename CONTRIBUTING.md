# Contributing to `direct-storage-rs`

## Regenerate `DirectStorage` metadata and bindings

When the `windows` (and `windows-core` and `windows-bindgen`) crates or `DirectStorage` NuGet packages are updated, or when changes are made to the bindings configuration, some steps need to be ran to update Rust code files.  This process is automated as a CI job, but described below after making various changes:

1. Update `windows` dependency versions in [`Cargo.toml`](Cargo.toml) and `Microsoft.Direct3D.DirectStorage` version in [`generate.proj`](.metadata/generate.proj) (if applicable);
2. Make changes to the metadata configuration in the [`.metadata/`](.metadata/) folder (if applicable);
3. (Re)generate [`.winmd`](.windows/winmd/Microsoft.Direct3D.DirectStorage.winmd) metadata by running:
   ```sh
   dotnet build .metadata
   ```
4. Make changes to the Rust bindings generation configuration in [`api_gen/`](api_gen/) and [`bindings.txt`](bindings.txt) (if applicable);
5. (Re)generate Rust code ([`src/Microsoft_Direct3D_DirectStorage.rs`](src/Microsoft_Direct3D_DirectStorage.rs)) by running:
   ```sh
   cargo r -p api_gen
   ```
