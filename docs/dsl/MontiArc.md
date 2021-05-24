# MontiCore - ADL

Refs: [https://github.com/MontiCore/monticore/blob/dev/docs/Languages.md](https://github.com/MontiCore/monticore/blob/dev/docs/Languages.md)

Systematic Language Extension Mechanisms for the MontiArc Architecture Description Language


https://github.com/MontiCore/monticore

```dsl
grammar MyStatemachine extends Automata,                  // MontiCore grammar 
                               MCBasicTypes, SetExpressions, MCCommonLiterals {     
  start Automaton;

  // overriding a nonterminal (to add optional conditions):
  Transition = from:Name@State ":" Expression? "->" to:Name@State;

  // add new variants of expressions
  LogicalNotExpr implements Expression = "!" Expression;

  XorExpr        implements Expression =
        left:Expression "xor" right:Expression;

  scope LetExpr  implements Expression =
        "let" (VarDeclaration || ",")+ "in" Expression;

  symbol VarDeclaration = MCType? Name "=" Expression ;
}
```

## Class Diagram For Analysis (CD4A) (MontiCore stable)

```
classdiagram MyLife { 
  abstract class Person {
    int age;
    Date birthday;
    List<String> nickNames;
  }
  package com.universityLib {
    <<myStereotype>> class Student extends Person {
      StudentStatus status;
    }
    enum StudentStatus { ENROLLED, FINISHED; }
  }
  
  composition Person -> Address [*]  {ordered};
  association [0..2] Person (parent) <-> (child) Person [*];
  association phonebook Person [String] -> PhoneNumber ;
}
```

## Feature Diagrams (not yet publicly available) (MontiCore stable)

```
featurediagram MyPhones {
  Phone -> Memory & OS & Camera? & Screen;
  Memory -> Internal & External?;
  Internal -> [1..2] of {Small, Medium, Large};
  OS -> iOS ^ Android;
  Screen -> Flexible | FullHD;

  Camera requires (iOS && External) || Android ;
}
```

## MontiArc (not yet publicly available) (MontiCore Stable)

```
component InteriorLight {                           // MontiArc language
  port in Boolean lightSignal,          // ports
       in Boolean doorSignal
       out OnOff status;
  ORGate or;                            // used subcomponents
  lightSignal -> or.a;                  // connectors
  doorSignal -> or.b;
  or.c -> cntr.signal;
  component LightController cntr {      // freshly defined subcomponent 
    port in OnOff signal,
         out OnOff status;
    statechart {                        // with behavior by a Statechart
      initial state Off / {status = OFF};
      state On;
      Off -> On [ signal == true ] / {status = ON}
      On -> Off [ signal == false ] / {status = OFF}
    }
  }
  cntr.status -> status;
}
```

## Object Diagrams (not yet publicly available) (MontiCore Stable)

```
objectdiagram MyFamily {
  alice:Person {
    age = 29;
    cars = [
      :BMW {
        color = BLUE;
      },
      tiger:Jaguar {
        color = RED;
        length = 5.3; 
      }
    ];
  };
  bob:Person {
    nicknames = ["Bob", "Bobby", "Robert"];
    cars = [tiger];
  };
  link married alice <-> bob;
}
```