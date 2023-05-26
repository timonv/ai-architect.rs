use pest::iterators::Pairs;
use pest::Parser;

mod errors;
use errors::ParseError;

mod grammar;
use grammar::{Entity, Package, Scope};

#[derive(Parser)]
#[grammar = "grammar_inc.pest"]
struct D3LParser;

fn parse(input: &str) -> Result<Package, ParseError> {
    let pairs = D3LParser::parse(Rule::D3L, input)?;

    // TODO: Only supports one package, why not more
    dbg!(&pairs);
    for pair in pairs {
        match pair.as_rule() {
            Rule::package => {
                dbg!(&pair);
                let inner_pairs = pair.into_inner();
                // NOTE: Cloning here to reset the iterator

                let mut root_package: Package = inner_pairs.clone().into();
                // TODO: This is a bit of a mess, need to clean this up

                // Maybe we can use a recursive function to parse the inner pairs
                for pair in inner_pairs {
                    match pair.as_rule() {
                        Rule::entities => {
                            for entity in pair.into_inner() {
                                match entity.as_rule() {
                                    Rule::entity => {
                                        let inner_values = entity.into_inner();
                                        let mut entity: Entity = inner_values.clone().into();

                                        for inner_value in inner_values {
                                            match inner_value.as_rule() {
                                                Rule::methods => {
                                                    for method in inner_value.into_inner() {
                                                        match method.as_rule() {
                                                            Rule::method => {
                                                                entity.methods.push(
                                                                    method.into_inner().into(),
                                                                );
                                                            }
                                                            _ => {
                                                                unreachable!("Unknown method rule")
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => (),
                                            }
                                        }
                                        root_package.entities.push(entity);
                                    }
                                    _ => unreachable!("Unkown entity rule"),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                return Ok(root_package);
            }
            _ => unreachable!(),
        }
    }
    return Err(ParseError::MissingRootPackage(
        "Root package not found".to_string(),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_root_package() {
        let example = "Test";
        let result = parse(example);
        assert!(result.is_err())
    }

    #[test]
    fn test_parse_package() {
        let example = "package Test";
        let result = parse(example).unwrap_or_else(|e| panic!("{}", e.to_string()));
        assert_eq!(&result.name, "Test")
    }

    #[test]
    fn test_parse_version() {
        let example = "package Test version 1.0.0";
        let result = parse(example).unwrap_or_else(|e| panic!("{}", e.to_string()));
        assert_eq!(&result.name, "Test");
        assert!(&result.version.is_some());
        let version = result.version.unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn test_parse_entities() {
        let example = "package Test version 1.0.0 {
                entity TestEntity
                }";
        let result = parse(example).unwrap_or_else(|e| panic!("{}", e.to_string()));
        dbg!(&result);
        assert_eq!(&result.name, "Test");
        let entities = result.entities;
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].name, "TestEntity");

        let example = "package Test version 1.0.0 {
                public entity TestEntity
                }";
        let result = parse(example).unwrap_or_else(|e| panic!("{}", e.to_string()));
        dbg!(&result);
        assert_eq!(&result.name, "Test");
        let entities = result.entities;
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].name, "TestEntity");
        assert_eq!(entities[0].scope, Scope::Public);
    }

    #[test]
    fn test_entity_with_attributes_scope_and_method() {
        let example = "package Test version 1.0.0 {
                    public entity TestEntity {
                        (
                            name: string
                        )
                        method get_name()
                    }
                }";
        let result = parse(example).unwrap_or_else(|e| panic!("{}", e.to_string()));
        dbg!(&result);
        assert_eq!(&result.name, "Test");
        let entity = result.entities.first().unwrap();
        assert_eq!(entity.scope, Scope::Public);

        let attribute = entity.attributes.first().unwrap();
        assert_eq!(attribute.name, "name");
        assert_eq!(attribute.atype, "string");
        let method = entity.methods.first().unwrap();
        assert_eq!(method.name, "get_name");
    }
}
