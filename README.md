# Binary Grammar Recognizer

A Rust-based **formal grammar recognizer** that parses and validates **binary numbers** and **expressions** built from binary operands and operators.

This project explores concepts from **formal language theory**, **recursive descent parsing**, and **Rust traits and enums** — implemented in a clean, modular way.

---

## Overview

This recognizer can:

* Identify **valid binary numbers** (`0`, `101`, `001`)
* Recognize **binary decimals** (`101.1`, `10.01`)
* Parse **binary operations** (`10 + 11`, `101.1 * 10.01`)
* Reject invalid inputs (`102`, `10..1`, `10.01.01`, `1 + 2`)

### Grammar Definition

```
<expr>    → <number> | <number> <op> <number>
<number>  → <whole> | <whole>.<fraction>
<whole>   → <digit> | <whole><digit>
<fraction>→ <digit> | <fraction><digit>
<op>      → + | - | * | /
<digit>   → 0 | 1
```

---

## Concepts Demonstrated

* **Traits**: custom extension of `&str` with safe splitting logic (`SplitAtChecked`)
* **Recursion**: used to parse nested number structures
* **Enums**: for representing binary types (`Whole`, `Decimal`) and operations
* **Pattern Matching**: for parsing different syntactic forms cleanly
* **Grammar Recognition**: validates structure instead of evaluating results


---

## Running the Project

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) installed
* Basic command-line familiarity

### Run

```bash
cargo run
```

You’ll see an output like:

```
101: valid -> Single(Whole("101"))
10.01 + 1: valid -> Operation { left: Decimal { whole: "10", fraction: "01" }, op: Add, right: Whole("1") }
101 / 2: invalid
```

---

## 🧪 Example Test Inputs

| Input        | Output                          |
| ------------ | ------------------------------- |
| `101`        | Whole binary                  |
| `10.01`      | Binary decimal                |
| `10 + 11`    | Binary addition expression    |
| `111 - 10.1` | Binary subtraction expression |
| `101 / 2`    | Invalid                       |
| `10.1.1`     | Invalid                       |

---

## Inspiration

Built as a learning project to:

* Understand **formal grammar and recursive parsing**
* Strengthen **Rust fundamentals**
* Learn to think like a **compiler designer**

Driven by curiosity and a refusal to waste any learning opportunity !
