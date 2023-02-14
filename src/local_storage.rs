use std::borrow::Borrow;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};

use crate::models::*;

pub fn update_local_storage(comp_rating: &CompetencyRating, id_suffix: &str) {
    let storage_id = storage_id(id_suffix);
    let competencies = get_competencies_from_localstorage(id_suffix);
    match competencies {
        Ok(mut comps) => {
            let comp = comps.iter_mut().position(|x| {
                x.discipline_id == comp_rating.discipline_id
                    && x.path_id == comp_rating.path_id
                    && x.area_id == comp_rating.area_id
                    && x.comp_id == comp_rating.comp_id
                    && x.rating.is_interest() == comp_rating.rating.is_interest()
            });
            match comp {
                Some(c) => {
                    if comp_rating.rating == Rating::Interest(0)
                        || comp_rating.rating == Rating::Competency(0)
                    {
                        comps.remove(c);
                    } else {
                        comps[c].rating = comp_rating.rating;
                    }

                    LocalStorage::set(storage_id, comps).unwrap();
                }
                _ => {
                    comps.push(*comp_rating);
                    LocalStorage::set(storage_id, comps).unwrap();
                }
            }
        }
        _ => {
            LocalStorage::set(storage_id, vec![comp_rating]).unwrap();
        }
    }
}

pub fn update_discs_from_ratings(
    fetched_discs: &mut Vec<Discipline>,
    ratings: &Vec<CompetencyRating>,
) -> Result<(), String> {
    for ele in ratings.iter() {
        if let Some(index) = fetched_discs.iter().position(|c| c.id == ele.discipline_id) {
            let update_result = fetched_discs[index].update_rating(
                ele.rating,
                ele.path_id,
                ele.area_id,
                ele.comp_id,
            );
            if update_result.is_err() {
                return update_result;
            }
        }
    }
    Ok(())
}

pub fn update_from_local_storage(fetched_discs: &mut Vec<Discipline>, id_suffix: &str) -> usize {
    let competencies = get_competencies_from_localstorage(id_suffix);
    match competencies {
        Ok(ratings) => {
            let update_result = update_discs_from_ratings(fetched_discs, ratings.borrow());
            if update_result.is_err() {
                let storage_id = storage_id(id_suffix);
                LocalStorage::delete(storage_id);
            }
            return ratings.len();
        }
        _ => {
            return 0;
        }
    }
}

pub fn get_competencies_from_localstorage(
    id_suffix: &str,
) -> Result<Vec<CompetencyRating>, StorageError> {
    let competencies: Result<Vec<CompetencyRating>, StorageError> =
        LocalStorage::get(storage_id(id_suffix));
    return competencies;
}

fn storage_id(id_suffix: &str) -> String {
    format!("competencies{}", id_suffix)
}
