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

Linq query in Rust (created by macros).

- Inspired by [LINQ in .NET](https://docs.microsoft.com/en-us/dotnet/csharp/linq/).
- [What's LINQ](https://en.wikipedia.org/wiki/Language_Integrated_Query)

**This project is under development!**

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
        linq!(from p; in x.clone(); where p <= &5; orderby -p; select p * 2).collect();
    
    assert_eq!(e, y);
}
```

If you are familier with LINQ in C#, you will find this easy to use.

When you use two `from` statement, import `linq::expansion`.

```rust
use linq::linq;
use linq::expansion;

fn select_many(){
    let x = 1..5;
    let y = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
    let e: Vec<i32> = linq!(from p; in x.clone(); from t; in 0..p; select t).collect();
    assert_eq!(e, y);
}
```

## Linq Keywords

- [x] from
  - [x] 2-from
  - [ ] multi-from
- [x] in
- [x] select
- [x] where
- [x] orderby
- [x] descending
- [ ] more...

## Query Operators

All *italic* items mean they are not in roadmap. Happy for your suggestions.

All **bold** items mean they are implemented in this project.

- [x] where => filter
- [x] select => map
- [x] select_many => **expansion**
- [x] skip => skip
- [x] skip_while => skip_while
- [x] take => take
- [x] take_while => take_while
- [ ] join
- [ ] *group_join*
- [x] concate => chain
- [ ] *order_by*
- [ ] *order_by_descending*
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

```sh
$ cargo test
```
