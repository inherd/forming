# CogArch-ADL

CogArch-ADL: Toward a Formal Description of a Reference Architecture for the Common Model of Cognition

# Definitions

Reference Model

参考模型是功能之间的划分以及各部分之间的数据流，它定义了将已知问题的标准分解成多个可协作解决问题的部分。

Reference Architecture

参考体系结构是映射到系统分解（软件元素和它们之间的数据流）的参考模型，它处理业务规则、满足质量属性的体系结构样式、软件开发的最佳实践以及支持该领域系统开发的软件元素。

Software Architecture

软件体系结构是参考体系结构的一种特殊特征（具体化）。 

# Syntax: Formal System

- Typing Rules:
- Transition Rules:
- Abstract Syntax:
- choice
- composition
- prefixes

```
# syntax of behaviors
behavior ::= type . behavior 
  | value . behavior
  | prefix . behavior
  | if (boolean) then { behavior1 }
                else { behavior2 } 
  | choose { behavior0... or behaviorn }
  | compose {behavior0... and behaviorn }
  | decompose behavior
  | replicate behavior
  | abstraction ( expression0..., expressionn) 
  | constraint name1 is (constraint1)
  | heuristic name1 is (heuristic1)
  | check( constraint1 | heuristic1 )

prefix ::= via connectionValue send value
    | via connectionValue receive variable : ValueType 
    | unobservable
    | if boolean do prefix
connection ::= connection name

# syntax of types and values
BaseType ::= Any | Natural | Integer | Real | Boolean | String | Behaviour
ConstructedType ::= tuple [ ValueType1, ..., ValueTypen ]
    | set [ ValueType]
    | inout [ ValueType ]
    | in [ ValueType ] 
    | out [ ValueType ]
    | ...

# syntax of types in CogArch-ADL
Buffer   |  abstraction ( expression0..., expressionn) 
Stimuli  | Real // internal or external stimuli
Percept  | Any // units of perception
ProcCont | Any // units of procedural memory content 
DecCont  | Any // units of declarative memory content
WMCont   | Any // units of working memory content
Action   | Any // actions (procedural and motor modules)
```