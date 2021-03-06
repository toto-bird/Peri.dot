# Peri.dot ([2.0 Pre 00](https://github.com/toto-bird/Peri.dot/releases/tag/2.0.0-pre-00))

![Peri.dot Logo](https://raw.githubusercontent.com/toto-bird/Peri.dot/master/logo.png)

---
![Language](https://img.shields.io/badge/Language-Rust-000000) ![Licence](https://img.shields.io/github/license/toto-bird/Peri.dot?label=Licence) ![Docs](https://img.shields.io/badge/Docs-Unwritten-red)
<br />
![Tests](https://img.shields.io/badge/Tests-0%20✔%200%20✘%200%20➟-brightgreen) ![Version](https://img.shields.io/badge/Pre--Release-2.0.0--pre--01-yellow)

---

### Pre-Release Notes
- ADDED __Types__
```peridot
# Rusty Peri.dot

# String                : Str
"Hello World!"
'Hello World!'

# Integer               : Int
10 11 -7

# Floating Point Number : Float
10.5 11.7 -1.4

# Boolean               : Bool
true false

# Array                 : Array<L, T>
[1, 2, 3, 4] [1.0, 2.0, 3.0, 4.0]

# Function              : Func<[P, P, ...], R>
var add = func(a: Int, b: Int): Int {
    a b+
}
add(8, 5)

# Built-In Function     : Func<[P, P, ...], R>
print("Hello")

### COMING SOON ###
# Sequence              : Seq<T>
# Untyped Array         : Uarray<T, T, ...>
# Table                 : Table<K, V>
# Enumeration           : Enum<N>
# Exception             : Excep<N>
# Module                : Mod<N>
# Structure             : Struct<N>
# Implementation        : Struct<N>
```


- ADDED: __RPN Arithmetic__
```peridot
# Peri.Py
(1 + 2) * 3 # ((1 + 2) * 3)
1 + 2 * 3   # (1 + (2 * 3))

# Rusty Peri.dot
1 2+ 3*     # ((1 + 2) * 3)
1 2 3*+   # (1 + (2 * 3))
```


- ADDED: __Variables__
  - __Initialization__
  - __Assignment__
  - __Access__
```peridot
# Rusty Peri.dot

var x = 10 10+
x = 30
x # 30
```


- ADDED: __Error Codes__
```
# Console

# Before
OperationException: 10 raised to negative value 1

# Now
[e24261] OperationException: 10 raised to negative value 1
```


- ADDED: __Origin__
```
# Console

# Average Error Messages:

  File `test.peri`, In `<root>`,
  Line 15, Column 1
    my_function()
    ^^^^^^^^^^^^^
  File `test.peri`, In `my_function`,
  Line 11, Column 1
    d h+
    ^^^^
[e22410] TypeException: String can not be added to Int


# Peri.dot Error Messages:

  ╔═File test.peri, In <root>,
  ║ Line 10, Column 1
  ║   var my_function = func() -> Str {
  ║                     ^^^^^^^^^^^^^
  File `test.peri`, In `<root>`,
  Line 15, Column 1
    my_function()
    ^^^^^^^^^^^^^
    ╔═File test.peri, In <root>,
    ║ Line 4, Column 9
    ║   var b = "Hello"
    ║           ^^^^^^^
  ╔═File test.peri, In <root>,
  ║ Line 3, Column 9
  ║   var d = c
  ║           ^
  ║ ╔═File test.peri, In <root>,
  ║ ║ Line 1, Column 9
  ║ ║   var a = 10
  ║ ║           ^^
  ╠═File test.peri, In <root>,
  ║ Line 2, Column 9
  ║   var b = a
  ║           ^
  File `test.peri`, In `my_function`,
  Line 11, Column 1
    b d+
    ^^^^
[e22410] TypeException: String can not be added to Int
```


- ADDED: __If, For, and While Statements__
```peridot
# Rusty Peri.dot

var x = if (10 0 <) {
    10
} elif (0 0 <) {
    0
} elif (-10 0 <) {
    -10
} else {
    -1000
}


for var i in ([0, 1, 2, 3, 4]) {
    i
}


var i = 0
while (i 10<) {
    var i = i 1+
}
```


- ADDED: __Comments and Multiline Comments__
```peridot
# Rusty Peri.dot

# This is a toEOL comment.
#= This is a multiline comment =#

#= Comment #= Stacked comment =# Comment =#

# This is commented #= This is a comment in a comment.
=# This is still commented because of the toEOL comment.
```


- ADDED: __Including Other Files__
```peridot
# Rusty Peri.dot

var module = include("mymodule")
```
---

[Homepage](https://toto-bird.github.io/Peri.dot-lang/)<br />
```diff
- WARNING: Documentation and Playground are currently outdated (1.1)
```
[Documentation](https://toto-bird.github.io/Peri.dot-lang/docs)<br />
[Playground](https://toto-bird.github.io/Peri.dot-lang/playground)<br />