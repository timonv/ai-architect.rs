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

    // TODO Only supports one package, why not more
    dbg!(&pairs);
    for pair in pairs {
        match pair.as_rule() {
            Rule::package => {
                let inner_pairs = pair.into_inner();
                // NOTE Cloning here to reset the iterator

                let mut root_package = Package::try_from(inner_pairs.clone())
                    .map_err(|e: &'static str| ParseError::MissingRootPackage(e.to_string()))?;

                for pair in inner_pairs {
                    match pair.as_rule() {
                        Rule::entities => {
                            for entity in pair.into_inner() {
                                match entity.as_rule() {
                                    Rule::entity => {
                                        root_package
                                            .entities
                                            .push(entity.into_inner().as_str().into());
                                    }
                                    _ => unreachable!(),
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
    }
}
