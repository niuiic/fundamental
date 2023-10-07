#let t(text) = {
  box(width: 13cm)[#align(left)[#text]]
}

$
"dp(n)" = cases(
  #t[0] n = 1,
  #t[min(dp(n - i) + 1)(ignore if it cannot arrive at position $n - i$)($1 lt.eq i < n$)] n > 1,
)
$
