use crate::models::Discipline;

pub fn export_tree(disc: &Discipline) -> String {
    let mut csv = "Discipline;Path;Area;Competency;Interest;Competency".to_string();

    for path in &disc.paths {
        for area in &path.areas {
            for comp in &area.competencies {
                csv.push_str(
                    format!(
                        "\n{};{};{};{};{};{}",
                        disc.name, path.name, area.name, comp.name, comp.interest, comp.competency
                    )
                    .as_str(),
                );
            }
        }
    }

    return csv;
}

#[cfg(test)]
mod test {
    use crate::models::{Area, Competency, Discipline, Path};

    use super::*;

    #[test]
    fn given_big_tree_then_good() {
        // arrange
        let disc = Discipline {
            id: 1,
            name: String::from("Utvikling"),
            paths: vec![
                Path {
                    id: 1,
                    name: String::from("Backend"),
                    areas: vec![Area {
                        id: 1,
                        name: "Språk".to_string(),
                        competencies: vec![
                            Competency {
                                id: 1,
                                name: String::from("C#"),
                                interest: 5,
                                competency: 3,
                            },
                            Competency {
                                id: 2,
                                name: String::from("Rust"),
                                interest: 5,
                                competency: 2,
                            },
                        ],
                    }],
                },
                Path {
                    id: 2,
                    name: String::from("Frontend"),
                    areas: vec![Area {
                        id: 1,
                        name: String::from("Rammeverk"),
                        competencies: vec![
                            Competency {
                                id: 1,
                                name: "vue.js".to_string(),
                                interest: 3,
                                competency: 3,
                            },
                            Competency {
                                id: 2,
                                name: "React.JS".to_string(),
                                interest: 3,
                                competency: 2,
                            },
                        ],
                    }],
                },
            ],
        };

        //act
        let csv_string = export_tree(&disc);

        assert_eq!(
            csv_string,
            "Discipline;Path;Area;Competency;Interest;Competency
Utvikling;Backend;Språk;C#;5;3
Utvikling;Backend;Språk;Rust;5;2
Utvikling;Frontend;Rammeverk;vue.js;3;3
Utvikling;Frontend;Rammeverk;React.JS;3;2"
        );
    }

    #[test]
    fn given_single_competency_then_export_should_return_single_line() {
        // arrange
        let disc = Discipline {
            id: 1,
            name: String::from("Utvikling"),
            paths: vec![Path {
                id: 1,
                name: String::from("Backend"),
                areas: vec![Area {
                    id: 1,
                    name: "Språk".to_string(),
                    competencies: vec![Competency {
                        id: 1,
                        name: String::from("C#"),
                        interest: 5,
                        competency: 3,
                    }],
                }],
            }],
        };

        //act
        let csv_string = export_tree(&disc);

        assert_eq!(
            csv_string,
            "Discipline;Path;Area;Competency;Interest;Competency
Utvikling;Backend;Språk;C#;5;3"
        );
    }

    #[test]
    fn given_empty_competency_then_export_should_return_only_header() {
        let disc = Discipline {
            id: 1,
            name: String::from("Utvikling"),
            paths: vec![Path {
                id: 1,
                name: String::from("Backend"),
                areas: vec![Area {
                    id: 1,
                    name: "Språk".to_string(),
                    competencies: vec![],
                }],
            }],
        };

        //act
        let csv_string = export_tree(&disc);

        assert_eq!(
            csv_string,
            "Discipline;Path;Area;Competency;Interest;Competency"
        );
    }
}
