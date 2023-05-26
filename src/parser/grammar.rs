use super::{Pairs, Rule};

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: Option<Version>,
}

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
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
                        .split('.')
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
