# Installation agent configuration interfaces

Service interfaces expose installation defaults, bounded environment
values/references, write-only OS secret input and managed compute without
importing controller/runtime implementations. Conditional revision writes and
original request identity apply to saved defaults. Repository operations retain
independent access checks; compute admission does not confer repository access.
AgentStore context publication uses existing immutable payload selection and
receipts. The optional native model-file picker is a host-owned service
interface with a fixed GGUF filter. It returns a draft selection without model
import, durable command creation or repository mutation. Existing explicit
compute Import retains its conditional revision and original request identity.
The interface shares the native folder dialog's bounded
single-flight/cancellation owner.

Draft catalog preview is an installation-local read with bounded typed runtime
options and write-only transient secret inputs. Service-owned provider
descriptors contain no credentials or executable selections. Preview does not
create a receipt, mutate settings, store secrets or publish repository state;
repository mutation authority remains unchanged.

The ChatGPT plan profile adds sanitized LocalTrusted account status and explicit
registration/cancel/sign-out commands to the optional execution manager. These
are application operations with device-private receipts, independent of
repository mutations. Credential bundles remain in Keychain and access tokens
cross only the generation-fenced local broker. Account-visible changes notify
the existing agent-settings freshness scope. Conditional saved configuration,
repository authorization, immutable AgentStore publication and receipt semantics
remain unchanged.
