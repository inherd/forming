# Forming

> Forming, is a lightweight architecture as code language. 轻量级架构即代码语言 

Architecture 3.0 based frameworks.

## Usage

// todo

## Samples

1. concept declare with struct & behavior

```
concept Blog(Displayable, Ownable) {
    struct {
        title, slug, description, gen_description, content, featured_image: String;
        id, user_id, site_id: Integer;
        created, updated: datetime;
    }
    behavior {
        get_absolute_url(): String;
        validate_unique();
        publish_date_since(): datetime;
        published(): Integer;
        save(blog: Blog);
        delete(id: Integer);
    }
}
```

2. api declare

```
api for BlogPost {
    in { title: String, description: String }
    out { blog: Blog }
    pre_cond {
       '字符串不为空': not empty
    }
    pre_cond {
       '博客不为空': 'not empty'
    }
} 
```

3. concept space

```
space Blog {
   package: 'com.phodal.blog', // or path
   type: 'Entity',
   items: { Blog, BlogCategory, BlogCategories, BlogRelatedPosts, Comments }
}
```

4. from CSV, uml and swagger (todo)

```
concepts => file(\"concepts.csv\")

concept  Blog {
    behavior { }
    struct uml::dir('').class('Blog')
}
```


## Todo

- [ ] 架构描述
- ~~[ ] 架构布局~~
   - [ ] ~~Diagrams as code~~
   - [ ] ~~Ascii Doctor~~
- [ ] 模型声明
   - [x] DDD Styles
- [ ] 模型代码生成 
   - [ ] spike [codeviz](https://github.com/udoprog/codeviz)
   - [ ] spike [genco](https://github.com/udoprog/genco)
- [ ] ~~Highlighted Core Documents~~
   - [ ]  ~~auto documentation~~

TBD:

- [ ] workflow design
   - [ ] collaboration style
   - [ ] physics design
   - [ ] associate patterns
   - [ ] design patterns in code
- [ ] core driven style
   - [ ] domain driven
   - [ ] use-cases driven
   - [ ] collaboration driven
- [ ] forming visualization
   - [ ] web page
   - [ ] common architecture patterns in svg
- [ ] Integration (in ext model)
  - [ ] UML parser for modeling
  - [ ] Swagger to API in/out
  - [ ] ~~Cucumber parser~~
- [ ] codegen
   - [ ] java code gen
   - [ ] code gen protocol

Documents
---

### Refs

- [Diagrams as code](https://diagrams.mingrammer.com/docs/getting-started/examples), GitHub: [Diagrams](https://github.com/mingrammer/diagrams)
- [Asciidoctor](https://asciidoctor.org/docs/asciidoctor-diagram/)
- [Rust libhoare](https://github.com/nrc/libhoare) is a simple Rust support for design by contract-style assertions.
- [genco](https://github.com/udoprog/genco) is a whitespace-aware quasiquoter for beautiful code generation.
- Iaakov Exman "Software Conceptual Integrity: Deconstruction, Then Reconstruction"

License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
