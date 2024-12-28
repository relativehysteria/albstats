//! Macro to extract specific types from photon values

#[macro_export]
macro_rules! ph_extract {
    ($params:ident, $idx:literal, $variant:ident) => {{
        let val = $params.get(&$idx)
            .ok_or($crate::DecodeError::ParameterMissing)?;

        match val {
            photon_decode::Value::$variant(x) => Ok(x),
            _ => Err($crate::DecodeError::UnexpectedPhotonType(val.clone()))
        }}
    }
}

#[macro_export]
macro_rules! ph_int {
    ($params:ident, $idx:literal, $type:ty) => {{
        let val = $params.get(&$idx)
            .ok_or($crate::DecodeError::ParameterMissing)?;

        match val {
            photon_decode::Value::Byte(x)    => Ok(*x as $type),
            photon_decode::Value::Integer(x) => Ok(*x as $type),
            photon_decode::Value::Short(x)   => Ok(*x as $type),
            photon_decode::Value::Long(x)    => Ok(*x as $type),
            _ => Err($crate::DecodeError::UnexpectedPhotonType(val.clone()))
        }
    }}
}

#[macro_export]
macro_rules! ph_float {
    ($params:ident, $idx:literal, $type:ty) => {{
        let val = $params.get(&$idx)
            .ok_or($crate::DecodeError::ParameterMissing)?;

        match val {
            photon_decode::Value::Float(x)  => Ok(*x as $type),
            photon_decode::Value::Double(x) => Ok(*x as $type),
            _ => Err($crate::DecodeError::UnexpectedPhotonType(val.clone()))
        }
    }}
}
