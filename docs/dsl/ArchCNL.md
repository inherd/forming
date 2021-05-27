# ArchCNL

Process: [ArchCNL Process](images/arch-cnl-process.png)

### Definition

 - 架构概念表示具有明确定义的属性的核心架构抽象的特定类型。它有一个定义良好的语义，并由一个术语来描述。
 - 架构关系是连接两个架构概念的命名关系。
 - 架构规则规定了一个架构概念与另一个架构概念之间允许（许可）、必须（义务）或不允许（禁止）的架构关系。
 - 架构概念语言是一种正式的、特定于领域的语言，定义了架构概念和架构关系软件架构师和开发人员需要描述软件系统的软件架构。
 - 架构实施，是建立和显式捕获软件开发团队所有成员，其每天使用的公共架构概念语言，即在源代码和文档中都可见。架构概念语言提供了一种明确的方法，
用于描述体系结构概念和关系，以及应用于它们的规则。另外，架构实施包括验证实现是否符合体系结构概念语言。 

Origin

- **Architecture Concept**, An architecture concept represents a specific type of a core architectural abstraction with 
well-defined properties. It has a well-defined semantics and is described by a term.
- **Architecture Relation**, An architecture relation is a named relationship that connects two architecture concepts.
- **Architecture Rule**,An architecture rule prescribes which architecture relation an architecture concept is allowed 
 to have (permission), must have (obligation), or is not allowed to have (prohibition) with another architecture concept.
- **Architecture Concept Language**, An architecture concept language is a formal, domain-specific language defining the 
architecture concepts and architecture relations software architects and developers need to describe the software 
architecture of a software system.  
_ **Architecture Enforcement**, Architecture enforcement is the process of establishing and explicitly capturing a common 
 architecture concept language that is used by all members of the software development team on a daily basis and that is 
 visible in the source code as well as the documentation. The architecture concept language provides an unambiguous way 
 to describe architecture concepts and relations, as well as the rules that apply to them. Additionally, architecture 
 enforcement encompasses the verification whether the implementation conforms with the architecture concept language.

1. Specify architecture rules as ArchCNL sentences
2. Transform ArchCNL sentences to an OWL ontology
3. Transform source code artifacts to ontologies
4. Extract the implemented architecture
5. Check the implemented architecture for violations with respect to architecture rules


### Logic-based Approaches

Layer "Model" is not allowed to depend on the layer "View".

```dsl
module Model: org.company.model.** 
module View: org.company.view.** 
Model cannot-depend View

Model = Package with name:"org.company.model.*" 
View = Package with name:"org.company.view.*" 
Model cannot depend on View

Model is not allowed to back call
```

```xml
<ruleset name="Model is not allowed to depend on View"> 
    <access-rule>
        <deny>
            <from class = "org.company.model.**" />
            <to class = "org.company.view.**" />
        </deny> 
    </access-rule>
</ruleset>
```

### Query-based Approaches

Layer "Model" is not allowed to depend on the layer "View".

```xml
<concept id="Model"> 
    <cypher>
        MATCH 
            model:Package
        WHERE
            model.name = "org.company.model"
        SET
            model:ModelLayer
    </cypher> 
</concept>
```

Concept mapping for the layers View and Logic is defined correspondingly with view.name = "org.company.view"

```xml
<concept id="DefinedDependencyViewLogic"> 
    <cypher>
        MATCH 
            view:Layer
        MATCH 
            logic:Layer
        CREATE UNIQUE 
            (view)-[:defines-dependency]->(logic)
    </cypher> 
</concept>
```

Concept mapping for the dependency between Logic and Model is defined correspondingly

```xml
<constraint id="UndefinedDependency">
    <cypher>
        MATCH
            (layer1:Layer) -[:depends-on]->(layer2:Layer)
        WHERE NOT 
            (layer1)-[defines-dependency]->(layer2) AND 
            layer1.name <> layer2.name
        RETURN
            layer1.name, layer2.name
    </cypher> 
</constraint>
```

## Syntax

```
⟨specification⟩  ::= ⟨sentence⟩+
⟨sentence⟩       ::= (⟨subject⟩ ['must'|'can'] ⟨roleExpression⟩ ⟨object⟩'.')
                  | ('If' ⟨conceptID⟩ ⟨roleName⟩ ('a'|'an') ⟨object⟩ ['then' | ','] 'it' 'must' ⟨roleName⟩
                  'this' ⟨object⟩'.')
⟨subject⟩        ::= 'No' ⟨object⟩ | ('Every')? ⟨object⟩ | 'Everything' | 'Nothing'
⟨roleExpression⟩ ::= 'only' ⟨roleName⟩ ('a'|'an') | 'be' ('a'|'an') | ⟨roleName⟩ ('at-most'|'at-least'|'exactly') ⟨count⟩ | ⟨roleName⟩ ('a'|'an')
⟨object⟩         ::= ⟨conceptName⟩ (⟨relativeClause⟩)? (('and'|'or') ⟨relativeClause⟩)* ⟨relativeClause⟩ ::= 'that' '(' ⟨roleName⟩ ⟨object⟩ ')'
⟨conceptName⟩    ::= ('A'..'Z')+(('A'..'Z')|('a'..'z'))*
⟨roleName⟩       ::= ('a'..'z')+('-'('a'..'z')+)*
⟨count⟩          ::= ('1'..'9')+
```


## 


Architecture conformance checking approaches used for the comparison and their supported architecture concepts and relations for defining the intended architecture.

| Approach | Architecture Concepts | Architecture Relations  | 
|----------|----------|----------|
| DCL      | module | access, declare, handle, create, extend, implement, derive, throw, useannotation, depend |
| Dicto ̄   | entity (class, file, package, method ...) | depend on, invoke, have annotation, have method, implement interface, have method parameter, throw, catch, contain code clones, contains cycles, lead to deadlock |
| HUSACCT  | layer, component, subsystem, interface, external component | use, implement, extend |
    
Some Rules:

```
No Client can use a LogicLayer.
No Client can use a StorageLayer.
No Client can use a TestDriver.
No CommonClass can use a LogicLayer.
No CommonClass can use a GUILayer.
No LogicLayer can use a StorageEntityType. Only StorageAPI can use a ObjectifyAPI. No TestDriver can use a LogicLayer.
No TestDriver can use a StorageLayer.
No TestDriver can use a GUILayer.
No GUILayer can use a LogicBackdoor.
```