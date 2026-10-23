use crate::argument::ArgumentList;
use crate::common::{Bracketed, Identifier, Parenthesized, Punctuated};
use crate::literal::{FloatLit, IntegerLit, StringLit};

/// Parses a list of attributes. Ex: `[ attribute1, attribute2 ]`
pub type ExtendedAttributeList<'a> = Bracketed<Punctuated<ExtendedAttribute<'a>, term!(,)>>;

/// Matches comma separated identifier list
pub type IdentifierList<'a> = Punctuated<Identifier<'a>, term!(,)>;

/// Matches comma separated integer list
pub type IntegerList<'a> = Punctuated<IntegerLit<'a>, term!(,)>;

ast_types! {
    /// Parses on of the forms of attribute
    enum ExtendedAttribute<'a> {
        /// Parses an argument list. Ex: `Constructor((double x, double y))`
        ///
        /// (( )) means ( ) chars
        ArgList(struct ExtendedAttributeArgList<'a> {
            identifier: Identifier<'a>,
            args: Parenthesized<ArgumentList<'a>>,
        }),
        /// Parses a named argument list. Ex: `NamedConstructor=Image((DOMString src))`
        ///
        /// (( )) means ( ) chars
        NamedArgList(struct ExtendedAttributeNamedArgList<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs_identifier: Identifier<'a>,
            args: Parenthesized<ArgumentList<'a>>,

        }),
        /// Parses an identifier list. Ex: `Exposed=((Window,Worker))`
        ///
        /// (( )) means ( ) chars
        IdentList(struct ExtendedAttributeIdentList<'a> {
            identifier: Identifier<'a>,
            assign: term!(=),
            list: Parenthesized<IdentifierList<'a>>,
        }),
        /// Parses an attribute with an identifier. Ex: `PutForwards=name`
        #[derive(Copy)]
        Ident(struct ExtendedAttributeIdent<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs: IdentifierOrString<'a>,
        }),
        /// Parses an attribute with a decimal value. Ex: `ReflectDefault=2.0`
        #[derive(Copy)]
        Decimal(struct ExtendedAttributeDecimal<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs: FloatLit<'a>,
        }),
        /// Parses an attribute with an integer list. Ex: `ReflectRange=((2, 600))`
        ///
        /// (( )) means ( ) chars
        IntegerList(struct ExtendedAttributeIntegerList<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs: Parenthesized<IntegerList<'a>>,
        }),
        /// Parses an attribute with an integer value. Ex: `ReflectDefault=2`
        #[derive(Copy)]
        Integer(struct ExtendedAttributeInteger<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs: IntegerLit<'a>,
        }),
        /// Parses an attribute with a wildcard. Ex: `Exposed=*`
        #[derive(Copy)]
        Wildcard(struct ExtendedAttributeWildCard<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs: term!(*),
        }),
        /// Parses a plain attribute. Ex: `Replaceable`
        #[derive(Copy)]
        NoArgs(struct ExtendedAttributeNoArgs<'a>(
            Identifier<'a>,
        )),
    }

    /// Parses `stringifier|static`
    #[derive(Copy)]
    enum IdentifierOrString<'a> {
        Identifier(Identifier<'a>),
        String(StringLit<'a>),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::literal::{DecLit, FloatValueLit};
    use crate::Parse;

    test!(should_parse_attribute_no_args { "Replaceable" =>
        "";
        ExtendedAttributeNoArgs => ExtendedAttributeNoArgs(Identifier("Replaceable"))
    });

    test!(should_parse_attribute_arg_list { "Constructor(double x, double y)" =>
        "";
        ExtendedAttributeArgList;
        identifier.0 == "Constructor";
        args.body.list.len() == 2;
    });

    test!(should_parse_attribute_ident { "PutForwards=name" =>
        "";
        ExtendedAttributeIdent;
        lhs_identifier.0 == "PutForwards";
        rhs == IdentifierOrString::Identifier(Identifier("name"));
    });

    test!(should_parse_ident_list { "Exposed=(Window,Worker)" =>
        "";
        ExtendedAttributeIdentList;
        identifier.0 == "Exposed";
        list.body.list.len() == 2;
    });

    test!(should_parse_named_arg_list { "NamedConstructor=Image(DOMString src)" =>
        "";
        ExtendedAttributeNamedArgList;
        lhs_identifier.0 == "NamedConstructor";
        rhs_identifier.0 == "Image";
        args.body.list.len() == 1;
    });

    test!(should_parse_decimal { "ReflectDefault=2.0" =>
        "";
        ExtendedAttributeDecimal;
        lhs_identifier.0 == "ReflectDefault";
        rhs == FloatLit::Value(FloatValueLit("2.0"));
    });

    test!(should_parse_integer_list { "ReflectRange=(2, 600)" =>
        "";
        ExtendedAttributeIntegerList;
        lhs_identifier.0 == "ReflectRange";
        rhs.body.list.len() == 2;
    });

    test!(should_parse_integer { "ReflectDefault=2" =>
        "";
        ExtendedAttributeInteger;
        lhs_identifier.0 == "ReflectDefault";
        rhs == IntegerLit::Dec(DecLit("2"));
    });
}
