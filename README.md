# adran

(*welsh*, 'ah-dran' - meaning 'section')

`Adran` is a small package which enables heirarchical expansion of markdown formatted text. using indexes, index ranges or subtext strings, return the nested text sections that are relevant to the matched region. `Adran` is likely to be a small to the point of laughably so package, spun off from a feature request for the related [`darn`](https://github.com/cashewe/darn) package. Its delivered separately as it may be useful without need for chunking.

## jist

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

### extra bits:
- meta may need some custom parsing, and maybe more types than im giving credit here. dont think we need stuff like code langauge for the sake of chunking though?
- there is heading depth, but content depth has no native md context. we will need to implement this to allow for 'sibling' based recontextualisation

I'm still optimistic this is a short week or so long project, but we'll have to see when we get there. its tooken me over two months to get to this point...

for now, dont look for overlap with darn. just assume that since darn preserves start / end index of chunks that they are inherantly overlapping. it may be that in practice they can share an mdast parsing phase or something in future, but thats in future - i dont want to get caught up doing that right away.

## jist 2.0: you can (not) change the jist

i think theres some refinement possible here.

ultimately the best consumer layer will look somewhat like:

```
rehydrate(start_idx, finish_idx, structure, depth)
```

where the `depth` dictates the number of 'layers' above the indexes to include. in other words:
- we dont need metafields, theyre a distraction
- we dont need nodetype, its a distraction
- we just need 'sections' which represent a nested collection of structures in the text, with parent / child sections and start / end idx's

from there, if i pass a depth of:

- 1: i will return the section at the bottom of the tree (this will be at the level of paragraph, table, list etc...)
- 2: i will include its parent section (this will be everything under the lowest level heading we have)
- 3: i will include its parent section along with its siblings (this will include a heading abve ours as well as those to the left / right)

then we just need to discover the deepest section (/s) containing our idx range and we jsut rehydrate by climbing up the tree using the parent / child ids

on the flip side, we will need to additionally provide a `section_heading` field i think. for the lowest level text this will probably be generic like 'paragraph' or 'table', whereas above that it will be the actual section title. this can help us create shallow location printouts if we want i.e. `depth=2, shallow_depth=3` might allow us to rehydrate up to the section level, but also provide the text for the title of the parent / sibling sections and '...' out all the body information.