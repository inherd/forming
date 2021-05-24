# π-ADL

> π-ADL: An Architecture Description Language based on the Higher-Order Typed
π-Calculus for Specifying Dynamic and Mobile Software Architectures

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
