# _types

the (mostly Enums?) that are useful across modules... tbh this module may get consumed at some point but given the json object is needed by parser and consumer services, im thinking ahead and pretending YAGNI doesnt exist...

## MDAST_type

the MDAST types are limited compared to the full list of possible types. this is an opinionated decision made to avoid storage of unneccessary data - the goal here is to rehydrate chunks to sensible levels of detail and although chunks *may* split say a link in half, im not too bothered about preserving that knoweldge for the links sake when id rather the behaviour instead rehydrate the whole paragraph - including the link.

for this reason, only structures likely to be large have bothered to be included - a baseline quality of chunk is on the user to figure out. we support node types that include:

- paragraphs of text
- headings
- tables
- lists

which should ultimately cover all markdown structures anyways as everything else exists within these structures as nested values.