# 4. ascii as digram

Date: 2021-05-27

## Status

2021-05-27 proposed

## Context

Asciidoctor： https://asciidoctor.org/docs/asciidoctor-diagram/

[ditaa]
....
+-------------+
| Asciidoctor |-------+
|   diagram   |       |
+-------------+       | PNG out
^                 |
| ditaa in        |
|                 v
+--------+   +--------+----+    /---------------\
|        | --+ Asciidoctor +--> |               |
|  Text  |   +-------------+    |   Beautiful   |
|Document|   |   !magic!   |    |    Output     |
|     {d}|   |             |    |               |
+---+----+   +-------------+    \---------------/
:                                   ^
|          Lots of work             |
+-----------------------------------+
....


## Decision

Decision here...

## Consequences

Consequences here...
