//! Macro to extract specific types from photon values

/// Generic function to extract and cast photon values
#[inline(always)]
pub fn extract_value<T, F>(
    params: &std::collections::HashMap<u8, photon_decode::Value>,
    idx: u8,
    extractor: F,
) -> Result<T, crate::DecodeError>
where
    F: Fn(&photon_decode::Value) -> Option<T>,
{
    let val = params.get(&idx)
        .ok_or(crate::DecodeError::ParameterMissing(idx as usize))?;

    extractor(val)
        .ok_or_else(|| crate::DecodeError::UnexpectedPhotonType(val.clone()))
}

/// Extract specific types from photon values using a function
#[macro_export]
macro_rules! ph_extract {
    ($params:ident, $idx:literal, $variant:ident) => {{
        $crate::ph_extract::extract_value($params, $idx, |val| match val {
            photon_decode::Value::$variant(x) => Some(x),
            _ => None,
        })
    }};
}

/// Extract integer types from photon values using a function
#[macro_export]
macro_rules! ph_int {
    ($params:ident, $idx:literal, $type:ty) => {{
        $crate::ph_extract::extract_value($params, $idx, |val| match val {
            photon_decode::Value::Byte(x) => Some(*x as $type),
            photon_decode::Value::Integer(x) => Some(*x as $type),
            photon_decode::Value::Short(x) => Some(*x as $type),
            photon_decode::Value::Long(x) => Some(*x as $type),
            _ => None,
        })
    }};
}

/// Extract floating-point types from photon values using a function
#[macro_export]
macro_rules! ph_float {
    ($params:ident, $idx:literal, $type:ty) => {{
        $crate::ph_extract::extract_value($params, $idx, |val| match val {
            photon_decode::Value::Float(x) => Some(*x as $type),
            photon_decode::Value::Double(x) => Some(*x as $type),
            _ => None,
        })
    }};
}
