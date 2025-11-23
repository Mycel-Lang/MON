// Linting rules configuration

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintRule {
    MaxNestingDepth,
    MaxObjectMembers,
    MaxArrayItems,
    UnusedAnchor,
    MagicNumber,
    MissingTypeValidation,
    DuplicateKey,
    ExcessiveSpreads,
}

impl LintRule {
    pub fn name(&self) -> &'static str {
        match self {
            LintRule::MaxNestingDepth => "max_nesting_depth",
            LintRule::MaxObjectMembers => "max_object_members",
            LintRule::MaxArrayItems => "max_array_items",
            LintRule::UnusedAnchor => "unused_anchor",
            LintRule::MagicNumber => "magic_number",
            LintRule::MissingTypeValidation => "missing_type_validation",
            LintRule::DuplicateKey => "duplicate_key",
            LintRule::ExcessiveSpreads => "excessive_spreads",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            LintRule::MaxNestingDepth => "Objects/arrays are nested too deeply",
            LintRule::MaxObjectMembers => "Object has too many members",
            LintRule::MaxArrayItems => "Array has too many items",
            LintRule::UnusedAnchor => "Anchor defined but never used",
            LintRule::MagicNumber => "Literal number without context",
            LintRule::MissingTypeValidation => "Data should have type validation",
            LintRule::DuplicateKey => "Duplicate key in object",
            LintRule::ExcessiveSpreads => "Too many spreads in single object",
        }
    }
}
