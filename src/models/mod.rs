use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Discipline {
    pub id: usize,
    pub name: String,
    pub paths: Vec<Path>,
}

impl Discipline {
    pub fn set_interest(&mut self, interest: u32, path_id: usize, area_id: usize, comp_id: usize) {
        let path = self.paths.iter_mut().find(|p| p.id == path_id).unwrap(); // TODO Handle Option
        path.set_interest(interest, area_id, comp_id);
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Path {
    pub id: usize,
    pub name: String,
    pub areas: Vec<Area>,
}

impl Path {
    pub fn set_interest(&mut self, interest: u32, area_id: usize, comp_id: usize) {
        let area = self.areas.iter_mut().find(|p| p.id == area_id).unwrap(); // TODO Handle Option
        area.set_interest(interest, comp_id);
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Area {
    pub id: usize,
    pub name: String,
    pub competencies: Vec<Competency>,
}

impl Area {
    pub fn set_interest(&mut self, interest: u32, comp_id: usize) {
        let comp = self
            .competencies
            .iter_mut()
            .find(|p| p.id == comp_id)
            .unwrap(); // TODO handle unwrap
        comp.set_interest(interest);
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Competency {
    pub id: usize,
    pub name: String,
    pub interest: u32,
}

impl Competency {
    pub fn set_interest(&mut self, interest: u32) {
        if self.interest == interest {
            self.interest = 0;
        } else {
            self.interest = interest;
        }
    }
}
