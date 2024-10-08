#[derive(Debug, Clone, Copy)]
pub enum InputFormat {
    TabSeparated,
    TabSeparatedRaw,
    TabSeparatedWithNames,
    TabSeparatedWithNamesAndTypes,
    TabSeparatedRawWithNames,
    TabSeparatedRawWithNamesAndTypes,
    Template,
    TemplateIgnoreSpaces,
    CSV,
    CSVWithNames,
    CSVWithNamesAndTypes,
    CustomSeparated,
    CustomSeparatedWithNames,
    CustomSeparatedWithNamesAndTypes,
    Values,
    JSON,
    JSONAsString,
    JSONAsObject,
    JSONStrings,
    JSONColumns,
    JSONColumnsWithMetadata,
    JSONCompact,
    JSONCompactColumns,
    JSONEachRow,
    JSONStringsEachRow,
    JSONCompactEachRow,
    JSONCompactEachRowWithNames,
    JSONCompactEachRowWithNamesAndTypes,
    JSONCompactStringsEachRow,
    JSONCompactStringsEachRowWithNames,
    JSONCompactStringsEachRowWithNamesAndTypes,
    JSONObjectEachRow,
    BSONEachRow,
    TSKV,
    Protobuf,
    ProtobufSingle,
    ProtobufList,
    Avro,
    AvroConfluent,
    Parquet,
    ParquetMetadata,
    Arrow,
    ArrowStream,
    ORC,
    One,
    Npy,
    RowBinary,
    RowBinaryWithNames,
    RowBinaryWithNamesAndTypes,
    RowBinaryWithDefaults,
    Native,
    CapnProto,
    LineAsString,
    Regexp,
    RawBLOB,
    MsgPack,
    MySQLDump,
    DWARF,
    Form,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    TabSeparated,
    TabSeparatedRaw,
    TabSeparatedWithNames,
    TabSeparatedWithNamesAndTypes,
    TabSeparatedRawWithNames,
    TabSeparatedRawWithNamesAndTypes,
    Template,
    CSV,
    CSVWithNames,
    CSVWithNamesAndTypes,
    CustomSeparated,
    CustomSeparatedWithNames,
    CustomSeparatedWithNamesAndTypes,
    Values,
    JSON,
    JSONStrings,
    JSONColumns,
    JSONColumnsWithMetadata,
    JSONCompact,
    JSONCompactStrings,
    JSONCompactColumns,
    JSONEachRow,
    PrettyJSONEachRow,
    JSONEachRowWithProgress,
    JSONStringsEachRow,
    JSONStringsEachRowWithProgress,
    JSONCompactEachRow,
    JSONCompactEachRowWithNames,
    JSONCompactEachRowWithNamesAndTypes,
    JSONCompactStringsEachRow,
    JSONCompactStringsEachRowWithNames,
    JSONCompactStringsEachRowWithNamesAndTypes,
    JSONObjectEachRow,
    BSONEachRow,
    TSKV,
    Pretty,
    PrettyNoEscapes,
    PrettyMonoBlock,
    PrettyNoEscapesMonoBlock,
    PrettyCompact,
    PrettyCompactNoEscapes,
    PrettyCompactMonoBlock,
    PrettyCompactNoEscapesMonoBlock,
    PrettySpace,
    PrettySpaceNoEscapes,
    PrettySpaceMonoBlock,
    PrettySpaceNoEscapesMonoBlock,
    Prometheus,
    Protobuf,
    ProtobufSingle,
    ProtobufList,
    Avro,
    Parquet,
    Arrow,
    ArrowStream,
    ORC,
    Npy,
    RowBinary,
    RowBinaryWithNames,
    RowBinaryWithNamesAndTypes,
    Native,
    Null,
    XML,
    CapnProto,
    LineAsString,
    RawBLOB,
    MsgPack,
    Markdown,
    Vertical,
}

