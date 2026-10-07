# Installation agent configuration interfaces

Service interfaces expose installation defaults, bounded environment
values/references, write-only OS secret input and managed compute without
importing controller/runtime implementations. Conditional revision writes and
original request identity apply to saved defaults. Repository operations retain
independent access checks; compute admission does not confer repository access.
AgentStore context publication uses existing immutable payload selection and
receipts.
