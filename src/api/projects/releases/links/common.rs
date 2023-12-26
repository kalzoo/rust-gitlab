// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::ParamValue;

/// The type of the link: other, runbook, image, package.
///
/// Defaults to `Other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LinkType {
    /// Link to any other file
    Other,
    /// Link to a runbook file
    Runbook,
    /// Link to an image
    Image,
    /// Link to a package
    Package,
}

#[allow(clippy::derivable_impls)]
impl Default for LinkType {
    fn default() -> Self {
        // XXX(rust-1.62): use `#[default]`
        LinkType::Other
    }
}

impl LinkType {
    pub(in super::super) fn as_str(self) -> &'static str {
        match self {
            Self::Other => "other",
            Self::Runbook => "runbook",
            Self::Image => "image",
            Self::Package => "package",
        }
    }
}

impl ParamValue<'static> for LinkType {
    fn as_value(&self) -> std::borrow::Cow<'static, str> {
        self.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::releases::links::LinkType;

    #[test]
    fn link_type_as_str() {
        let items = &[
            (LinkType::Other, "other"),
            (LinkType::Runbook, "runbook"),
            (LinkType::Image, "image"),
            (LinkType::Package, "package"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn link_type_default() {
        assert_eq!(LinkType::default(), LinkType::Other);
    }
}
