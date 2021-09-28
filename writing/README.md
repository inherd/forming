# Writing 

> a document-code sync tools for setup.

## DSL Design

by Lines

```writing
doc-code: file("src/lib.rs").line()[2, 5]
```

by Section

```writing
doc-section: file("src/lib.rs").section("section1")
```

by Function (todo)

```writing
doc-func: file("src/lib.rs").func()["it_works"]
```

License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
