# adran

![piccy](docs/watering_the_lovely_plants.jpg)

(*welsh*, 'ah-dran' - meaning 'section')

`Adran` is a small package which enables heirarchical expansion of markdown formatted text based on index ranges - especially useful for expanding chunks in RAG pipelines. `adran` builds its heirarchies based on markdown heading nests, and can be used to identify the sections, parent sections and sibling sections of a range of text from the source file.

## Setup

To use `adran`, you must first install it, ideally into a python virtual environment:

```
pip install uv

uv venv --python 3.13
source .venv/bin/activate # or .venv\Scripts\activate on windows

uv pip install darn_it
```

## Usage

You can import it into your python session to use:

```
from adran import Parser

parser = Parser(
  markdown_path="README.md"
)
nodes = parser.parse()
```
this will create a json formatted output explaining the section structure of your markdown text input, for instance the output for this README can be found in `tests/README_parse_output.json`

you can then choose to consume this data at a later point, by providing the index range in your markdown file that you wish to expand from:

```
outcome = parser.recall_text_indices(
    start=2_800,
    end=2_900,
    text_depth=1,
    heading_depth=None,
    text_siblings=False,
    heading_siblings=True,
)
```

yielding the following outcome:

```
[
    RecallEntry(heading='adran', body_range=None, depth=1),
    RecallEntry(heading='Setup', body_range=None, depth=2),
    RecallEntry(heading='Usage', body_range=None, depth=2),
    RecallEntry(heading='Recall Text Indices Variables', body_range=None, depth=3),
    RecallEntry(heading='Start / End', body_range=None, depth=4),
    RecallEntry(heading='Text / Heading Depth', body_range=(1582, 2194), depth=4),
    RecallEntry(heading='Text / Heading Siblings', body_range=None, depth=4)
]
```

users may then wish to use the built in `.rehydrate_range()` method to then access the text:

```
outcome.rehydrate_range(
    # the markdown path must be provided either here or in the Parser
)
```

### Recall Text Indices Variables

#### Start / End

These variables define the index range in the markdown file you want to expand from.

#### Text / Heading Depth

These variables define how far up the section tree you wish to climb. see the following table for details:

| value | meaning |
|---|---|
| 0 | do not even include the section your range is included within |
| 1 | include the sections your range is explicitly covering, but nothing more |
| 2 | include the parent section of your current subsection, (along with any siblings, if siblings are turned on) |
| N | incude the 'N - 1' parent sections of your current subsection, (along with any siblings, if siblings are turned on) |
| None | Terminal value, selects all so you dont need to know depth ahead of time |

#### Text / Heading Siblings

These variables decide whether `adran` will expand strictly linearly or include sibling sections during its expansion (i.e. other nodes not in the range that are of the same level of depth). the siblings will be bound by your rules on depth - but bare in mind children of siblings will therefore always be included since these will always be at the right depth if the parent is.

## But... Why?

Though small chunks are often useful for vector search, the limited context can cause RAG systems to give incorrect or partial answers. by allowing a posthoc reexpansion step, your pipeline is able to find the best of both worlds.
