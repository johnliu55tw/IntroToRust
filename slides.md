<!-- $theme: gaia -->

# Intro To Rust

---

# Why Another Programming Language?

---

## Why Another Programming Language?

* Memory safe without garbage collection

* Compiled language. [Blazingly fast!](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp)

* Makes concurrency easy...ier

* Tough but very kind compiler

---

## Memory safety? How unsafe could it be?

* Accessing uninitialized data

* Dangling pointers

* Double free

* Data races


---


## Memory safety? How unsafe could it be?

* Accessing uninitialized data

* Dangling pointers

* Double free

* Data races

* ==**Shared mutable state is the root of all :smiling_imp:**==

---

## Ownership and Borrowing

##### Where magic happened :sparkles:

---

### Ownership

* Variable Bindings have the ==ownership== of what they bound to

```
fn foo() {
    // v is the owner of the vector!
    let v = vec![1, 2, 3];
}
```
---

### Ownership

* Ownership could be transfered
```
fn main() {
    // v is the owner of the vector
    let v = vec![1, 2, 3];
    // Not anymore. The ownership has been transfered
    // to i_want_v
    let i_want_v = v;
    println!("{}", v[0]); // Error!!!!
}
```
---
 
### Ownership

* ==Pass-by-Value== is ==Pass-the-Ownership==

```
fn take_ownership(v: Vec<i32>) {
    println!("I got the ownership!!!");
}

fn main() {
    // v is the owner of the vector
    let v = vec![1, 2, 3];
    // Now pass the ownership to take_ownership
    take_ownership(v);
    println!("{}", v[0]); // Error!!!!
}
```

---

### Ownership

* ==Pass-by-Value== is ==Pass-the-Ownership==

```
fn take_ownership(v: Vec<i32>) {
    println!("I got the ownership!!!");
}

fn main() {
    // v is the owner of the vector
    let v = vec![1, 2, 3];
    // Now pass the ownership to take_ownership
    take_ownership(v);
    println!("{}", v[0]); // Error!!!!
}
```
* Unless you got `Copy` trait. :smirk:

---

### Ownership

#### Why so serious, compiler?

* What if we don't care about ownership?

```
let v = vec![1, 2, 3];
let mut v2 = v;
v2.truncate(2); // Shorten the length to 2 elements long
println!("Let me access v[2]...{}", v[2]) // Error!!
```

---

### Ownership

#### Why so serious, compiler?

* Because we have **stack** and **heap**

---

### Ownership

#### Why so serious, compiler?

* Because we have **stack** and **heap**

* Shared and mutable state is the root of all :smiling_imp:

---

### Ownership

#### Why so serious, compiler?

* Because we have **stack** and **heap**

* ~~Shared~~ and mutable state is the root of all :smiling_imp:

---

### Ownership

"Hey function, I need that ownership back." 
"Ok then."

```
fn foo(v1: Vec<i32>, v2: Vec<i32>) 
      -> (Vec<i32>, Vec<i32>, i32) {
    // do stuff with v1 and v2...
    // hand back ownership and the result of our function
    (v1, v2, 42)
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let (v1, v2, answer) = foo(v1, v2);
```

---

### Ownership

"Hey function, I need that ownership back." 
"Ok then."

```
fn foo(v1: Vec<i32>, v2: Vec<i32>) 
      -> (Vec<i32>, Vec<i32>, i32) {
    // do stuff with v1 and v2...
    // hand back ownership and the result of our function
    (v1, v2, 42)
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let (v1, v2, answer) = foo(v1, v2);
```
#### :scream: :scream: :scream:

---

### References And Borrowing

* The resources could be **borrowed**! :tada:

* Borrowed by **references**
	
    * Shared Reference
    
    * Mutable Reference

---

### References And Borrowing

#### Shared Reference `&T`

```
fn sum_vec(v: &Vec<i32>) -> i32 {
    // Borrow a shared reference of a vector and
    // calculating sum of all elements
    return v.iter().fold(0, |a, &b| a + b);
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let sum = sum_vec(&v);
    println!("Sum of {:?} is {}", v, sum); // No Error!
}
```

---

### References And Borrowing

#### Mutable Reference `&mut T`

```
fn add_element(v: &mut Vec<i32>, value: i32) {
    v.push(value); // Mutate the vector
}

fn main() {
    let mut v = vec![1, 2, 3, 4]; // mut is needed
    add_element(&mut v, 5); // Borrowed as mutable
}
```

---

### References And Borrowing

#### :smiling_imp: :smiling_imp: :smiling_imp:

```
let mut v = vec![1, 2, 3];
let v2 = &mut v;
v2.truncate(2);
// Oh no! v doesn't know the referenced vector
// has changed!
println!("Let me access v[2]...{}", v[2]);
```

