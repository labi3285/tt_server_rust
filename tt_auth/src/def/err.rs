
pub mod code {
    use tt_core::def::err::Code;

    pub static TOKEN: Code =                Code(1000, "令牌失败");
    pub static VALIDATE: Code =             Code(1001, "验证失败");
    pub static NOT_FOUND: Code =            Code(1002, "解析失败");
    pub static RESERVED: Code =            Code(1003, "保留数据");

}