impl InputFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabSeparated => "TabSeparated",
            Self::TabSeparatedRaw => "TabSeparatedRaw",
            Self::TabSeparatedWithNames => "TabSeparatedWithNames",
            Self::TabSeparatedWithNamesAndTypes => "TabSeparatedWithNamesAndTypes",
            Self::TabSeparatedRawWithNames => "TabSeparatedRawWithNames",
            Self::TabSeparatedRawWithNamesAndTypes => "TabSeparatedRawWithNamesAndTypes",
            Self::Template => "Template",
            Self::TemplateIgnoreSpaces => "TemplateIgnoreSpaces",
            Self::CSV => "CSV",
            Self::CSVWithNames => "CSVWithNames",
            Self::CSVWithNamesAndTypes => "CSVWithNamesAndTypes",
            Self::CustomSeparated => "CustomSeparated",
            Self::CustomSeparatedWithNames => "CustomSeparatedWithNames",
            Self::CustomSeparatedWithNamesAndTypes => "CustomSeparatedWithNamesAndTypes",
            Self::Values => "Values",
            Self::JSON => "JSON",
            Self::JSONAsString => "JSONAsString",
            Self::JSONAsObject => "JSONAsObject",
            Self::JSONStrings => "JSONStrings",
            Self::JSONColumns => "JSONColumns",
            Self::JSONColumnsWithMetadata => "JSONColumnsWithMetadata",
            Self::JSONCompact => "JSONCompact",
            Self::JSONCompactColumns => "JSONCompactColumns",
            Self::JSONEachRow => "JSONEachRow",
            Self::JSONStringsEachRow => "JSONStringsEachRow",
            Self::JSONCompactEachRow => "JSONCompactEachRow",
            Self::JSONCompactEachRowWithNames => "JSONCompactEachRowWithNames",
            Self::JSONCompactEachRowWithNamesAndTypes => "JSONCompactEachRowWithNamesAndTypes",
            Self::JSONCompactStringsEachRow => "JSONCompactStringsEachRow",
            Self::JSONCompactStringsEachRowWithNames => "JSONCompactStringsEachRowWithNames",
            Self::JSONCompactStringsEachRowWithNamesAndTypes => {
                "JSONCompactStringsEachRowWithNamesAndTypes"
            }
            Self::JSONObjectEachRow => "JSONObjectEachRow",
            Self::BSONEachRow => "BSONEachRow",
            Self::TSKV => "TSKV",
            Self::Protobuf => "Protobuf",
            Self::ProtobufSingle => "ProtobufSingle",
            Self::ProtobufList => "ProtobufList",
            Self::Avro => "Avro",
            Self::AvroConfluent => "AvroConfluent",
            Self::Parquet => "Parquet",
            Self::ParquetMetadata => "ParquetMetadata",
            Self::Arrow => "Arrow",
            Self::ArrowStream => "ArrowStream",
            Self::ORC => "ORC",
            Self::One => "One",
            Self::Npy => "Npy",
            Self::RowBinary => "RowBinary",
            Self::RowBinaryWithNames => "RowBinaryWithNames",
            Self::RowBinaryWithNamesAndTypes => "RowBinaryWithNamesAndTypes",
            Self::RowBinaryWithDefaults => "RowBinaryWithDefaults",
            Self::Native => "Native",
            Self::CapnProto => "CapnProto",
            Self::LineAsString => "LineAsString",
            Self::Regexp => "Regexp",
            Self::RawBLOB => "RawBLOB",
            Self::MsgPack => "MsgPack",
            Self::MySQLDump => "MySQLDump",
            Self::DWARF => "DWARF",
            Self::Form => "Form",
        }
    }
}

