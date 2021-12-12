use command::Command;

pub struct Client<T> {
    inner: T,
    scheme: String,
}

pub trait Call {
    fn call(&self, command: &Command);
}

#[cfg(unix)]
struct UnixClient {
    socket_fd: String,
}

#[cfg(windows)]
struct WindowsClient {

}

const FILE_PATH: &str = "/tmp/wasmship.sock";

impl<T> Client<T> {

    pub fn new(client: T) -> Client<T> {
        Client {
            inner: client,
            scheme: "http".to_string(),
        }
    }

    pub fn init() -> Client<T>{
        #[cfg(unix)]
        let client = Self::new(UnixClient::new());
        #[cfg(windows)]
        let client = Self::new(WindowsClient::new());
        return client;
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Call for Client<T>
where
    T: Call
{
    fn call(&self, command: &Command) {
        self.inner.call(command);
    }
}

#[cfg(unix)]
impl Call for UnixClient {
    fn call(&self, command: &Command) {

    }
}

#[cfg(unix)]
impl UnixClient {
    fn new() -> UnixClient {
        UnixClient {
            socket_fd: FILE_PATH.to_string(),
        }
    }
}

#[cfg(windows)]
impl Call for WindowsClient {
    fn call(&self, command: &Command) {
        unimplemented!("support unix first.")
    }
}

#[cfg(windows)]
impl WindowsClient {
    fn new() -> WindowsClient {
        WindowsClient {}
    }
}