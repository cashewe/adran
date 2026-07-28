from pprint import pprint
from adran import Parser

parser = Parser("README.md")
nodes = parser.parse()

print(nodes)

nodes_txt = nodes.to_json()
new_parser = Parser(node=nodes_txt)
pprint(
    new_parser.recall_text_indices(
        start=1_800,
        end=1_900,
        text_depth=1,
        heading_depth=None,
        text_siblings=False,
        heading_siblings=True,
    )
)

# >>>
# [
#    RecallEntry(heading='adran', body_range=None, depth=1),
#    RecallEntry(heading='Setup', body_range=None, depth=2),
#    RecallEntry(heading='Usage', body_range=None, depth=2),
#    RecallEntry(heading='Recall Text Indices Variables', body_range=None, depth=3),
#    RecallEntry(heading='Start / End', body_range=None, depth=4),
#    RecallEntry(heading='Text / Heading Depth', body_range=(1582, 2194), depth=4),
#    RecallEntry(heading='Text / Heading Siblings', body_range=None, depth=4)
# ]