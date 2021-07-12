use std::fmt;

use sgx_serialize::{SerializeHelper, DeSerializeHelper};

#[derive(Debug, Clone, PartialEq, Eq, Hash,Serializable, DeSerializable)]
pub enum Token {
    /// An end-of-file marker, not a real token
    EOF,
    /// A keyword (like SELECT) or an optionally quoted SQL identifier
    Word(Word),
    /// An unsigned numeric literal
    Number(String),
    /// A character that could not be tokenized
    Char(char),
    /// Single quoted string: i.e: 'string'
    SingleQuotedString(String),
    /// "National" string literal: i.e: N'string'
    NationalStringLiteral(String),
    /// Hexadecimal string literal: i.e.: X'deadbeef'
    HexStringLiteral(String),
    /// Comma
    Comma,
    /// Whitespace (space, tab, etc)
    Whitespace(Whitespace),
    /// Double equals sign `==`
    DoubleEq,
    /// Equality operator `=`
    Eq,
    /// Not Equals operator `<>` (or `!=` in some dialects)
    Neq,
    /// Less Than operator `<`
    Lt,
    /// Greater Than operator `>`
    Gt,
    /// Less Than Or Equals operator `<=`
    LtEq,
    /// Greater Than Or Equals operator `>=`
    GtEq,
    /// Plus operator `+`
    Plus,
    /// Minus operator `-`
    Minus,
    /// Multiplication operator `*`
    Mult,
    /// Division operator `/`
    Div,
    /// Modulo Operator `%`
    Mod,
    /// String concatenation `||`
    StringConcat,
    /// Left parenthesis `(`
    LParen,
    /// Right parenthesis `)`
    RParen,
    /// Period (used for compound identifiers or projections into nested types)
    Period,
    /// Colon `:`
    Colon,
    /// SemiColon `;` used as separator for COPY and payload
    SemiColon,
    /// Backslash `\` used in terminating the COPY payload with `\.`
    Backslash,
    /// Left bracket `[`
    LBracket,
    /// Right bracket `]`
    RBracket,
    /// Ampersand `&`
    Ampersand,
    /// Pipe `|`
    Pipe,
    /// Caret `^`
    Caret,
    /// Left brace `{`
    LBrace,
    /// Right brace `}`
    RBrace,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash,Serializable, DeSerializable)]
pub struct Word {
    /// The value of the token, without the enclosing quotes, and with the
    /// escape sequences (if any) processed (TODO: escapes are not handled)
    pub value: String,
    /// An identifier can be "quoted" (&lt;delimited identifier> in ANSI parlance).
    /// The standard and most implementations allow using double quotes for this,
    /// but some implementations support other quoting styles as well (e.g. \[MS SQL])
    pub quote_style: Option<char>,
    /// If the word was not quoted and it matched one of the known keywords,
    /// this will have one of the values from dialect::keywords, otherwise empty
    pub keyword: Keyword,
}


impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.quote_style {
            Some(s) if s == '"' => {
                write!(f, "\"{}\"", self.value )
            }
            None => f.write_str(&self.value),
            _ => panic!("Unexpected quote_style!"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash,Serializable, DeSerializable)]
pub enum Whitespace {
    Space,
    Newline,
    Tab,
    LineComment(String),
}

impl fmt::Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::Newline => f.write_str("\n"),
            Whitespace::Tab => f.write_str("\t"),
            Whitespace::LineComment(s) => write!(f, "{}", s),
        }
    }
}
#[derive(Debug, PartialEq,Serializable, DeSerializable)]
pub struct TokenizerError {
    pub message: String,
    pub line: u64,
    pub col: u64,
}



