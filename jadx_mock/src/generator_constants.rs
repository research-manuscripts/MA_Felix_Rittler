pub const MAX_NAME_LENGTH: i32 = 25;
pub const MAX_PATH_LENGTH: i32 = 100;
pub const MAX_CHILD_COUNT: i32 = 10;
pub const MAX_TREE_ITEMS: i32 = 100;
pub const MAX_TAB_COUNT: i32 = 10;
pub const MAX_SEARCH_RESULT_COUNT: i32 = 21;

pub enum IconSet {
    FileIcons,
    EntityIcons,
    AllIcons,
}

pub const ALL_ICONS: [&str; 19] = [
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
    "src/assets/icons-16/package_folder.png",
    "src/assets/icons-16/annotation_obj.png",
    "src/assets/icons-16/certificate_obj.png",
];

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

pub const FILE_ICONS: [&str; 12] = [
    "src/assets/icons-16/int_obj.png",
    "src/assets/icons-16/class_obj.png",
    "src/assets/icons-16/template_obj.png",
    "src/assets/icons-16/java_model_obj.png",
    "src/assets/icons-16/enum_obj.png",
    "src/assets/icons-16/file_obj.png",
    "src/assets/icons-16/template_obj.png",
    "src/assets/icons-16/cf_obj.png",
    "src/assets/icons-16/package_folder.png",
    "src/assets/icons-16/annotation_obj.png",
    "src/assets/icons-16/jcu_obj.png",
    "src/assets/icons-16/certificate_obj.png",
];

pub const DEMO_TEXT: &str =
    "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor
invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam
et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est
Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed
diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam
voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd
gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor
sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore
et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo
dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum
dolor sit amet.
Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat,
vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio
dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait
nulla facilisi. Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam
nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat.
Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl
ut aliquip ex ea commodo consequat. Duis autem vel eum iriure dolor in hendrerit in
vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis
at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril
delenit augue duis dolore te feugait nulla facilisi.
Nam liber tempor cum soluta nobis eleifend option congue nihil imperdiet doming id quod
mazim placerat facer possim assum. Lorem ipsum dolor sit amet, consectetuer adipiscing
elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat
volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit
lobortis nisl ut aliquip ex ea commodo consequat.
Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat,
vel illum dolore eu feugiat nulla facilisis.

At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no
sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur
sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam
erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita
kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor
sit amet, consetetur sadipscing elitr, At accusam aliquyam diam diam dolore dolores duo
eirmod eos erat, et nonumy sed tempor et et invidunt justo labore Stet clita ea et gubergren,
kasd magna no rebum. sanctus sea sed takimata ut vero voluptua. est Lorem ipsum dolor sit
amet. Lorem ipsum dolor sit amet, consetetur";

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
