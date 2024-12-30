pub struct UnsafeSend<T> (pub T);
unsafe impl<T> Send for UnsafeSend<T> { }
impl<T> std::ops::Deref for UnsafeSend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}