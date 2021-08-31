# Eiffel Language

https://www.eiffel.org/doc/eiffel/Eiffel_programming_language_syntax


```eiffel
my_list.do_all (agent (s: STRING)
     require
         not_void: s /= Void
     do
         s.append_character (',')
     ensure
         appended: s.count = old s.count + 1
     end)
```

```eiffel
connect_to_server (server: SOCKET)
      -- Connect to a server or give up after 10 attempts.
    require
        server /= Void and then server.address /= Void
    local
        attempts: INTEGER
    do
        server.connect
    ensure
      connected: server.is_connected
    rescue
        if attempts < 10 then
            attempts := attempts + 1
            retry
        end
    end

```

中文：http://softwarepractitioner.org/translations/meyer/contract.shtml

```
     put (x: ELEMENT; key: STRING) is
                     -- Insert x so that it will be retrievable through key.
             require
                     count <= capacity
                     not key.empty
             do
                     ... Some insertion algorithm ...
             ensure
                     has (x)
                     item (key) = x
                     count = old count + 1
             end
```

require 子句引出了输入条件，或前提条件（precondition）； ensure 引出了输出条件，或后置条件（postcondition）。 这两个条件都是与软件单元（译者注：此处为DICTIONARY [ELEMENT]） 相关联的断言（assertion），或逻辑条件（契约子句）。 在前提条件中，count是当前数据项的数量，capacity是允许的最大数； 在后置条件中，has是个布尔查询，它告诉你一个给定的元素是否存在， 而item的结果是一个与指定关键名相连的元素；old count 表示进入put时的count值。

