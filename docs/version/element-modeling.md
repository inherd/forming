// composition over inheritance
# Elemental Modeling

## element 

// # facts {}
//
// factum: event/command, evidence, schema, usecase, component
// aggregate: by event, by evidence, by schema, usecase, component
// aggregate_by: aggregate, contract, database, usecase, component
// process(flow(流)):
// identify(change(变化), capture(捕获), disambiguate(消除歧义)):
// element(or entry, or entity):
// context(or boundary):

### frontend samples

page, dialog: Home, CreateBlog, BlogList, BlogDetail, BlogSearch

domain event: "create_blog()" -> 

streaming event:

- input title      
- select_blog_category (source: input -> output)
- set_blog_status
- select_publish_time
- select_related_posts
- set_keywords
- save_draft
- set_generate_description
- public_content (-> Http Request -> Http Response -> request/response mapper)

form - realtime validate.

interface()

## events, effects

// # actions {}
//
// map: 需求映射
// filter(lookup(查找)): 过滤元素
// reduce: 归并
// merge/combine: 合并?
// collect:

- rule -> validate
- process -> bpm
- model

## process

// # process {}
// parallel { }
//
//

## misc

// # version control
//
// tag(add(0.5))


// # adrs?

// # diagram as code
// 
// 