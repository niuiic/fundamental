#let t(text) = {
  box(width: 8cm)[#align(left)[#text]]
}

$
"dp(n)" = cases(
  #t[[]] n = 0,
  #t[["()"]] n = 1,
  #t[["(" + dp(i) + ")" + dp(n - i - 1)](0 $lt.eq$ i < n)] n > 1,
)
$
