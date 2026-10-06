# Git interchange identity

The initial bridge profile is `jj-git-sha1-0.45.1-v1`. Git object type and SHA-1
OID describe the exact Git header plus body, separately from the native BLAKE2b
file identity that stores the body. Representation associations retain both IDs
and the profile; native IDs/codecs never change for Git interoperability.
