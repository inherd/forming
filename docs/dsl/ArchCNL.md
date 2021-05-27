# ArchCNL

### Definition

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

```
module Model: org.company.model.** 
module View: org.company.view.** 
Model cannot-depend View

Model = Package with name:"org.company.model.*" 
View = Package with name:"org.company.view.*" 
Model cannot depend on View

Model is not allowed to back call

<ruleset name="Model is not allowed to depend on View"> 
    <access-rule>
        <deny>
            <from class = "org.company.model.**"> 
            <to class = "org.company.view.**">
        </deny> 
    </access-rule>
</ruleset>
```

### Query-based Approaches

Layer "Model" is not allowed to depend on the layer "View".

```
<concept id="Model"> 
    <cypher>
        MATCH model:Package
        WHERE
        model.name = "org.company.model"
        SET model:ModelLayer
    </cypher> 
</concept>
```

## Syntax

```
⟨specification⟩ ::= ⟨sentence⟩+
⟨sentence⟩ ::= (⟨subject⟩ [‘must’|‘can’] ⟨roleExpression⟩ ⟨object⟩‘.’)
| (‘If’ ⟨conceptID⟩ ⟨roleName⟩ (‘a’|‘an’) ⟨object⟩ [‘then’ | ‘,’] ‘it’ ‘must’ ⟨roleName⟩
‘this’ ⟨object⟩‘.’)
⟨subject⟩ ::= ‘No’ ⟨object⟩ | (‘Every’)? ⟨object⟩ | ‘Everything’ | ‘Nothing’
⟨roleExpression⟩ ::= ‘only’ ⟨roleName⟩ (‘a’|‘an’) | ‘be’ (‘a’|‘an’) | ⟨roleName⟩ (‘at-most’|‘at-least’|‘exactly’) ⟨count⟩ | ⟨roleName⟩ (‘a’|‘an’)
⟨object⟩ ::= ⟨conceptName⟩ (⟨relativeClause⟩)? ((‘and’|‘or’) ⟨relativeClause⟩)* ⟨relativeClause⟩ ::= ‘that’ ‘(’ ⟨roleName⟩ ⟨object⟩ ‘)’
⟨conceptName⟩ ::= (‘A’..‘Z’)+((‘A’..‘Z’)|(‘a’..‘z’))*
⟨roleName⟩ ::= (‘a’..‘z’)+(‘-’(‘a’..‘z’)+)*
⟨count⟩ ::= (‘1’..‘9’)+
```