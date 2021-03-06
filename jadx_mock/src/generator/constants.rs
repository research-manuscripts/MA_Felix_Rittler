/// Constants for the mock data generator

/// Maximum name length of names for classes, etc.
pub const MAX_NAME_LENGTH: i32 = 25;
/// Maximum name length of paths for packages, etc.
pub const MAX_PATH_LENGTH: i32 = 100;
/// Maximum number of childs of a node in the project tree
pub const MAX_CHILD_COUNT: i32 = 10;
/// Total maximum of elements of the project tree
pub const MAX_TREE_ITEMS: i32 = 100;
/// Maximum depth of project tree
pub const MAX_TREE_DEPTH: i32 = 5;
/// Maximum number of items in the editor tab bar
pub const MAX_TAB_COUNT: i32 = 10;
/// Maximum number of search results (class search, usage search etc.)
pub const MAX_SEARCH_RESULT_COUNT: i32 = 21;
/// Maximum number that is show for pagination text of search result table (does not affect results in search result table)
pub const SEARCH_RESULTS_MAX_ITEM_COUNT: i32 = 2000;
/// Maximum number of items on a page for the pagination text (does not affect results in search result table)
pub const SEARCH_RESULTS_MAX_PAGE_SIZE: i32 = 200;

/// Different types of icons within JADX
pub enum IconSet {
    FileIcons,
    EntityIcons,
    AllIcons,
}

/// Paths of the screenshots for the file editor content
pub const EDITOR_SCREENSHOTS: [&str; 8] = [
    "src/assets/editor_screenshots/arsc.png",
    "src/assets/editor_screenshots/class_end.png",
    "src/assets/editor_screenshots/class_middle.png",
    "src/assets/editor_screenshots/class_start.png",
    "src/assets/editor_screenshots/enum.png",
    "src/assets/editor_screenshots/interface.png",
    "src/assets/editor_screenshots/small_file.png",
    "src/assets/editor_screenshots/xml_file.png",
];

/// List of all used icons of JADX
pub const ALL_ICONS: [&str; 18] = [
    "src/assets/icons-16/int_obj.png",
    "src/assets/icons-16/class_obj.png",
    "src/assets/icons-16/package_obj.png",
    "src/assets/icons-16/packagefolder_obj.png",
    "src/assets/icons-16/folder.png",
    "src/assets/icons-16/template_obj.png",
    "src/assets/icons-16/java_model_obj.png",
    "src/assets/icons-16/enum_obj.png",
    "src/assets/icons-16/file_obj.png",
    "src/assets/icons-16/methdef_obj.png",
    "src/assets/icons-16/jcu_obj.png",
    "src/assets/icons-16/methpub_obj.png",
    "src/assets/icons-16/methpro_obj.png",
    "src/assets/icons-16/methpri_obj.png",
    "src/assets/icons-16/template_obj.png",
    "src/assets/icons-16/cf_obj.png",
    "src/assets/icons-16/annotation_obj.png",
    "src/assets/icons-16/certificate_obj.png",
];

/// Icons of java entites
pub const ENTITY_ICONS: [&str; 10] = [
    "src/assets/icons-16/int_obj.png",
    "src/assets/icons-16/class_obj.png",
    "src/assets/icons-16/template_obj.png",
    "src/assets/icons-16/java_model_obj.png",
    "src/assets/icons-16/enum_obj.png",
    "src/assets/icons-16/methdef_obj.png",
    "src/assets/icons-16/methpub_obj.png",
    "src/assets/icons-16/methpro_obj.png",
    "src/assets/icons-16/methpri_obj.png",
    "src/assets/icons-16/annotation_obj.png",
];

/// Icons of java or other files within APK or DEX files
pub const FILE_ICONS: [&str; 12] = [
    "src/assets/icons-16/int_obj.png",
    "src/assets/icons-16/class_obj.png",
    "src/assets/icons-16/template_obj.png",
    "src/assets/icons-16/java_model_obj.png",
    "src/assets/icons-16/enum_obj.png",
    "src/assets/icons-16/file_obj.png",
    "src/assets/icons-16/template_obj.png",
    "src/assets/icons-16/cf_obj.png",
    "src/assets/icons-16/packagefolder_obj.png",
    "src/assets/icons-16/annotation_obj.png",
    "src/assets/icons-16/jcu_obj.png",
    "src/assets/icons-16/certificate_obj.png",
];

