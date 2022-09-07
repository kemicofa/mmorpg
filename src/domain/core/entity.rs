pub trait Entity {
    fn update(&self) {}
    fn id(&self) -> &str;
}