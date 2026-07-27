from pprint import pprint
from adran import Parser

parser = Parser("README.md")
nodes = parser.parse()

pprint(nodes)

nodes_txt = nodes.to_json()
new_parser = Parser(node=nodes_txt)
pprint(
    new_parser.recall_text_indices(
        start=2_800,
        end=2_900,
        text_depth=2,
        heading_depth=3,
        text_siblings=False,
        heading_siblings=True,
    )
)

