pub mod code {
    use tt_core::def::err::Code;


    pub static CONNECT: Code =              Code(2000, "解析失败");

    pub static TRANS: Code =                Code(2001, "创建事务报错");
    pub static COMMIT: Code =               Code(2002, "提交报错");
    pub static ROLLBACLK: Code =            Code(2003, "回滚报错");

    pub static QUERY_ALL: Code =            Code(2004, "查询列表报错");
    pub static QUERY_ONE: Code =            Code(2005, "查询单条报错");
    pub static QUERY_ONE_OPTIONAL: Code =   Code(2006, "查询单条报错");
    pub static EXEC: Code =                 Code(2007, "SQL执行报错");
    
}
