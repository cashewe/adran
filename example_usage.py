from pprint import pprint
from adran import Parser

parser = Parser("README.md")
nodes = parser.parse()

outcome = parser.recall_text_indices(
    start=1_800,
    end=1_900,
    text_depth=1,
    heading_depth=2,
    text_siblings=False,
    heading_siblings=False,
)

print(outcome.rehydrate_range())
