# MD Parser

We will use [markdown_rs](https://docs.rs/markdown/) to do the core MDAST parsing here.

as a reminder, we aim to achieve the following:
- parse to mdast
- recursively convert mdast to json-ifiable node object structure:

```
  - id
  - type
  - range
  - parent
  - children
  - depth
  - meta
    - wording (for headings)
    - columns (for tables)
    ... ?
```

- pass this to the user to store as they wish