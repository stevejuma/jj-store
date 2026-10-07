# Agent runtime notices

The bundled local agent includes Buzz agent source from
https://github.com/block/buzz, revision
ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be, under Apache-2.0. Its license and
modification record are retained in
runtime/local-agent/vendor/buzz-agent/LICENSE and
runtime/local-agent/UPSTREAM.md. The distribution must retain this notice and
Apache-2.0 license.

The compute helper uses MeshLLM SDK and host runtime from
https://github.com/Mesh-LLM/mesh-llm, revision
9f192c9d821991ba43d79bb15fd2f2f12e597571 (0.76.0-rc9). The workspace is MIT OR
Apache-2.0; SDK/host runtime packages are Apache-2.0. Native inference archives
retain their upstream manifests, ABI versions and license files. Models are
downloaded or imported separately and retain their respective licenses.

Iroh supplies encrypted, identity-authenticated transport for explicitly paired
machines. The owned admission layer does not enable public discovery or default
relays.

jj-acp is bundled as a separately supervised process, under Apache-2.0. Its
boundary remains protocol, session and process ownership; device configuration,
credential custody, compute and repository authority belong to jj-store.
