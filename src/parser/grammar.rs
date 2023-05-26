use super::{Pairs, Rule};

// TODO: Replace breaks with errors. Need to find a way to work around grammar pest side missing
// compile time types, looks like they're constants (Is that even true?). Perhaps guard on the pairs type, switch to try_from
// and return a ParseError::RuleError if the type is not what we expect.

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
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq)]
pub enum Scope {
    Public,
    Private,
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub atype: String,
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
                Rule::entity => package.entities.push(pair.into_inner().into()), // Early return we're done,
                _ => unreachable!("Unreachable code from Package#from"),
            }
        }

        package
    }
}

impl<'i> From<Pairs<'i, Rule>> for Entity {
    fn from(pairs: Pairs<'i, Rule>) -> Self {
        let mut entity = Entity {
            name: "".to_string(),
            methods: vec![],
            scope: Scope::Private,
            attributes: vec![],
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
                Rule::attribute => entity.attributes.push(pair.into_inner().into()),
                Rule::method => entity.methods.push(pair.into_inner().into()),
                _ => unreachable!("Unreachable code from Entity#from"),
            }
        }
        entity
    }
}

impl<'i> From<Pairs<'i, Rule>> for Method {
    fn from(pairs: Pairs<'i, Rule>) -> Self {
        let mut method = Method {
            name: "".to_string(),
            attributes: vec![],
        };
        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => method.name = pair.as_str().to_string(),
                Rule::parameters => (),
                _ => unreachable!("Unreachable code from Method#from"),
            }
        }
        method
    }
}

impl<'i> From<Pairs<'i, Rule>> for Attribute {
    fn from(pairs: Pairs<'i, Rule>) -> Self {
        let mut attribute = Attribute {
            name: "".to_string(),
            atype: "".to_string(),
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => attribute.name = pair.as_str().to_string(),
                Rule::atype => attribute.atype = pair.as_str().to_string(),
                _ => unreachable!("Unreachable code from Attribute#from"),
            }
        }
        attribute
    }
}
