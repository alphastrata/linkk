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
/// // you can do this:
/// link!(MyType<u32>, MyType2<u64>);
///
/// let value = 42u32;
/// ///etc...
/// ```
#[macro_export]
macro_rules! link {
    ($vis:expr, $struct1:ident<$t:ident>, $vis2:expr,$struct2:ident<$t2:ident>) => {

        $vis struct $struct1<$t, $t2> {
            tx: std::sync::mpsc::Sender<$t>,
            rx: std::sync::mpsc::Receiver<$t2>,
        }
        $vis2 struct $struct2<$t, $t2> {
            tx: std::sync::mpsc::Sender<$t2>,
            rx: std::sync::mpsc::Receiver<$t>,
        }
        impl<$t> $struct1<$t> {
            pub fn send(&self, t: $t) -> std::result::Result<(), $crate::LinkError> {
                self.tx.send(t).map_err($crate::LinkError::from)
            }

            pub fn recv(&self) -> std::result::Result<$t2, $crate::LinkError> {
                self.rx.recv().map_err($crate::LinkError::from)
            }
        }

        impl<$t2> $struct2<$t2> {
            pub fn send(&self, t: $t2) -> std::result::Result<(), $crate::LinkError> {
                self.tx.send(t).map_err($crate::LinkError::from)
            }

            pub fn recv(&self) -> std::result::Result<$t, $crate::LinkError> {
                self.rx.recv().map_err($crate::LinkError::from)
            }
        }

    (pub enum $error:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(thiserror::Error, Debug)]
        #[allow(dead_code)]
        #[allow(non_camel_case_types)]
        #[repr(u8)]
        $error pub enum $error {
            $(
                #[error("{0}")]
                $variant(#[from] std::sync::mpsc::RecvError),
            )+
        }
    };
    // #[cfg(feature = "errors")]
    // {
    //     #[derive(Debug, thiserror::Error)]
    //     pub enum LinkError {
    //         #[error("Failed to send data over channel")]
    //         SendError(#[from] std::sync::mpsc::SendError<$t>),
    //         #[error("Failed to receive data over channel")]
    //         RecvError(#[from] std::sync::mpsc::RecvError),
    //     }
    //   }

    $vis fn init() -> ($t1, $t2){
            let (tx1, rx2) = std::sync::mpsc::channel();
            let (tx2, rx1) = std::sync::mpsc::channel();
            (
                $struct1 { tx: tx1, rx: rx1 },
                $struct2 { tx: tx2, rx: rx2 },
            )
        }
        //
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_macro() {
        let value = 42u32;

        let (link1, link2) = link!(MyType<u32>, MyType2<u64>);
        link1.send(42).unwrap();
        let result = link2.recv().unwrap();
        assert_eq!(result, 42u64);

        link1.tx.send(value).unwrap();
        assert_eq!(link2.rx.recv().unwrap(), value as u64);
    }
}
