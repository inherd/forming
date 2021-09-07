# Conceptual

## Conceptual Space

wiki: https://en.wikipedia.org/wiki/Conceptual_space

Conceptual space: A conceptual space is a geometric structure that represents a number of quality dimensions, which denote basic features by which concepts and objects can be compared, such as weight, color, taste, temperature, pitch, and the three ordinary spatial dimensions.

概念空间: 概念空间是代表若干性質維度的几何结构，對於能夠相比較的概念和對象，概念空間可以表示其基本特徵，例如重量、颜色 、味道 、温度、间距和三个常規空间維度 。

```
// or context?
space = {
  
}
```

`space` or `context`

## Concept Procedure

Cucumber like, with NLP

```cucumber 
procedure("login") = """
any string in here
"""
// or
procedure("login") = file("login.cucumber")
// and ?
procedure("创建博客") = domain(Blog).create("any") {
  
}
```

## Concepts Relationship

Neo4j DSL samples: https://github.com/neo4j-contrib/cypher-dsl

Homepage: http://neo4j-contrib.github.io/cypher-dsl/current/

Examples?:

```
var m = node("Movie").named("m")
// or alias("m")

m.title = "电影"
m.code = "movie"

relation(movie, name).[type]()
```

## Concepts representation

Format：CSV

Future format：JSON、xml?

Single concept:

```
concepts = file("forming/concepts.csv") 
```

Multiple concept

```
concepts = dir("forming/*.csv")
```

or 

```
concepts = dir("forming/**/*.csv")
```

### domain

## Concept in Formalization

### Concept Matrix

```
instance Viewable Adder where
view Adder{..} = grid [[label "x", view x],
                      [label "y", view y],
                      [label "x+y", view z]]
```

## Target

Why-How-What

To-By-Using in 《系统架构:复杂系统的产品设计与开发》, en "System Architecture: Strategy and Product Development for Complex Systems"

examples:

```
To（为了改变）
 - （廉价地、环保地）
By（通过驾驶）
 - （省油、易于操纵）
Using（使用车）
 - （油/汽混合动力）
```


# Architectural

## Layered Architecture

```
domain("Blog") = {
  package: "com.phodal.blog",
  // optional dir
  dir: "com/phodal/blog"  
}
```

## Conceptual Fluent API

```
domain("Blog")
  .analysis()
  .vision()
  .usecases("*.cucumber") // ?
  .design(
    {
        prototype(),
        ui(),
    }
  )
  .concepts()
  .architecture()
  .development()
  .quality()
```

## Architecture

Define 

```
architecture = {
  language: Ruby,
  stacks { }
}
```


## Concepts DataStruct

Kotlin

```
// name = "Blog"
concept("Blog") = {
  layered {
     name: "Blog" // default to concept
     type: "Entity"
     package: "${layered.based}" // custom
  }
  struct {
    id: String,
    name: String,
    description: String,
    author: String -> relation(Author.id)
  }
  interface { 
    create(title: String): Blog;
    delete(id: String);
    update(title: String, description: String);
  }
}
```

`interface` or `behavior` ?

or 

loading from uml file: `struct = uml.file("").model("Blog")`

loading from uml dir: `struct = uml.dir("").model("Blog")`

or another form:
```
---
File: A.context
---

concept Blog {
  layered {
     name: "Blog" -- default to concept
     type: "Entity"
     package: "${layered.based}" -- custom
  }
  struct {
    id: String,
    name: String,
    description: String,
    author: String -- relation to Author.id
  }
  behavior {
    create(title: String): Blog;
    delete(id: String);
    update(title: String, description: String);
  }
}

concept Author {
	layered {
     name: "Author" -- default to concept
     type: "Entity"
     package: "${layered.based}" -- custom
  }
  struct {
    id: String,
    name: String,
    gender: Enum
  }
  behavior {
		---
		...
		---
  }
}
```

## Design by Contract

Since in the concept and between multiple concepts, their should be relationships, we need to describe this relationship and use them inside the code. We can introduce `contract` keyword.

```
---
A.contract
---
import "A.context"

contract for Blog, Author {
	struct {
		Blog.author <- Author.id
	}
	behavior {
		---
		...
		---
	}
}

contract for Blog {
	struct {
		---
		...
		---
	}
	behavior {
		---
		...
		---
	}
}

contract for Blog.create {
	precondition {
	}
	during {
	}
	postcondition {
	}
}

contract for Author {
	struct {
		---
		...
		---
	}
	behavior {
		---
		...
		---
	}
}
```

## Architecture Characteristics

```
characteristics = {

}
```

## RESTful API

```
api("Blog") {
  /create {
      in { }
      out {}
  } 
  /delete {
      in { } 
      out {  }
  }
  // or function
  create {
"""
"""
     in(title: String) 
     out(Struct(Blog)) 

     precondition { 
        标题应该大于 2: title.len > 2;
     } 
     postcondition { 
        不为空: not null
     } 
  }
}

```

## Architectural Layered


```
layered domain "Blog" = {
   fs {
      file_template: "${name.lower()}_${type.upper()}",
   }
}
```

## Architecture Diagram

```
shape: label， arrow, triangle, rectangle
```

```
layered diagram {
    row {  
       column  {
            "ITEM A",
           { name: "ITEM B", foreground: black, backend: white, border: black } 
       }
    } 
}
```

## Implementation

db_config {
  config { } ?
}

api "/blog" {
  db  {
    table ??
    field ??
  } 
}

## Decisions

```
decisions { 
  adr {
    dir("docs/adr")
  }
}
```

### Test

two type design 

```
test for api "/blog" {
  
}
```

or 

```
api "/blog" {
   test {
   
   }
}
```