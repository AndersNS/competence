use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Discipline {
    pub id: usize,
    pub name: String,
    pub paths: Vec<Path>,
}

impl Discipline {
    pub fn update_rating(
        &mut self,
        update: RatingUpdate,
        path_id: usize,
        area_id: usize,
        comp_id: usize,
    ) {
        let path = self.paths.iter_mut().find(|p| p.id == path_id).unwrap(); // TODO Handle Option
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
pub enum RatingUpdate {
    Interest(i32),
    Competency(i32),
}

impl Path {
    pub fn update_rating(&mut self, update: RatingUpdate, area_id: usize, comp_id: usize) {
        let area = self.areas.iter_mut().find(|p| p.id == area_id).unwrap(); // TODO Handle Option
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
    pub fn update_rating(&mut self, update: RatingUpdate, comp_id: usize) {
        let comp = self
            .competencies
            .iter_mut()
            .find(|p| p.id == comp_id)
            .unwrap(); // TODO handle unwrap
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
    pub fn update_rating(&mut self, update: RatingUpdate) {
        match update {
            RatingUpdate::Interest(interest) => {
                if self.interest == interest {
                    self.interest = 0;
                } else {
                    self.interest = interest;
                }
            }

            RatingUpdate::Competency(competency) => {
                if self.competency == competency {
                    self.competency = 0;
                } else {
                    self.competency = competency;
                }
            }
        }
    }
}
