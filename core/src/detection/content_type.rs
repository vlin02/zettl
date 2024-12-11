pub enum ContentType {
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

pub const CONTENT_TYPES: [ContentType; 113] = [
  ContentType::Ai,
  ContentType::Apk,
  ContentType::ApplePlist,
  ContentType::Asm,
  ContentType::Asp,
  ContentType::Batch,
  ContentType::Bmp,
  ContentType::Bzip,
  ContentType::C,
  ContentType::Cab,
  ContentType::Cat,
  ContentType::Chm,
  ContentType::Coff,
  ContentType::Crx,
  ContentType::Cs,
  ContentType::Css,
  ContentType::Csv,
  ContentType::Deb,
  ContentType::Dex,
  ContentType::Dmg,
  ContentType::Doc,
  ContentType::Docx,
  ContentType::Elf,
  ContentType::Emf,
  ContentType::Eml,
  ContentType::Epub,
  ContentType::Flac,
  ContentType::Gif,
  ContentType::Go,
  ContentType::Gzip,
  ContentType::Hlp,
  ContentType::Html,
  ContentType::Ico,
  ContentType::Ini,
  ContentType::InternetShortcut,
  ContentType::Iso,
  ContentType::Jar,
  ContentType::Java,
  ContentType::JavaBytecode,
  ContentType::JavaScript,
  ContentType::Jpeg,
  ContentType::Json,
  ContentType::Latex,
  ContentType::Lisp,
  ContentType::Lnk,
  ContentType::M3u,
  ContentType::Macho,
  ContentType::Makefile,
  ContentType::Markdown,
  ContentType::Mht,
  ContentType::Mp3,
  ContentType::Mp4,
  ContentType::MsCompress,
  ContentType::Msi,
  ContentType::Mum,
  ContentType::Odex,
  ContentType::Odp,
  ContentType::Ods,
  ContentType::Odt,
  ContentType::Ogg,
  ContentType::Outlook,
  ContentType::Pcap,
  ContentType::Pdf,
  ContentType::PeBin,
  ContentType::Pem,
  ContentType::Perl,
  ContentType::Php,
  ContentType::Png,
  ContentType::PostScript,
  ContentType::PowerShell,
  ContentType::Ppt,
  ContentType::Pptx,
  ContentType::Python,
  ContentType::PythonBytecode,
  ContentType::Rar,
  ContentType::Rdf,
  ContentType::Rpm,
  ContentType::Rst,
  ContentType::Rtf,
  ContentType::Ruby,
  ContentType::Rust,
  ContentType::Scala,
  ContentType::SevenZip,
  ContentType::Shell,
  ContentType::Smali,
  ContentType::Sql,
  ContentType::Squashfs,
  ContentType::Svg,
  ContentType::Swf,
  ContentType::SymlinkText,
  ContentType::Tar,
  ContentType::Tga,
  ContentType::Tiff,
  ContentType::Torrent,
  ContentType::Ttf,
  ContentType::Txt,
  ContentType::Unknown,
  ContentType::Vba,
  ContentType::Wav,
  ContentType::Webm,
  ContentType::Webp,
  ContentType::WinRegistry,
  ContentType::Wmf,
  ContentType::Xar,
  ContentType::Xls,
  ContentType::Xlsb,
  ContentType::Xlsx,
  ContentType::Xml,
  ContentType::Xpi,
  ContentType::Xz,
  ContentType::Yaml,
  ContentType::Zip,
  ContentType::ZlibStream,
];

pub const IS_TEXT: [bool; 113] = [
  false, // Ai
  false, // Apk
  true,  // ApplePlist
  true,  // Asm
  true,  // Asp
  true,  // Batch
  false, // Bmp
  false, // Bzip
  true,  // C
  false, // Cab
  false, // Cat
  false, // Chm
  false, // Coff
  false, // Crx
  true,  // Cs
  true,  // Css
  true,  // Csv
  false, // Deb
  false, // Dex
  false, // Dmg
  false, // Doc
  false, // Docx
  false, // Elf
  false, // Emf
  true,  // Eml
  false, // Epub
  false, // Flac
  false, // Gif
  true,  // Go
  false, // Gzip
  false, // Hlp
  true,  // Html
  false, // Ico
  true,  // Ini
  true,  // InternetShortcut
  false, // Iso
  false, // Jar
  true,  // Java
  false, // JavaBytecode
  true,  // JavaScript
  false, // Jpeg
  true,  // Json
  true,  // Latex
  true,  // Lisp
  false, // Lnk
  false, // M3u
  false, // Macho
  true,  // Makefile
  true,  // Markdown
  true,  // Mht
  false, // Mp3
  false, // Mp4
  false, // MsCompress
  false, // Msi
  true,  // Mum
  false, // Odex
  false, // Odp
  false, // Ods
  false, // Odt
  false, // Ogg
  false, // Outlook
  false, // Pcap
  false, // Pdf
  false, // PeBin
  true,  // Pem
  true,  // Perl
  true,  // Php
  false, // Png
  false, // PostScript
  true,  // PowerShell
  false, // Ppt
  false, // Pptx
  true,  // Python
  false, // PythonBytecode
  false, // Rar
  true,  // Rdf
  false, // Rpm
  true,  // Rst
  true,  // Rtf
  true,  // Ruby
  true,  // Rust
  true,  // Scala
  false, // SevenZip
  true,  // Shell
  true,  // Smali
  true,  // Sql
  false, // Squashfs
  true,  // Svg
  false, // Swf
  true,  // SymlinkText
  false, // Tar
  false, // Tga
  false, // Tiff
  false, // Torrent
  false, // Ttf
  true,  // Txt
  false, // Unknown
  true,  // Vba
  false, // Wav
  false, // Webm
  false, // Webp
  true,  // WinRegistry
  false, // Wmf
  false, // Xar
  false, // Xls
  false, // Xlsb
  false, // Xlsx
  true,  // Xml
  false, // Xpi
  false, // Xz
  true,  // Yaml
  false, // Zip
  false, // ZlibStream
];