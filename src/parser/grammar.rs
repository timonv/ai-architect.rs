use super::{Pairs, Rule};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: Option<Version>,
    pub entities: Vec<Entity>,
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub methods: Vec<Method>,
    pub scope: Scope,
}

#[derive(Debug, PartialEq)]
pub enum Scope {
    Public,
    Private,
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
}

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl<'i> From<Pairs<'i, Rule>> for Package {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut package = Package {
            name: "".into(),
            version: None,
            entities: vec![],
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => {
                    package = Package {
                        name: pair.as_str().into(),
                        version: None,
                        entities: vec![],
                    }
                }
                Rule::version => {
                    let versions: Vec<Option<u8>> = pair
                        .into_inner()
                        .as_str()
                        .split('.')
                        .map(|n| Some(n.parse().unwrap_or(0)))
                        .collect();

                    package.version = Some(Version {
                        major: versions[0].unwrap_or(0),
                        minor: versions[1].unwrap_or(0),
                        patch: versions[2].unwrap_or(0),
                    })
                }
                _ => break, // Early return we're done,
            }
        }

        return package;
    }
}

impl<'i> From<Pairs<'i, Rule>> for Entity {
    fn from(pairs: Pairs<'i, Rule>) -> Self {
        let mut entity = Entity {
            name: "".to_string(),
            methods: vec![],
            scope: Scope::Private,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => {
                    entity.name = pair.as_str().to_string();
                }
                Rule::scope => {
                    entity.scope = match pair.as_str() {
                        "public" => Scope::Public,
                        _ => Scope::Private,
                    }
                }
                _ => break,
            }
        }
        return entity;
    }
}

impl<'i> From<Pairs<'i, Rule>> for Method {
    fn from(pairs: Pairs<'i, Rule>) -> Self {
        let mut method = Method {
            name: "".to_string(),
        };
        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => method.name = pair.as_str().to_string(),
                _ => break,
            }
        }
        return method;
    }
}
