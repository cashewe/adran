# MD Parser

We will use [markdown_rs](https://docs.rs/markdown/) to do the core MDAST parsing here.

from that as a basis, our goal is to create a flat tee structure that can be parsed in either direction fairly effortlessly - managed via a unique node id and `children` and `parent` attributes.

for the sake of making parsing evetually easy, we will design with a few core rules:

1. the tree will be flat - treated as a searchable 'list' of nodes
2. all sections will contain their heading as an attribute, to allow it to be consumed without parsing the underlying doc if needs be
3. there will only be one 'layer' of raw MDAST types, so things like lists will not represent the underlying list elements, text etc... inside the tree. we dont care about that here so lets skip it.