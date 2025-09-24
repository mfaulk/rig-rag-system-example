use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cfrdoc {
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "@noNamespaceSchemaLocation")]
    pub xsi_no_namespace_schema_location: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AMDDATE")]
    pub amddate: String,
    #[serde(rename = "FMTR")]
    pub fmtr: Fmtr,
    #[serde(rename = "TITLE")]
    pub title: Title,
    #[serde(rename = "BMTR")]
    pub bmtr: Bmtr,
}

#[derive(Serialize, Deserialize)]
pub struct Fmtr {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TITLEPG")]
    pub titlepg: Titlepg,
    #[serde(rename = "BTITLE")]
    pub btitle: Btitle,
    #[serde(rename = "TOC")]
    pub toc: FmtrToc,
    #[serde(rename = "CITE")]
    pub cite: Cite,
    #[serde(rename = "EXPLA")]
    pub expla: Expla,
    #[serde(rename = "THISTITL")]
    pub thistitl: Thistitl,
}

#[derive(Serialize, Deserialize)]
pub struct Titlepg {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CODE")]
    pub code: Code,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocFmtrTitlepgPrtpage,
    #[serde(rename = "TITLENUM")]
    pub titlenum: String,
    #[serde(rename = "SUBJECT")]
    pub subject: String,
    #[serde(rename = "PARTS")]
    pub parts: String,
    #[serde(rename = "REVISED")]
    pub revised: String,
    #[serde(rename = "CONTAINS")]
    pub contains: String,
    #[serde(rename = "DATE")]
    pub date: String,
    #[serde(rename = "PUB")]
    pub titlepg_pub: Pub,
}

