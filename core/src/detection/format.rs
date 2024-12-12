#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Format {
    Ai,
    Apk,
    ApplePlist,
    Asm,
    Asp,
    Batch,
    Bmp,
    Bzip,
    C,
    Cab,
    Cat,
    Chm,
    Coff,
    Crx,
    Cs,
    Css,
    Csv,
    Deb,
    Dex,
    Dmg,
    Doc,
    Docx,
    Elf,
    Emf,
    Eml,
    Epub,
    Flac,
    Gif,
    Go,
    Gzip,
    Hlp,
    Html,
    Ico,
    Ini,
    InternetShortcut,
    Iso,
    Jar,
    Java,
    JavaBytecode,
    JavaScript,
    Jpeg,
    Json,
    Latex,
    Lisp,
    Lnk,
    M3u,
    Macho,
    Makefile,
    Markdown,
    Mht,
    Mp3,
    Mp4,
    MsCompress,
    Msi,
    Mum,
    Odex,
    Odp,
    Ods,
    Odt,
    Ogg,
    Outlook,
    Pcap,
    Pdf,
    PeBin,
    Pem,
    Perl,
    Php,
    Png,
    PostScript,
    PowerShell,
    Ppt,
    Pptx,
    Python,
    PythonBytecode,
    Rar,
    Rdf,
    Rpm,
    Rst,
    Rtf,
    Ruby,
    Rust,
    Scala,
    SevenZip,
    Shell,
    Smali,
    Sql,
    Squashfs,
    Svg,
    Swf,
    SymlinkText,
    Tar,
    Tga,
    Tiff,
    Torrent,
    Ttf,
    Txt,
    Unknown,
    Vba,
    Wav,
    Webm,
    Webp,
    WinRegistry,
    Wmf,
    Xar,
    Xls,
    Xlsb,
    Xlsx,
    Xml,
    Xpi,
    Xz,
    Yaml,
    Zip,
    ZlibStream,
}

pub const FORMATS: [Format; 113] = [
    Format::Ai,
    Format::Apk,
    Format::ApplePlist,
    Format::Asm,
    Format::Asp,
    Format::Batch,
    Format::Bmp,
    Format::Bzip,
    Format::C,
    Format::Cab,
    Format::Cat,
    Format::Chm,
    Format::Coff,
    Format::Crx,
    Format::Cs,
    Format::Css,
    Format::Csv,
    Format::Deb,
    Format::Dex,
    Format::Dmg,
    Format::Doc,
    Format::Docx,
    Format::Elf,
    Format::Emf,
    Format::Eml,
    Format::Epub,
    Format::Flac,
    Format::Gif,
    Format::Go,
    Format::Gzip,
    Format::Hlp,
    Format::Html,
    Format::Ico,
    Format::Ini,
    Format::InternetShortcut,
    Format::Iso,
    Format::Jar,
    Format::Java,
    Format::JavaBytecode,
    Format::JavaScript,
    Format::Jpeg,
    Format::Json,
    Format::Latex,
    Format::Lisp,
    Format::Lnk,
    Format::M3u,
    Format::Macho,
    Format::Makefile,
    Format::Markdown,
    Format::Mht,
    Format::Mp3,
    Format::Mp4,
    Format::MsCompress,
    Format::Msi,
    Format::Mum,
    Format::Odex,
    Format::Odp,
    Format::Ods,
    Format::Odt,
    Format::Ogg,
    Format::Outlook,
    Format::Pcap,
    Format::Pdf,
    Format::PeBin,
    Format::Pem,
    Format::Perl,
    Format::Php,
    Format::Png,
    Format::PostScript,
    Format::PowerShell,
    Format::Ppt,
    Format::Pptx,
    Format::Python,
    Format::PythonBytecode,
    Format::Rar,
    Format::Rdf,
    Format::Rpm,
    Format::Rst,
    Format::Rtf,
    Format::Ruby,
    Format::Rust,
    Format::Scala,
    Format::SevenZip,
    Format::Shell,
    Format::Smali,
    Format::Sql,
    Format::Squashfs,
    Format::Svg,
    Format::Swf,
    Format::SymlinkText,
    Format::Tar,
    Format::Tga,
    Format::Tiff,
    Format::Torrent,
    Format::Ttf,
    Format::Txt,
    Format::Unknown,
    Format::Vba,
    Format::Wav,
    Format::Webm,
    Format::Webp,
    Format::WinRegistry,
    Format::Wmf,
    Format::Xar,
    Format::Xls,
    Format::Xlsb,
    Format::Xlsx,
    Format::Xml,
    Format::Xpi,
    Format::Xz,
    Format::Yaml,
    Format::Zip,
    Format::ZlibStream,
];

