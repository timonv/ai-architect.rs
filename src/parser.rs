use pest::iterators::Pairs;
use pest::Parser;
use std::fmt;

#[derive(Parser)]
#[grammar = "grammar_inc.pest"]
struct D3LParser;

// enum Grammar {
//     Package,
// }

#[derive(Debug)]
enum Error {
    RuleError(String),
    MissingRootPackage(String),
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Error::RuleError(e.to_string())
    }
}

impl<'i> TryFrom<Pairs<'i, Rule>> for Package {
    type Error = &'static str;

    fn try_from(pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let mut maybe_package: Option<Package> = None;
        let mut maybe_version: Option<Version> = None;

        dbg!(&pairs);
        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier => {
                    maybe_package = Some(Package {
                        name: pair.as_str().into(),
                        version: None,
                    })
                }
                Rule::version => {
                    let versions: Vec<Option<u8>> = pair
                        .into_inner()
                        .as_str()
                        .split(".")
                        .map(|n| Some(n.parse().unwrap_or(0)))
                        .collect();

                    maybe_version = Some(Version {
                        major: versions[0].unwrap_or(0),
                        minor: versions[1].unwrap_or(0),
                        patch: versions[2].unwrap_or(0),
                    })
                }
                _ => return Err("Invalid rule in root package"),
            }
        }

        match maybe_package {
            Some(mut package) => {
                package.version = maybe_version;
                Ok(package)
            }
            None => Err("No package found"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RuleError(ref s) => write!(f, "RuleError: {}", s),
            Error::MissingRootPackage(ref s) => write!(f, "MissingRootPackage: {}", s),
        }
    }
}

#[derive(Debug)]
struct Package {
    name: String,
    version: Option<Version>,
}

#[derive(Debug, PartialEq)]
struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

fn parse(input: &str) -> Result<Package, Error> {
    let mut pairs = D3LParser::parse(Rule::D3L, input)?;
    let root_package: Package = match pairs.next() {
        Some(pair) => pair
            .into_inner()
            .try_into()
            .map_err(|e: &'static str| Error::MissingRootPackage(e.to_string()))?,
        None => {
            return Err(Error::MissingRootPackage(
                "No pairs found to convert into package".to_string(),
            ))
        }
    };
    Ok(root_package)
    //
    // match pairs.next() {
    //     Some(pair) =>
    //     None => Err(Error::RuleError("No pairs found".into())),
    // }
    //
    // for pair in pairs {
    //     println!("Rule:    {:?}", pair.as_rule());
    //     println!("Span:    {:?}", pair.as_span());
    //     println!("Text:    {}", pair.as_str());
    //
    //     for inner_pair in pair.into_inner() {
    //         match inner_pair.as_rule() {
    //             Rule::package => {
    //                 root_package = k(Package {
    //                     name: pair.into_inner().as_str().into(),
    //                     version: None,
    //                 })
    //             }
    //             Rule::version => {
    //                 root_package.version = Version(
    //                     inner_pair.into_inner().as_str().parse().unwrap(),
    //                     inner_pair.into_inner().as_str().parse().unwrap(),
    //                     inner_pair.into_inner().as_str().parse().unwrap(),
    //                 )
    //             }
    //             _ => unreachable!("{}", inner_pair.as_str()),
    //         }
    //     }
    // }
    //
    // return Ok(root_package);
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
        assert_eq!(
            result.version.unwrap(),
            Version {
                major: 1,
                minor: 0,
                patch: 0
            }
        );
    }
}
