// pub enum RustNumber{
//     F64(f64),
//     I64(i64)
// }

#[derive(Debug)]
pub enum TSType {
    Fn,
    Number,
    String,
    Array(String),
    Tuples,
    Object,
    Boolean,
    Void,
    Undefined,
    Optional(String),
    Custom(String),
}

// enum RustBracketed {
//     Option,
//     Vec,
//     None
// }

// impl Into<String> for RustBracketed {
//     fn into(self) -> String {
//         match self{
//             RustBracketed::Option => "Option".into(),
//             RustBracketed::Vec => "Vec".into(),
//             RustBracketed::None => "".into(),
//         }
//     }
// }



impl Into<TSType> for String {
    fn into(self) -> TSType {
        let size = self.len();

        match self.as_str() {
            "String" => TSType::String,
            "bool" => TSType::Boolean,
            "undefined" => TSType::Undefined,
            x if size > 6 && &x[0..6] == "Option" => {
                let inside:TSType = x[7.. size-1].to_string().into();
                TSType::Optional(inside.into())
            }
            x if size > 3 && &x[0..3] == "Vec" => {
                let inside: TSType = x[4..size-1].to_string().into();
                TSType::Array(inside.into())
            },
            x if size <= 3 && (&x[0..1] == "i"||&x[0..1] == "u") => TSType::Number,
            x => TSType::Custom(x.into())
        }
    }
}

impl Into<String> for TSType {
    fn into(self) -> String {
        match self {
            TSType::Fn => todo!(),
            TSType::Number => "number".into(),
            TSType::String => "string".into(),
            TSType::Array(x) => format!("Array<{x}>"),
            TSType::Tuples => todo!(),
            TSType::Object => todo!(),
            TSType::Boolean => "boolean".into(),
            TSType::Void => "void".into(),
            TSType::Undefined => "undefined".into(),
            TSType::Optional(x) => x,
            TSType::Custom(x) => x
        }
    }
}