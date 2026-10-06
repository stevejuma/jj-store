# Thread presentation template

Forum directory summaries derive their preview from the latest visible human
message. A later rename or lifecycle event advances the activity timestamp
without clearing that message preview. Tombstoned messages contribute neither
preview nor visible message counts.

A persistent Thread is presented as a Community channel. M15.5 MUST use
Community channel navigation and room-selection semantics, the channel header,
conversation timeline, shared composer, message actions, reply affordances,
reactions, history loading and responsive behavior as its template. The locally
owned conversation, composer, message and reaction components are the first
composition owners; inspect the current Community channel application and
contract stories before copying the remaining presentation parts. jj-store owns
the authorized adapter and explicit captured-root refresh behavior. It MUST
preserve the Community visual and interaction template while owning all copied
presentation locally. Community package/checkout dependencies are prohibited by
JST-GOV-008. Native adapters replace Community identity and policy assumptions;
Nostr policy never becomes repository authority. Native Thread
identity/lifecycle and receipt semantics remain the persistence contract;
matching channel presentation does not imply running an agent or importing Nostr
membership.
