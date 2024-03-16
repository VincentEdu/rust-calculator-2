pub use usize as FunctionId;

pub const ID_ADD: FunctionId = 0;
pub const ID_SUB: FunctionId = 1;
pub const ID_MUL: FunctionId = 2;
pub const ID_DIV: FunctionId = 3;
pub const ID_MOD: FunctionId = 4;
pub const ID_POW: FunctionId = 5;
pub const ID_SQRT: FunctionId = 6;
pub const ID_ABS: FunctionId = 7;
pub const ID_NEG: FunctionId = 8;
pub const ID_SIN: FunctionId = 9;
pub const ID_COS: FunctionId = 10;
pub const ID_TAN: FunctionId = 11;
pub const ID_LN: FunctionId = 12;
pub const ID_OPEN_BRACKET: FunctionId = 13;
pub const ID_CLOSE_BRACKET: FunctionId = 14;
pub const ID_SQR: FunctionId = 15;
pub const ID_INV: FunctionId = 16;

pub const PRIODITY_ADDITIVE: i32 = 6;
pub const PRIODITY_MULTIPLICATIVE: i32 = 5;
pub const PRIODITY_USER_FUNCTION: i32 = 2;
pub const PRIODITY_UNARY_OP: i32 = 3;
