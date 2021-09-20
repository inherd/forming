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
// doc-deps: colored;version=1.8.0
// or 
#**
 import com.phodal.zero
**#

// for one line
import com.phodal.blog; // hidden 

## doc-assert: zero
```

### Document-Code binding

1. code-document binding
2. documentation test

```
// doc-code: file("World/Hello.java").line()[5, 12]
```

### Document Concept binding

1. read from CSV and output it


### Document in Code

1. like Rust code

```java
/**

**/

// doc-code: document("section1").start()
public void should_execute() {
    
}
// doc-code: document("section1").end()
```

### Annotation

Java doc as annotation

```java
@param   (name of the parameter, followed by its description)
@return  (omit @return for tests that return void; required otherwise)
@succeedIf  (summarize the conditions under which the test case succeeds)
@failIf  (summarize the conditions under which the test case fails)
@qualityAttribute (specify the quality aspect addressed: performance, etc.)
@scope  (specify the test case purpose: unit, integration, etc.)
@author   (author name/surname)
@version  (version number + checkout date)
@see  package.Class#method(Type,...) (ref to the function under test)
```

License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
