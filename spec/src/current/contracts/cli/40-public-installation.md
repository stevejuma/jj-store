# Public installation commands

JST-CLI-002/JST-CLI-004 govern distribution of `jj-store` and `jj-store-jj`.
`cargo install jj-store --locked` installs both executables and preserves an
existing upstream `jj`. Command families and native semantics remain unchanged.
Paired executable discovery uses the collision-safe client from the same bundle.
Cargo installations provide binaries; complete downloadable archives
additionally provide UI/Help and the separate embedded RustFS helper.
Installation paths and state paths remain explicit and private. Optional jj-acp
is separately installed.
