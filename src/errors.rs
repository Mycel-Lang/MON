use miette::{Diagnostic, NamedSource, SourceSpan};
use mon_core::error::{MonError, ParserError, ResolverError, ValidationError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MonCliError {
    #[error("Syntax Error: {message}")]
    #[diagnostic(code(mon::syntax_error), help("{help}"))]
    Parser {
        message: String,
        help: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("{label}")]
        span: SourceSpan,
        label: String,
    },

    #[error("Resolution Error: {message}")]
    #[diagnostic(code(mon::resolution_error), help("{help}"))]
    Resolver {
        message: String,
        help: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("{label}")]
        span: SourceSpan,
        label: String,
    },

    #[error("Validation Error: {message}")]
    #[diagnostic(code(mon::validation_error), help("{help}"))]
    Validation {
        message: String,
        help: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("{label}")]
        span: SourceSpan,
        label: String,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Mon(#[from] MonError),
}

impl MonCliError {
    pub fn from_mon_error(err: MonError) -> Self {
        match err {
            MonError::Parser(boxed_err) => Self::from_parser_error(*boxed_err),
            MonError::Resolver(boxed_err) => Self::from_resolver_error(*boxed_err),
        }
    }

    fn from_parser_error(err: ParserError) -> Self {
        match err {
            ParserError::UnexpectedToken { src, span, expected } => {
                let (help, label) = match expected.as_str() {
                    "RBrace" => (
                        "Make sure all objects are properly closed with '}'.  Check for missing commas between key-value pairs.".to_string(),
                        "expected '}' here - did you forget to close an object?".to_string(),
                    ),
                    "RBracket" => (
                        "Make sure all arrays are properly closed with ']'. Check for missing commas between array elements.".to_string(),
                        "expected ']' here - did you forget to close an array?".to_string(),
                    ),
                    "Comma" => (
                        "MON requires commas between object fields and array elements. Add a comma after the previous item.".to_string(),
                        "expected ',' here".to_string(),
                    ),
                    "Colon" => (
                        "Object keys must be followed by a colon ':' before the value.".to_string(),
                        "expected ':' after key".to_string(),
                    ),
                    _ => (
                        format!("Check the syntax at this location. Expected: {}", expected),
                        format!("expected {} here", expected),
                    ),
                };
                
                MonCliError::Parser {
                    message: format!("Unexpected token, expected {}", expected),
                    help,
                    src,
                    span,
                    label,
                }
            },
            ParserError::UnexpectedEof { src, span } => MonCliError::Parser {
                message: "Unexpected end of file".to_string(),
                help: "The file ended unexpectedly. Make sure all objects {...} and arrays [...] are properly closed.".to_string(),
                src,
                span,
                label: "unexpected end of file - missing closing bracket?".to_string(),
            },
            ParserError::MissingExpectedToken { src, span, expected } => {
                let help = match expected.as_str() {
                    "LBrace" => "MON files should typically start with an object '{'. Wrap your content in curly braces.".to_string(),
                    "RBrace" => "Missing closing brace '}'. Make sure all objects are properly closed.".to_string(),
                    _ => format!("Missing required token: {}. Check the syntax at this location.", expected),
                };
                
                MonCliError::Parser {
                    message: format!("Missing expected token: {}", expected),
                    help,
                    src,
                    span,
                    label: format!("expected {} here", expected),
                }
            },
        }
    }

