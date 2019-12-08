---
title = "KaTeX (Math) Formulas"
---

If you have [KaTeX](https://github.com/KaTeX/KaTeX) installed and available on your path, _mkbook_ will try to render any code blocks with a language tag of `katex` as inline math blocks.

For example:

~~~
```katex
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
```
~~~

is rendered as:

```katex
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
```

This feature is still experimental, but I find it handy for my books.
