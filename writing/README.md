# Writing 

> a document-code sync tools for setup.

## MVP Scenario

knowledge transfer

1. Live Document

### Executable Documentation

1. document executable
2. code hide in markdown
3. testable document

Examples: https://github.com/phodal/exemd

```java
// exemd-deps: colored;version=1.8.0
// or 
#**
 import com.phodal.zero
**#

// for one line
import com.phodal.blog; // hidden 

## exemd-assert: zero 
```

### Document in Code

1. like Rust code

```java
/**

**/
public void should_execute() {
    
}
```

### Document-Code binding

1. code-document binding
2. documentation test

```
// exemd-code: file("World/Hello.java").line()[5, 12]
```

### Document Concept binding

1. read from CSV and output it

License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