#[derive(Debug, Clone, PartialEq, Eq, Hash,Serializable, DeSerializable)]
pub enum Keyword {
    ABORT,
    ABS,
    ACTION,
    ADD,
    ALL,
    ALLOCATE,
    ALTER,
    ANALYZE,
    AND,
    ANY,
    APPLY,
    ARE,
    ARRAY,
    ARRAY_AGG,
    ARRAY_MAX_CARDINALITY,
    AS,
    ASC,
    ASENSITIVE,
    ASSERT,
    ASYMMETRIC,
    AT,
    ATOMIC,
    AUTHORIZATION,
    AUTOINCREMENT,
    AUTO_INCREMENT,
    AVG,
    AVRO,
    BEGIN,
    BEGIN_FRAME,
    BEGIN_PARTITION,
    BETWEEN,
    BIGINT,
    BINARY,
    BLOB,
    BOOLEAN,
    BOTH,
    BY,
    BYTEA,
    CACHE,
    CALL,
    CALLED,
    CARDINALITY,
    CASCADE,
    CASCADED,
    CASE,
    CAST,
    CEIL,
    CEILING,
    CHAIN,
    CHAR,
    CHARACTER,
    CHARACTER_LENGTH,
    CHAR_LENGTH,
    CHECK,
    CLOB,
    CLOSE,
    CLUSTER,
    COALESCE,
    COLLATE,
    COLLECT,
    COLUMN,
    COLUMNS,
    COMMIT,
    COMMITTED,
    COMPUTE,
    CONDITION,
    CONNECT,
    CONSTRAINT,
    CONTAINS,
    CONVERT,
    COPY,
    CORR,
    CORRESPONDING,
    COUNT,
    COVAR_POP,
    COVAR_SAMP,
    CREATE,
    CROSS,
    CSV,
    CUBE,
    CUME_DIST,
    CURRENT,
    CURRENT_CATALOG,
    CURRENT_DATE,
    CURRENT_DEFAULT_TRANSFORM_GROUP,
    CURRENT_PATH,
    CURRENT_ROLE,
    CURRENT_ROW,
    CURRENT_SCHEMA,
    CURRENT_TIME,
    CURRENT_TIMESTAMP,
    CURRENT_TRANSFORM_GROUP_FOR_TYPE,
    CURRENT_USER,
    CURSOR,
    CYCLE,
    DATABASE,
    DATE,
    DAY,
    DEALLOCATE,
    DEC,
    DECIMAL,
    DECLARE,
    DEFAULT,
    DELETE,
    DELIMITED,
    DENSE_RANK,
    DEREF,
    DESC,
    DESCRIBE,
    DETERMINISTIC,
    DIRECTORY,
    DISCONNECT,
    DISTINCT,
    DISTRIBUTE,
    DOUBLE,
    DROP,
    DYNAMIC,
    EACH,
    ELEMENT,
    ELSE,
    END,
    END_EXEC,
    END_FRAME,
    END_PARTITION,
    EQUALS,
    ERROR,
    ESCAPE,
    EVERY,
    EXCEPT,
    EXEC,
    EXECUTE,
    EXISTS,
    EXP,
    EXPLAIN,
    EXTENDED,
    EXTERNAL,
    EXTRACT,
    FAIL,
    FALSE,
    FETCH,
    FIELDS,
    FILTER,
    FIRST,
    FIRST_VALUE,
    FLOAT,
    FLOOR,
    FOLLOWING,
    FOR,
    FOREIGN,
    FORMAT,
    FRAME_ROW,
    FREE,
    FROM,
    FULL,
    FUNCTION,
    FUSION,
    GET,
    GLOBAL,
    GRANT,
    GROUP,
    GROUPING,
    GROUPS,
    HAVING,
    HEADER,
    HIVEVAR,
    HOLD,
    HOUR,
    IDENTITY,
    IF,
    IGNORE,
    ILIKE,
    IN,
    INDEX,
    INDICATOR,
    INNER,
    INOUT,
    INPUTFORMAT,
    INSENSITIVE,
    INSERT,
    INT,
    INTEGER,
    INTERSECT,
    INTERSECTION,
    INTERVAL,
    INTO,
    IS,
    ISOLATION,
    JOIN,
    JSONFILE,
    KEY,
    LAG,
    LANGUAGE,
    LARGE,
    LAST,
    LAST_VALUE,
    LATERAL,
    LEAD,
    LEADING,
    LEFT,
    LEVEL,
    LIKE,
    LIKE_REGEX,
    LIMIT,
    LISTAGG,
    LN,
    LOCAL,
    LOCALTIME,
    LOCALTIMESTAMP,
    LOCATION,
    LOWER,
    MANAGEDLOCATION,
    MATCH,
    MATERIALIZED,
    MAX,
    MEMBER,
    MERGE,
    METADATA,
    METHOD,
    MIN,
    MINUTE,
    MOD,
    MODIFIES,
    MODULE,
    MONTH,
    MSCK,
    MULTISET,
    NATIONAL,
    NATURAL,
    NCHAR,
    NCLOB,
    NEW,
    NEXT,
    NO,
    NONE,
    NORMALIZE,
    NOSCAN,
    NOT,
    NTH_VALUE,
    NTILE,
    NULL,
    NULLIF,
    NULLS,
    NUMERIC,
    OBJECT,
    OCCURRENCES_REGEX,
    OCTET_LENGTH,
    OF,
    OFFSET,
    OLD,
    ON,
    ONLY,
    OPEN,
    OR,
    ORC,
    ORDER,
    OUT,
    OUTER,
    OUTPUTFORMAT,
    OVER,
    OVERFLOW,
    OVERLAPS,
    OVERLAY,
    OVERWRITE,
    PARAMETER,
    PARQUET,
    PARTITION,
    PARTITIONED,
    PARTITIONS,
    PERCENT,
    PERCENTILE_CONT,
    PERCENTILE_DISC,
    PERCENT_RANK,
    PERIOD,
    PORTION,
    POSITION,
    POSITION_REGEX,
    POWER,
    PRECEDES,
    PRECEDING,
    PRECISION,
    PREPARE,
    PRIMARY,
    PROCEDURE,
    PURGE,
    RANGE,
    RANK,
    RCFILE,
    READ,
    READS,
    REAL,
    RECURSIVE,
    REF,
    REFERENCES,
    REFERENCING,
    REGCLASS,
    REGR_AVGX,
    REGR_AVGY,
    REGR_COUNT,
    REGR_INTERCEPT,
    REGR_R2,
    REGR_SLOPE,
    REGR_SXX,
    REGR_SXY,
    REGR_SYY,
    RELEASE,
    RENAME,
    REPAIR,
    REPEATABLE,
    REPLACE,
    RESTRICT,
    RESULT,
    RETURN,
    RETURNS,
    REVOKE,
    RIGHT,
    ROLLBACK,
    ROLLUP,
    ROW,
    ROWID,
    ROWS,
    ROW_NUMBER,
    SAVEPOINT,
    SCHEMA,
    SCOPE,
    SCROLL,
    SEARCH,
    SECOND,
    SELECT,
    SENSITIVE,
    SEQUENCEFILE,
    SERDE,
    SERIALIZABLE,
    SESSION,
    SESSION_USER,
    SET,
    SHOW,
    SIMILAR,
    SMALLINT,
    SOME,
    SORT,
    SPECIFIC,
    SPECIFICTYPE,
    SQL,
    SQLEXCEPTION,
    SQLSTATE,
    SQLWARNING,
    SQRT,
    START,
    STATIC,
    STATISTICS,
    STDDEV_POP,
    STDDEV_SAMP,
    STDIN,
    STORED,
    STRING,
    SUBMULTISET,
    SUBSTRING,
    SUBSTRING_REGEX,
    SUCCEEDS,
    SUM,
    SYMMETRIC,
    SYNC,
    SYSTEM,
    SYSTEM_TIME,
    SYSTEM_USER,
    TABLE,
    TABLESAMPLE,
    TBLPROPERTIES,
    TEMP,
    TEMPORARY,
    TEXT,
    TEXTFILE,
    THEN,
    TIES,
    TIME,
    TIMESTAMP,
    TIMEZONE_HOUR,
    TIMEZONE_MINUTE,
    TO,
    TOP,
    TRAILING,
    TRANSACTION,
    TRANSLATE,
    TRANSLATE_REGEX,
    TRANSLATION,
    TREAT,
    TRIGGER,
    TRIM,
    TRIM_ARRAY,
    TRUE,
    TRUNCATE,
    TRY_CAST,
    UESCAPE,
    UNBOUNDED,
    UNCOMMITTED,
    UNION,
    UNIQUE,
    UNKNOWN,
    UNNEST,
    UPDATE,
    UPPER,
    USER,
    USING,
    UUID,
    VALUE,
    VALUES,
    VALUE_OF,
    VARBINARY,
    VARCHAR,
    VARYING,
    VAR_POP,
    VAR_SAMP,
    VERBOSE,
    VERSIONING,
    VIEW,
    VIRTUAL,
    WHEN,
    WHENEVER,
    WHERE,
    WIDTH_BUCKET,
    WINDOW,
    WITH,
    WITHIN,
    WITHOUT,
    WORK,
    WRITE,
    YEAR,
    ZONE
}