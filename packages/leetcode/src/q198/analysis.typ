#let t(text) = {
  box(width: 16cm)[#align(left)[#text]]
}

$
"memo(n)" = cases(
  #t[[(0, nums[0])]] n = 1,
  #t[[(0, nums[0]), (1, nums[1])]] n = 2,
  #t[memo[n - 1].map(|x| x.0 < i - 1 ? (i, nums[i] + prev) : None).filter(|x| x != None)] n > 2,
)
$

$
"dp(n)" = "memo(n).map"(|v| v.y).max()
$
