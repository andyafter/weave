# Weighted Average Calculator

High-Performance In-Memory Weighted Average Calculation in Rust.

## Table of Contents

- [Weighted Average Calculator](#weighted-average-calculator)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Clone the Repository](#clone-the-repository)

## Introduction

The **Weighted Average Calculator** is a Rust-based tool designed for high-performance, in-memory calculations of weighted averages. It efficiently handles large datasets and offers both sequential and parallel processing capabilities to optimize performance based on your needs.

## Features

- **Efficient Calculation**: Computes weighted averages using optimized Rust iterators.
- **Parallel Processing**: Utilizes multiple CPU cores for faster computations on large datasets via the `rayon` crate.
- **Error Handling**: Gracefully handles edge cases, such as mismatched input lengths and zero total weights.
- **Easy Integration**: Simple function signatures make it easy to integrate into existing Rust projects.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50 or later)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Clone the Repository

```bash
git clone https://github.com/your_username/weighted_average_calculator.git
cd weighted_average_calculator
```
