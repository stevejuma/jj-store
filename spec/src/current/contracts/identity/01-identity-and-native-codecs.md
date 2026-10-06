# Identity and native codecs

The upstream SimpleBackend cannot be adopted wholesale: copies are unsupported
and submodule serialization panics. Complete these variants in the project
codec. Full native values, not generic Serde JSON, determine persistence.
Preserve invisible intermediate commits needed by evolution. The profile is
verified in M0 before user data is stored; new-repository compatibility is not a
promise of Git-backend import compatibility.
