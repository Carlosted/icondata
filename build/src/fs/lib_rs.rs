use std::path::PathBuf;

use anyhow::Result;
use askama::Template;

use crate::{dirs::LibType, Packages, icon::svg::ParsedSvg};

#[derive(Debug)]
pub struct LibRs {
    pub path: PathBuf,
}

impl LibRs {
    pub fn contents(lib_type: &LibType) -> Result<String> {
        match lib_type {
            LibType::IconLib(pkg) => {
                #[derive(Template)]
                #[template(path = "icon_lib/lib.rs", escape = "none")]
                struct Template<'a> {
                    features_svgs: Vec<(&'a str, &'a ParsedSvg)>,
                    short_name: String,
                }

                let icons = pkg.icons();
                let features_svgs = icons.iter().map(|icon| (icon.feature.name.as_ref(), &icon.svg)).collect::<Vec<_>>();
                let short_name = pkg.meta.short_name.to_string();

                Ok(Template { features_svgs, short_name }.render()?)
            }
            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/lib.rs", escape = "none")]
                struct Template<'a> {
                    short_names: Vec<&'a str>,
                }

                let short_names = Packages::get()?
                    .iter()
                    .map(|package| package.meta.short_name.as_ref())
                    .collect::<Vec<_>>();

                Ok(Template { short_names }.render()?)
            }
            LibType::IconIndex => {
                #[derive(Template)]
                #[template(path = "icon_index/lib.rs", escape = "none")]
                struct Template<'a> {
                    short_names: Vec<&'a str>,
                }

                let short_names = Packages::get()?
                    .iter()
                    .map(|package| package.meta.short_name.as_ref())
                    .collect::<Vec<_>>();

                Ok(Template { short_names }.render()?)
            }

            LibType::Boilerplate => unreachable!("Boilerplate does not have a lib.rs file"),
        }
    }
}

mod filters {
    use heck::ToShoutySnakeCase;
    use xml::attribute::OwnedAttribute;
    pub fn shouty_snake_case<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let input = s.to_string();
        Ok(input.to_shouty_snake_case())
    }

    pub fn attribute_value(opt: &Option<OwnedAttribute>) -> ::askama::Result<String> {
        Ok(format!("{:?}", opt.as_ref().map(|attr| &attr.value)))
    }
}
