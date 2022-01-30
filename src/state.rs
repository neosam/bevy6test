#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum State {
    Loading,
    PostLoading,
    Ingame,
}