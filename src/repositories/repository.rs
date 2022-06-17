pub trait Repository {
    fn load<S>(src: &mut S) -> Result<(), std::io::Error>
    where
        S: std::io::Read;
}
