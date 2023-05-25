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
    let mut pairs = D3LParser::parse(Rule::D3L, input)?;
    let root_package: Package = match pairs.next() {
        Some(pair) => pair
            .into_inner()
            .try_into()
            .map_err(|e: &'static str| ParseError::MissingRootPackage(e.to_string()))?,
        None => {
            return Err(ParseError::MissingRootPackage(
                "No pairs found to convert into package".to_string(),
            ))
        }
    };
    Ok(root_package)
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
        let result = parse(example).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(&result.name, "Test")
    }

    #[test]
    fn test_parse_version() {
        let example = "package Test version 1.0.0";
        let result = parse(example).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(&result.name, "Test");
        assert!(&result.version.is_some());
        let version = result.version.unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }
}
