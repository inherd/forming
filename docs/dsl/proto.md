

https://github.com/udoprog/reproto

```
package proto.v1;

use common as c;

tuple Sample {
  timestamp: unsigned/64;
  value: double;
}

type Graph {
  samples: [Sample];
}

interface System {
  requests_per_second: Graph;

  WebServer {
    name "web-server";

    last_logged_in: c.User;
  }

  Database {
    name "database";

    transactions: Graph;
  }
}

type GraphsRequest {
  systems: [string];
}

type GraphsResponse {
  systems: [System];
}
```