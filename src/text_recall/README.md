# text recall

This module is all about consuming the json tree to access the information we actually need at inference time or whatever you call the time when you need the information you access at the time im trying to refer to.

the interface is going to look like the following:

```
recall_text_indices(
    nodes: // json node model output from md_parser
    start_idx: int
    end_idx: int
    text_depth: int // decides how far up the tree you go from the lowest level your idx's cover
    heading_depth: int // decides how far up the tree for headings you go from the lowest level your idxs cover
    text_siblings: bool // set true to include sibling nodes, set false to only care about linear successors
    heading_siblings: bool // set true to include sibling headings, set false to only care about direct successors
) -> // its a vec that contains plain text snippets as well as index pairs of what needs to be pulled
```

a couple of points on this:
- the output should be an ordered list, but its dtypes look like they need to be one of two things: either a string representing the text of a heading (complete with correct '#' count and followed by some ...'s) or a start / end index pair taken from a node that now needs inclusion
- headings covered by the body of a section neednt be separately included as a heading, that will duplicate the outputs.
- depth of 1 mean take only the content from the lowest levels covered by idx. depth 2 means go one layer up. 3 means take the parent again - but if we set siblings to false then this needs to be carefully controlled so as not to include the free text covered by the sibling nodes. if the sibling heading is set true it should show the heading in its correct place with elipses to represent the missing text, if sibling headings is false then the sibling should simply be invisible for the output.
- the `depth` variables should be able to be set to maximum if the user wants without htem needing to know ahead of time what the maximum depth is. prehaps a '-1' value or some terminal would be good. we probably dont want this to be the default behaviour though as the users are unlikely to remember to set depth always

the goal once settled is to pass the indexes / free text structure up to python such that a python user could feasibly read the indexed text out of the source file to construct the hydrated object. this isnt handled by the package as users may prefer to add this stuff as metadata to the embedding than to pull it ahead of time for instance. this means that the indexes need to be correcly aligned for python code, or that the python code needs to be written with rusts byte based indexing in mind.

the python code does not live in this module, look to `adran` for that.