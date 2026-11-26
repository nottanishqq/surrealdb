/**
[test]
reason = "EXPIRE ON may appear once on a table definition"
issue = 1317

[test.results]
parsing-error = """
Conflicting clause on this table definition
  --> [21:32]
   |
21 | DEFINE TABLE twice EXPIRE ON a EXPIRE ON b;
   |                                ^^^^^^ A table definition takes each clause once
  --> [21:20]
   |
21 | DEFINE TABLE twice EXPIRE ON a EXPIRE ON b;
   |                    ^^^^^^ First given here
"""

*/

DEFINE TABLE twice EXPIRE ON a EXPIRE ON b;
