use pest::iterators::Pairs;
use pest::Parser;

mod errors;
use errors::ParseError;

mod grammar;
use grammar::Package;

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
                return Ok(pair.into_inner().into());
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
    use crate::parser::grammar::Scope;

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
