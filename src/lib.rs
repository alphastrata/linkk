//! This crate is real simple, if makes a set of channels and criss-crosses them.
//!
//! I find myself using this pattern a bit lately to get... things that're hard to get talking to
//! eachother talking to eachother.
//! There is probably a better way -- but, I don't know it.
//!
//! Conceptually, I think of it as making a bridge, it needn't send the same `<T>` across,
//! infact you can put all sorts of things in there.. I know, I have.
//!
//!
//!```rust
//!use linkk;
//!linkk::setup_linkk!(pub, Window2os<u32>, Os2Window<u64>);
//!
//!let (w2os, os2w) = make_new_linkk();
//!// link2 receives from link1
//!w2os.send(42).unwrap();
//!assert_eq!(os2w.recv().unwrap(), 42u32);
//!
//!// link1 receives from link2
//!os2w.tx.send(43 as u64).unwrap();
//!assert_eq!(w2os.rx.recv().unwrap(), 43);
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
macro_rules! setup_linkk {
    ($v:vis, $struct1:ident<$t:ty>, $struct2:ident<$t2:ty>) => {
        //
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

            pub fn new(tx: std::sync::mpsc::Sender<$t>, rx: std::sync::mpsc::Receiver<$t2>)-> Self {
                Self {
                    tx,
                    rx
                }
           }
        }

        impl $struct2 {
            $v fn send(&self, t: $t2) -> std::result::Result<(), std::sync::mpsc::SendError<$t2>> {
                self.tx.send(t)
            }

            $v fn recv(&self) -> Result<$t, std::sync::mpsc::RecvError> {
                self.rx.recv()
            }

            pub fn new(tx: std::sync::mpsc::Sender<$t2>, rx: std::sync::mpsc::Receiver<$t>)-> Self {
                Self {
                    tx,
                    rx
                }
           }

        }

        pub fn make_new_linkk() -> ($struct1, $struct2) {
            let (tx1, rx1) = std::sync::mpsc::channel::<$t>();
            let (tx2, rx2) = std::sync::mpsc::channel::<$t2>();

            ($struct1 { tx: tx1, rx: rx2 }, $struct2 { tx: tx2, rx: rx1 })
        }
        //
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_macro() {
        setup_linkk!(pub, Window2os<u32>, Os2Window<u64>);

        let (link2, link1) = make_new_linkk();

        // link2 receives from link1
        link2.send(42).unwrap();
        assert_eq!(link1.recv().unwrap(), 42u32);

        // link1 receives from link2
        link1.tx.send(43 as u64).unwrap();
        assert_eq!(link2.rx.recv().unwrap(), 43);
    }
}
