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
