# Linq in Rust

[![](https://img.shields.io/travis/StardustDL/Linq-in-Rust.svg)](https://travis-ci.org/StardustDL/Linq-in-Rust)
[![](https://img.shields.io/codecov/c/gh/StardustDL/Linq-in-Rust.svg)](https://codecov.io/gh/StardustDL/Linq-in-Rust)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/f5f2e7f76dbb45cca9a9abc084568b9f)](https://www.codacy.com/app/StardustDL/Linq-in-Rust?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=StardustDL/Linq-in-Rust&amp;utm_campaign=Badge_Grade)
[![](https://img.shields.io/librariesio/github/StardustDL/Linq-in-Rust.svg)](https://libraries.io/cargo/linq)
[![](https://img.shields.io/crates/v/linq.svg)](https://crates.io/crates/linq)
[![](https://img.shields.io/crates/v/linq.svg?label=docs&&colorA=blue)](https://docs.rs/linq/)
![](https://img.shields.io/crates/d/linq.svg)
![](https://img.shields.io/crates/l/linq.svg)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/StardustDL/Linq-in-Rust.svg)](http://isitmaintained.com/project/StardustDL/Linq-in-Rust "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/StardustDL/Linq-in-Rust.svg)](http://isitmaintained.com/project/StardustDL/Linq-in-Rust "Percentage of issues still open")

Linq query in Rust (created by declarative macros).

- Inspired by [LINQ in .NET](https://docs.microsoft.com/en-us/dotnet/csharp/linq/).
- [What's LINQ](https://en.wikipedia.org/wiki/Language_Integrated_Query)

**This project is under development!** API might be **changed**.

## Quick Start

This is an example:

```rust
use linq::linq;

fn try_linq(){
    let x = 1..100;

    let mut y: Vec<i32> = x.clone().filter(|p| p <= &5).collect();
    y.sort_by_key(|t| -t);
    let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();

    let e: Vec<i32> =
        linq!(from p in x.clone(), where p <= &5, orderby -p, select p * 2).collect();
    
    assert_eq!(e, y);
}
```

If you are familier with LINQ in C#, you will find this is easy to use.

## Usage

The query statement begins with `from` clause and ends with `select` clause. Use `,` to seperate every clause.

```rust
linq!(from x in coll, select x)
```

### From

```rust
from <id> in <iter expr>,
```

Also you can enumerate elements of each set in the collection (Attention: for this type, you can't access the value that is in the first `from` clause in `select` clause):

```rust
let x = 1..5;
let y = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
let e: Vec<i32> = linq!(from p in x.clone(), from t in 0..p, select t).collect();

assert_eq!(e, y);
```

If you want to zip or enumerate value-pairs of two sets, use `zfrom` for the second `from`:

```rust
let x = 1..5;
let y = vec![
    (1, 0),
    (2, 0),
    (2, 1),
    (3, 0),
    (3, 1),
    (3, 2),
    (4, 0),
    (4, 1),
    (4, 2),
    (4, 3),
];
let e: Vec<_> = linq!(from p in x.clone(), zfrom t in 0..p, select (p,t)).collect();

assert_eq!(e, y);
```

The expression in `zfrom` recieve the cloned value in the first `from`,
and the elements in two sets will be cloned for `select` clause.

### Where

```rust
while <expr>,
```

You can use `where` clause in single-from query, and the expression will recieve a variable named the `id` in `from` clause. The expression need to return a boolean value.

### Orderby

```rust
orderby <expr>,
orderby <expr>, descending,
```

You can use `orderby` clause in single-from query. This query will collect the iterator, and sort them by the expression, then return the new iterator.

## Linq Keywords

- [x] from
  - [x] from (child-from)
  - [x] zfrom (zip-from)
- [x] in
- [x] select
- [x] where
- [x] orderby
- [x] descending
- [ ] group_by
- [ ] more...

## Query Operators

All *italic* items mean they are not in roadmap. Happy for your suggestions.

All **bold** items mean they are implemented in this project. You can find them in module `linq::ops`.

- [x] where => **where_by** => filter
- [x] select => **select_one** => map
- [x] select_many => **select_many, select_many_zip, select_two**
- [x] skip => skip
- [x] skip_while => skip_while
- [x] take => take
- [x] take_while => take_while
- [ ] join
- [ ] *group_join*
- [x] concate => chain
- [x] order_by => **order_by**
- [x] order_by_descending => **order_by**
- [ ] *then_by*
- [ ] *then_by_descending*
- [x] reverse => rev
- [ ] *group_by*
- [ ] *distinct*
- [ ] *union*
- [ ] *intersect*
- [ ] *except*
- [x] first => next
- [ ] single
- [x] element_at => nth
- [x] all => all
- [x] any => any
- [ ] contains
- [x] count => count
- [ ] sum
- [ ] min
- [ ] max
- [ ] average
- [ ] aggregate

## Development

We need more unit-test samples. If you have any ideas, open issues to tell us.

Since the expression procedural macros is not stable, I only create macros by declarative macros.

```sh
$ cargo test
```