#[derive(Serialize, Deserialize)]
pub struct Code {
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrTitlepgPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Pub {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Btitle {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocFmtrBtitlePrtpage,
    #[serde(rename = "OENOTICE")]
    pub oenotice: Oenotice,
    #[serde(rename = "GPO")]
    pub gpo: Gpo,
    #[serde(rename = "SUDOCS")]
    pub sudocs: Sudocs,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrBtitlePrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Oenotice {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: Vec<FmtrBtitleOenoticeHd>,
    #[serde(rename = "GPH")]
    pub gph: OenoticeGph,
    #[serde(rename = "P")]
    pub p: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrBtitleOenoticeHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OenoticeGph {
    #[serde(rename = "@SPAN")]
    pub span: String,
    #[serde(rename = "@DEEP")]
    pub deep: String,
    #[serde(rename = "@HTYPE")]
    pub htype: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "GID")]
    pub gid: String,
}

#[derive(Serialize, Deserialize)]
pub struct Gpo {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "GPH")]
    pub gph: GpoGph,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct GpoGph {
    #[serde(rename = "@SPAN")]
    pub span: String,
    #[serde(rename = "@DEEP")]
    pub deep: String,
    #[serde(rename = "@HTYPE")]
    pub htype: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "GID")]
    pub gid: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sudocs {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "P")]
    pub p: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrToc {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocFmtrTocPrtpage,
    #[serde(rename = "HD")]
    pub hd: CfrdocFmtrTocHd,
    #[serde(rename = "PGHD")]
    pub pghd: String,
    #[serde(rename = "EXPL")]
    pub expl: Expl,
    #[serde(rename = "TITLENO")]
    pub titleno: TocTitleno,
    #[serde(rename = "FAIDS")]
    pub faids: TocFaids,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrTocPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrTocHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Expl {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SUBJECT")]
    pub subject: String,
    #[serde(rename = "PG")]
    pub pg: String,
}

#[derive(Serialize, Deserialize)]
pub struct TocTitleno {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: FmtrTocTitlenoHd,
    #[serde(rename = "CHAPTI")]
    pub chapti: TitlenoChapti,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrTocTitlenoHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TitlenoChapti {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SUBJECT")]
    pub subject: String,
    #[serde(rename = "PG")]
    pub pg: String,
}

#[derive(Serialize, Deserialize)]
pub struct TocFaids {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: FmtrTocFaidsHd,
    #[serde(rename = "SUBJECT")]
    pub subject: Vec<String>,
    #[serde(rename = "PG")]
    pub pg: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrTocFaidsHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Cite {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocFmtrCitePrtpage,
    #[serde(rename = "P")]
    pub p: CfrdocFmtrCiteP,
    #[serde(rename = "CITEP")]
    pub citep: Citep,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrCitePrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrCiteP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: FmtrCitePE,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrCitePE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Citep {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: FmtrCiteCitepE,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrCiteCitepE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Expla {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocFmtrExplaPrtpage,
    #[serde(rename = "HD")]
    pub hd: CfrdocFmtrExplaHd,
    #[serde(rename = "P")]
    pub p: Vec<String>,
    #[serde(rename = "IPAR")]
    pub ipar: Ipar,
    #[serde(rename = "SIDEHED")]
    pub sidehed: Vec<Sidehed>,
    #[serde(rename = "SIG")]
    pub sig: Sig,
    #[serde(rename = "DATE")]
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrExplaPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrExplaHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Ipar {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "P")]
    pub p: Vec<FmtrExplaIparP>,
    #[serde(rename = "STUB")]
    pub stub: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrExplaIparP {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Sidehed {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: FmtrExplaSidehedHd,
    #[serde(rename = "P")]
    pub p: Vec<FmtrExplaSidehedP>,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrExplaSidehedHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrExplaSidehedP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<ExplaSidehedPPrtpage>,
    #[serde(rename = "E")]
    pub e: Option<Vec<ExplaSidehedPE>>,
}

#[derive(Serialize, Deserialize)]
pub struct ExplaSidehedPPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExplaSidehedPE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Sig {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "POSITION")]
    pub position: String,
    #[serde(rename = "OFFICE")]
    pub office: String,
}

#[derive(Serialize, Deserialize)]
pub struct Thistitl {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocFmtrThistitlPrtpage,
    #[serde(rename = "HD")]
    pub hd: CfrdocFmtrThistitlHd,
    #[serde(rename = "P")]
    pub p: Vec<CfrdocFmtrThistitlP>,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrThistitlPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrThistitlHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocFmtrThistitlP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<FmtrThistitlPE>,
}

#[derive(Serialize, Deserialize)]
pub struct FmtrThistitlPE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LRH")]
    pub lrh: String,
    #[serde(rename = "RRH")]
    pub rrh: String,
    #[serde(rename = "CFRTITLE")]
    pub cfrtitle: Cfrtitle,
    #[serde(rename = "CHAPTER")]
    pub chapter: Chapter,
}

#[derive(Serialize, Deserialize)]
pub struct Cfrtitle {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TITLEHD")]
    pub titlehd: Titlehd,
    #[serde(rename = "CFRTOC")]
    pub cfrtoc: Cfrtoc,
}

#[derive(Serialize, Deserialize)]
pub struct Titlehd {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: TitleCfrtitleTitlehdPrtpage,
    #[serde(rename = "HD")]
    pub hd: TitleCfrtitleTitlehdHd,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct TitleCfrtitleTitlehdPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct TitleCfrtitleTitlehdHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Cfrtoc {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PTHD")]
    pub pthd: String,
    #[serde(rename = "CHAPTI")]
    pub chapti: CfrtocChapti,
}

#[derive(Serialize, Deserialize)]
pub struct CfrtocChapti {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SUBJECT")]
    pub subject: CfrtocChaptiSubject,
    #[serde(rename = "PG")]
    pub pg: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrtocChaptiSubject {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: CfrtocChaptiSubjectE,
}

#[derive(Serialize, Deserialize)]
pub struct CfrtocChaptiSubjectE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TOC")]
    pub toc: ChapterToc,
    #[serde(rename = "SUBCHAP")]
    pub subchap: ChapterSubchap,
}

#[derive(Serialize, Deserialize)]
pub struct ChapterToc {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TOCHD")]
    pub tochd: Tochd,
    #[serde(rename = "EDNOTE")]
    pub ednote: TocEdnote,
    #[serde(rename = "SUBCHAP")]
    pub subchap: TocSubchap,
    #[serde(rename = "PTHD")]
    pub pthd: String,
    #[serde(rename = "PGHD")]
    pub pghd: String,
    #[serde(rename = "CHAPTI")]
    pub chapti: TocChapti,
}

#[derive(Serialize, Deserialize)]
pub struct Tochd {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: ChapterTocTochdPrtpage,
    #[serde(rename = "HD")]
    pub hd: ChapterTocTochdHd,
}

#[derive(Serialize, Deserialize)]
pub struct ChapterTocTochdPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChapterTocTochdHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TocEdnote {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: ChapterTocEdnoteHd,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChapterTocEdnoteHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TocSubchap {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: ChapterTocSubchapHd,
}

#[derive(Serialize, Deserialize)]
pub struct ChapterTocSubchapHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TocChapti {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PT")]
    pub pt: Vec<String>,
    #[serde(rename = "SUBJECT")]
    pub subject: Vec<String>,
    #[serde(rename = "PG")]
    pub pg: Vec<TocChaptiPg>,
}

#[derive(Serialize, Deserialize)]
pub struct TocChaptiPg {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<TocChaptiPgPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct TocChaptiPgPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChapterSubchap {
    #[serde(rename = "@TYPE")]
    pub subchap_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: TitleChapterSubchapPrtpage,
    #[serde(rename = "HD")]
    pub hd: TitleChapterSubchapHd,
    #[serde(rename = "PART")]
    pub part: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
pub struct TitleChapterSubchapPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct TitleChapterSubchapHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "EAR")]
    pub ear: String,
    #[serde(rename = "HD")]
    pub hd: ChapterSubchapPartHd,
    #[serde(rename = "CONTENTS")]
    pub contents: Option<Contents>,
    #[serde(rename = "AUTH")]
    pub auth: Auth,
    #[serde(rename = "SOURCE")]
    pub source: Option<PartSource>,
    #[serde(rename = "SUBPART")]
    pub subpart: Option<Vec<PartSubpart>>,
    #[serde(rename = "EDNOTE")]
    pub ednote: Option<PartEdnote>,
    #[serde(rename = "SECTION")]
    pub section: Option<Vec<PartSection>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<ChapterSubchapPartPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct ChapterSubchapPartHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Contents {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SUBPART")]
    pub subpart: Option<Vec<ContentsSubpart>>,
    #[serde(rename = "SECHD")]
    pub sechd: Option<String>,
    #[serde(rename = "SECTNO")]
    pub sectno: Option<Vec<String>>,
    #[serde(rename = "SUBJECT")]
    pub subject: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentsSubpart {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: Option<PartContentsSubpartHd>,
    #[serde(rename = "SECHD")]
    pub sechd: Option<String>,
    #[serde(rename = "SECTNO")]
    pub sectno: Option<Vec<String>>,
    #[serde(rename = "SUBJECT")]
    pub subject: Option<Vec<ContentsSubpartSubject>>,
    #[serde(rename = "RESERVED")]
    pub reserved: Option<String>,
    #[serde(rename = "APP")]
    pub app: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<PartContentsSubpartPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct PartContentsSubpartHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentsSubpartSubject {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<ContentsSubpartSubjectPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentsSubpartSubjectPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartContentsSubpartPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: SubchapPartAuthHd,
    #[serde(rename = "P")]
    pub p: String,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubchapPartAuthPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct SubchapPartAuthHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubchapPartAuthPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartSource {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: SubchapPartSourceHd,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubchapPartSourceHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSubpart {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: Option<SubchapPartSubpartHd>,
    #[serde(rename = "SECTION")]
    pub section: Option<Vec<SubpartSection>>,
    #[serde(rename = "RESERVED")]
    pub reserved: Option<String>,
    #[serde(rename = "SOURCE")]
    pub source: Option<SubpartSource>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubchapPartSubpartPrtpage>,
    #[serde(rename = "APPENDIX")]
    pub appendix: Option<Appendix>,
}

#[derive(Serialize, Deserialize)]
pub struct SubchapPartSubpartHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSection {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SECTNO")]
    pub sectno: String,
    #[serde(rename = "SUBJECT")]
    pub subject: Option<String>,
    #[serde(rename = "P")]
    pub p: Option<Vec<PartSubpartSectionP>>,
    #[serde(rename = "EXTRACT")]
    pub extract: Option<Vec<Extract>>,
    #[serde(rename = "FP")]
    pub fp: Option<Vec<SubpartSectionFp>>,
    #[serde(rename = "CITA")]
    pub cita: Option<String>,
    #[serde(rename = "EFFDNOT")]
    pub effdnot: Option<SubpartSectionEffdnot>,
    #[serde(rename = "APPRO")]
    pub appro: Option<String>,
    #[serde(rename = "FTNT")]
    pub ftnt: Option<Ftnt>,
    #[serde(rename = "GPH")]
    pub gph: Option<Vec<SectionGph>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<PartSubpartSectionPrtpage>,
    #[serde(rename = "RESERVED")]
    pub reserved: Option<String>,
    #[serde(rename = "BOXTXT")]
    pub boxtxt: Option<Boxtxt>,
    #[serde(rename = "NOTE")]
    pub note: Option<Vec<Note>>,
    #[serde(rename = "GPOTABLE")]
    pub gpotable: Option<Vec<Gpotable>>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSubpartSectionP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "FR")]
    pub fr: Option<Vec<String>>,
    #[serde(rename = "E")]
    pub e: Option<Vec<SubpartSectionPE>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubpartSectionPPrtpage>,
    #[serde(rename = "SU")]
    pub su: Option<Vec<String>>,
    #[serde(rename = "FTREF")]
    pub ftref: Option<Ftref>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionPE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SectionPEPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionPEPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionPPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ftref {
}

#[derive(Serialize, Deserialize)]
pub struct Extract {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: Option<Vec<SubpartSectionExtractHd>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubpartSectionExtractPrtpage>,
    #[serde(rename = "P")]
    pub p: Option<Vec<SubpartSectionExtractP>>,
    #[serde(rename = "FP")]
    pub fp: Option<Vec<SectionExtractFp>>,
    #[serde(rename = "SCOL2")]
    pub scol2: Option<Scol2>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionExtractHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionExtractPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionExtractP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<Vec<SectionExtractPE>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SectionExtractPPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionExtractPE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionExtractPPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct SectionExtractFp {
    #[serde(rename = "@SOURCE")]
    pub source: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<SectionExtractFpE>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SectionExtractFpPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionExtractFpE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionExtractFpPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Scol2 {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LI")]
    pub li: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionFp {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubpartSectionFpPrtpage>,
    #[serde(rename = "E")]
    pub e: Option<SubpartSectionFpE>,
    #[serde(rename = "SU")]
    pub su: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionFpPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionFpE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionEffdnot {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: SubpartSectionEffdnotHd,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionEffdnotHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Ftnt {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "P")]
    pub p: SubpartSectionFtntP,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionFtntP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SU")]
    pub su: String,
}

#[derive(Serialize, Deserialize)]
pub struct SectionGph {
    #[serde(rename = "@SPAN")]
    pub span: String,
    #[serde(rename = "@DEEP")]
    pub deep: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "GID")]
    pub gid: String,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubpartSectionGphPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionGphPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartSubpartSectionPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Boxtxt {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: SubpartSectionNoteHd,
    #[serde(rename = "P")]
    pub p: Vec<SubpartSectionNoteP>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionNoteHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSectionNoteP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<Vec<SectionNotePE>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SectionNotePPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionNotePE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionNotePPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Gpotable {
    #[serde(rename = "@COLS")]
    pub cols: String,
    #[serde(rename = "@OPTS")]
    pub opts: String,
    #[serde(rename = "@CDEF")]
    pub cdef: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TTITLE")]
    pub ttitle: Option<String>,
    #[serde(rename = "BOXHD")]
    pub boxhd: Option<Boxhd>,
    #[serde(rename = "ROW")]
    pub row: Vec<Row>,
    #[serde(rename = "TNOTE")]
    pub tnote: Option<Tnote>,
}

#[derive(Serialize, Deserialize)]
pub struct Boxhd {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CHED")]
    pub ched: Vec<Ched>,
}

#[derive(Serialize, Deserialize)]
pub struct Ched {
    #[serde(rename = "@H")]
    pub h: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SU")]
    pub su: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Row {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ENT")]
    pub ent: Vec<Ent>,
}

#[derive(Serialize, Deserialize)]
pub struct Ent {
    #[serde(rename = "@I")]
    pub i: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SU")]
    pub su: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<GpotableRowEntE>,
}

#[derive(Serialize, Deserialize)]
pub struct GpotableRowEntE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Tnote {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SU")]
    pub su: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartSource {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: PartSubpartSourceHd,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartSubpartSourceHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubchapPartSubpartPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Appendix {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "EAR")]
    pub ear: String,
    #[serde(rename = "HD")]
    pub hd: Vec<PartSubpartAppendixHd>,
    #[serde(rename = "P")]
    pub p: Vec<PartSubpartAppendixP>,
    #[serde(rename = "GPH")]
    pub gph: Vec<AppendixGph>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSubpartAppendixHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSubpartAppendixP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubpartAppendixPPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartAppendixPPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppendixGph {
    #[serde(rename = "@SPAN")]
    pub span: String,
    #[serde(rename = "@DEEP")]
    pub deep: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TCAP")]
    pub tcap: Tcap,
    #[serde(rename = "GID")]
    pub gid: String,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<SubpartAppendixGphPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct Tcap {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: AppendixGphTcapE,
}

#[derive(Serialize, Deserialize)]
pub struct AppendixGphTcapE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubpartAppendixGphPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartEdnote {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: SubchapPartEdnoteHd,
    #[serde(rename = "P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubchapPartEdnoteHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSection {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SECTNO")]
    pub sectno: String,
    #[serde(rename = "SUBJECT")]
    pub subject: String,
    #[serde(rename = "P")]
    pub p: Vec<SubchapPartSectionP>,
    #[serde(rename = "FP")]
    pub fp: Option<Vec<PartSectionFp>>,
    #[serde(rename = "CITA")]
    pub cita: Option<String>,
    #[serde(rename = "EAR")]
    pub ear: Option<String>,
    #[serde(rename = "EFFDNOT")]
    pub effdnot: Option<PartSectionEffdnot>,
    #[serde(rename = "APPRO")]
    pub appro: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubchapPartSectionP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<Vec<PartSectionPE>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<PartSectionPPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSectionPE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSectionPPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartSectionFp {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<PartSectionFpE>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<PartSectionFpPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSectionFpE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PartSectionFpPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartSectionEffdnot {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: PartSectionEffdnotHd,
    #[serde(rename = "P")]
    pub p: String,
    #[serde(rename = "REVTXT")]
    pub revtxt: Revtxt,
}

#[derive(Serialize, Deserialize)]
pub struct PartSectionEffdnotHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Revtxt {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SECTION")]
    pub section: Vec<RevtxtSection>,
}

#[derive(Serialize, Deserialize)]
pub struct RevtxtSection {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SECTNO")]
    pub sectno: String,
    #[serde(rename = "SUBJECT")]
    pub subject: String,
    #[serde(rename = "STARS")]
    pub stars: Option<SectionStars>,
    #[serde(rename = "P")]
    pub p: Vec<EffdnotRevtxtSectionP>,
}

#[derive(Serialize, Deserialize)]
pub struct SectionStars {
}

#[derive(Serialize, Deserialize)]
pub struct EffdnotRevtxtSectionP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Option<Vec<RevtxtSectionPE>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<RevtxtSectionPPrtpage>,
    #[serde(rename = "STARS")]
    pub stars: Option<PStars>,
}

#[derive(Serialize, Deserialize)]
pub struct RevtxtSectionPE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RevtxtSectionPPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct PStars {
}

#[derive(Serialize, Deserialize)]
pub struct ChapterSubchapPartPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Bmtr {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "FAIDS")]
    pub faids: BmtrFaids,
    #[serde(rename = "TOCTAC")]
    pub toctac: Toctac,
    #[serde(rename = "ALPHLIST")]
    pub alphlist: Alphlist,
    #[serde(rename = "LSA")]
    pub lsa: Lsa,
}

#[derive(Serialize, Deserialize)]
pub struct BmtrFaids {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocBmtrFaidsPrtpage,
    #[serde(rename = "HD")]
    pub hd: CfrdocBmtrFaidsHd,
    #[serde(rename = "P")]
    pub p: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrFaidsPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrFaidsHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Toctac {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CHHD")]
    pub chhd: String,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocBmtrToctacPrtpage,
    #[serde(rename = "HD")]
    pub hd: CfrdocBmtrToctacHd,
    #[serde(rename = "REV")]
    pub rev: String,
    #[serde(rename = "TITLENO")]
    pub titleno: Vec<ToctacTitleno>,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrToctacPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrToctacHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ToctacTitleno {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "HD")]
    pub hd: Option<BmtrToctacTitlenoHd>,
    #[serde(rename = "CHAPNO")]
    pub chapno: Option<Vec<String>>,
    #[serde(rename = "SUBJECT")]
    pub subject: Option<Vec<ToctacTitlenoSubject>>,
    #[serde(rename = "SUBTITL")]
    pub subtitl: Option<Vec<String>>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<BmtrToctacTitlenoPrtpage>,
    #[serde(rename = "RESERVED")]
    pub reserved: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BmtrToctacTitlenoHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ToctacTitlenoSubject {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<ToctacTitlenoSubjectPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct ToctacTitlenoSubjectPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct BmtrToctacTitlenoPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Alphlist {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocBmtrAlphlistPrtpage,
    #[serde(rename = "HD")]
    pub hd: CfrdocBmtrAlphlistHd,
    #[serde(rename = "REV")]
    pub rev: String,
    #[serde(rename = "AGHD")]
    pub aghd: String,
    #[serde(rename = "CFRHD")]
    pub cfrhd: String,
    #[serde(rename = "AGENCY")]
    pub agency: Vec<String>,
    #[serde(rename = "CFRID")]
    pub cfrid: Vec<Cfrid>,
    #[serde(rename = "SUBAGCY")]
    pub subagcy: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrAlphlistPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrAlphlistHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Cfrid {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: Option<BmtrAlphlistCfridPrtpage>,
}

#[derive(Serialize, Deserialize)]
pub struct BmtrAlphlistCfridPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct Lsa {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LRH")]
    pub lrh: String,
    #[serde(rename = "RRH")]
    pub rrh: String,
    #[serde(rename = "PRTPAGE")]
    pub prtpage: CfrdocBmtrLsaPrtpage,
    #[serde(rename = "HD")]
    pub hd: CfrdocBmtrLsaHd,
    #[serde(rename = "P")]
    pub p: Vec<CfrdocBmtrLsaP>,
    #[serde(rename = "PUBYEAR")]
    pub pubyear: Vec<Pubyear>,
    #[serde(rename = "ALL")]
    pub all: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrLsaPrtpage {
    #[serde(rename = "@P")]
    pub p: String,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrLsaHd {
    #[serde(rename = "@SOURCE")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CfrdocBmtrLsaP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "E")]
    pub e: Vec<BmtrLsaPE>,
}

#[derive(Serialize, Deserialize)]
pub struct BmtrLsaPE {
    #[serde(rename = "@T")]
    pub t: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Pubyear {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "YEAR")]
    pub year: String,
    #[serde(rename = "CFRNO")]
    pub cfrno: Option<String>,
    #[serde(rename = "VOLHD")]
    pub volhd: Option<String>,
    #[serde(rename = "PGHD")]
    pub pghd: Option<String>,
    #[serde(rename = "CHAPNO")]
    pub chapno: Option<String>,
    #[serde(rename = "ENTRY")]
    pub entry: Option<Vec<String>>,
    #[serde(rename = "PG")]
    pub pg: Option<Vec<String>>,
    #[serde(rename = "NOREG")]
    pub noreg: Option<String>,
}

