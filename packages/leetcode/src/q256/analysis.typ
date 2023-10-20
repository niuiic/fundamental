#let t(text) = {
  box(width: 16cm)[#align(left)[#text]]
}

$
"dp(n)" = cases(
  #t[(0, costs[0][0]), (1, costs[1][1]), (2, costs[2][2]),] n = 1,
  #t[dp(n - 1).map(|x| min(arr))] n > 1,
)
$

$
"arr = [x.1 + costs[n - 1][0], x.1 + costs[n - 1][1], x.1 + costs[n - 1][2]] (exclude these do not meet conditions )"
$

$
"result = dp(n).map(|x| x.1).min"
$
