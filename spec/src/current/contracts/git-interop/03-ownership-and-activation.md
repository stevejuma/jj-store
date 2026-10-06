# Ownership and activation

CLI modules own parsed command adaptation, local bindings, private cache and Git
subprocesses. JJ integration owns conversion and native ref semantics. WAL owns
typed immutable records and publication; service/RPC owns authorized requests.
No transport dependency enters model, storage or WAL.

Bridge activation is explicit through native `git remote add`, never repository
opening. A receipt-bearing format-4 transition preserves every existing root,
receipt, origin and native object identity. Existing formats stay readable.
WorkStore homes remain locally supported; their origin ledger stays frozen until
a separately qualified WorkStore reducer exists. Bridge support cannot infer
permission for linked code repositories.
