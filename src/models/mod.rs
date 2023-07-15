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
    ) -> Result<(), String> {
        let path = self
            .paths
            .iter_mut()
            .find(|p| p.id == path_id)
            .ok_or("Could not find path".to_string())?;
        path.update_rating(update, area_id, comp_id)
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
        matches!(self, Rating::Interest(_))
    }
}

impl Path {
    pub fn update_rating(
        &mut self,
        update: Rating,
        area_id: usize,
        comp_id: usize,
    ) -> Result<(), String> {
        let area = self
            .areas
            .iter_mut()
            .find(|p| p.id == area_id)
            .ok_or("Could not find area".to_string())?;
        area.update_rating(update, comp_id)
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Area {
    pub id: usize,
    pub name: String,
    pub competencies: Vec<Competency>,
}

impl Area {
    pub fn update_rating(&mut self, update: Rating, comp_id: usize) -> Result<(), String> {
        let comp = self
            .competencies
            .iter_mut()
            .find(|p| p.id == comp_id)
            .ok_or("Could not find competency".to_string())?;
        comp.update_rating(update)
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
    pub fn update_rating(&mut self, update: Rating) -> Result<(), String> {
        match update {
            Rating::Interest(interest) => {
                self.interest = interest;
                Ok(())
            }

            Rating::Competency(competency) => {
                self.competency = competency;
                Ok(())
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
