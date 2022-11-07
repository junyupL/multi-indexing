# multi-indexing
Rust macro that allows multiple parameters when indexing\
For example, expr[2, "cat"] is syntax sugar for (*expr.index2(2, "cat"))\
asdf[abc, 3.4, 11, None] is syntax sugar for (*asdf.index4(abc, 3.4, 11, None))\
pointer[] is not is syntax sugar for (*pointer.index0()).\
Instead, it is syntax sugar for just (*pointer).\
indexable_var[expr] is still just indexable_var[expr].\
Array expressions like [2, 3, 5] and [3; 5] are not allowed, but array_macro has replacements for them. 