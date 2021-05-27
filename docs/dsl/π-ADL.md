# π-ADL

> π-ADL: An Architecture Description Language based on the Higher-Order Typed
π-Calculus for Specifying Dynamic and Mobile Software Architectures

architecture:

- static architectures: the architecture does not evolve during the execution of the system;
- dynamic architectures: the architecture can evolve during the execution of the system, e.g. components can be created, de- leted, reconfigured, or moved at runtime;
- mobile architectures: components can logically move during the execution of the system.

structural viewpoint:

- components (units of computation of a system),
- connectors (interconnections among components for supporting their interactions),
- configurations of components and connectors.

behavioural viewpoint:

- actions a system executes or participates in,
- relations among actions to specify behaviours,
- behaviours of components and connectors, and how they interact.

##  Abstract syntax

```
# syntax of types
ValueType ::= BaseType 
          | ConstructedType

# syntax of type environments
∆ ::= | ∆, name: ValueType
      | ∅

# syntax of base types
BaseType ::= Any
            | Natural
            | Integer
            | Real
            | Boolean
            | String
            | Behaviour

# syntax of constructed types
ConstructedType ::= tuple [ ValueType1, ..., ValueTypen ]
    | view [ label1 : ValueType1, ..., labeln : ValueTypen ]
    | union [ ValueType1, ..., ValueTypen ]
    | quote [ name ]
    | variant [ label1 : ValueType1, ..., labeln : ValueTypen ]
    | location[ValueType]
    | sequence[ValueType]
    | set[ValueType]
    | bag[ValueType]
    | inout [ ValueType ] 
    | in [ ValueType ] 
    | out [ ValueType ]
    | ValueType0, ..., ValueTypen → Behaviour

# syntax of behaviours

behaviour ::= type . behaviour 
        | value . behaviour 
        | prefix . behaviour
        | if boolean then { behaviour1 }
                    else { behaviour2 }
        | choose { behaviour0 ... or behaviour }
        | compose { behaviour0 ... and behaviourn } 
        | decompose behaviour
        | replicate behaviour
        | done -- inaction
        | abstraction ( expression0 ..., expressionn ) -- application
        
prefix ::= via connectionValue send value
        | via connectionValue receive variable : ValueType 
        | unobservable
        | if boolean do prefix
        
# syntax of values
value ::= baseValue
        | constructedValue
        
constructedValue ::= ...
        | connectionValue
        | behaviourValue
        | abstractionValue

# syntax of architectural abstractions
architecturalAbstraction ::= archtype name is abstraction
archtype ::= architecture | component | connector
abstraction ::= abstraction ( name0 : ValueType0, ...,
                namen : ValueTypen ) { 
                    type0 . ... . typen .
                    value0 . ... . valuen .
                    port0 . ... . portn . 
                    behaviour is { behaviour }
                } assuming { property } 
type ::= type name is ValueType 
value ::= value name is expression 
port ::= port name is restriction {
        connection0 . ... . connectionn
    } assuming { protocol is property }
restriction ::= free | restricted
connection ::= connection name is mode ( ValueType ) 
mode ::= in | out | inout
```


### architecture

```dsl
architecture DasDef is abstraction() {
  type Key is Any. type Data is Any. type Entry is tuple[Key, Data]. 
    port update is  {  connection in is in(Entry) }.
    port request is { connection key is in(Key).
                      connection data is out(Data) } 
    assuming {
      protocol is { ( via key receive any. true*. via data send any )* }
    }.
  ... 
}
```

### component

```
component SensorDef is abstraction() {
   type Key is Any. type Data is Any. type Entry is tuple[Key, Data]. 
   port incoming is { connection in is in(Entry) }.
   port outgoing is { connection toLink is out(Entry) }.
  ...
} assuming {
protocol is { ( via incoming::in receive any. true*.
                via outgoing::toLink send any )* }
}
```
