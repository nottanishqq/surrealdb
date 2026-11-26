/**
[test]
reason = "EXPIRE ON: DEFINE grammar, INFO rendering only when set, ALTER keeping it, and OVERWRITE removing it"
issue = 1317

[[test.results]]
value = "NONE"

[[test.results]]
value = "{ accesses: {  }, analyzers: {  }, apis: {  }, buckets: {  }, compatibility: 4, configs: {  }, extensions: {  }, functions: {  }, models: {  }, params: {  }, sequences: {  }, tables: { session: 'DEFINE TABLE session TYPE ANY SCHEMALESS EXPIRE ON expires_at PERMISSIONS NONE' }, types: {  }, users: {  } }"

[[test.results]]
value = "NONE"

[[test.results]]
value = "{ accesses: {  }, analyzers: {  }, apis: {  }, buckets: {  }, compatibility: 4, configs: {  }, extensions: {  }, functions: {  }, models: {  }, params: {  }, sequences: {  }, tables: { session: 'DEFINE TABLE session TYPE NORMAL SCHEMAFULL EXPIRE ON `valid until` PERMISSIONS NONE' }, types: {  }, users: {  } }"

[[test.results]]
value = "NONE"

[[test.results]]
value = """{ accesses: {  }, analyzers: {  }, apis: {  }, buckets: {  }, compatibility: 4, configs: {  }, extensions: {  }, functions: {  }, models: {  }, params: {  }, sequences: {  }, tables: { session: "DEFINE TABLE session TYPE NORMAL SCHEMAFULL COMMENT 'logins' EXPIRE ON `valid until` PERMISSIONS NONE" }, types: {  }, users: {  } }"""

[[test.results]]
value = "NONE"

[[test.results]]
value = "{ accesses: {  }, analyzers: {  }, apis: {  }, buckets: {  }, compatibility: 4, configs: {  }, extensions: {  }, functions: {  }, models: {  }, params: {  }, sequences: {  }, tables: { session: 'DEFINE TABLE session TYPE ANY SCHEMALESS PERMISSIONS NONE' }, types: {  }, users: {  } }"

*/

DEFINE TABLE session EXPIRE ON expires_at;
INFO FOR DB;

DEFINE TABLE OVERWRITE session SCHEMAFULL EXPIRE ON `valid until`;
INFO FOR DB;

-- An ALTER of another clause keeps the expiry field.
ALTER TABLE session COMMENT 'logins';
INFO FOR DB;

DEFINE TABLE OVERWRITE session;
INFO FOR DB;