impl Format {
    pub fn is_text(&self) -> bool {
        match self {
            Format::Ai => false,
            Format::Apk => false,
            Format::ApplePlist => true,
            Format::Asm => true,
            Format::Asp => true,
            Format::Batch => true,
            Format::Bmp => false,
            Format::Bzip => false,
            Format::C => true,
            Format::Cab => false,
            Format::Cat => false,
            Format::Chm => false,
            Format::Coff => false,
            Format::Crx => false,
            Format::Cs => true,
            Format::Css => true,
            Format::Csv => true,
            Format::Deb => false,
            Format::Dex => false,
            Format::Dmg => false,
            Format::Doc => false,
            Format::Docx => false,
            Format::Elf => false,
            Format::Emf => false,
            Format::Eml => true,
            Format::Epub => false,
            Format::Flac => false,
            Format::Gif => false,
            Format::Go => true,
            Format::Gzip => false,
            Format::Hlp => false,
            Format::Html => true,
            Format::Ico => false,
            Format::Ini => true,
            Format::InternetShortcut => true,
            Format::Iso => false,
            Format::Jar => false,
            Format::Java => true,
            Format::JavaBytecode => false,
            Format::JavaScript => true,
            Format::Jpeg => false,
            Format::Json => true,
            Format::Latex => true,
            Format::Lisp => true,
            Format::Lnk => false,
            Format::M3u => false,
            Format::Macho => false,
            Format::Makefile => true,
            Format::Markdown => true,
            Format::Mht => true,
            Format::Mp3 => false,
            Format::Mp4 => false,
            Format::MsCompress => false,
            Format::Msi => false,
            Format::Mum => true,
            Format::Odex => false,
            Format::Odp => false,
            Format::Ods => false,
            Format::Odt => false,
            Format::Ogg => false,
            Format::Outlook => false,
            Format::Pcap => false,
            Format::Pdf => false,
            Format::PeBin => false,
            Format::Pem => true,
            Format::Perl => true,
            Format::Php => true,
            Format::Png => false,
            Format::PostScript => false,
            Format::PowerShell => true,
            Format::Ppt => false,
            Format::Pptx => false,
            Format::Python => true,
            Format::PythonBytecode => false,
            Format::Rar => false,
            Format::Rdf => true,
            Format::Rpm => false,
            Format::Rst => true,
            Format::Rtf => true,
            Format::Ruby => true,
            Format::Rust => true,
            Format::Scala => true,
            Format::SevenZip => false,
            Format::Shell => true,
            Format::Smali => true,
            Format::Sql => true,
            Format::Squashfs => false,
            Format::Svg => true,
            Format::Swf => false,
            Format::SymlinkText => true,
            Format::Tar => false,
            Format::Tga => false,
            Format::Tiff => false,
            Format::Torrent => false,
            Format::Ttf => false,
            Format::Txt => true,
            Format::Unknown => false,
            Format::Vba => true,
            Format::Wav => false,
            Format::Webm => false,
            Format::Webp => false,
            Format::WinRegistry => true,
            Format::Wmf => false,
            Format::Xar => false,
            Format::Xls => false,
            Format::Xlsb => false,
            Format::Xlsx => false,
            Format::Xml => true,
            Format::Xpi => false,
            Format::Xz => false,
            Format::Yaml => true,
            Format::Zip => false,
            Format::ZlibStream => false,
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Format::Ai => "Ai",
            Format::Apk => "Apk",
            Format::ApplePlist => "ApplePlist",
            Format::Asm => "Asm",
            Format::Asp => "Asp",
            Format::Batch => "Batch",
            Format::Bmp => "Bmp",
            Format::Bzip => "Bzip",
            Format::C => "C",
            Format::Cab => "Cab",
            Format::Cat => "Cat",
            Format::Chm => "Chm",
            Format::Coff => "Coff",
            Format::Crx => "Crx",
            Format::Cs => "Cs",
            Format::Css => "Css",
            Format::Csv => "Csv",
            Format::Deb => "Deb",
            Format::Dex => "Dex",
            Format::Dmg => "Dmg",
            Format::Doc => "Doc",
            Format::Docx => "Docx",
            Format::Elf => "Elf",
            Format::Emf => "Emf",
            Format::Eml => "Eml",
            Format::Epub => "Epub",
            Format::Flac => "Flac",
            Format::Gif => "Gif",
            Format::Go => "Go",
            Format::Gzip => "Gzip",
            Format::Hlp => "Hlp",
            Format::Html => "Html",
            Format::Ico => "Ico",
            Format::Ini => "Ini",
            Format::InternetShortcut => "InternetShortcut",
            Format::Iso => "Iso",
            Format::Jar => "Jar",
            Format::Java => "Java",
            Format::JavaBytecode => "JavaBytecode",
            Format::JavaScript => "JavaScript",
            Format::Jpeg => "Jpeg",
            Format::Json => "Json",
            Format::Latex => "Latex",
            Format::Lisp => "Lisp",
            Format::Lnk => "Lnk",
            Format::M3u => "M3u",
            Format::Macho => "Macho",
            Format::Makefile => "Makefile",
            Format::Markdown => "Markdown",
            Format::Mht => "Mht",
            Format::Mp3 => "Mp3",
            Format::Mp4 => "Mp4",
            Format::MsCompress => "MsCompress",
            Format::Msi => "Msi",
            Format::Mum => "Mum",
            Format::Odex => "Odex",
            Format::Odp => "Odp",
            Format::Ods => "Ods",
            Format::Odt => "Odt",
            Format::Ogg => "Ogg",
            Format::Outlook => "Outlook",
            Format::Pcap => "Pcap",
            Format::Pdf => "Pdf",
            Format::PeBin => "PeBin",
            Format::Pem => "Pem",
            Format::Perl => "Perl",
            Format::Php => "Php",
            Format::Png => "Png",
            Format::PostScript => "PostScript",
            Format::PowerShell => "PowerShell",
            Format::Ppt => "Ppt",
            Format::Pptx => "Pptx",
            Format::Python => "Python",
            Format::PythonBytecode => "PythonBytecode",
            Format::Rar => "Rar",
            Format::Rdf => "Rdf",
            Format::Rpm => "Rpm",
            Format::Rst => "Rst",
            Format::Rtf => "Rtf",
            Format::Ruby => "Ruby",
            Format::Rust => "Rust",
            Format::Scala => "Scala",
            Format::SevenZip => "SevenZip",
            Format::Shell => "Shell",
            Format::Smali => "Smali",
            Format::Sql => "Sql",
            Format::Squashfs => "Squashfs",
            Format::Svg => "Svg",
            Format::Swf => "Swf",
            Format::SymlinkText => "SymlinkText",
            Format::Tar => "Tar",
            Format::Tga => "Tga",
            Format::Tiff => "Tiff",
            Format::Torrent => "Torrent",
            Format::Ttf => "Ttf",
            Format::Txt => "Txt",
            Format::Unknown => "Unknown",
            Format::Vba => "Vba",
            Format::Wav => "Wav",
            Format::Webm => "Webm",
            Format::Webp => "Webp",
            Format::WinRegistry => "WinRegistry",
            Format::Wmf => "Wmf",
            Format::Xar => "Xar",
            Format::Xls => "Xls",
            Format::Xlsb => "Xlsb",
            Format::Xlsx => "Xlsx",
            Format::Xml => "Xml",
            Format::Xpi => "Xpi",
            Format::Xz => "Xz",
            Format::Yaml => "Yaml",
            Format::Zip => "Zip",
            Format::ZlibStream => "ZlibStream",
        }
    }
}
