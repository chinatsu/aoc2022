# aoc2022-01

straight into the console to https://adventofcode.com/2022/day/1/input :)

<details><summary>Part 1</summary>

Naïve solution, but alas

```javascript
document.body.textContent.split("\n\n").map(e=>e.split("\n").reduce((d,b)=>d- -b)).sort((a,b)=>b-a)[0]
```

</details>


<details><summary>Part 2</summary>

Builds on part 1

```javascript
eval(document.body.textContent.split("\n\n").map(e=>e.split("\n").reduce((d,b)=>d- -b)).sort((a,b)=>b-a).slice(0,3).join`+`) 
```

</details>