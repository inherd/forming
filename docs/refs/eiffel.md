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