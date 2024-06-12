use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind;
use gitql_ast::expression::StringExpression;
use gitql_ast::expression::StringValueType;
use gitql_core::environment::Environment;
use gitql_core::types::DataType;

use crate::diagnostic::Diagnostic;
use crate::format_checker::is_valid_date_format;
use crate::format_checker::is_valid_datetime_format;
use crate::format_checker::is_valid_time_format;
use crate::tokenizer::Location;

/// The return result after performing types checking with implicit casting option
pub enum TypeCheckResult {
    /// Both right and left hand sides types are equals without implicit casting
    Equals,
    /// Both right and left hand sides types are not equals and can't perform implicit casting
    NotEqualAndCantImplicitCast,
    /// Not Equals and can't perform implicit casting with error message provided
    Error(Box<Diagnostic>),
    /// Right hand side type will match the left side after implicit casting
    RightSideCasted(Box<dyn Expression>),
    /// Left hand side type will match the right side after implicit casting
    LeftSideCasted(Box<dyn Expression>),
}

/// Check if expression type and data type are equals
/// If not then check if one can be implicit casted to the other
#[allow(clippy::borrowed_box)]
pub fn is_expression_type_equals(
    scope: &Environment,
    expr: &Box<dyn Expression>,
    data_type: &DataType,
) -> TypeCheckResult {
    let expr_type = expr.expr_type(scope);

    // Both types are already equals without need for implicit casting
    if expr_type == *data_type {
        return TypeCheckResult::Equals;
    }

    // Cast expr type from Text literal to time
    if (data_type.is_time() || data_type.is_variant_with(&DataType::Time))
        && expr_type.is_text()
        && expr.kind() == ExpressionKind::String
    {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_time_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Time and Text `{}` because it can't be implicitly casted to Time",
                    string_literal_value
                )).add_help("A valid Time format must match `HH:MM:SS` or `HH:MM:SS.SSS`")
                .add_help("You can use `MAKETIME(hour, minute, second)` function to create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Cast expr type from Text literal to Date
    if (data_type.is_date() || data_type.is_variant_with(&DataType::Date))
        && expr_type.is_text()
        && expr.kind() == ExpressionKind::String
    {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_date_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Date and Text `{}` because it can't be implicitly casted to Date",
                    string_literal_value
                )).add_help("A valid Date format must match `YYYY-MM-DD`")
                .add_help("You can use `MAKEDATE(year, dayOfYear)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Date,
        }));
    }

    // Cast right hand side type from Text literal to DateTime
    if (data_type.is_datetime() || data_type.is_variant_with(&DataType::DateTime))
        && expr_type.is_text()
        && expr.kind() == ExpressionKind::String
    {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_datetime_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare DateTime and Text `{}` because it can't be implicitly casted to DateTime",
                    string_literal_value
                )).add_help("A valid DateTime format must match `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD HH:MM:SS.SSS`")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::DateTime,
        }));
    }

    TypeCheckResult::NotEqualAndCantImplicitCast
}

