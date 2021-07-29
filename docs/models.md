 
实例模型捕捉 

- 需求模型（Requirements Model）。
- 领域模型（Domain Model）。
- 架构模型（Architecture Model）。
- 映射架构模型（Mapped Architecture Model）。
- 实现模型（Implementation Model）。


需求模型

概念描述模型

```
// Characterization
cha(concept) = {domain, ([attrib_1, range_1], [attrib_2, range_2],...,[attrib_k, range_k])}

// origin examples
// Characterization(cash-withdrawal) = { bank-operation, ([type, cash], [amount, “cash- limited”], [duration, “one-day”]}

// add description
concept(normal_login) = {login, [(key, user_name), (key, password)]}

concept(phone_login) = {login, [(key, phone_number), (key, verify_code)]}

// process(normal_login) =
```