/// List of fonts names for the font chooser
pub const FONTS: [&str; 152] = [
    "Abadi MT Condensed Light",
    "Albertus Extra Bold",
    "Albertus Medium",
    "Antique Olive",
    "Arial",
    "Arial Black",
    "Arial MT",
    "Arial Narrow",
    "Bazooka",
    "Book Antiqua",
    "Bookman Old Style",
    "Boulder",
    "Calisto MT",
    "Calligrapher",
    "Century Gothic",
    "Century Schoolbook",
    "Cezanne",
    "CG Omega",
    "CG Times",
    "Charlesworth",
    "Chaucer",
    "Clarendon Condensed",
    "Comic Sans MS",
    "Copperplate Gothic Bold",
    "Copperplate Gothic Light",
    "Cornerstone",
    "Coronet",
    "Courier",
    "Courier New",
    "Cuckoo",
    "Dauphin",
    "Denmark",
    "Fransiscan",
    "Garamond",
    "Geneva",
    "Haettenschweiler",
    "Heather",
    "Helvetica",
    "Herald",
    "Impact",
    "Jester",
    "Letter Gothic",
    "Lithograph",
    "Lithograph Light",
    "Long Island",
    "Lucida Console",
    "Lucida Handwriting",
    "Lucida Sans",
    "Lucida Sans Unicode",
    "Marigold",
    "Market",
    "Matisse ITC",
    "MS LineDraw",
    "News GothicMT",
    "OCR A Extended",
    "Old Century",
    "Pegasus",
    "Pickwick",
    "Poster",
    "Pythagoras",
    "Sceptre",
    "Sherwood",
    "Signboard",
    "Socket",
    "Steamer",
    "Storybook",
    "Subway",
    "Tahoma",
    "Technical",
    "Teletype",
    "Tempus Sans ITC",
    "Times",
    "Times New Roman",
    "Times New Roman PS",
    "Trebuchet MS",
    "Tristan",
    "Tubular",
    "Unicorn",
    "Univers",
    "Univers Condensed",
    "Vagabond",
    "Verdana",
    "Westminster 	Allegro",
    "Amazone BT",
    "AmerType Md BT",
    "Arrus BT",
    "Aurora Cn BT",
    "AvantGarde Bk BT",
    "AvantGarde Md BT",
    "BankGothic Md BT",
    "Benguiat Bk BT",
    "BernhardFashion BT",
    "BernhardMod BT",
    "BinnerD",
    "Bremen Bd BT",
    "CaslonOpnface BT",
    "Charter Bd BT",
    "Charter BT",
    "ChelthmITC Bk BT",
    "CloisterBlack BT",
    "CopperplGoth Bd BT",
    "English 111 Vivace BT",
    "EngraversGothic BT",
    "Exotc350 Bd BT",
    "Freefrm721 Blk BT",
    "FrnkGothITC Bk BT",
    "Futura Bk BT",
    "Futura Lt BT",
    "Futura Md BT",
    "Futura ZBlk BT",
    "FuturaBlack BT",
    "Galliard BT",
    "Geometr231 BT",
    "Geometr231 Hv BT",
    "Geometr231 Lt BT",
    "GeoSlab 703 Lt BT",
    "GeoSlab 703 XBd BT",
    "GoudyHandtooled BT",
    "GoudyOLSt BT",
    "Humanst521 BT",
    "Humanst 521 Cn BT",
    "Humanst521 Lt BT",
    "Incised901 Bd BT",
    "Incised901 BT",
    "Incised901 Lt BT",
    "Informal011 BT",
    "Kabel Bk BT",
    "Kabel Ult BT",
    "Kaufmann Bd BT",
    "Kaufmann BT",
    "Korinna BT",
    "Lydian BT",
    "Monotype Corsiva",
    "NewsGoth BT",
    "Onyx BT",
    "OzHandicraft BT",
    "PosterBodoni BT",
    "PTBarnum BT",
    "Ribbon131 Bd BT",
    "Serifa BT",
    "Serifa Th BT",
    "ShelleyVolante BT",
    "Souvenir Lt BT",
    "Staccato222 BT",
    "Swis721 BlkEx BT",
    "Swiss911 XCm BT",
    "TypoUpright BT",
    "ZapfEllipt BT",
    "ZapfHumnst BT",
    "ZapfHumnst Dm BT",
    "Zurich BlkEx BT",
    "Zurich Ex BT ",
];
