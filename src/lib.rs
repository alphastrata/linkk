//! This crate is real simple, if makes a set of channels and criss-crosses them.
//!
//! I find myself using this pattern a bit lately to get... things that're hard to get talking to
//! eachother talking to eachother.
//! There is probably a better way -- but, I don't know it.
//!
//! Conceptually, I think of it as making a bridge, it needn't send the same `<T>` across,
//! infact you can put all sorts of things in there.. I know, I have.
//!
//!  ```rust
//!use link::link;
//!
//!let (link1, link2) = link!(pub, MyType<u32>, MyType2<u64>);
//!// link2 receives from link1
//!link1.send(42).unwrap();
//!assert_eq!(link2.recv().unwrap(), 42u32);
//!// link1 receives from link2
//!link2.tx.send(43 as u64).unwrap();
//!assert_eq!(link1.rx.recv().unwrap(), 43);
//! ```
//!
//! ## Which should save you typing that alernative which would be all this:
//!
//! ```
//!pub struct Link1 {
//!    tx: std::sync::mpsc::Sender<u32>,
//!    rx: std::sync::mpsc::Receiver<u64>,
//!}
//!pub struct Link2 {
//!    tx: std::sync::mpsc::Sender<u64>,
//!    rx: std::sync::mpsc::Receiver<u32>,
//!}
//!impl Link1 {
//!    fn send(&self, t: u32) -> std::result::Result<(), std::sync::mpsc::SendError<u32>> {
//!        self.tx.send(t)
//!    }
//!
//!    fn recv(&self) -> Result<u64, std::sync::mpsc::RecvError> {
//!        self.rx.recv()
//!    }
//!}
//!
//!impl Link2 {
//!    fn send(&self, t: u64) -> std::result::Result<(), std::sync::mpsc::SendError<u64>> {
//!        self.tx.send(t)
//!    }
//!
//!    fn recv(&self) -> Result<u32, std::sync::mpsc::RecvError> {
//!        self.rx.recv()
//!    }
//!}
//!fn init() {
//!    let (tx1, rx1) = std::sync::mpsc::channel::<u32>();
//!    let (tx2, rx2) = std::sync::mpsc::channel::<u64>();
//!
//!    let link1 = Link1 { tx: tx1, rx: rx2 };
//!    let link2 = Link2 { tx: tx2, rx: rx1 };
//!}
//!```
//! See the tests for example usage.

#[macro_export]
macro_rules! link {
    ($v:vis, $struct1:ident<$t:ty>, $struct2:ident<$t2:ty>) => {{
        #[derive(Debug, thiserror::Error)]
        $v enum LinkError {
            #[error("Failed to send data over channel")]
            SendError1(#[from] std::sync::mpsc::SendError<$t>),
            #[error("Failed to send data over channel")]
            SendError2(#[from] std::sync::mpsc::SendError<$t2>),
            #[error("Failed to receive data over channel")]
            RecvError(#[from] std::sync::mpsc::RecvError),
        }

        $v struct $struct1 {
            $v tx: std::sync::mpsc::Sender<$t>,
            $v rx: std::sync::mpsc::Receiver<$t2>,
        }

        $v struct $struct2 {
            $v tx: std::sync::mpsc::Sender<$t2>,
            $v rx: std::sync::mpsc::Receiver<$t>,
        }

        impl $struct1 {
            $v fn send(&self, t: $t) -> std::result::Result<(), std::sync::mpsc::SendError<$t>> {
                self.tx.send(t)
            }

            $v fn recv(&self) -> Result<$t2, std::sync::mpsc::RecvError> {
                self.rx.recv()
            }
        }

        impl $struct2 {
            $v fn send(&self, t: $t2) -> std::result::Result<(), std::sync::mpsc::SendError<$t2>> {
                self.tx.send(t)
            }

            $v fn recv(&self) -> Result<$t, std::sync::mpsc::RecvError> {
                self.rx.recv()
            }
        }

        let (tx1, rx1) = std::sync::mpsc::channel::<$t>();
        let (tx2, rx2) = std::sync::mpsc::channel::<$t2>();

        ($struct1 { tx: tx1, rx: rx2 }, $struct2 { tx: tx2, rx: rx1 })

    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_macro() {
        let (link1, link2) = link!(pub, MyType<u32>, MyType2<u64>);

        // link2 receives from link1
        link1.send(42).unwrap();
        assert_eq!(link2.recv().unwrap(), 42u32);

        // link1 receives from link2
        link2.tx.send(43 as u64).unwrap();
        assert_eq!(link1.rx.recv().unwrap(), 43);
    }

    #[test]
    fn common_fruit() {
        use std::collections::HashSet;
        let (link1, link2) = link!(pub, Link1<Vec<String>>, Link2<HashSet<i32>>);

        let fruits = vec!["apple", "banana", "orange"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let mut hashset = HashSet::new();
        hashset.insert(456789);

        // link2 receives from link1
        link1.send(fruits).unwrap();
        assert!(link2.recv().unwrap().iter().any(|x| x == "banana"));

        // link1 receives from link2
        link2.send(hashset).unwrap();
        assert!(link1.recv().unwrap().get(&456789).is_some());
    }
}
