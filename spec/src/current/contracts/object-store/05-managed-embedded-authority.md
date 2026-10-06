# Managed embedded authority

Format-2 embedded storage owns the pinned RustFS helper as a child process and
passes its private credentials only through that child's environment. Local
initialization creates an explicit private volume, stable installation identity,
restricted credentials and ownership record before publishing the state
directory atomically. The runtime holds the installation lock for its complete
lifetime and selects a concrete literal-loopback RustFS port before launch.

The first owned start may create its configured bucket and immutable sentinel.
It records completion only after reading back the exact sentinel bytes. Every
later start requires the existing volume, bucket and matching sentinel; it must
not turn missing authority into a new empty repository. Storage reaches this
state before native repository recovery or public listener binding. Shutdown
stops admission, drains the listener and adapters, awaits RustFS, then releases
the lock.
