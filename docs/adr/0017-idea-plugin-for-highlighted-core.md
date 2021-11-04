# 17. idea plugin for highlighted core

Date: 2021-11-01

## Status

2021-11-01 proposed

2021-11-04 accepted

## Context

Highlighted core is a concept in live coding, which will parse concepts and display in idea with realtime.

It could be:

1. set in a panel window with runtime query
2. suggest for function_name or class_name ?
3. show tips for custom naming node ?

User journal

1. create `*.concept` files
    - `core.concept` as highlighted core
    - others as a normal core
2. Mark Directory/Files as Ubiquitous Language
3. parse `*.concept` file as csv to `structs`
4. when user:
    - typing class_name or function_name: concept suggest
    - hover concept comments?
    - showing function_name or class_name ?

Language support: Java, Kotlin, Rust ?

## Decision

Decision here...

## Consequences

Consequences here...
