# linkk

## About:

This crate provides a simple way to make a set of channels and criss-cross them. This pattern is useful for getting things that are _hard_ to get talking to each other to communicate.

Conceptually, it can be thought of as making a bridge, and it can be used to send any type of data across the channels.

There's almost certainly a nicer way of doing this... but, I dunno what that is.

## Installation

> Add this to your Cargo.toml:

```toml
[dependencies]
linkk = "0.1.2"
```

## Usage:

Here is an example of how to use this crate:

```rust
use linkk::*;

setup_linkk!(pub, Window2os<u32>, Os2Window<u64>);

    let (link2, link1) = make_new_linkk();

    // link2 receives from link1
    link2.send(42).unwrap();
    assert_eq!(link1.recv().unwrap(), 42u32);

    // link1 receives from link2
    link1.tx.send(43 as u64).unwrap();
    assert_eq!(link2.rx.recv().unwrap(), 43);
```
