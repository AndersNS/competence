use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Discipline {
    pub id: usize,
    pub name: String,
    pub paths: Vec<Path>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Path {
    pub id: usize,
    pub name: String,
    pub areas: Vec<Area>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Area {
    pub id: usize,
    pub name: String,
    pub competencies: Vec<Competency>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Competency {
    pub id: usize,
    pub name: String,
}
