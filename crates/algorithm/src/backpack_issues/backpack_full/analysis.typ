#let t(text) = {
  box(width: 10cm)[#align(left)[#text]]
}

$
f(x, y) = cases(
  #t[0] x = 0,
  #t[0] y < y_x,
  #t[$max(f(x - 1, y - i * y_x) + i * v_x), i in [0, y / y_x]$] "other",
)
$

- $f(x, y)$ stands for the whole value of items put in the backpack.
- $x$ stands for the first $x$ types of items.
- $y$ stands for the remaining space of the backpack.
- $y_x$ stands for the space occupied by the $x$th item.
- $v_x$ stands for the value of the $x$th item.
- $i$ stands for the number of items, belongs to type $x$th, to put in the backpack.
