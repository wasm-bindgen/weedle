use crate::attribute::ExtendedAttributeList;
use crate::common::{Default, Identifier};
use crate::types::Type;
use crate::Parse;

/// Parses dictionary members
pub type DictionaryMembers<'a> = Vec<DictionaryMember<'a>>;

/// Parses dictionary member per the WebIDL spec grammar:
///
/// ```text
/// DictionaryMember ::
///     ExtendedAttributeList DictionaryMemberRest
///
/// DictionaryMemberRest ::
///     required TypeWithExtendedAttributes identifier ;
///     Type identifier Default ;
///
/// TypeWithExtendedAttributes ::
///     ExtendedAttributeList Type
/// ```
///
/// This means:
/// - Required members: `[member-attrs]? required [type-attrs]? Type identifier ;`
/// - Optional members: `[member-attrs]? Type identifier Default? ;`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DictionaryMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub required: Option<crate::term::Required>,
    pub type_: Type<'a>,
    pub identifier: Identifier<'a>,
    pub default: Option<Default<'a>>,
    pub semi_colon: crate::term::SemiColon,
}

impl<'a> Parse<'a> for DictionaryMember<'a> {
    fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
        // First, try to parse optional extended attributes (member-level)
        let (input, attributes) = <Option<ExtendedAttributeList<'a>>>::parse(input)?;

        // Try to parse `required`
        let (input, required) = <Option<crate::term::Required>>::parse(input)?;

        if required.is_some() {
            // Per the spec, required members use TypeWithExtendedAttributes:
            //   required [type-attrs]? Type identifier ;
            // Try to parse type-level extended attributes after `required`.
            // If present, merge them into the member attributes for backward
            // compatibility with consumers that only look at `attributes`.
            let (input, type_attributes) = <Option<ExtendedAttributeList<'a>>>::parse(input)?;
            let (input, type_) = Type::parse(input)?;
            let (input, identifier) = Identifier::parse(input)?;
            let (input, semi_colon) = <crate::term::SemiColon>::parse(input)?;

            // Merge: if both member-level and type-level attributes are present,
            // prefer the type-level attributes (the spec-correct position).
            // In practice they shouldn't both be present on the same member.
            let merged_attributes = match (attributes, type_attributes) {
                (_, Some(ta)) => Some(ta),
                (ma, None) => ma,
            };

            Ok((
                input,
                DictionaryMember {
                    attributes: merged_attributes,
                    required,
                    type_,
                    identifier,
                    default: None,
                    semi_colon,
                },
            ))
        } else {
            // Optional member: [member-attrs]? Type identifier Default? ;
            let (input, type_) = Type::parse(input)?;
            let (input, identifier) = Identifier::parse(input)?;
            let (input, default) = <Option<Default<'a>>>::parse(input)?;
            let (input, semi_colon) = <crate::term::SemiColon>::parse(input)?;

            Ok((
                input,
                DictionaryMember {
                    attributes,
                    required,
                    type_,
                    identifier,
                    default,
                    semi_colon,
                },
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    test!(should_parse_dictionary_member { "required long num;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_some();
        identifier.0 == "num";
        default.is_none();
    });

    test!(should_parse_required_with_type_ext_attrs { "required [EnforceRange] unsigned long num;" =>
        "";
        DictionaryMember;
        attributes.is_some();
        required.is_some();
        identifier.0 == "num";
        default.is_none();
    });

    test!(should_parse_member_attrs_before_required { "[EnforceRange] required unsigned long num;" =>
        "";
        DictionaryMember;
        attributes.is_some();
        required.is_some();
        identifier.0 == "num";
        default.is_none();
    });

    test!(should_parse_optional_member { "long num;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_none();
        identifier.0 == "num";
        default.is_none();
    });

    test!(should_parse_optional_member_with_default { "long num = 5;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_none();
        identifier.0 == "num";
        default.is_some();
    });

    test!(should_parse_optional_member_with_attrs { "[Clamp] long num;" =>
        "";
        DictionaryMember;
        attributes.is_some();
        required.is_none();
        identifier.0 == "num";
        default.is_none();
    });
}
