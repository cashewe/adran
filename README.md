# adran

(*welsh*, 'ah-dran' - meaning 'section')

`Adran` is a small package which enables heirarchical expansion of markdown formatted text. using indexes, index ranges or subtext strings, return the nested text sections that are relevant to the matched region. `Adran` is likely to be a small to the point of laughably so package, spun off from a feature request for the related [`darn`](https://github.com/cashewe/darn) package. Its delivered separately as it may be useful without need for chunking.

# jist

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
- second object recieves an idx / idx range and... parses the json... somehow...

that last part is pretty vague huh... we'll need to figure *something* out for it, but for now i think we'll assume itll fall out in the wash and focus on the correct parsing of the mdast. 

extra bits:
- meta may need some custom parsing, and maybe more types than im giving credit here. dont think we need stuff like code langauge for the sake of chunking though?
- there is heading depth, but content depth has no native md context. we will need to implement this to allow for 'sibling' based recontextualisation

I'm still optimistic this is a short week or so long project, but we'll have to see when we get there. its tooken me over two months to get to this point...