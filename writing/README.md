# Writing 

> a document-code sync tools for setup.

## process

1. read markdown file
2. parse markdown code
3. insert new code
4. generate new code

## DSL Design

by Lines

```writing
// doc-code: file("src/lib.rs").line()[1, 5]
```

by Section (Todo)

```writing
// doc-section: file("src/lib.rs").section("section1")
```

by Function (Todo)

```writing
// doc-func: file("src/lib.rs").func()["pre_process_file", "process_file"]
```

## Documents

[API 库的文档体系支持：主流编程语言的文档设计](https://www.phodal.com/blog/api-ducumentation-design-dsl-base/)

[文档工程体验设计：重塑开发者体验](https://www.phodal.com/blog/documentation-enginnering-experience-design/)

License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
