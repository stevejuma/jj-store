# Remote connection ownership

JST-SEC-009/JST-API-011/JST-UI-022 keep gateway networking and agent supervision
in the server, owned authorization interfaces in the service and scoped state
and navigation in the application. Storage, model and WAL remain independent of
UI, gateway networking and controller runtimes. The dependency additions are
server-owned HTTP/WebSocket transports, not authority adapters.

The server list, connection dialog and footer compose maintained owned
presentation and released Design Core. No Community runtime import, npm override
or checkout dependency is introduced. Controller owners are disposed only on
credential loss/replacement, removal and application teardown; ordinary
selection retains them with hidden readers paused.
