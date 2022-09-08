#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameStates {
    LoadingAssets,
    GeneratingWorld,
    InGame,
}