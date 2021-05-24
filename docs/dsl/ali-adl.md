ALI notation to show how the goals of [3] are realized in the language. ALI comprises seven parts:

1. meta types: which provides a notation for capturing meta-information
2. interface types: which provides a notation for creating types of interfaces
3. connector types: where architectural connectors are defined
4. component types: where architectural components are defined
5. pattern templates: where design patterns are defined
6. features: where the system features are catalogued
7. system: where the system architecture is described

## Meta

```
meta type MyMetaType1 {
  tag creator, description: text; 
  tag cost, version: number;
  tag edited*: date;
}

meta: MyMetaType1 { 
  creator: “John Smith”; 
  cost: 5,000;
  version: 1;
  edited: 12-02-2006;
  description: “A GUI component ...”;
}
```

## Interfaces

```

```