/// Check if two expressions types are equals
/// If not then check if one can be implicit casted to the other
#[allow(clippy::borrowed_box)]
pub fn are_types_equals(
    scope: &Environment,
    lhs: &Box<dyn Expression>,
    rhs: &Box<dyn Expression>,
) -> TypeCheckResult {
    let lhs_type = lhs.expr_type(scope);
    let rhs_type = rhs.expr_type(scope);

    // Both types are already equals without need for implicit casting
    if lhs_type == rhs_type {
        return TypeCheckResult::Equals;
    }

    // Cast right hand side type from Text literal to time
    if (lhs_type.is_time() || lhs_type.is_variant_with(&DataType::Time))
        && rhs_type.is_text()
        && rhs.kind() == ExpressionKind::String
    {
        let expr = rhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_time_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Time and Text `{}` because it can't be implicitly casted to Time",
                    string_literal_value
                )).add_help("A valid Time format must match `HH:MM:SS` or `HH:MM:SS.SSS`")
                .add_help("You can use `MAKETIME(hour, minute, second)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Cast left hand side type from Text literal to time
    if lhs_type.is_text()
        && (rhs_type.is_time() || rhs_type.is_variant_with(&DataType::Time))
        && lhs.kind() == ExpressionKind::String
    {
        let expr = lhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_time_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Text `{}` and Time because it can't be implicitly casted to Time",
                    string_literal_value
                )).add_help("A valid Time format must match `HH:MM:SS` or `HH:MM:SS.SSS`")
                .add_help("You can use `MAKETIME(hour, minute, second)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::LeftSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Cast right hand side type from Text literal to Date
    if (lhs_type.is_date() || lhs_type.is_variant_with(&DataType::Date))
        && rhs_type.is_text()
        && rhs.kind() == ExpressionKind::String
    {
        let expr = rhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_date_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Date and Text(`{}`) because Text can't be implicitly casted to Date",
                    string_literal_value
                )).add_help("A valid Date format should be matching `YYYY-MM-DD`")
                .add_help("You can use `MAKEDATE(year, dayOfYear)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Date,
        }));
    }

    // Cast left hand side type from Text literal to Date
    if lhs_type.is_text()
        && (rhs_type.is_date() || rhs_type.is_variant_with(&DataType::Date))
        && lhs.kind() == ExpressionKind::String
    {
        let expr = lhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_date_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Text(`{}`) and Date because Text can't be implicitly casted to Date",
                    string_literal_value
                )).add_help("A valid Date format should be matching `YYYY-MM-DD`")
                .add_help("You can use `MAKEDATE(year, dayOfYear)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::LeftSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Date,
        }));
    }

    // Cast right hand side type from Text literal to DateTime
    if (lhs_type.is_datetime() || lhs_type.is_variant_with(&DataType::DateTime))
        && rhs_type.is_text()
        && rhs.kind() == ExpressionKind::String
    {
        let expr = rhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_datetime_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare DateTime and Text `{}` because it can't be implicitly casted to DateTime",
                    string_literal_value
                )).add_help("A valid DateTime format must match `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD HH:MM:SS.SSS`")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::DateTime,
        }));
    }

    // Cast Left hand side type from Text literal to DateTime
    if lhs_type.is_text()
        && (rhs_type.is_datetime() || rhs_type.is_variant_with(&DataType::DateTime))
        && lhs.kind() == ExpressionKind::String
    {
        let expr = lhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_datetime_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Text `{}` and DateTime because it can't be implicitly casted to DateTime",
                    string_literal_value
                )).add_help("A valid DateTime format must match `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD HH:MM:SS.SSS`")
                .as_boxed(),
            );
        }

        return TypeCheckResult::LeftSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::DateTime,
        }));
    }

    TypeCheckResult::NotEqualAndCantImplicitCast
}

/// Checks if all values has the same type
/// If they have the same type, return it or return None
pub fn check_all_values_are_same_type(
    env: &mut Environment,
    arguments: &[Box<dyn Expression>],
) -> Option<DataType> {
    let arguments_count = arguments.len();
    if arguments_count == 0 {
        return Some(DataType::Any);
    }

    let data_type = arguments[0].expr_type(env);
    for argument in arguments.iter().take(arguments_count).skip(1) {
        let expr_type = argument.expr_type(env);
        if data_type != expr_type {
            return None;
        }
    }

    Some(data_type)
}

