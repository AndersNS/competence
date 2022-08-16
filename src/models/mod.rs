use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TreeId {
    pub id: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Discipline {
    pub id: usize,
    pub name: String,
    pub paths: Vec<Path>,
}

impl Discipline {
    pub fn update_rating(
        &mut self,
        update: Rating,
        path_id: usize,
        area_id: usize,
        comp_id: usize,
    ) {
        let path = self.paths.iter_mut().find(|p| p.id == path_id).unwrap();
        path.update_rating(update, area_id, comp_id);
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Path {
    pub id: usize,
    pub name: String,
    pub areas: Vec<Area>,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Rating {
    Interest(i32),
    Competency(i32),
}

impl Rating {
    pub fn is_interest(&self) -> bool {
        match self {
            Rating::Interest(_) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

impl Path {
    pub fn update_rating(&mut self, update: Rating, area_id: usize, comp_id: usize) {
        let area = self.areas.iter_mut().find(|p| p.id == area_id).unwrap();
        area.update_rating(update, comp_id);
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Area {
    pub id: usize,
    pub name: String,
    pub competencies: Vec<Competency>,
}

impl Area {
    pub fn update_rating(&mut self, update: Rating, comp_id: usize) {
        let comp = self
            .competencies
            .iter_mut()
            .find(|p| p.id == comp_id)
            .unwrap();
        comp.update_rating(update);
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Competency {
    pub id: usize,
    pub name: String,
    pub interest: i32,
    pub competency: i32,
}

impl Competency {
    pub fn update_rating(&mut self, update: Rating) {
        match update {
            Rating::Interest(interest) => {
                self.interest = interest;
            }

            Rating::Competency(competency) => {
                self.competency = competency;
            }
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct CompetencyRating {
    pub discipline_id: usize,
    pub path_id: usize,
    pub area_id: usize,
    pub comp_id: usize,
    pub rating: Rating,
}

impl CompetencyRating {
    pub fn new(rating: Rating, competency_id: usize) -> Self {
        Self {
            discipline_id: 0,
            path_id: 0,
            area_id: 0,
            comp_id: competency_id,
            rating,
        }
    }
}
