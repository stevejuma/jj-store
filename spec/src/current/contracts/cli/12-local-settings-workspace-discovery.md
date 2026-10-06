# Local settings workspace discovery

The foreground CLI server composes an owned device-workspace verifier for
LocalTrusted settings. It uses JJ workspace discovery and the existing native
connection/profile validation without loading repository state, snapshotting
files or reconciling heads. Browser links must target that service's configured
Unix socket and the selected repository. Authenticated serving and unsupported
Git-backed serving do not advertise workspace management. Development tests use
explicit binaries and isolated state; existing installations are not restarted.