---

### References And Borrowing

#### :smiling_imp: :smiling_imp: :smiling_imp:

```
let mut v = vec![1, 2, 3];
let v2 = &mut v;
v2.truncate(2);
// Oh no! v doesn't know the referenced vector
// has changed!
println!("Let me access v[2]...{}", v[2]);
```

```
error: cannot borrow `v` as immutable because
       it is also borrowed as mutable
```
:raised_hands: :raised_hands: :raised_hands:

---

### References And Borrowing

#### The Rules

1. Any borrow must last for a scope no greater than that of the owner.

2. Either one of the following, but not both at the same time:

	* One or more references (`&T`) to the resource
	* ==Exactly one== mutable reference (`&mut T`) to the resource

---

### References And Borrowing

#### The Rules

1. Any borrow must last for a scope no greater than that of the owner.

2. Either one of the following, but not both at the same time:

	* One or more references (`&T`) to the resource
	* ==Exactly one== mutable reference (`&mut T`) to the resource

* Shared and mutable state is the root of all :smiling_imp:

---

### References And Borrowing

#### The Rules

1. Any borrow must last for a scope no greater than that of the owner.

2. Either one of the following, but not both at the same time:

	* **One or more references (`&T`) to the resource**
	* ==Exactly one== mutable reference (`&mut T`) to the resource

* Shared and ~~mutable~~ state is the root of all :smiling_imp:

---

### References And Borrowing

#### The Rules

1. Any borrow must last for a scope no greater than that of the owner.

2. Either one of the following, but not both at the same time:

	* One or more references (`&T`) to the resource
	* **==Exactly one== mutable reference (`&mut T`) to the resource**

* ~~Shared~~ and mutable state is the root of all :smiling_imp:

---
### References And Borrowing

#### Why so serious, compiler?

* Iterator Invalidation

* Use after free

---

### References And Borrowing

#### Why so serious, compiler?

* Iterator Invalidation ex. 1

```
let mut v = vec![1, 2, 3];

for i in &v {
    println!("{}", i);
    v.push(34);
}
```

---

### References And Borrowing

#### Why so serious, compiler?

* Iterator Invalidation ex. 1

```
let mut v = vec![1, 2, 3];

for i in &v {
    println!("{}", i);
    v.push(34);
}
```
```
error: cannot borrow `v` as mutable because it is also
borrowed as immutable
    v.push(34);
    ^
```

---

### References And Borrowing

#### Why so serious, compiler?

* Iterator Invalidation ex. 2

```
// Copy elements to another vector
fn append_vec(from: &Vec<i32>, to: mut &Vec<i32>) {
    for elem in from.iter() {
        to.push(*elem);
    }
}
```

---

### References And Borrowing

#### Why so serious, compiler?

* Iterator Invalidation ex. 2

```
// What if from and to are the same?
fn append_vec(from: &Vec<i32>, to: mut &Vec<i32>) {
    for elem in from.iter() {
        to.push(*elem);
    }
}
```

---

### References And Borrowing

#### Why so serious, compiler?

* Iterator Invalidation ex. 2

```
// Dangling pointer!!!!
fn append_vec(from: &Vec<i32>, to: mut &Vec<i32>) {
    for elem in from.iter() {
        to.push(*elem);
    }
}
```

---

### References And Borrowing

#### Why so serious, compiler?

* Iterator Invalidation ex. 2

```
// Dangling pointer!!!!
fn append_vec(from: &Vec<i32>, to: mut &Vec<i32>) {
    for elem in from.iter() {
        to.push(*elem);
    }
}
```
```
error: cannot borrow `new_v` as mutable because it is also
borrowed as immutable
// Whew~
```

---

### References And Borrowing

#### Why so serious, compiler?

* Use after free

```
fn main() {
    let y: &i32;
    {
        let x = 5;
        y = &x; // borrow a reference from x
    } // Oh no! x died! what is y now!?
    println!("{}", y);
}
```

---

### References And Borrowing

#### Why so serious, compiler?

* Use after free

```
fn main() {
    let y: &i32;
    {
        let x = 5;
        y = &x; // borrow a reference from x
    } // Oh no! x died! what is y now!?
    println!("{}", y);
}
```
```
error: `x` does not live long enough
    y = &x;
         ^
```

---

### But What If I Need Both?

---

### Concurrency in Rust

* Shared nothing (`channel`)

* Shared Immutable Memory (`Arc<T>`)

* Mutation with Synchronization (`Arc<Mutex<T>>`)

---

### Conclusions

* Rust combines high-level features with low-level control

* Rust gives strong safety guarantees beyond what GC can offer:

    * Deterministic destruction

	* Data race freedom

	* Iterator Invalidation