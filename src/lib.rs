/// A macro that creates two structs containing pairs of channels with crisscrossed types.
///
/// This macro takes two type parameters, `$struct1` and `$struct2`, and creates two structs with
/// the specified types. Each struct contains a pair of channels with the specified type
/// parameters. The channels are crisscrossed, so `tx` of `$struct1` sends data to `rx` of
/// `$struct2`, and `tx` of `$struct2` sends data to `rx` of `$struct1`.
///
/// # Examples
/// ```ignore
/// // Instead of:
/// # use std::sync::mpsc::{Sender, Receiver};
/// # struct MyType<T,U> {
/// #     tx1: Sender<T>,
/// #     rx2: Receiver<U>,
/// # }
/// # struct MyType2<T,U> {
/// #     tx2: Sender<U>,
/// #     rx1: Receiver<T>,
/// # }
/// // and so on...
///
/// // you can do this:
/// let (mytype1, mytype2) = link!(MyType<u32>, MyType2<u64>);
///
///
/// ///etc...
/// ```
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

        // pub fn init() -> ($t, $t2) {
        let (tx1, rx2) = std::sync::mpsc::channel::<$t>();
        let (tx2, rx1) = std::sync::mpsc::channel::<$t2>();

        ($struct1 { tx: tx1, rx: rx1 }, $struct2 { tx: tx2, rx: rx2 })
        // }
        //
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_macro() {
        let value = 42u32;

        let (link1, link2) = link!(pub, MyType<u32>, MyType2<u64>);

        link1.send(42).unwrap();
        let result = link2.recv().unwrap();
        assert_eq!(result, 42u32);

        link1.tx.send(value).unwrap();
        assert_eq!(link2.rx.recv().unwrap(), value);
    }
}
