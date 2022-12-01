# aoc2022-01

straight into the console to https://adventofcode.com/2022/day/1/input :)

<details><summary>Part 1</summary>

Naïve solution, but alas

```javascript
Math.max(...document.body.textContent.split("\n\n").map(e=>e.split("\n").reduce((d,b)=>d- -b)))
```

</details>


<details><summary>Part 2</summary>

Sorting and slicing instead of `Math.max`, otherwise the same.

```javascript
eval(document.body.textContent.split("\n\n").map(e=>e.split("\n").reduce((d,b)=>d- -b)).sort((a,b)=>b-a).slice(0,3).join`+`) 
```

</details>