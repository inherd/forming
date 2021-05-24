# ACME

refs: https://acme.able.cs.cmu.edu/docs/language_overview.html

```dsl
System simple_cs = {
    Component client = { Port send-request; };
    Component server = { Port receive-request; };
    Connector rpc = { Roels { caller, callee}};
    Attachments {
        client.send-request to rpc.caller;
        server.receive-request to rpc.callee;
    }
}
```

more 

```dsl
System simple_cs = {
  Component client = {
    Port send-request;
    Property Aesop-style : style-id = client-server;
    Property UniCon-style : style-id = client-server;
    Property source-code : external = "CODE-LIB/client.c";
  }
  Component server = {
    Port receive-request;
    Property idempotence : boolean = true;
    Property max-concurrent-clients : integer = 1;
    source-code : external = "CODE-LIB/server.c";
  }
  Connector rpc = {
    Role caller;
    Role callee;
    Property asynchronous : boolean = true;
    max-roles : integer = 2;
    protocol : Wright = " ... ";
  }
  Attachment client.send-request to rpc.caller;
  Attachment server.receive-request to rpc.callee;
}
```

dsl 2

```
exists  client, server, rpc |
  component(client) ^
  component(server) ^
  connector(rpc) ^
  attached(client.send-request,rpc.caller) ^
  attached(server.receive-request,rpc.callee) ^
  client != server ^ 
  server != rpc ^ 
  client != rpc ^
  (for all y:component (y) =>
                  y = client | y = server) ^
  (for all y:connector(y) => y = rpc) ^
  (for all p,q: attached(p,q) =>
       (p=client.send-request ^ q=rpc.caller)
     | (p=server.receive-request ^ q=rpc.callee))
```