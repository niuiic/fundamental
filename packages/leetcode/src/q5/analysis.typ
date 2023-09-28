#let t(text) = {
  box(width: 8cm)[#align(left)[#text]]
}

$
"dp(start, end)" = cases(
  #t[true] "start = end",
  #t[s[start] == s[end]]" start = end - 1",
  #t[s[start] == s[end] && dp(start + 1, end - 1)] "start < end - 1",
)
$
