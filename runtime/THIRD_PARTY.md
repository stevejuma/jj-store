# Agent and compute runtime notices

The supervised `lapis-acp`, `lapis-agent`, `lapis-agent-tools` and
`lapis-compute` helpers come from
[Lapis AI](https://github.com/lapismd/lapis-ai). The consumer source revision is
recorded in `scripts/lapis-ai-source.json`. Their maintained implementation and
third-party source records live in that package; jj-store no longer carries
duplicate local-agent or compute sources.

Every staged Lapis AI bundle retains its `LICENSE.md`, `LICENSE.agent`,
`NOTICE.acp.md`, `notices/` attribution and `runtime-manifest.toml`.
Distributions must preserve those files, including the full license texts under
`notices/licenses/`. The manifest pins the ACP import, Buzz agent and MeshLLM
source revisions and verifies the helper/native artifact inventory.

The maintained agent derives from [Buzz](https://github.com/block/buzz) revision
`ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be` (Apache-2.0). Compute uses
[MeshLLM](https://github.com/Mesh-LLM/mesh-llm) revision
`9f192c9d821991ba43d79bb15fd2f2f12e597571` (0.76.0-rc9). Native inference
archives retain their manifests, ABI versions and licenses, including the
llama.cpp/ggml MIT notice. Models are downloaded or imported separately and
retain their respective licenses.

Iroh supplies encrypted, identity-authenticated transport for explicitly paired
machines. The application owns admission, device configuration, credential
custody, durable receipts and repository publication authority.
