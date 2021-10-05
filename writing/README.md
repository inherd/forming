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

## Documents

[API 库的文档体系支持：主流编程语言的文档设计](https://www.phodal.com/blog/api-ducumentation-design-dsl-base/)

[文档工程体验设计：重塑开发者体验](https://www.phodal.com/blog/documentation-enginnering-experience-design/)

License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
