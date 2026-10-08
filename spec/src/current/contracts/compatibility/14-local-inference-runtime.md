# Local inference runtime compatibility

M25 pins Buzz agent source to ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be and
MeshLLM SDK/host runtime to 9f192c9d821991ba43d79bb15fd2f2f12e597571. The owned
wrapper advertises permission-gated `ask` mode. Trusted stdio MCP coding tools
are configured by the controlling application; warm initialization opens no tool
session. Version/ABI/checksum validation is required for native inference
artifacts. Third-party notices and source pins ship with the helpers.

MacOS ARM64 and Linux CPU acceptance are separate gates. Compilation and fixture
providers do not qualify a model, installed permission exchange or distribution.
Public discovery and automatic provider fallback are disabled. The loopback
OpenAI gateway requires a private process token; paired QUIC admission uses
exact machine identities. A peer receives inference only. Sharing-off/revocation
closes its admitted transport without stopping local inference. Trusted
connection tasks, request sizes, response sizes, duration and concurrency remain
bounded.

Downloaded and imported model paths are recorded in private managed storage.
Start requires an installed path and performs no implicit weight download.
Imports copy explicit GGUF files; removals touch managed weights only. Native
model-file selection is an additive installation capability and fixed
empty-input HTTP route. Hosts without a composed dialog default to false and
retain manual path import. Version-1 optional-path results distinguish selection
from cancellation; selection never changes a model or repository format. The JJ
Agent display name and app artwork preserve the saved `local` runner identity,
private companion executable names, source pins and ABI artifacts.

The gateway preserves the selected logical model identity. Native startup uses
the installed non-symlink path with local-required source policy and disabled
telemetry/blobstore plugins. The initial context limit is 8,192 tokens with two
native slots, independent of a model's advertised maximum context. Managed
local/paired agents default to an 8,192-token context budget and a 2,048-token
response budget. Explicit Advanced environment overrides/unsets retain their
precedence; external provider agents retain provider defaults.

Provider discovery adds an owned helper command with ready/empty/authentication
and unavailable statuses. Legacy `local`, `paired` and compatible `openai`
configuration retains its meaning; first-party OpenAI uses `openai-api`. Public
runner provider metadata and draft preview input are additive. There is no
repository format migration. Native JJ identities and existing compute ABI/pins
are unchanged under JST-COMPAT-001/JST-COMPAT-003/JST-COMPAT-005. Fixture
provider responses do not qualify authenticated cloud endpoints or packaged
platforms.
