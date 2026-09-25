/// Generates a UUID-based opaque ID newtype.
///
/// The type name must end in `Id`.  The Display prefix is the lowercase
/// form of the name with the `Id` suffix stripped.
///
/// Usage:
///   define_id!(MedicationId);   // → "medication-{uuid}"
///   define_id!(ScriptId);       // → "script-{uuid}"
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $name(uuid::Uuid);

        impl $name {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self(uuid::Uuid::now_v7())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = stringify!($name);
                let prefix = name.strip_suffix("Id").unwrap_or(name).to_ascii_lowercase();
                write!(f, "{prefix}-{}", self.0)
            }
        }
    };
}

pub(crate) use define_id;
