use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use crate::io::{AsyncRead, Interest, PollEvented, ReadBuf, Ready};

cfg_net! {
    pub struct MachPortRecv {
        io: PollEvented<mio::net::MachPortRecvRight>,
    }
}

impl MachPortRecv {
    /// Waits for any of the requested ready states.
    ///
    /// This function is usually paired with `try_read()` or `try_write()`. It
    /// can be used to concurrently read / write to the same socket on a single
    /// task without splitting the socket.
    ///
    /// The function may complete without the socket being ready. This is a
    /// false-positive and attempting an operation will return with
    /// `io::ErrorKind::WouldBlock`. The function can also return with an empty
    /// [`Ready`] set, so you should always check the returned value and possibly
    /// wait again if the requested states are not set.
    pub async fn ready(&self, interest: Interest) -> io::Result<Ready> {
        let event = self.io.registration().readiness(interest).await?;
        Ok(event.ready)
    }

    pub fn try_read(&self, buf: &mut [u8]) -> io::Result<usize> {
        use std::io::Read;

        self.io
            .registration()
            .try_io(Interest::MACH_PORT, || (&*self.io).read(buf))
    }

    pub(crate) fn poll_read_priv(
        &self,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // Safety: `TcpStream::read` correctly handles reads into uninitialized memory
        unsafe { self.io.poll_read(cx, buf) }
    }
}

impl AsyncRead for MachPortRecv {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.poll_read_priv(cx, buf)
    }
}