/// Check That function call arguments types are matches the parameter types
/// Return a Diagnostic Error if anything is wrong
pub fn check_function_call_arguments(
    env: &Environment,
    arguments: &mut [Box<dyn Expression>],
    parameters: &[DataType],
    function_name: String,
    location: Location,
) -> Result<(), Box<Diagnostic>> {
    let parameters_count = parameters.len();
    let arguments_count = arguments.len();

    let mut has_varargs_parameter = false;
    let mut optional_parameters_count = 0;
    if parameters_count != 0 {
        let last_parameter = parameters.last().unwrap();
        has_varargs_parameter = last_parameter.is_varargs();

        // Count number of optional parameters
        for parameter_type in parameters.iter().take(parameters_count) {
            if parameter_type.is_optional() {
                optional_parameters_count += 1;
            }
        }
    }

    let mut min_arguments_count = parameters_count - optional_parameters_count;
    if has_varargs_parameter {
        min_arguments_count -= 1;
    }

    if arguments_count < min_arguments_count {
        return Err(Diagnostic::error(&format!(
            "Function `{}` expects at least `{}` arguments but got `{}`",
            function_name, min_arguments_count, arguments_count
        ))
        .with_location(location)
        .as_boxed());
    }

    if !has_varargs_parameter && arguments_count > parameters_count {
        return Err(Diagnostic::error(&format!(
            "Function `{}` expects `{}` arguments but got `{}`",
            function_name, arguments_count, parameters_count
        ))
        .with_location(location)
        .as_boxed());
    }

    // Type check the min required arguments
    for index in 0..min_arguments_count {
        let parameter_type = parameters.get(index).unwrap();
        let argument = arguments.get(index).unwrap();

        // Catch undefined arguments
        if argument.expr_type(env).is_undefined() {
            return Err(Diagnostic::error(&format!(
                "Function `{}` argument number {} has Undefined type",
                function_name, index,
            ))
            .add_help("Make sure you used a correct field name")
            .add_help("Check column names for each table from docs website")
            .with_location(location)
            .as_boxed());
        }

        match is_expression_type_equals(env, argument, parameter_type) {
            TypeCheckResult::Equals => {}
            TypeCheckResult::RightSideCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            TypeCheckResult::LeftSideCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            TypeCheckResult::NotEqualAndCantImplicitCast => {
                let argument_type = argument.expr_type(env);
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                    function_name, index, argument_type, parameter_type
                ))
                .with_location(location).as_boxed());
            }
            TypeCheckResult::Error(error) => return Err(error),
        }
    }

    // Type check the optional parameters
    let last_optional_param_index = min_arguments_count + optional_parameters_count;
    for index in min_arguments_count..last_optional_param_index {
        if index >= arguments_count {
            return Ok(());
        }

        let parameter_type = parameters.get(index).unwrap();
        let argument = arguments.get(index).unwrap();

        // Catch undefined arguments
        if argument.expr_type(env).is_undefined() {
            return Err(Diagnostic::error(&format!(
                "Function `{}` argument number {} has Undefined type",
                function_name, index,
            ))
            .add_help("Make sure you used a correct field name")
            .add_help("Check column names for each table from docs website")
            .with_location(location)
            .as_boxed());
        }

        match is_expression_type_equals(env, argument, parameter_type) {
            TypeCheckResult::Equals => {}
            TypeCheckResult::RightSideCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            TypeCheckResult::LeftSideCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            TypeCheckResult::NotEqualAndCantImplicitCast => {
                let argument_type = argument.expr_type(env);
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                    function_name, index, argument_type, parameter_type
                ))
                .with_location(location).as_boxed());
            }
            TypeCheckResult::Error(error) => return Err(error),
        }
    }

    // Type check the variable parameters if exists
    if has_varargs_parameter {
        let varargs_type = parameters.last().unwrap();
        for index in last_optional_param_index..arguments_count {
            let argument = arguments.get(index).unwrap();

            // Catch undefined arguments
            if argument.expr_type(env).is_undefined() {
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} has Undefined type",
                    function_name, index,
                ))
                .add_help("Make sure you used a correct field name")
                .add_help("Check column names for each table from docs website")
                .with_location(location)
                .as_boxed());
            }

            match is_expression_type_equals(env, argument, varargs_type) {
                TypeCheckResult::Equals => {}
                TypeCheckResult::RightSideCasted(new_expr) => {
                    arguments[index] = new_expr;
                }
                TypeCheckResult::LeftSideCasted(new_expr) => {
                    arguments[index] = new_expr;
                }
                TypeCheckResult::NotEqualAndCantImplicitCast => {
                    let argument_type = argument.expr_type(env);
                    return Err(Diagnostic::error(&format!(
                        "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                        function_name, index, argument_type, varargs_type
                    ))
                    .with_location(location).as_boxed());
                }
                TypeCheckResult::Error(error) => return Err(error),
            }
        }
    }

    Ok(())
}

/// Check that all selected fields types are defined correctly
/// Return a Diagnostic Error if anything is wrong
pub fn type_check_selected_fields(
    env: &mut Environment,
    table_name: &str,
    fields_names: &Vec<String>,
    location: Location,
) -> Result<(), Box<Diagnostic>> {
    for field_name in fields_names {
        if let Some(data_type) = env.resolve_type(field_name) {
            if data_type.is_undefined() {
                return Err(
                    Diagnostic::error(&format!("No field with name `{}`", field_name))
                        .with_location(location)
                        .as_boxed(),
                );
            }
            continue;
        }

        return Err(Diagnostic::error(&format!(
            "Table `{}` has no field with name `{}`",
            table_name, field_name
        ))
        .add_help("Check the documentations to see available fields for each tables")
        .with_location(location)
        .as_boxed());
    }
    Ok(())
}

/// Check that all projection columns are valid for this table name
/// Return a Diagnostic Error if anything is wrong
pub fn type_check_projection_symbols(
    env: &mut Environment,
    table_name: &str,
    projection_names: &[String],
    projection_locations: &[Location],
) -> Result<(), Box<Diagnostic>> {
    if table_name.is_empty() && !projection_names.is_empty() {
        return Err(Diagnostic::error(&format!(
            "Unresolved field with name `{}`",
            projection_names[0]
        ))
        .with_location(projection_locations[0])
        .as_boxed());
    }

    let count = projection_names.len();
    let table_fields = &env.schema.tables_fields_names[table_name];
    for i in 0..count {
        if !table_fields.contains(&projection_names[i].as_str()) {
            return Err(Diagnostic::error(&format!(
                "Table {} has no field with name `{}`",
                table_name, projection_names[i]
            ))
            .add_help("Check the documentations to see available fields for each tables")
            .with_location(projection_locations[i])
            .as_boxed());
        }
    }

    Ok(())
}