    fn from_resolver_error(err: ResolverError) -> Self {
        match err {
            ResolverError::ModuleNotFound { path, src, span } => MonCliError::Resolver {
                message: format!("Module not found: {}", path),
                help: format!("The imported module '{}' could not be found. Check the file path and make sure the file exists.", path),
                src,
                span,
                label: "module not found".to_string(),
            },
            ResolverError::AnchorNotFound { name, src, span } => MonCliError::Resolver {
                message: format!("Anchor not found: {}", name),
                help: format!("The anchor '{}' is referenced but not defined. Make sure you've defined it with 'name: &anchor' before using '*anchor'.", name),
                src,
                span,
                label: "undefined anchor".to_string(),
            },
            ResolverError::SpreadOnNonObject { name, src, span } => MonCliError::Resolver {
                message: format!("Cannot spread non-object: {}", name),
                help: "The spread operator '...' can only be used on objects. Make sure the value you're spreading is an object {...}.".to_string(),
                src,
                span,
                label: "not an object".to_string(),
            },
            ResolverError::SpreadOnNonArray { name, src, span } => MonCliError::Resolver {
                message: format!("Cannot spread non-array: {}", name),
                help: "The spread operator '...' can only be used on arrays. Make sure the value you're spreading is an array [...].".to_string(),
                src,
                span,
                label: "not an array".to_string(),
            },
            ResolverError::CircularDependency { cycle, src, span } => MonCliError::Resolver {
                message: format!("Circular dependency detected: {}", cycle),
                help: format!("A circular reference was detected: {}. Check your import chain and anchor references to break the cycle.", cycle),
                src,
                span,
                label: "circular dependency here".to_string(),
            },
            ResolverError::Validation(val_err) => Self::from_validation_error(val_err),
            ResolverError::WrappedParserError(boxed_err) => Self::from_parser_error(*boxed_err),
        }
    }

    fn from_validation_error(err: ValidationError) -> Self {
        match err {
            ValidationError::TypeMismatch { field_name, expected_type, found_type, src, span } => {
                MonCliError::Validation {
                    message: format!(
                        "Type mismatch in field '{}': expected {}, got {}",
                        field_name, expected_type, found_type
                    ),
                    help: format!(
                        "The field '{}' should be of type '{}', but you provided '{}'. Check the type signature and make sure the value matches.",
                        field_name, expected_type, found_type
                    ),
                    src,
                    span,
                    label: format!("expected {}, got {}", expected_type, found_type),
                }
            }
            ValidationError::MissingField { field_name, struct_name, src, span } => {
                MonCliError::Validation {
                    message: format!(
                        "Missing required field '{}' in struct '{}'",
                        field_name, struct_name
                    ),
                    help: format!(
                        "The field '{}' is required in struct '{}' but was not provided. Add this field to your object.",
                        field_name, struct_name
                    ),
                    src,
                    span,
                    label: format!("missing field '{}'", field_name),
                }
            }
            ValidationError::UnexpectedField { field_name, struct_name, src, span } => {
                MonCliError::Validation {
                    message: format!(
                        "Unexpected field '{}' in struct '{}'",
                        field_name, struct_name
                    ),
                    help: format!(
                        "The field '{}' is not defined in the struct '{}' schema. Remove this field or update the struct definition.",
                        field_name, struct_name
                    ),
                    src,
                    span,
                    label: format!("unexpected field '{}'", field_name),
                }
            }
            ValidationError::UndefinedType { type_name, src, span } => MonCliError::Validation {
                message: format!("Undefined type: {}", type_name),
                help: format!(
                    "The type '{}' is not defined. Make sure you've defined this type with '#struct' or '#enum' before using it.",
                    type_name
                ),
                src,
                span,
                label: "undefined type".to_string(),
            },
            ValidationError::UndefinedEnumVariant { variant_name, enum_name, src, span } => {
                MonCliError::Validation {
                    message: format!("Undefined enum variant: {}::{}", enum_name, variant_name),
                    help: format!(
                        "The variant '{}' is not defined in enum '{}'. Check the enum definition for valid variants.",
                        variant_name, enum_name
                    ),
                    src,
                    span,
                    label: format!("undefined variant '{}'", variant_name),
                }
            }
            ValidationError::UnimplementedCollectionValidation { field_name, src, span } => {
                MonCliError::Validation {
                    message: format!(
                        "Collection validation not yet implemented for field '{}'",
                        field_name
                    ),
                    help: format!(
                        "Validation for the collection field '{}' is not yet supported in mon-core v0.0.1.",
                        field_name
                    ),
                    src,
                    span,
                    label: "unimplemented validation".to_string(),
                }
            }
        }
    }
}