impl OutputFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabSeparated => "TabSeparated",
            Self::TabSeparatedRaw => "TabSeparatedRaw",
            Self::TabSeparatedWithNames => "TabSeparatedWithNames",
            Self::TabSeparatedWithNamesAndTypes => "TabSeparatedWithNamesAndTypes",
            Self::TabSeparatedRawWithNames => "TabSeparatedRawWithNames",
            Self::TabSeparatedRawWithNamesAndTypes => "TabSeparatedRawWithNamesAndTypes",
            Self::Template => "Template",
            Self::CSV => "CSV",
            Self::CSVWithNames => "CSVWithNames",
            Self::CSVWithNamesAndTypes => "CSVWithNamesAndTypes",
            Self::CustomSeparated => "CustomSeparated",
            Self::CustomSeparatedWithNames => "CustomSeparatedWithNames",
            Self::CustomSeparatedWithNamesAndTypes => "CustomSeparatedWithNamesAndTypes",
            Self::Values => "Values",
            Self::JSON => "JSON",
            Self::JSONStrings => "JSONStrings",
            Self::JSONColumns => "JSONColumns",
            Self::JSONColumnsWithMetadata => "JSONColumnsWithMetadata",
            Self::JSONCompact => "JSONCompact",
            Self::JSONCompactStrings => "JSONCompactStrings",
            Self::JSONCompactColumns => "JSONCompactColumns",
            Self::JSONEachRow => "JSONEachRow",
            Self::PrettyJSONEachRow => "PrettyJSONEachRow",
            Self::JSONEachRowWithProgress => "JSONEachRowWithProgress",
            Self::JSONStringsEachRow => "JSONStringsEachRow",
            Self::JSONStringsEachRowWithProgress => "JSONStringsEachRowWithProgress",
            Self::JSONCompactEachRow => "JSONCompactEachRow",
            Self::JSONCompactEachRowWithNames => "JSONCompactEachRowWithNames",
            Self::JSONCompactEachRowWithNamesAndTypes => "JSONCompactEachRowWithNamesAndTypes",
            Self::JSONCompactStringsEachRow => "JSONCompactStringsEachRow",
            Self::JSONCompactStringsEachRowWithNames => "JSONCompactStringsEachRowWithNames",
            Self::JSONCompactStringsEachRowWithNamesAndTypes => {
                "JSONCompactStringsEachRowWithNamesAndTypes"
            }
            Self::JSONObjectEachRow => "JSONObjectEachRow",
            Self::BSONEachRow => "BSONEachRow",
            Self::TSKV => "TSKV",
            Self::Pretty => "Pretty",
            Self::PrettyNoEscapes => "PrettyNoEscapes",
            Self::PrettyMonoBlock => "PrettyMonoBlock",
            Self::PrettyNoEscapesMonoBlock => "PrettyNoEscapesMonoBlock",
            Self::PrettyCompact => "PrettyCompact",
            Self::PrettyCompactNoEscapes => "PrettyCompactNoEscapes",
            Self::PrettyCompactMonoBlock => "PrettyCompactMonoBlock",
            Self::PrettyCompactNoEscapesMonoBlock => "PrettyCompactNoEscapesMonoBlock",
            Self::PrettySpace => "PrettySpace",
            Self::PrettySpaceNoEscapes => "PrettySpaceNoEscapes",
            Self::PrettySpaceMonoBlock => "PrettySpaceMonoBlock",
            Self::PrettySpaceNoEscapesMonoBlock => "PrettySpaceNoEscapesMonoBlock",
            Self::Prometheus => "Prometheus",
            Self::Protobuf => "Protobuf",
            Self::ProtobufSingle => "ProtobufSingle",
            Self::ProtobufList => "ProtobufList",
            Self::Avro => "Avro",
            Self::Parquet => "Parquet",
            Self::Arrow => "Arrow",
            Self::ArrowStream => "ArrowStream",
            Self::ORC => "ORC",
            Self::Npy => "Npy",
            Self::RowBinary => "RowBinary",
            Self::RowBinaryWithNames => "RowBinaryWithNames",
            Self::RowBinaryWithNamesAndTypes => "RowBinaryWithNamesAndTypes",
            Self::Native => "Native",
            Self::Null => "Null",
            Self::XML => "XML",
            Self::CapnProto => "CapnProto",
            Self::LineAsString => "LineAsString",
            Self::RawBLOB => "RawBLOB",
            Self::MsgPack => "MsgPack",
            Self::Markdown => "Markdown",
            Self::Vertical => "Vertical",
        }
    }
}
