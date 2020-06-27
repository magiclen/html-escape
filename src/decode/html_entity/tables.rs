/// The table of HTML named entities ordered by the names.
///
/// `&amp;`, `&lt;`, `&gt;` and `&quot;` can be uppercase.
///
/// Source: https://dev.w3.org/html5/html-author/charref
pub static NAMED_ENTITIES: [(&[u8], &str); 1452] = [
    (b"AElig", "\u{00C6}"),                // LATIN CAPITAL LETTER AE
    (b"AMP", "\u{0026}"),                  // AMPERSAND
    (b"Aacute", "\u{00C1}"),               // LATIN CAPITAL LETTER A WITH ACUTE
    (b"Abreve", "\u{0102}"),               // LATIN CAPITAL LETTER A WITH BREVE
    (b"Acirc", "\u{00C2}"),                // LATIN CAPITAL LETTER A WITH CIRCUMFLEX
    (b"Acy", "\u{0410}"),                  // CYRILLIC CAPITAL LETTER A
    (b"Afr", "\u{1D504}"),                 // MATHEMATICAL FRAKTUR CAPITAL A
    (b"Agrave", "\u{00C0}"),               // LATIN CAPITAL LETTER A WITH GRAVE
    (b"Alpha", "\u{0391}"),                // GREEK CAPITAL LETTER ALPHA
    (b"Amacr", "\u{0100}"),                // LATIN CAPITAL LETTER A WITH MACRON
    (b"And", "\u{2A53}"),                  // DOUBLE LOGICAL AND
    (b"Aogon", "\u{0104}"),                // LATIN CAPITAL LETTER A WITH OGONEK
    (b"Aopf", "\u{1D538}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL A
    (b"ApplyFunction", "\u{2061}"),        // FUNCTION APPLICATION
    (b"Aring", "\u{00C5}"),                // LATIN CAPITAL LETTER A WITH RING ABOVE
    (b"Ascr", "\u{1D49C}"),                // MATHEMATICAL SCRIPT CAPITAL A
    (b"Atilde", "\u{00C3}"),               // LATIN CAPITAL LETTER A WITH TILDE
    (b"Auml", "\u{00C4}"),                 // LATIN CAPITAL LETTER A WITH DIAERESIS
    (b"Barv", "\u{2AE7}"),                 // SHORT DOWN TACK WITH OVERBAR
    (b"Barwed", "\u{2306}"),               // PERSPECTIVE
    (b"Bcy", "\u{0411}"),                  // CYRILLIC CAPITAL LETTER BE
    (b"Beta", "\u{0392}"),                 // GREEK CAPITAL LETTER BETA
    (b"Bfr", "\u{1D505}"),                 // MATHEMATICAL FRAKTUR CAPITAL B
    (b"Bopf", "\u{1D539}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL B
    (b"CHcy", "\u{0427}"),                 // CYRILLIC CAPITAL LETTER CHE
    (b"Cacute", "\u{0106}"),               // LATIN CAPITAL LETTER C WITH ACUTE
    (b"Cap", "\u{22D2}"),                  // DOUBLE INTERSECTION
    (b"CapitalDifferentialD", "\u{2145}"), // DOUBLE-STRUCK ITALIC CAPITAL D
    (b"Ccaron", "\u{010C}"),               // LATIN CAPITAL LETTER C WITH CARON
    (b"Ccedil", "\u{00C7}"),               // LATIN CAPITAL LETTER C WITH CEDILLA
    (b"Ccirc", "\u{0108}"),                // LATIN CAPITAL LETTER C WITH CIRCUMFLEX
    (b"Cconint", "\u{2230}"),              // VOLUME INTEGRAL
    (b"Cdot", "\u{010A}"),                 // LATIN CAPITAL LETTER C WITH DOT ABOVE
    (b"Cfr", "\u{212D}"),                  // BLACK-LETTER CAPITAL C
    (b"Chi", "\u{03A7}"),                  // GREEK CAPITAL LETTER CHI
    (b"Colon", "\u{2237}"),                // PROPORTION
    (b"Colone", "\u{2A74}"),               // DOUBLE COLON EQUAL
    (b"Conint", "\u{222F}"),               // SURFACE INTEGRAL
    (b"Copf", "\u{2102}"),                 // DOUBLE-STRUCK CAPITAL C
    (b"Cross", "\u{2A2F}"),                // VECTOR OR CROSS PRODUCT
    (b"Cscr", "\u{1D49E}"),                // MATHEMATICAL SCRIPT CAPITAL C
    (b"Cup", "\u{22D3}"),                  // DOUBLE UNION
    (b"DDotrahd", "\u{2911}"),             // RIGHTWARDS ARROW WITH DOTTED STEM
    (b"DJcy", "\u{0402}"),                 // CYRILLIC CAPITAL LETTER DJE
    (b"DScy", "\u{0405}"),                 // CYRILLIC CAPITAL LETTER DZE
    (b"DZcy", "\u{040F}"),                 // CYRILLIC CAPITAL LETTER DZHE
    (b"Dagger", "\u{2021}"),               // DOUBLE DAGGER
    (b"Darr", "\u{21A1}"),                 // DOWNWARDS TWO HEADED ARROW
    (b"Dashv", "\u{2AE4}"),                // VERTICAL BAR DOUBLE LEFT TURNSTILE
    (b"Dcaron", "\u{010E}"),               // LATIN CAPITAL LETTER D WITH CARON
    (b"Dcy", "\u{0414}"),                  // CYRILLIC CAPITAL LETTER DE
    (b"Delta", "\u{0394}"),                // GREEK CAPITAL LETTER DELTA
    (b"Dfr", "\u{1D507}"),                 // MATHEMATICAL FRAKTUR CAPITAL D
    (b"DifferentialD", "\u{2146}"),        // DOUBLE-STRUCK ITALIC SMALL D
    (b"Dopf", "\u{1D53B}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL D
    (b"Dot", "\u{00A8}"),                  // DIAERESIS
    (b"DotDot", "\u{20DC}"),               // COMBINING FOUR DOTS ABOVE
    (b"DownArrowBar", "\u{2913}"),         // DOWNWARDS ARROW TO BAR
    (b"DownBreve", "\u{0311}"),            // COMBINING INVERTED BREVE
    (b"DownLeftRightVector", "\u{2950}"),  // LEFT BARB DOWN RIGHT BARB DOWN HARPOON
    (b"DownLeftTeeVector", "\u{295E}"),    // LEFTWARDS HARPOON WITH BARB DOWN FROM BAR
    (b"DownLeftVectorBar", "\u{2956}"),    // LEFTWARDS HARPOON WITH BARB DOWN TO BAR
    (b"DownRightTeeVector", "\u{295F}"),   // RIGHTWARDS HARPOON WITH BARB DOWN FROM BAR
    (b"DownRightVectorBar", "\u{2957}"),   // RIGHTWARDS HARPOON WITH BARB DOWN TO BAR
    (b"DownTeeArrow", "\u{21A7}"),         // DOWNWARDS ARROW FROM BAR
    (b"Dscr", "\u{1D49F}"),                // MATHEMATICAL SCRIPT CAPITAL D
    (b"Dstrok", "\u{0110}"),               // LATIN CAPITAL LETTER D WITH STROKE
    (b"ENG", "\u{014A}"),                  // LATIN CAPITAL LETTER ENG
    (b"ETH", "\u{00D0}"),                  // LATIN CAPITAL LETTER ETH
    (b"Eacute", "\u{00C9}"),               // LATIN CAPITAL LETTER E WITH ACUTE
    (b"Ecaron", "\u{011A}"),               // LATIN CAPITAL LETTER E WITH CARON
    (b"Ecirc", "\u{00CA}"),                // LATIN CAPITAL LETTER E WITH CIRCUMFLEX
    (b"Ecy", "\u{042D}"),                  // CYRILLIC CAPITAL LETTER E
    (b"Edot", "\u{0116}"),                 // LATIN CAPITAL LETTER E WITH DOT ABOVE
    (b"Efr", "\u{1D508}"),                 // MATHEMATICAL FRAKTUR CAPITAL E
    (b"Egrave", "\u{00C8}"),               // LATIN CAPITAL LETTER E WITH GRAVE
    (b"Emacr", "\u{0112}"),                // LATIN CAPITAL LETTER E WITH MACRON
    (b"EmptySmallSquare", "\u{25FB}"),     // WHITE MEDIUM SQUARE
    (b"EmptyVerySmallSquare", "\u{25AB}"), // WHITE SMALL SQUARE
    (b"Eogon", "\u{0118}"),                // LATIN CAPITAL LETTER E WITH OGONEK
    (b"Eopf", "\u{1D53C}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL E
    (b"Epsilon", "\u{0395}"),              // GREEK CAPITAL LETTER EPSILON
    (b"Equal", "\u{2A75}"),                // TWO CONSECUTIVE EQUALS SIGNS
    (b"Escr", "\u{2130}"),                 // SCRIPT CAPITAL E
    (b"Esim", "\u{2A73}"),                 // EQUALS SIGN ABOVE TILDE OPERATOR
    (b"Eta", "\u{0397}"),                  // GREEK CAPITAL LETTER ETA
    (b"Euml", "\u{00CB}"),                 // LATIN CAPITAL LETTER E WITH DIAERESIS
    (b"ExponentialE", "\u{2147}"),         // DOUBLE-STRUCK ITALIC SMALL E
    (b"Fcy", "\u{0424}"),                  // CYRILLIC CAPITAL LETTER EF
    (b"Ffr", "\u{1D509}"),                 // MATHEMATICAL FRAKTUR CAPITAL F
    (b"FilledSmallSquare", "\u{25FC}"),    // BLACK MEDIUM SQUARE
    (b"Fopf", "\u{1D53D}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL F
    (b"Fscr", "\u{2131}"),                 // SCRIPT CAPITAL F
    (b"GJcy", "\u{0403}"),                 // CYRILLIC CAPITAL LETTER GJE
    (b"GT", "\u{003E}"),                   // GREATER-THAN SIGN
    (b"Gamma", "\u{0393}"),                // GREEK CAPITAL LETTER GAMMA
    (b"Gammad", "\u{03DC}"),               // GREEK LETTER DIGAMMA
    (b"Gbreve", "\u{011E}"),               // LATIN CAPITAL LETTER G WITH BREVE
    (b"Gcedil", "\u{0122}"),               // LATIN CAPITAL LETTER G WITH CEDILLA
    (b"Gcirc", "\u{011C}"),                // LATIN CAPITAL LETTER G WITH CIRCUMFLEX
    (b"Gcy", "\u{0413}"),                  // CYRILLIC CAPITAL LETTER GHE
    (b"Gdot", "\u{0120}"),                 // LATIN CAPITAL LETTER G WITH DOT ABOVE
    (b"Gfr", "\u{1D50A}"),                 // MATHEMATICAL FRAKTUR CAPITAL G
    (b"Gg", "\u{22D9}"),                   // VERY MUCH GREATER-THAN
    (b"Gopf", "\u{1D53E}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL G
    (b"GreaterGreater", "\u{2AA2}"),       // DOUBLE NESTED GREATER-THAN
    (b"Gscr", "\u{1D4A2}"),                // MATHEMATICAL SCRIPT CAPITAL G
    (b"Gt", "\u{226B}"),                   // MUCH GREATER-THAN
    (b"HARDcy", "\u{042A}"),               // CYRILLIC CAPITAL LETTER HARD SIGN
    (b"Hat", "\u{005E}"),                  // CIRCUMFLEX ACCENT
    (b"Hcirc", "\u{0124}"),                // LATIN CAPITAL LETTER H WITH CIRCUMFLEX
    (b"Hfr", "\u{210C}"),                  // BLACK-LETTER CAPITAL H
    (b"Hstrok", "\u{0126}"),               // LATIN CAPITAL LETTER H WITH STROKE
    (b"IEcy", "\u{0415}"),                 // CYRILLIC CAPITAL LETTER IE
    (b"IJlig", "\u{0132}"),                // LATIN CAPITAL LIGATURE IJ
    (b"IOcy", "\u{0401}"),                 // CYRILLIC CAPITAL LETTER IO
    (b"Iacute", "\u{00CD}"),               // LATIN CAPITAL LETTER I WITH ACUTE
    (b"Icirc", "\u{00CE}"),                // LATIN CAPITAL LETTER I WITH CIRCUMFLEX
    (b"Icy", "\u{0418}"),                  // CYRILLIC CAPITAL LETTER I
    (b"Idot", "\u{0130}"),                 // LATIN CAPITAL LETTER I WITH DOT ABOVE
    (b"Igrave", "\u{00CC}"),               // LATIN CAPITAL LETTER I WITH GRAVE
    (b"Imacr", "\u{012A}"),                // LATIN CAPITAL LETTER I WITH MACRON
    (b"ImaginaryI", "\u{2148}"),           // DOUBLE-STRUCK ITALIC SMALL I
    (b"Int", "\u{222C}"),                  // DOUBLE INTEGRAL
    (b"InvisibleComma", "\u{2063}"),       // INVISIBLE SEPARATOR
    (b"InvisibleTimes", "\u{2062}"),       // INVISIBLE TIMES
    (b"Iogon", "\u{012E}"),                // LATIN CAPITAL LETTER I WITH OGONEK
    (b"Iopf", "\u{1D540}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL I
    (b"Iota", "\u{0399}"),                 // GREEK CAPITAL LETTER IOTA
    (b"Iscr", "\u{2110}"),                 // SCRIPT CAPITAL I
    (b"Itilde", "\u{0128}"),               // LATIN CAPITAL LETTER I WITH TILDE
    (b"Iukcy", "\u{0406}"),                // CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I
    (b"Iuml", "\u{00CF}"),                 // LATIN CAPITAL LETTER I WITH DIAERESIS
    (b"Jcirc", "\u{0134}"),                // LATIN CAPITAL LETTER J WITH CIRCUMFLEX
    (b"Jcy", "\u{0419}"),                  // CYRILLIC CAPITAL LETTER SHORT I
    (b"Jfr", "\u{1D50D}"),                 // MATHEMATICAL FRAKTUR CAPITAL J
    (b"Jopf", "\u{1D541}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL J
    (b"Jscr", "\u{1D4A5}"),                // MATHEMATICAL SCRIPT CAPITAL J
    (b"Jsercy", "\u{0408}"),               // CYRILLIC CAPITAL LETTER JE
    (b"Jukcy", "\u{0404}"),                // CYRILLIC CAPITAL LETTER UKRAINIAN IE
    (b"KHcy", "\u{0425}"),                 // CYRILLIC CAPITAL LETTER HA
    (b"KJcy", "\u{040C}"),                 // CYRILLIC CAPITAL LETTER KJE
    (b"Kappa", "\u{039A}"),                // GREEK CAPITAL LETTER KAPPA
    (b"Kcedil", "\u{0136}"),               // LATIN CAPITAL LETTER K WITH CEDILLA
    (b"Kcy", "\u{041A}"),                  // CYRILLIC CAPITAL LETTER KA
    (b"Kfr", "\u{1D50E}"),                 // MATHEMATICAL FRAKTUR CAPITAL K
    (b"Kopf", "\u{1D542}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL K
    (b"Kscr", "\u{1D4A6}"),                // MATHEMATICAL SCRIPT CAPITAL K
    (b"LJcy", "\u{0409}"),                 // CYRILLIC CAPITAL LETTER LJE
    (b"LT", "\u{003C}"),                   // LESS-THAN SIGN
    (b"Lacute", "\u{0139}"),               // LATIN CAPITAL LETTER L WITH ACUTE
    (b"Lambda", "\u{039B}"),               // GREEK CAPITAL LETTER LAMDA
    (b"Lang", "\u{27EA}"),                 // MATHEMATICAL LEFT DOUBLE ANGLE BRACKET
    (b"Larr", "\u{219E}"),                 // LEFTWARDS TWO HEADED ARROW
    (b"Lcaron", "\u{013D}"),               // LATIN CAPITAL LETTER L WITH CARON
    (b"Lcedil", "\u{013B}"),               // LATIN CAPITAL LETTER L WITH CEDILLA
    (b"Lcy", "\u{041B}"),                  // CYRILLIC CAPITAL LETTER EL
    (b"LeftDownTeeVector", "\u{2961}"),    // DOWNWARDS HARPOON WITH BARB LEFT FROM BAR
    (b"LeftDownVectorBar", "\u{2959}"),    // DOWNWARDS HARPOON WITH BARB LEFT TO BAR
    (b"LeftRightVector", "\u{294E}"),      // LEFT BARB UP RIGHT BARB UP HARPOON
    (b"LeftTeeArrow", "\u{21A4}"),         // LEFTWARDS ARROW FROM BAR
    (b"LeftTeeVector", "\u{295A}"),        // LEFTWARDS HARPOON WITH BARB UP FROM BAR
    (b"LeftTriangleBar", "\u{29CF}"),      // LEFT TRIANGLE BESIDE VERTICAL BAR
    (b"LeftUpDownVector", "\u{2951}"),     // UP BARB LEFT DOWN BARB LEFT HARPOON
    (b"LeftUpTeeVector", "\u{2960}"),      // UPWARDS HARPOON WITH BARB LEFT FROM BAR
    (b"LeftUpVectorBar", "\u{2958}"),      // UPWARDS HARPOON WITH BARB LEFT TO BAR
    (b"LeftVectorBar", "\u{2952}"),        // LEFTWARDS HARPOON WITH BARB UP TO BAR
    (b"LessLess", "\u{2AA1}"),             // DOUBLE NESTED LESS-THAN
    (b"Lfr", "\u{1D50F}"),                 // MATHEMATICAL FRAKTUR CAPITAL L
    (b"Ll", "\u{22D8}"),                   // VERY MUCH LESS-THAN
    (b"Lmidot", "\u{013F}"),               // LATIN CAPITAL LETTER L WITH MIDDLE DOT
    (b"Lopf", "\u{1D543}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL L
    (b"Lscr", "\u{2112}"),                 // SCRIPT CAPITAL L
    (b"Lstrok", "\u{0141}"),               // LATIN CAPITAL LETTER L WITH STROKE
    (b"Lt", "\u{226A}"),                   // MUCH LESS-THAN
    (b"Map", "\u{2905}"),                  // RIGHTWARDS TWO-HEADED ARROW FROM BAR
    (b"Mcy", "\u{041C}"),                  // CYRILLIC CAPITAL LETTER EM
    (b"MediumSpace", "\u{205F}"),          // MEDIUM MATHEMATICAL SPACE
    (b"Mfr", "\u{1D510}"),                 // MATHEMATICAL FRAKTUR CAPITAL M
    (b"Mopf", "\u{1D544}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL M
    (b"Mu", "\u{039C}"),                   // GREEK CAPITAL LETTER MU
    (b"NJcy", "\u{040A}"),                 // CYRILLIC CAPITAL LETTER NJE
    (b"Nacute", "\u{0143}"),               // LATIN CAPITAL LETTER N WITH ACUTE
    (b"Ncaron", "\u{0147}"),               // LATIN CAPITAL LETTER N WITH CARON
    (b"Ncedil", "\u{0145}"),               // LATIN CAPITAL LETTER N WITH CEDILLA
    (b"Ncy", "\u{041D}"),                  // CYRILLIC CAPITAL LETTER EN
    (b"NewLine", "\u{000A}"),              // LINE FEED (LF)
    (b"Nfr", "\u{1D511}"),                 // MATHEMATICAL FRAKTUR CAPITAL N
    (b"NoBreak", "\u{2060}"),              // WORD JOINER
    (b"Nopf", "\u{2115}"),                 // DOUBLE-STRUCK CAPITAL N
    (b"Not", "\u{2AEC}"),                  // DOUBLE STROKE NOT SIGN
    (b"NotCupCap", "\u{226D}"),            // NOT EQUIVALENT TO
    (b"Nscr", "\u{1D4A9}"),                // MATHEMATICAL SCRIPT CAPITAL N
    (b"Ntilde", "\u{00D1}"),               // LATIN CAPITAL LETTER N WITH TILDE
    (b"Nu", "\u{039D}"),                   // GREEK CAPITAL LETTER NU
    (b"OElig", "\u{0152}"),                // LATIN CAPITAL LIGATURE OE
    (b"Oacute", "\u{00D3}"),               // LATIN CAPITAL LETTER O WITH ACUTE
    (b"Ocirc", "\u{00D4}"),                // LATIN CAPITAL LETTER O WITH CIRCUMFLEX
    (b"Ocy", "\u{041E}"),                  // CYRILLIC CAPITAL LETTER O
    (b"Odblac", "\u{0150}"),               // LATIN CAPITAL LETTER O WITH DOUBLE ACUTE
    (b"Ofr", "\u{1D512}"),                 // MATHEMATICAL FRAKTUR CAPITAL O
    (b"Ograve", "\u{00D2}"),               // LATIN CAPITAL LETTER O WITH GRAVE
    (b"Omacr", "\u{014C}"),                // LATIN CAPITAL LETTER O WITH MACRON
    (b"Omega", "\u{03A9}"),                // GREEK CAPITAL LETTER OMEGA
    (b"Omicron", "\u{039F}"),              // GREEK CAPITAL LETTER OMICRON
    (b"Oopf", "\u{1D546}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL O
    (b"Or", "\u{2A54}"),                   // DOUBLE LOGICAL OR
    (b"Oscr", "\u{1D4AA}"),                // MATHEMATICAL SCRIPT CAPITAL O
    (b"Oslash", "\u{00D8}"),               // LATIN CAPITAL LETTER O WITH STROKE
    (b"Otilde", "\u{00D5}"),               // LATIN CAPITAL LETTER O WITH TILDE
    (b"Otimes", "\u{2A37}"),               // MULTIPLICATION SIGN IN DOUBLE CIRCLE
    (b"Ouml", "\u{00D6}"),                 // LATIN CAPITAL LETTER O WITH DIAERESIS
    (b"OverBrace", "\u{23DE}"),            // TOP CURLY BRACKET
    (b"OverParenthesis", "\u{23DC}"),      // TOP PARENTHESIS
    (b"Pcy", "\u{041F}"),                  // CYRILLIC CAPITAL LETTER PE
    (b"Pfr", "\u{1D513}"),                 // MATHEMATICAL FRAKTUR CAPITAL P
    (b"Phi", "\u{03A6}"),                  // GREEK CAPITAL LETTER PHI
    (b"Pi", "\u{03A0}"),                   // GREEK CAPITAL LETTER PI
    (b"Popf", "\u{2119}"),                 // DOUBLE-STRUCK CAPITAL P
    (b"Pr", "\u{2ABB}"),                   // DOUBLE PRECEDES
    (b"Prime", "\u{2033}"),                // DOUBLE PRIME
    (b"Pscr", "\u{1D4AB}"),                // MATHEMATICAL SCRIPT CAPITAL P
    (b"Psi", "\u{03A8}"),                  // GREEK CAPITAL LETTER PSI
    (b"QUOT", "\u{0022}"),                 // QUOTATION MARK
    (b"Qfr", "\u{1D514}"),                 // MATHEMATICAL FRAKTUR CAPITAL Q
    (b"Qscr", "\u{1D4AC}"),                // MATHEMATICAL SCRIPT CAPITAL Q
    (b"RBarr", "\u{2910}"),                // RIGHTWARDS TWO-HEADED TRIPLE DASH ARROW
    (b"Racute", "\u{0154}"),               // LATIN CAPITAL LETTER R WITH ACUTE
    (b"Rang", "\u{27EB}"),                 // MATHEMATICAL RIGHT DOUBLE ANGLE BRACKET
    (b"Rarr", "\u{21A0}"),                 // RIGHTWARDS TWO HEADED ARROW
    (b"Rarrtl", "\u{2916}"),               // RIGHTWARDS TWO-HEADED ARROW WITH TAIL
    (b"Rcaron", "\u{0158}"),               // LATIN CAPITAL LETTER R WITH CARON
    (b"Rcedil", "\u{0156}"),               // LATIN CAPITAL LETTER R WITH CEDILLA
    (b"Rcy", "\u{0420}"),                  // CYRILLIC CAPITAL LETTER ER
    (b"Rho", "\u{03A1}"),                  // GREEK CAPITAL LETTER RHO
    (b"RightDownTeeVector", "\u{295D}"),   // DOWNWARDS HARPOON WITH BARB RIGHT FROM BAR
    (b"RightDownVectorBar", "\u{2955}"),   // DOWNWARDS HARPOON WITH BARB RIGHT TO BAR
    (b"RightTeeVector", "\u{295B}"),       // RIGHTWARDS HARPOON WITH BARB UP FROM BAR
    (b"RightTriangleBar", "\u{29D0}"),     // VERTICAL BAR BESIDE RIGHT TRIANGLE
    (b"RightUpDownVector", "\u{294F}"),    // UP BARB RIGHT DOWN BARB RIGHT HARPOON
    (b"RightUpTeeVector", "\u{295C}"),     // UPWARDS HARPOON WITH BARB RIGHT FROM BAR
    (b"RightUpVectorBar", "\u{2954}"),     // UPWARDS HARPOON WITH BARB RIGHT TO BAR
    (b"RightVectorBar", "\u{2953}"),       // RIGHTWARDS HARPOON WITH BARB UP TO BAR
    (b"RoundImplies", "\u{2970}"),         // RIGHT DOUBLE ARROW WITH ROUNDED HEAD
    (b"Rscr", "\u{211B}"),                 // SCRIPT CAPITAL R
    (b"RuleDelayed", "\u{29F4}"),          // RULE-DELAYED
    (b"SHCHcy", "\u{0429}"),               // CYRILLIC CAPITAL LETTER SHCHA
    (b"SHcy", "\u{0428}"),                 // CYRILLIC CAPITAL LETTER SHA
    (b"SOFTcy", "\u{042C}"),               // CYRILLIC CAPITAL LETTER SOFT SIGN
    (b"Sacute", "\u{015A}"),               // LATIN CAPITAL LETTER S WITH ACUTE
    (b"Sc", "\u{2ABC}"),                   // DOUBLE SUCCEEDS
    (b"Scaron", "\u{0160}"),               // LATIN CAPITAL LETTER S WITH CARON
    (b"Scedil", "\u{015E}"),               // LATIN CAPITAL LETTER S WITH CEDILLA
    (b"Scirc", "\u{015C}"),                // LATIN CAPITAL LETTER S WITH CIRCUMFLEX
    (b"Scy", "\u{0421}"),                  // CYRILLIC CAPITAL LETTER ES
    (b"Sfr", "\u{1D516}"),                 // MATHEMATICAL FRAKTUR CAPITAL S
    (b"Sigma", "\u{03A3}"),                // GREEK CAPITAL LETTER SIGMA
    (b"Sopf", "\u{1D54A}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL S
    (b"Sscr", "\u{1D4AE}"),                // MATHEMATICAL SCRIPT CAPITAL S
    (b"Sub", "\u{22D0}"),                  // DOUBLE SUBSET
    (b"Sup", "\u{22D1}"),                  // DOUBLE SUPERSET
    (b"THORN", "\u{00DE}"),                // LATIN CAPITAL LETTER THORN
    (b"TSHcy", "\u{040B}"),                // CYRILLIC CAPITAL LETTER TSHE
    (b"TScy", "\u{0426}"),                 // CYRILLIC CAPITAL LETTER TSE
    (b"Tab", "\u{0009}"),                  // CHARACTER TABULATION
    (b"Tau", "\u{03A4}"),                  // GREEK CAPITAL LETTER TAU
    (b"Tcaron", "\u{0164}"),               // LATIN CAPITAL LETTER T WITH CARON
    (b"Tcedil", "\u{0162}"),               // LATIN CAPITAL LETTER T WITH CEDILLA
    (b"Tcy", "\u{0422}"),                  // CYRILLIC CAPITAL LETTER TE
    (b"Tfr", "\u{1D517}"),                 // MATHEMATICAL FRAKTUR CAPITAL T
    (b"Theta", "\u{0398}"),                // GREEK CAPITAL LETTER THETA
    (b"Topf", "\u{1D54B}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL T
    (b"Tscr", "\u{1D4AF}"),                // MATHEMATICAL SCRIPT CAPITAL T
    (b"Tstrok", "\u{0166}"),               // LATIN CAPITAL LETTER T WITH STROKE
    (b"Uacute", "\u{00DA}"),               // LATIN CAPITAL LETTER U WITH ACUTE
    (b"Uarr", "\u{219F}"),                 // UPWARDS TWO HEADED ARROW
    (b"Uarrocir", "\u{2949}"),             // UPWARDS TWO-HEADED ARROW FROM SMALL CIRCLE
    (b"Ubrcy", "\u{040E}"),                // CYRILLIC CAPITAL LETTER SHORT U
    (b"Ubreve", "\u{016C}"),               // LATIN CAPITAL LETTER U WITH BREVE
    (b"Ucirc", "\u{00DB}"),                // LATIN CAPITAL LETTER U WITH CIRCUMFLEX
    (b"Ucy", "\u{0423}"),                  // CYRILLIC CAPITAL LETTER U
    (b"Udblac", "\u{0170}"),               // LATIN CAPITAL LETTER U WITH DOUBLE ACUTE
    (b"Ufr", "\u{1D518}"),                 // MATHEMATICAL FRAKTUR CAPITAL U
    (b"Ugrave", "\u{00D9}"),               // LATIN CAPITAL LETTER U WITH GRAVE
    (b"Umacr", "\u{016A}"),                // LATIN CAPITAL LETTER U WITH MACRON
    (b"UnderBar", "\u{0332}"),             // COMBINING LOW LINE
    (b"UnderBrace", "\u{23DF}"),           // BOTTOM CURLY BRACKET
    (b"UnderParenthesis", "\u{23DD}"),     // BOTTOM PARENTHESIS
    (b"Uogon", "\u{0172}"),                // LATIN CAPITAL LETTER U WITH OGONEK
    (b"Uopf", "\u{1D54C}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL U
    (b"UpArrowBar", "\u{2912}"),           // UPWARDS ARROW TO BAR
    (b"UpTeeArrow", "\u{21A5}"),           // UPWARDS ARROW FROM BAR
    (b"Upsi", "\u{03D2}"),                 // GREEK UPSILON WITH HOOK SYMBOL
    (b"Upsilon", "\u{03A5}"),              // GREEK CAPITAL LETTER UPSILON
    (b"Uring", "\u{016E}"),                // LATIN CAPITAL LETTER U WITH RING ABOVE
    (b"Uscr", "\u{1D4B0}"),                // MATHEMATICAL SCRIPT CAPITAL U
    (b"Utilde", "\u{0168}"),               // LATIN CAPITAL LETTER U WITH TILDE
    (b"Uuml", "\u{00DC}"),                 // LATIN CAPITAL LETTER U WITH DIAERESIS
    (b"VDash", "\u{22AB}"),                // DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE
    (b"Vbar", "\u{2AEB}"),                 // DOUBLE UP TACK
    (b"Vcy", "\u{0412}"),                  // CYRILLIC CAPITAL LETTER VE
    (b"Vdash", "\u{22A9}"),                // FORCES
    (b"Vdashl", "\u{2AE6}"),               // LONG DASH FROM LEFT MEMBER OF DOUBLE VERTICAL
    (b"Verbar", "\u{2016}"),               // DOUBLE VERTICAL LINE
    (b"VerticalSeparator", "\u{2758}"),    // LIGHT VERTICAL BAR
    (b"Vfr", "\u{1D519}"),                 // MATHEMATICAL FRAKTUR CAPITAL V
    (b"Vopf", "\u{1D54D}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL V
    (b"Vscr", "\u{1D4B1}"),                // MATHEMATICAL SCRIPT CAPITAL V
    (b"Vvdash", "\u{22AA}"),               // TRIPLE VERTICAL BAR RIGHT TURNSTILE
    (b"Wcirc", "\u{0174}"),                // LATIN CAPITAL LETTER W WITH CIRCUMFLEX
    (b"Wfr", "\u{1D51A}"),                 // MATHEMATICAL FRAKTUR CAPITAL W
    (b"Wopf", "\u{1D54E}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL W
    (b"Wscr", "\u{1D4B2}"),                // MATHEMATICAL SCRIPT CAPITAL W
    (b"Xfr", "\u{1D51B}"),                 // MATHEMATICAL FRAKTUR CAPITAL X
    (b"Xi", "\u{039E}"),                   // GREEK CAPITAL LETTER XI
    (b"Xopf", "\u{1D54F}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL X
    (b"Xscr", "\u{1D4B3}"),                // MATHEMATICAL SCRIPT CAPITAL X
    (b"YAcy", "\u{042F}"),                 // CYRILLIC CAPITAL LETTER YA
    (b"YIcy", "\u{0407}"),                 // CYRILLIC CAPITAL LETTER YI
    (b"YUcy", "\u{042E}"),                 // CYRILLIC CAPITAL LETTER YU
    (b"Yacute", "\u{00DD}"),               // LATIN CAPITAL LETTER Y WITH ACUTE
    (b"Ycirc", "\u{0176}"),                // LATIN CAPITAL LETTER Y WITH CIRCUMFLEX
    (b"Ycy", "\u{042B}"),                  // CYRILLIC CAPITAL LETTER YERU
    (b"Yfr", "\u{1D51C}"),                 // MATHEMATICAL FRAKTUR CAPITAL Y
    (b"Yopf", "\u{1D550}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
    (b"Yscr", "\u{1D4B4}"),                // MATHEMATICAL SCRIPT CAPITAL Y
    (b"Yuml", "\u{0178}"),                 // LATIN CAPITAL LETTER Y WITH DIAERESIS
    (b"ZHcy", "\u{0416}"),                 // CYRILLIC CAPITAL LETTER ZHE
    (b"Zacute", "\u{0179}"),               // LATIN CAPITAL LETTER Z WITH ACUTE
    (b"Zcaron", "\u{017D}"),               // LATIN CAPITAL LETTER Z WITH CARON
    (b"Zcy", "\u{0417}"),                  // CYRILLIC CAPITAL LETTER ZE
    (b"Zdot", "\u{017B}"),                 // LATIN CAPITAL LETTER Z WITH DOT ABOVE
    (b"ZeroWidthSpace", "\u{200B}"),       // ZERO WIDTH SPACE
    (b"Zeta", "\u{0396}"),                 // GREEK CAPITAL LETTER ZETA
    (b"Zfr", "\u{2128}"),                  // BLACK-LETTER CAPITAL Z
    (b"Zscr", "\u{1D4B5}"),                // MATHEMATICAL SCRIPT CAPITAL Z
    (b"aacute", "\u{00E1}"),               // LATIN SMALL LETTER A WITH ACUTE
    (b"abreve", "\u{0103}"),               // LATIN SMALL LETTER A WITH BREVE
    (b"ac", "\u{223E}"),                   // INVERTED LAZY S
    (b"acd", "\u{223F}"),                  // SINE WAVE
    (b"acirc", "\u{00E2}"),                // LATIN SMALL LETTER A WITH CIRCUMFLEX
    (b"acute", "\u{00B4}"),                // ACUTE ACCENT
    (b"acy", "\u{0430}"),                  // CYRILLIC SMALL LETTER A
    (b"aelig", "\u{00E6}"),                // LATIN SMALL LETTER AE
    (b"afr", "\u{1D51E}"),                 // MATHEMATICAL FRAKTUR SMALL A
    (b"agrave", "\u{00E0}"),               // LATIN SMALL LETTER A WITH GRAVE
    (b"alefsym", "\u{2135}"),              // ALEF SYMBOL
    (b"alpha", "\u{03B1}"),                // GREEK SMALL LETTER ALPHA
    (b"amacr", "\u{0101}"),                // LATIN SMALL LETTER A WITH MACRON
    (b"amalg", "\u{2A3F}"),                // AMALGAMATION OR COPRODUCT
    (b"amp", "\u{0026}"),                  // AMPERSAND
    (b"and", "\u{2227}"),                  // LOGICAL AND
    (b"andand", "\u{2A55}"),               // TWO INTERSECTING LOGICAL AND
    (b"andd", "\u{2A5C}"),                 // LOGICAL AND WITH HORIZONTAL DASH
    (b"andslope", "\u{2A58}"),             // SLOPING LARGE AND
    (b"andv", "\u{2A5A}"),                 // LOGICAL AND WITH MIDDLE STEM
    (b"ang", "\u{2220}"),                  // ANGLE
    (b"ange", "\u{29A4}"),                 // ANGLE WITH UNDERBAR
    (b"angmsd", "\u{2221}"),               // MEASURED ANGLE
    (b"angmsdaa", "\u{29A8}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING UP AND RIGHT */
    (b"angmsdab", "\u{29A9}"), // MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING UP AND LEFT
    (b"angmsdac", "\u{29AA}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING DOWN AND RIGHT */
    (b"angmsdad", "\u{29AB}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING DOWN AND LEFT */
    (b"angmsdae", "\u{29AC}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING RIGHT AND UP */
    (b"angmsdaf", "\u{29AD}"), // MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING LEFT AND UP
    (b"angmsdag", "\u{29AE}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING RIGHT AND DOWN */
    (b"angmsdah", "\u{29AF}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING LEFT AND DOWN */
    (b"angrt", "\u{221F}"),    // RIGHT ANGLE
    (b"angrtvb", "\u{22BE}"),  // RIGHT ANGLE WITH ARC
    (b"angrtvbd", "\u{299D}"), // MEASURED RIGHT ANGLE WITH DOT
    (b"angsph", "\u{2222}"),   // SPHERICAL ANGLE
    (b"angst", "\u{212B}"),    // ANGSTROM SIGN
    (b"angzarr", "\u{237C}"),  // RIGHT ANGLE WITH DOWNWARDS ZIGZAG ARROW
    (b"aogon", "\u{0105}"),    // LATIN SMALL LETTER A WITH OGONEK
    (b"aopf", "\u{1D552}"),    // MATHEMATICAL DOUBLE-STRUCK SMALL A
    (b"apE", "\u{2A70}"),      // APPROXIMATELY EQUAL OR EQUAL TO
    (b"apacir", "\u{2A6F}"),   // ALMOST EQUAL TO WITH CIRCUMFLEX ACCENT
    (b"ape", "\u{224A}"),      // ALMOST EQUAL OR EQUAL TO
    (b"apid", "\u{224B}"),     // TRIPLE TILDE
    (b"apos", "\u{0027}"),     // APOSTROPHE
    (b"aring", "\u{00E5}"),    // LATIN SMALL LETTER A WITH RING ABOVE
    (b"ascr", "\u{1D4B6}"),    // MATHEMATICAL SCRIPT SMALL A
    (b"ast", "\u{002A}"),      // ASTERISK
    (b"asymp", "\u{2248}"),    // ALMOST EQUAL TO
    (b"asympeq", "\u{224D}"),  // EQUIVALENT TO
    (b"atilde", "\u{00E3}"),   // LATIN SMALL LETTER A WITH TILDE
    (b"auml", "\u{00E4}"),     // LATIN SMALL LETTER A WITH DIAERESIS
    (b"awconint", "\u{2233}"), // ANTICLOCKWISE CONTOUR INTEGRAL
    (b"awint", "\u{2A11}"),    // ANTICLOCKWISE INTEGRATION
    (b"bNot", "\u{2AED}"),     // REVERSED DOUBLE STROKE NOT SIGN
    (b"barvee", "\u{22BD}"),   // NOR
    (b"barwed", "\u{2305}"),   // PROJECTIVE
    (b"bbrk", "\u{23B5}"),     // BOTTOM SQUARE BRACKET
    (b"bbrktbrk", "\u{23B6}"), // BOTTOM SQUARE BRACKET OVER TOP SQUARE BRACKET
    (b"bcong", "\u{224C}"),    // ALL EQUAL TO
    (b"bcy", "\u{0431}"),      // CYRILLIC SMALL LETTER BE
    (b"becaus", "\u{2235}"),   // BECAUSE
    (b"bemptyv", "\u{29B0}"),  // REVERSED EMPTY SET
    (b"bepsi", "\u{03F6}"),    // GREEK REVERSED LUNATE EPSILON SYMBOL
    (b"bernou", "\u{212C}"),   // SCRIPT CAPITAL B
    (b"beta", "\u{03B2}"),     // GREEK SMALL LETTER BETA
    (b"beth", "\u{2136}"),     // BET SYMBOL
    (b"bfr", "\u{1D51F}"),     // MATHEMATICAL FRAKTUR SMALL B
    (b"blank", "\u{2423}"),    // OPEN BOX
    (b"blk12", "\u{2592}"),    // MEDIUM SHADE
    (b"blk14", "\u{2591}"),    // LIGHT SHADE
    (b"blk34", "\u{2593}"),    // DARK SHADE
    (b"block", "\u{2588}"),    // FULL BLOCK
    (b"bnot", "\u{2310}"),     // REVERSED NOT SIGN
    (b"bopf", "\u{1D553}"),    // MATHEMATICAL DOUBLE-STRUCK SMALL B
    (b"bottom", "\u{22A5}"),   // UP TACK
    (b"bowtie", "\u{22C8}"),   // BOWTIE
    (b"boxDL", "\u{2557}"),    // BOX DRAWINGS DOUBLE DOWN AND LEFT
    (b"boxDR", "\u{2554}"),    // BOX DRAWINGS DOUBLE DOWN AND RIGHT
    (b"boxDl", "\u{2556}"),    // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
    (b"boxDr", "\u{2553}"),    // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
    (b"boxH", "\u{2550}"),     // BOX DRAWINGS DOUBLE HORIZONTAL
    (b"boxHD", "\u{2566}"),    // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
    (b"boxHU", "\u{2569}"),    // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
    (b"boxHd", "\u{2564}"),    // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
    (b"boxHu", "\u{2567}"),    // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
    (b"boxUL", "\u{255D}"),    // BOX DRAWINGS DOUBLE UP AND LEFT
    (b"boxUR", "\u{255A}"),    // BOX DRAWINGS DOUBLE UP AND RIGHT
    (b"boxUl", "\u{255C}"),    // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
    (b"boxUr", "\u{2559}"),    // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
    (b"boxV", "\u{2551}"),     // BOX DRAWINGS DOUBLE VERTICAL
    (b"boxVH", "\u{256C}"),    // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
    (b"boxVL", "\u{2563}"),    // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
    (b"boxVR", "\u{2560}"),    // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
    (b"boxVh", "\u{256B}"),    // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
    (b"boxVl", "\u{2562}"),    // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
    (b"boxVr", "\u{255F}"),    // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
    (b"boxbox", "\u{29C9}"),   // TWO JOINED SQUARES
    (b"boxdL", "\u{2555}"),    // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
    (b"boxdR", "\u{2552}"),    // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
    (b"boxdl", "\u{2510}"),    // BOX DRAWINGS LIGHT DOWN AND LEFT
    (b"boxdr", "\u{250C}"),    // BOX DRAWINGS LIGHT DOWN AND RIGHT
    (b"boxh", "\u{2500}"),     // BOX DRAWINGS LIGHT HORIZONTAL
    (b"boxhD", "\u{2565}"),    // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
    (b"boxhU", "\u{2568}"),    // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
    (b"boxhd", "\u{252C}"),    // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
    (b"boxhu", "\u{2534}"),    // BOX DRAWINGS LIGHT UP AND HORIZONTAL
    (b"boxuL", "\u{255B}"),    // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
    (b"boxuR", "\u{2558}"),    // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
    (b"boxul", "\u{2518}"),    // BOX DRAWINGS LIGHT UP AND LEFT
    (b"boxur", "\u{2514}"),    // BOX DRAWINGS LIGHT UP AND RIGHT
    (b"boxv", "\u{2502}"),     // BOX DRAWINGS LIGHT VERTICAL
    (b"boxvH", "\u{256A}"),    // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
    (b"boxvL", "\u{2561}"),    // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
    (b"boxvR", "\u{255E}"),    // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
    (b"boxvh", "\u{253C}"),    // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
    (b"boxvl", "\u{2524}"),    // BOX DRAWINGS LIGHT VERTICAL AND LEFT
    (b"boxvr", "\u{251C}"),    // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
    (b"bprime", "\u{2035}"),   // REVERSED PRIME
    (b"breve", "\u{02D8}"),    // BREVE
    (b"brvbar", "\u{00A6}"),   // BROKEN BAR
    (b"bscr", "\u{1D4B7}"),    // MATHEMATICAL SCRIPT SMALL B
    (b"bsemi", "\u{204F}"),    // REVERSED SEMICOLON
    (b"bsim", "\u{223D}"),     // REVERSED TILDE
    (b"bsime", "\u{22CD}"),    // REVERSED TILDE EQUALS
    (b"bsol", "\u{005C}"),     // REVERSE SOLIDUS
    (b"bsolb", "\u{29C5}"),    // SQUARED FALLING DIAGONAL SLASH
    (b"bull", "\u{2022}"),     // BULLET
    (b"bump", "\u{224E}"),     // GEOMETRICALLY EQUIVALENT TO
    (b"bumpE", "\u{2AAE}"),    // EQUALS SIGN WITH BUMPY ABOVE
    (b"bumpe", "\u{224F}"),    // DIFFERENCE BETWEEN
    (b"cacute", "\u{0107}"),   // LATIN SMALL LETTER C WITH ACUTE
    (b"cap", "\u{2229}"),      // INTERSECTION
    (b"capand", "\u{2A44}"),   // INTERSECTION WITH LOGICAL AND
    (b"capbrcup", "\u{2A49}"), // INTERSECTION ABOVE BAR ABOVE UNION
    (b"capcap", "\u{2A4B}"),   // INTERSECTION BESIDE AND JOINED WITH INTERSECTION
    (b"capcup", "\u{2A47}"),   // INTERSECTION ABOVE UNION
    (b"capdot", "\u{2A40}"),   // INTERSECTION WITH DOT
    (b"caret", "\u{2041}"),    // CARET INSERTION POINT
    (b"caron", "\u{02C7}"),    // CARON
    (b"ccaps", "\u{2A4D}"),    // CLOSED INTERSECTION WITH SERIFS
    (b"ccaron", "\u{010D}"),   // LATIN SMALL LETTER C WITH CARON
    (b"ccedil", "\u{00E7}"),   // LATIN SMALL LETTER C WITH CEDILLA
    (b"ccirc", "\u{0109}"),    // LATIN SMALL LETTER C WITH CIRCUMFLEX
    (b"ccups", "\u{2A4C}"),    // CLOSED UNION WITH SERIFS
    (b"ccupssm", "\u{2A50}"),  // CLOSED UNION WITH SERIFS AND SMASH PRODUCT
    (b"cdot", "\u{010B}"),     // LATIN SMALL LETTER C WITH DOT ABOVE
    (b"cedil", "\u{00B8}"),    // CEDILLA
    (b"cemptyv", "\u{29B2}"),  // EMPTY SET WITH SMALL CIRCLE ABOVE
    (b"cent", "\u{00A2}"),     // CENT SIGN
    (b"cfr", "\u{1D520}"),     // MATHEMATICAL FRAKTUR SMALL C
    (b"chcy", "\u{0447}"),     // CYRILLIC SMALL LETTER CHE
    (b"check", "\u{2713}"),    // CHECK MARK
    (b"chi", "\u{03C7}"),      // GREEK SMALL LETTER CHI
    (b"cir", "\u{25CB}"),      // WHITE CIRCLE
    (b"cirE", "\u{29C3}"),     // CIRCLE WITH TWO HORIZONTAL STROKES TO THE RIGHT
    (b"circ", "\u{02C6}"),     // MODIFIER LETTER CIRCUMFLEX ACCENT
    (b"cire", "\u{2257}"),     // RING EQUAL TO
    (b"cirfnint", "\u{2A10}"), // CIRCULATION FUNCTION
    (b"cirmid", "\u{2AEF}"),   // VERTICAL LINE WITH CIRCLE ABOVE
    (b"cirscir", "\u{29C2}"),  // CIRCLE WITH SMALL CIRCLE TO THE RIGHT
    (b"clubs", "\u{2663}"),    // BLACK CLUB SUIT
    (b"colon", "\u{003A}"),    // COLON
    (b"colone", "\u{2254}"),   // COLON EQUALS
    (b"comma", "\u{002C}"),    // COMMA
    (b"commat", "\u{0040}"),   // COMMERCIAL AT
    (b"comp", "\u{2201}"),     // COMPLEMENT
    (b"compfn", "\u{2218}"),   // RING OPERATOR
    (b"cong", "\u{2245}"),     // APPROXIMATELY EQUAL TO
    (b"congdot", "\u{2A6D}"),  // CONGRUENT WITH DOT ABOVE
    (b"conint", "\u{222E}"),   // CONTOUR INTEGRAL
    (b"copf", "\u{1D554}"),    // MATHEMATICAL DOUBLE-STRUCK SMALL C
    (b"coprod", "\u{2210}"),   // N-ARY COPRODUCT
    (b"copy", "\u{00A9}"),     // COPYRIGHT SIGN
    (b"copysr", "\u{2117}"),   // SOUND RECORDING COPYRIGHT
    (b"crarr", "\u{21B5}"),    // DOWNWARDS ARROW WITH CORNER LEFTWARDS
    (b"cross", "\u{2717}"),    // BALLOT X
    (b"cscr", "\u{1D4B8}"),    // MATHEMATICAL SCRIPT SMALL C
    (b"csub", "\u{2ACF}"),     // CLOSED SUBSET
    (b"csube", "\u{2AD1}"),    // CLOSED SUBSET OR EQUAL TO
    (b"csup", "\u{2AD0}"),     // CLOSED SUPERSET
    (b"csupe", "\u{2AD2}"),    // CLOSED SUPERSET OR EQUAL TO
    (b"ctdot", "\u{22EF}"),    // MIDLINE HORIZONTAL ELLIPSIS
    (b"cudarrl", "\u{2938}"),  // RIGHT-SIDE ARC CLOCKWISE ARROW
    (b"cudarrr", "\u{2935}"),  // ARROW POINTING RIGHTWARDS THEN CURVING DOWNWARDS
    (b"cuepr", "\u{22DE}"),    // EQUAL TO OR PRECEDES
    (b"cuesc", "\u{22DF}"),    // EQUAL TO OR SUCCEEDS
    (b"cularr", "\u{21B6}"),   // ANTICLOCKWISE TOP SEMICIRCLE ARROW
    (b"cularrp", "\u{293D}"),  // TOP ARC ANTICLOCKWISE ARROW WITH PLUS
    (b"cup", "\u{222A}"),      // UNION
    (b"cupbrcap", "\u{2A48}"), // UNION ABOVE BAR ABOVE INTERSECTION
    (b"cupcap", "\u{2A46}"),   // UNION ABOVE INTERSECTION
    (b"cupcup", "\u{2A4A}"),   // UNION BESIDE AND JOINED WITH UNION
    (b"cupdot", "\u{228D}"),   // MULTISET MULTIPLICATION
    (b"cupor", "\u{2A45}"),    // UNION WITH LOGICAL OR
    (b"curarr", "\u{21B7}"),   // CLOCKWISE TOP SEMICIRCLE ARROW
    (b"curarrm", "\u{293C}"),  // TOP ARC CLOCKWISE ARROW WITH MINUS
    (b"curren", "\u{00A4}"),   // CURRENCY SIGN
    (b"cuvee", "\u{22CE}"),    // CURLY LOGICAL OR
    (b"cuwed", "\u{22CF}"),    // CURLY LOGICAL AND
    (b"cwconint", "\u{2232}"), // CLOCKWISE CONTOUR INTEGRAL
    (b"cwint", "\u{2231}"),    // CLOCKWISE INTEGRAL
    (b"cylcty", "\u{232D}"),   // CYLINDRICITY
    (b"dArr", "\u{21D3}"),     // DOWNWARDS DOUBLE ARROW
    (b"dHar", "\u{2965}"), /* DOWNWARDS HARPOON WITH BARB LEFT BESIDE DOWNWARDS HARPOON WITH BARB RIGHT */
    (b"dagger", "\u{2020}"), // DAGGER
    (b"daleth", "\u{2138}"), // DALET SYMBOL
    (b"darr", "\u{2193}"), // DOWNWARDS ARROW
    (b"dashv", "\u{22A3}"), // LEFT TACK
    (b"dblac", "\u{02DD}"), // DOUBLE ACUTE ACCENT
    (b"dcaron", "\u{010F}"), // LATIN SMALL LETTER D WITH CARON
    (b"dcy", "\u{0434}"),  // CYRILLIC SMALL LETTER DE
    (b"ddarr", "\u{21CA}"), // DOWNWARDS PAIRED ARROWS
    (b"deg", "\u{00B0}"),  // DEGREE SIGN
    (b"delta", "\u{03B4}"), // GREEK SMALL LETTER DELTA
    (b"demptyv", "\u{29B1}"), // EMPTY SET WITH OVERBAR
    (b"dfisht", "\u{297F}"), // DOWN FISH TAIL
    (b"dfr", "\u{1D521}"), // MATHEMATICAL FRAKTUR SMALL D
    (b"dharl", "\u{21C3}"), // DOWNWARDS HARPOON WITH BARB LEFTWARDS
    (b"dharr", "\u{21C2}"), // DOWNWARDS HARPOON WITH BARB RIGHTWARDS
    (b"diam", "\u{22C4}"), // DIAMOND OPERATOR
    (b"diams", "\u{2666}"), // BLACK DIAMOND SUIT
    (b"disin", "\u{22F2}"), // ELEMENT OF WITH LONG HORIZONTAL STROKE
    (b"divide", "\u{00F7}"), // DIVISION SIGN
    (b"divonx", "\u{22C7}"), // DIVISION TIMES
    (b"djcy", "\u{0452}"), // CYRILLIC SMALL LETTER DJE
    (b"dlcorn", "\u{231E}"), // BOTTOM LEFT CORNER
    (b"dlcrop", "\u{230D}"), // BOTTOM LEFT CROP
    (b"dollar", "\u{0024}"), // DOLLAR SIGN
    (b"dopf", "\u{1D555}"), // MATHEMATICAL DOUBLE-STRUCK SMALL D
    (b"dot", "\u{02D9}"),  // DOT ABOVE
    (b"drcorn", "\u{231F}"), // BOTTOM RIGHT CORNER
    (b"drcrop", "\u{230C}"), // BOTTOM RIGHT CROP
    (b"dscr", "\u{1D4B9}"), // MATHEMATICAL SCRIPT SMALL D
    (b"dscy", "\u{0455}"), // CYRILLIC SMALL LETTER DZE
    (b"dsol", "\u{29F6}"), // SOLIDUS WITH OVERBAR
    (b"dstrok", "\u{0111}"), // LATIN SMALL LETTER D WITH STROKE
    (b"dtdot", "\u{22F1}"), // DOWN RIGHT DIAGONAL ELLIPSIS
    (b"dtri", "\u{25BF}"), // WHITE DOWN-POINTING SMALL TRIANGLE
    (b"dtrif", "\u{25BE}"), // BLACK DOWN-POINTING SMALL TRIANGLE
    (b"duarr", "\u{21F5}"), // DOWNWARDS ARROW LEFTWARDS OF UPWARDS ARROW
    (b"duhar", "\u{296F}"), /* DOWNWARDS HARPOON WITH BARB LEFT BESIDE UPWARDS HARPOON WITH BARB RIGHT */
    (b"dwangle", "\u{29A6}"), // OBLIQUE ANGLE OPENING UP
    (b"dzcy", "\u{045F}"),  // CYRILLIC SMALL LETTER DZHE
    (b"dzigrarr", "\u{27FF}"), // LONG RIGHTWARDS SQUIGGLE ARROW
    (b"eDDot", "\u{2A77}"), // EQUALS SIGN WITH TWO DOTS ABOVE AND TWO DOTS BELOW
    (b"eDot", "\u{2251}"),  // GEOMETRICALLY EQUAL TO
    (b"eacute", "\u{00E9}"), // LATIN SMALL LETTER E WITH ACUTE
    (b"easter", "\u{2A6E}"), // EQUALS WITH ASTERISK
    (b"ecaron", "\u{011B}"), // LATIN SMALL LETTER E WITH CARON
    (b"ecir", "\u{2256}"),  // RING IN EQUAL TO
    (b"ecirc", "\u{00EA}"), // LATIN SMALL LETTER E WITH CIRCUMFLEX
    (b"ecolon", "\u{2255}"), // EQUALS COLON
    (b"ecy", "\u{044D}"),   // CYRILLIC SMALL LETTER E
    (b"edot", "\u{0117}"),  // LATIN SMALL LETTER E WITH DOT ABOVE
    (b"efDot", "\u{2252}"), // APPROXIMATELY EQUAL TO OR THE IMAGE OF
    (b"efr", "\u{1D522}"),  // MATHEMATICAL FRAKTUR SMALL E
    (b"eg", "\u{2A9A}"),    // DOUBLE-LINE EQUAL TO OR GREATER-THAN
    (b"egrave", "\u{00E8}"), // LATIN SMALL LETTER E WITH GRAVE
    (b"egs", "\u{2A96}"),   // SLANTED EQUAL TO OR GREATER-THAN
    (b"egsdot", "\u{2A98}"), // SLANTED EQUAL TO OR GREATER-THAN WITH DOT INSIDE
    (b"el", "\u{2A99}"),    // DOUBLE-LINE EQUAL TO OR LESS-THAN
    (b"elinters", "\u{23E7}"), // ELECTRICAL INTERSECTION
    (b"ell", "\u{2113}"),   // SCRIPT SMALL L
    (b"els", "\u{2A95}"),   // SLANTED EQUAL TO OR LESS-THAN
    (b"elsdot", "\u{2A97}"), // SLANTED EQUAL TO OR LESS-THAN WITH DOT INSIDE
    (b"emacr", "\u{0113}"), // LATIN SMALL LETTER E WITH MACRON
    (b"empty", "\u{2205}"), // EMPTY SET
    (b"emsp", "\u{2003}"),  // EM SPACE
    (b"emsp13", "\u{2004}"), // THREE-PER-EM SPACE
    (b"emsp14", "\u{2005}"), // FOUR-PER-EM SPACE
    (b"eng", "\u{014B}"),   // LATIN SMALL LETTER ENG
    (b"ensp", "\u{2002}"),  // EN SPACE
    (b"eogon", "\u{0119}"), // LATIN SMALL LETTER E WITH OGONEK
    (b"eopf", "\u{1D556}"), // MATHEMATICAL DOUBLE-STRUCK SMALL E
    (b"epar", "\u{22D5}"),  // EQUAL AND PARALLEL TO
    (b"eparsl", "\u{29E3}"), // EQUALS SIGN AND SLANTED PARALLEL
    (b"eplus", "\u{2A71}"), // EQUALS SIGN ABOVE PLUS SIGN
    (b"epsi", "\u{03F5}"),  // GREEK LUNATE EPSILON SYMBOL
    (b"epsiv", "\u{03B5}"), // GREEK SMALL LETTER EPSILON
    (b"equals", "\u{003D}"), // EQUALS SIGN
    (b"equest", "\u{225F}"), // QUESTIONED EQUAL TO
    (b"equiv", "\u{2261}"), // IDENTICAL TO
    (b"equivDD", "\u{2A78}"), // EQUIVALENT WITH FOUR DOTS ABOVE
    (b"eqvparsl", "\u{29E5}"), // IDENTICAL TO AND SLANTED PARALLEL
    (b"erDot", "\u{2253}"), // IMAGE OF OR APPROXIMATELY EQUAL TO
    (b"erarr", "\u{2971}"), // EQUALS SIGN ABOVE RIGHTWARDS ARROW
    (b"escr", "\u{212F}"),  // SCRIPT SMALL E
    (b"esdot", "\u{2250}"), // APPROACHES THE LIMIT
    (b"esim", "\u{2242}"),  // MINUS TILDE
    (b"eta", "\u{03B7}"),   // GREEK SMALL LETTER ETA
    (b"eth", "\u{00F0}"),   // LATIN SMALL LETTER ETH
    (b"euml", "\u{00EB}"),  // LATIN SMALL LETTER E WITH DIAERESIS
    (b"euro", "\u{20AC}"),  // EURO SIGN
    (b"excl", "\u{0021}"),  // EXCLAMATION MARK
    (b"exist", "\u{2203}"), // THERE EXISTS
    (b"fcy", "\u{0444}"),   // CYRILLIC SMALL LETTER EF
    (b"female", "\u{2640}"), // FEMALE SIGN
    (b"ffilig", "\u{FB03}"), // LATIN SMALL LIGATURE FFI
    (b"fflig", "\u{FB00}"), // LATIN SMALL LIGATURE FF
    (b"ffllig", "\u{FB04}"), // LATIN SMALL LIGATURE FFL
    (b"ffr", "\u{1D523}"),  // MATHEMATICAL FRAKTUR SMALL F
    (b"filig", "\u{FB01}"), // LATIN SMALL LIGATURE FI
    (b"flat", "\u{266D}"),  // MUSIC FLAT SIGN
    (b"fllig", "\u{FB02}"), // LATIN SMALL LIGATURE FL
    (b"fltns", "\u{25B1}"), // WHITE PARALLELOGRAM
    (b"fnof", "\u{0192}"),  // LATIN SMALL LETTER F WITH HOOK
    (b"fopf", "\u{1D557}"), // MATHEMATICAL DOUBLE-STRUCK SMALL F
    (b"forall", "\u{2200}"), // FOR ALL
    (b"fork", "\u{22D4}"),  // PITCHFORK
    (b"forkv", "\u{2AD9}"), // ELEMENT OF OPENING DOWNWARDS
    (b"fpartint", "\u{2A0D}"), // FINITE PART INTEGRAL
    (b"frac12", "\u{00BD}"), // VULGAR FRACTION ONE HALF
    (b"frac13", "\u{2153}"), // VULGAR FRACTION ONE THIRD
    (b"frac14", "\u{00BC}"), // VULGAR FRACTION ONE QUARTER
    (b"frac15", "\u{2155}"), // VULGAR FRACTION ONE FIFTH
    (b"frac16", "\u{2159}"), // VULGAR FRACTION ONE SIXTH
    (b"frac18", "\u{215B}"), // VULGAR FRACTION ONE EIGHTH
    (b"frac23", "\u{2154}"), // VULGAR FRACTION TWO THIRDS
    (b"frac25", "\u{2156}"), // VULGAR FRACTION TWO FIFTHS
    (b"frac34", "\u{00BE}"), // VULGAR FRACTION THREE QUARTERS
    (b"frac35", "\u{2157}"), // VULGAR FRACTION THREE FIFTHS
    (b"frac38", "\u{215C}"), // VULGAR FRACTION THREE EIGHTHS
    (b"frac45", "\u{2158}"), // VULGAR FRACTION FOUR FIFTHS
    (b"frac56", "\u{215A}"), // VULGAR FRACTION FIVE SIXTHS
    (b"frac58", "\u{215D}"), // VULGAR FRACTION FIVE EIGHTHS
    (b"frac78", "\u{215E}"), // VULGAR FRACTION SEVEN EIGHTHS
    (b"frasl", "\u{2044}"), // FRACTION SLASH
    (b"frown", "\u{2322}"), // FROWN
    (b"fscr", "\u{1D4BB}"), // MATHEMATICAL SCRIPT SMALL F
    (b"gE", "\u{2267}"),    // GREATER-THAN OVER EQUAL TO
    (b"gEl", "\u{2A8C}"),   // GREATER-THAN ABOVE DOUBLE-LINE EQUAL ABOVE LESS-THAN
    (b"gacute", "\u{01F5}"), // LATIN SMALL LETTER G WITH ACUTE
    (b"gamma", "\u{03B3}"), // GREEK SMALL LETTER GAMMA
    (b"gammad", "\u{03DD}"), // GREEK SMALL LETTER DIGAMMA
    (b"gap", "\u{2A86}"),   // GREATER-THAN OR APPROXIMATE
    (b"gbreve", "\u{011F}"), // LATIN SMALL LETTER G WITH BREVE
    (b"gcirc", "\u{011D}"), // LATIN SMALL LETTER G WITH CIRCUMFLEX
    (b"gcy", "\u{0433}"),   // CYRILLIC SMALL LETTER GHE
    (b"gdot", "\u{0121}"),  // LATIN SMALL LETTER G WITH DOT ABOVE
    (b"ge", "\u{2265}"),    // GREATER-THAN OR EQUAL TO
    (b"gel", "\u{22DB}"),   // GREATER-THAN EQUAL TO OR LESS-THAN
    (b"ges", "\u{2A7E}"),   // GREATER-THAN OR SLANTED EQUAL TO
    (b"gescc", "\u{2AA9}"), // GREATER-THAN CLOSED BY CURVE ABOVE SLANTED EQUAL
    (b"gesdot", "\u{2A80}"), // GREATER-THAN OR SLANTED EQUAL TO WITH DOT INSIDE
    (b"gesdoto", "\u{2A82}"), // GREATER-THAN OR SLANTED EQUAL TO WITH DOT ABOVE
    (b"gesdotol", "\u{2A84}"), // GREATER-THAN OR SLANTED EQUAL TO WITH DOT ABOVE LEFT
    (b"gesles", "\u{2A94}"), // GREATER-THAN ABOVE SLANTED EQUAL ABOVE LESS-THAN ABOVE SLANTED EQUAL
    (b"gfr", "\u{1D524}"),  // MATHEMATICAL FRAKTUR SMALL G
    (b"gimel", "\u{2137}"), // GIMEL SYMBOL
    (b"gjcy", "\u{0453}"),  // CYRILLIC SMALL LETTER GJE
    (b"gl", "\u{2277}"),    // GREATER-THAN OR LESS-THAN
    (b"glE", "\u{2A92}"),   // GREATER-THAN ABOVE LESS-THAN ABOVE DOUBLE-LINE EQUAL
    (b"gla", "\u{2AA5}"),   // GREATER-THAN BESIDE LESS-THAN
    (b"glj", "\u{2AA4}"),   // GREATER-THAN OVERLAPPING LESS-THAN
    (b"gnE", "\u{2269}"),   // GREATER-THAN BUT NOT EQUAL TO
    (b"gnap", "\u{2A8A}"),  // GREATER-THAN AND NOT APPROXIMATE
    (b"gne", "\u{2A88}"),   // GREATER-THAN AND SINGLE-LINE NOT EQUAL TO
    (b"gnsim", "\u{22E7}"), // GREATER-THAN BUT NOT EQUIVALENT TO
    (b"gopf", "\u{1D558}"), // MATHEMATICAL DOUBLE-STRUCK SMALL G
    (b"grave", "\u{0060}"), // GRAVE ACCENT
    (b"gscr", "\u{210A}"),  // SCRIPT SMALL G
    (b"gsim", "\u{2273}"),  // GREATER-THAN OR EQUIVALENT TO
    (b"gsime", "\u{2A8E}"), // GREATER-THAN ABOVE SIMILAR OR EQUAL
    (b"gsiml", "\u{2A90}"), // GREATER-THAN ABOVE SIMILAR ABOVE LESS-THAN
    (b"gt", "\u{003E}"),    // GREATER-THAN SIGN
    (b"gtcc", "\u{2AA7}"),  // GREATER-THAN CLOSED BY CURVE
    (b"gtcir", "\u{2A7A}"), // GREATER-THAN WITH CIRCLE INSIDE
    (b"gtdot", "\u{22D7}"), // GREATER-THAN WITH DOT
    (b"gtlPar", "\u{2995}"), // DOUBLE LEFT ARC GREATER-THAN BRACKET
    (b"gtquest", "\u{2A7C}"), // GREATER-THAN WITH QUESTION MARK ABOVE
    (b"gtrarr", "\u{2978}"), // GREATER-THAN ABOVE RIGHTWARDS ARROW
    (b"hArr", "\u{21D4}"),  // LEFT RIGHT DOUBLE ARROW
    (b"hairsp", "\u{200A}"), // HAIR SPACE
    (b"hamilt", "\u{210B}"), // SCRIPT CAPITAL H
    (b"hardcy", "\u{044A}"), // CYRILLIC SMALL LETTER HARD SIGN
    (b"harr", "\u{2194}"),  // LEFT RIGHT ARROW
    (b"harrcir", "\u{2948}"), // LEFT RIGHT ARROW THROUGH SMALL CIRCLE
    (b"harrw", "\u{21AD}"), // LEFT RIGHT WAVE ARROW
    (b"hcirc", "\u{0125}"), // LATIN SMALL LETTER H WITH CIRCUMFLEX
    (b"hearts", "\u{2665}"), // BLACK HEART SUIT
    (b"hellip", "\u{2026}"), // HORIZONTAL ELLIPSIS
    (b"hercon", "\u{22B9}"), // HERMITIAN CONJUGATE MATRIX
    (b"hfr", "\u{1D525}"),  // MATHEMATICAL FRAKTUR SMALL H
    (b"hoarr", "\u{21FF}"), // LEFT RIGHT OPEN-HEADED ARROW
    (b"homtht", "\u{223B}"), // HOMOTHETIC
    (b"hopf", "\u{1D559}"), // MATHEMATICAL DOUBLE-STRUCK SMALL H
    (b"horbar", "\u{2015}"), // HORIZONTAL BAR
    (b"hscr", "\u{1D4BD}"), // MATHEMATICAL SCRIPT SMALL H
    (b"hstrok", "\u{0127}"), // LATIN SMALL LETTER H WITH STROKE
    (b"hybull", "\u{2043}"), // HYPHEN BULLET
    (b"hyphen", "\u{2010}"), // HYPHEN
    (b"iacute", "\u{00ED}"), // LATIN SMALL LETTER I WITH ACUTE
    (b"icirc", "\u{00EE}"), // LATIN SMALL LETTER I WITH CIRCUMFLEX
    (b"icy", "\u{0438}"),   // CYRILLIC SMALL LETTER I
    (b"iecy", "\u{0435}"),  // CYRILLIC SMALL LETTER IE
    (b"iexcl", "\u{00A1}"), // INVERTED EXCLAMATION MARK
    (b"ifr", "\u{1D526}"),  // MATHEMATICAL FRAKTUR SMALL I
    (b"igrave", "\u{00EC}"), // LATIN SMALL LETTER I WITH GRAVE
    (b"iinfin", "\u{29DC}"), // INCOMPLETE INFINITY
    (b"iiota", "\u{2129}"), // TURNED GREEK SMALL LETTER IOTA
    (b"ijlig", "\u{0133}"), // LATIN SMALL LIGATURE IJ
    (b"imacr", "\u{012B}"), // LATIN SMALL LETTER I WITH MACRON
    (b"image", "\u{2111}"), // BLACK-LETTER CAPITAL I
    (b"imath", "\u{0131}"), // LATIN SMALL LETTER DOTLESS I
    (b"imof", "\u{22B7}"),  // IMAGE OF
    (b"imped", "\u{01B5}"), // LATIN CAPITAL LETTER Z WITH STROKE
    (b"incare", "\u{2105}"), // CARE OF
    (b"infin", "\u{221E}"), // INFINITY
    (b"infintie", "\u{29DD}"), // TIE OVER INFINITY
    (b"int", "\u{222B}"),   // INTEGRAL
    (b"intcal", "\u{22BA}"), // INTERCALATE
    (b"integers", "\u{2124}"), // DOUBLE-STRUCK CAPITAL Z
    (b"intlarhk", "\u{2A17}"), // INTEGRAL WITH LEFTWARDS ARROW WITH HOOK
    (b"iocy", "\u{0451}"),  // CYRILLIC SMALL LETTER IO
    (b"iogon", "\u{012F}"), // LATIN SMALL LETTER I WITH OGONEK
    (b"iopf", "\u{1D55A}"), // MATHEMATICAL DOUBLE-STRUCK SMALL I
    (b"iota", "\u{03B9}"),  // GREEK SMALL LETTER IOTA
    (b"iprod", "\u{2A3C}"), // INTERIOR PRODUCT
    (b"iquest", "\u{00BF}"), // INVERTED QUESTION MARK
    (b"iscr", "\u{1D4BE}"), // MATHEMATICAL SCRIPT SMALL I
    (b"isin", "\u{2208}"),  // ELEMENT OF
    (b"isinE", "\u{22F9}"), // ELEMENT OF WITH TWO HORIZONTAL STROKES
    (b"isindot", "\u{22F5}"), // ELEMENT OF WITH DOT ABOVE
    (b"isins", "\u{22F4}"), // SMALL ELEMENT OF WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    (b"isinsv", "\u{22F3}"), // ELEMENT OF WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    (b"itilde", "\u{0129}"), // LATIN SMALL LETTER I WITH TILDE
    (b"iukcy", "\u{0456}"), // CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
    (b"iuml", "\u{00EF}"),  // LATIN SMALL LETTER I WITH DIAERESIS
    (b"jcirc", "\u{0135}"), // LATIN SMALL LETTER J WITH CIRCUMFLEX
    (b"jcy", "\u{0439}"),   // CYRILLIC SMALL LETTER SHORT I
    (b"jfr", "\u{1D527}"),  // MATHEMATICAL FRAKTUR SMALL J
    (b"jmath", "\u{0237}"), // LATIN SMALL LETTER DOTLESS J
    (b"jopf", "\u{1D55B}"), // MATHEMATICAL DOUBLE-STRUCK SMALL J
    (b"jscr", "\u{1D4BF}"), // MATHEMATICAL SCRIPT SMALL J
    (b"jsercy", "\u{0458}"), // CYRILLIC SMALL LETTER JE
    (b"jukcy", "\u{0454}"), // CYRILLIC SMALL LETTER UKRAINIAN IE
    (b"kappa", "\u{03BA}"), // GREEK SMALL LETTER KAPPA
    (b"kappav", "\u{03F0}"), // GREEK KAPPA SYMBOL
    (b"kcedil", "\u{0137}"), // LATIN SMALL LETTER K WITH CEDILLA
    (b"kcy", "\u{043A}"),   // CYRILLIC SMALL LETTER KA
    (b"kfr", "\u{1D528}"),  // MATHEMATICAL FRAKTUR SMALL K
    (b"kgreen", "\u{0138}"), // LATIN SMALL LETTER KRA
    (b"khcy", "\u{0445}"),  // CYRILLIC SMALL LETTER HA
    (b"kjcy", "\u{045C}"),  // CYRILLIC SMALL LETTER KJE
    (b"kopf", "\u{1D55C}"), // MATHEMATICAL DOUBLE-STRUCK SMALL K
    (b"kscr", "\u{1D4C0}"), // MATHEMATICAL SCRIPT SMALL K
    (b"lAarr", "\u{21DA}"), // LEFTWARDS TRIPLE ARROW
    (b"lArr", "\u{21D0}"),  // LEFTWARDS DOUBLE ARROW
    (b"lAtail", "\u{291B}"), // LEFTWARDS DOUBLE ARROW-TAIL
    (b"lBarr", "\u{290E}"), // LEFTWARDS TRIPLE DASH ARROW
    (b"lE", "\u{2266}"),    // LESS-THAN OVER EQUAL TO
    (b"lEg", "\u{2A8B}"),   // LESS-THAN ABOVE DOUBLE-LINE EQUAL ABOVE GREATER-THAN
    (b"lHar", "\u{2962}"), // LEFTWARDS HARPOON WITH BARB UP ABOVE LEFTWARDS HARPOON WITH BARB DOWN
    (b"lacute", "\u{013A}"), // LATIN SMALL LETTER L WITH ACUTE
    (b"laemptyv", "\u{29B4}"), // EMPTY SET WITH LEFT ARROW ABOVE
    (b"lambda", "\u{03BB}"), // GREEK SMALL LETTER LAMDA
    (b"lang", "\u{27E8}"), // MATHEMATICAL LEFT ANGLE BRACKET
    (b"langd", "\u{2991}"), // LEFT ANGLE BRACKET WITH DOT
    (b"lap", "\u{2A85}"),  // LESS-THAN OR APPROXIMATE
    (b"laquo", "\u{00AB}"), // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    (b"larr", "\u{2190}"), // LEFTWARDS ARROW
    (b"larrb", "\u{21E4}"), // LEFTWARDS ARROW TO BAR
    (b"larrbfs", "\u{291F}"), // LEFTWARDS ARROW FROM BAR TO BLACK DIAMOND
    (b"larrfs", "\u{291D}"), // LEFTWARDS ARROW TO BLACK DIAMOND
    (b"larrhk", "\u{21A9}"), // LEFTWARDS ARROW WITH HOOK
    (b"larrlp", "\u{21AB}"), // LEFTWARDS ARROW WITH LOOP
    (b"larrpl", "\u{2939}"), // LEFT-SIDE ARC ANTICLOCKWISE ARROW
    (b"larrsim", "\u{2973}"), // LEFTWARDS ARROW ABOVE TILDE OPERATOR
    (b"larrtl", "\u{21A2}"), // LEFTWARDS ARROW WITH TAIL
    (b"lat", "\u{2AAB}"),  // LARGER THAN
    (b"latail", "\u{2919}"), // LEFTWARDS ARROW-TAIL
    (b"late", "\u{2AAD}"), // LARGER THAN OR EQUAL TO
    (b"lbarr", "\u{290C}"), // LEFTWARDS DOUBLE DASH ARROW
    (b"lbbrk", "\u{2772}"), // LIGHT LEFT TORTOISE SHELL BRACKET ORNAMENT
    (b"lbrke", "\u{298B}"), // LEFT SQUARE BRACKET WITH UNDERBAR
    (b"lbrksld", "\u{298F}"), // LEFT SQUARE BRACKET WITH TICK IN BOTTOM CORNER
    (b"lbrkslu", "\u{298D}"), // LEFT SQUARE BRACKET WITH TICK IN TOP CORNER
    (b"lcaron", "\u{013E}"), // LATIN SMALL LETTER L WITH CARON
    (b"lcedil", "\u{013C}"), // LATIN SMALL LETTER L WITH CEDILLA
    (b"lceil", "\u{2308}"), // LEFT CEILING
    (b"lcub", "\u{007B}"), // LEFT CURLY BRACKET
    (b"lcy", "\u{043B}"),  // CYRILLIC SMALL LETTER EL
    (b"ldca", "\u{2936}"), // ARROW POINTING DOWNWARDS THEN CURVING LEFTWARDS
    (b"ldquo", "\u{201C}"), // LEFT DOUBLE QUOTATION MARK
    (b"ldquor", "\u{201E}"), // DOUBLE LOW-9 QUOTATION MARK
    (b"ldrdhar", "\u{2967}"), /* LEFTWARDS HARPOON WITH BARB DOWN ABOVE RIGHTWARDS HARPOON WITH BARB DOWN */
    (b"ldrushar", "\u{294B}"), // LEFT BARB DOWN RIGHT BARB UP HARPOON
    (b"ldsh", "\u{21B2}"),    // DOWNWARDS ARROW WITH TIP LEFTWARDS
    (b"le", "\u{2264}"),      // LESS-THAN OR EQUAL TO
    (b"leg", "\u{22DA}"),     // LESS-THAN EQUAL TO OR GREATER-THAN
    (b"les", "\u{2A7D}"),     // LESS-THAN OR SLANTED EQUAL TO
    (b"lescc", "\u{2AA8}"),   // LESS-THAN CLOSED BY CURVE ABOVE SLANTED EQUAL
    (b"lesdot", "\u{2A7F}"),  // LESS-THAN OR SLANTED EQUAL TO WITH DOT INSIDE
    (b"lesdoto", "\u{2A81}"), // LESS-THAN OR SLANTED EQUAL TO WITH DOT ABOVE
    (b"lesdotor", "\u{2A83}"), // LESS-THAN OR SLANTED EQUAL TO WITH DOT ABOVE RIGHT
    (b"lesges", "\u{2A93}"), /* LESS-THAN ABOVE SLANTED EQUAL ABOVE GREATER-THAN ABOVE SLANTED EQUAL */
    (b"lfisht", "\u{297C}"), // LEFT FISH TAIL
    (b"lfloor", "\u{230A}"), // LEFT FLOOR
    (b"lfr", "\u{1D529}"),   // MATHEMATICAL FRAKTUR SMALL L
    (b"lg", "\u{2276}"),     // LESS-THAN OR GREATER-THAN
    (b"lgE", "\u{2A91}"),    // LESS-THAN ABOVE GREATER-THAN ABOVE DOUBLE-LINE EQUAL
    (b"lhard", "\u{21BD}"),  // LEFTWARDS HARPOON WITH BARB DOWNWARDS
    (b"lharu", "\u{21BC}"),  // LEFTWARDS HARPOON WITH BARB UPWARDS
    (b"lharul", "\u{296A}"), // LEFTWARDS HARPOON WITH BARB UP ABOVE LONG DASH
    (b"lhblk", "\u{2584}"),  // LOWER HALF BLOCK
    (b"ljcy", "\u{0459}"),   // CYRILLIC SMALL LETTER LJE
    (b"llarr", "\u{21C7}"),  // LEFTWARDS PAIRED ARROWS
    (b"llhard", "\u{296B}"), // LEFTWARDS HARPOON WITH BARB DOWN BELOW LONG DASH
    (b"lltri", "\u{25FA}"),  // LOWER LEFT TRIANGLE
    (b"lmidot", "\u{0140}"), // LATIN SMALL LETTER L WITH MIDDLE DOT
    (b"lmoust", "\u{23B0}"), // UPPER LEFT OR LOWER RIGHT CURLY BRACKET SECTION
    (b"lnE", "\u{2268}"),    // LESS-THAN BUT NOT EQUAL TO
    (b"lnap", "\u{2A89}"),   // LESS-THAN AND NOT APPROXIMATE
    (b"lne", "\u{2A87}"),    // LESS-THAN AND SINGLE-LINE NOT EQUAL TO
    (b"lnsim", "\u{22E6}"),  // LESS-THAN BUT NOT EQUIVALENT TO
    (b"loang", "\u{27EC}"),  // MATHEMATICAL LEFT WHITE TORTOISE SHELL BRACKET
    (b"loarr", "\u{21FD}"),  // LEFTWARDS OPEN-HEADED ARROW
    (b"lobrk", "\u{27E6}"),  // MATHEMATICAL LEFT WHITE SQUARE BRACKET
    (b"lopar", "\u{2985}"),  // LEFT WHITE PARENTHESIS
    (b"lopf", "\u{1D55D}"),  // MATHEMATICAL DOUBLE-STRUCK SMALL L
    (b"loplus", "\u{2A2D}"), // PLUS SIGN IN LEFT HALF CIRCLE
    (b"lotimes", "\u{2A34}"), // MULTIPLICATION SIGN IN LEFT HALF CIRCLE
    (b"lowast", "\u{2217}"), // ASTERISK OPERATOR
    (b"lowbar", "\u{005F}"), // LOW LINE
    (b"loz", "\u{25CA}"),    // LOZENGE
    (b"lozf", "\u{29EB}"),   // BLACK LOZENGE
    (b"lpar", "\u{0028}"),   // LEFT PARENTHESIS
    (b"lparlt", "\u{2993}"), // LEFT ARC LESS-THAN BRACKET
    (b"lrarr", "\u{21C6}"),  // LEFTWARDS ARROW OVER RIGHTWARDS ARROW
    (b"lrhar", "\u{21CB}"),  // LEFTWARDS HARPOON OVER RIGHTWARDS HARPOON
    (b"lrhard", "\u{296D}"), // RIGHTWARDS HARPOON WITH BARB DOWN BELOW LONG DASH
    (b"lrm", "\u{200E}"),    // LEFT-TO-RIGHT MARK
    (b"lrtri", "\u{22BF}"),  // RIGHT TRIANGLE
    (b"lsaquo", "\u{2039}"), // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
    (b"lscr", "\u{1D4C1}"),  // MATHEMATICAL SCRIPT SMALL L
    (b"lsh", "\u{21B0}"),    // UPWARDS ARROW WITH TIP LEFTWARDS
    (b"lsim", "\u{2272}"),   // LESS-THAN OR EQUIVALENT TO
    (b"lsime", "\u{2A8D}"),  // LESS-THAN ABOVE SIMILAR OR EQUAL
    (b"lsimg", "\u{2A8F}"),  // LESS-THAN ABOVE SIMILAR ABOVE GREATER-THAN
    (b"lsqb", "\u{005B}"),   // LEFT SQUARE BRACKET
    (b"lsquo", "\u{2018}"),  // LEFT SINGLE QUOTATION MARK
    (b"lsquor", "\u{201A}"), // SINGLE LOW-9 QUOTATION MARK
    (b"lstrok", "\u{0142}"), // LATIN SMALL LETTER L WITH STROKE
    (b"lt", "\u{003C}"),     // LESS-THAN SIGN
    (b"ltcc", "\u{2AA6}"),   // LESS-THAN CLOSED BY CURVE
    (b"ltcir", "\u{2A79}"),  // LESS-THAN WITH CIRCLE INSIDE
    (b"ltdot", "\u{22D6}"),  // LESS-THAN WITH DOT
    (b"lthree", "\u{22CB}"), // LEFT SEMIDIRECT PRODUCT
    (b"ltimes", "\u{22C9}"), // LEFT NORMAL FACTOR SEMIDIRECT PRODUCT
    (b"ltlarr", "\u{2976}"), // LESS-THAN ABOVE LEFTWARDS ARROW
    (b"ltquest", "\u{2A7B}"), // LESS-THAN WITH QUESTION MARK ABOVE
    (b"ltrPar", "\u{2996}"), // DOUBLE RIGHT ARC LESS-THAN BRACKET
    (b"ltri", "\u{25C3}"),   // WHITE LEFT-POINTING SMALL TRIANGLE
    (b"ltrie", "\u{22B4}"),  // NORMAL SUBGROUP OF OR EQUAL TO
    (b"ltrif", "\u{25C2}"),  // BLACK LEFT-POINTING SMALL TRIANGLE
    (b"lurdshar", "\u{294A}"), // LEFT BARB UP RIGHT BARB DOWN HARPOON
    (b"luruhar", "\u{2966}"), /* LEFTWARDS HARPOON WITH BARB UP ABOVE RIGHTWARDS HARPOON WITH BARB UP */
    (b"mDDot", "\u{223A}"),   // GEOMETRIC PROPORTION
    (b"macr", "\u{00AF}"),    // MACRON
    (b"male", "\u{2642}"),    // MALE SIGN
    (b"malt", "\u{2720}"),    // MALTESE CROSS
    (b"map", "\u{21A6}"),     // RIGHTWARDS ARROW FROM BAR
    (b"marker", "\u{25AE}"),  // BLACK VERTICAL RECTANGLE
    (b"mcomma", "\u{2A29}"),  // MINUS SIGN WITH COMMA ABOVE
    (b"mcy", "\u{043C}"),     // CYRILLIC SMALL LETTER EM
    (b"mdash", "\u{2014}"),   // EM DASH
    (b"mfr", "\u{1D52A}"),    // MATHEMATICAL FRAKTUR SMALL M
    (b"mho", "\u{2127}"),     // INVERTED OHM SIGN
    (b"micro", "\u{00B5}"),   // MICRO SIGN
    (b"mid", "\u{2223}"),     // DIVIDES
    (b"midcir", "\u{2AF0}"),  // VERTICAL LINE WITH CIRCLE BELOW
    (b"middot", "\u{00B7}"),  // MIDDLE DOT
    (b"minus", "\u{2212}"),   // MINUS SIGN
    (b"minusb", "\u{229F}"),  // SQUARED MINUS
    (b"minusd", "\u{2238}"),  // DOT MINUS
    (b"minusdu", "\u{2A2A}"), // MINUS SIGN WITH DOT BELOW
    (b"mlcp", "\u{2ADB}"),    // TRANSVERSAL INTERSECTION
    (b"mnplus", "\u{2213}"),  // MINUS-OR-PLUS SIGN
    (b"models", "\u{22A7}"),  // MODELS
    (b"mopf", "\u{1D55E}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL M
    (b"mscr", "\u{1D4C2}"),   // MATHEMATICAL SCRIPT SMALL M
    (b"mu", "\u{03BC}"),      // GREEK SMALL LETTER MU
    (b"mumap", "\u{22B8}"),   // MULTIMAP
    (b"nVDash", "\u{22AF}"),  // NEGATED DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE
    (b"nVdash", "\u{22AE}"),  // DOES NOT FORCE
    (b"nabla", "\u{2207}"),   // NABLA
    (b"nacute", "\u{0144}"),  // LATIN SMALL LETTER N WITH ACUTE
    (b"nap", "\u{2249}"),     // NOT ALMOST EQUAL TO
    (b"napos", "\u{0149}"),   // LATIN SMALL LETTER N PRECEDED BY APOSTROPHE
    (b"natur", "\u{266E}"),   // MUSIC NATURAL SIGN
    (b"nbsp", "\u{00A0}"),    // NO-BREAK SPACE
    (b"ncap", "\u{2A43}"),    // INTERSECTION WITH OVERBAR
    (b"ncaron", "\u{0148}"),  // LATIN SMALL LETTER N WITH CARON
    (b"ncedil", "\u{0146}"),  // LATIN SMALL LETTER N WITH CEDILLA
    (b"ncong", "\u{2247}"),   // NEITHER APPROXIMATELY NOR ACTUALLY EQUAL TO
    (b"ncup", "\u{2A42}"),    // UNION WITH OVERBAR
    (b"ncy", "\u{043D}"),     // CYRILLIC SMALL LETTER EN
    (b"ndash", "\u{2013}"),   // EN DASH
    (b"ne", "\u{2260}"),      // NOT EQUAL TO
    (b"neArr", "\u{21D7}"),   // NORTH EAST DOUBLE ARROW
    (b"nearhk", "\u{2924}"),  // NORTH EAST ARROW WITH HOOK
    (b"nearr", "\u{2197}"),   // NORTH EAST ARROW
    (b"nequiv", "\u{2262}"),  // NOT IDENTICAL TO
    (b"nesear", "\u{2928}"),  // NORTH EAST ARROW AND SOUTH EAST ARROW
    (b"nexist", "\u{2204}"),  // THERE DOES NOT EXIST
    (b"nfr", "\u{1D52B}"),    // MATHEMATICAL FRAKTUR SMALL N
    (b"nge", "\u{2271}"),     // NEITHER GREATER-THAN NOR EQUAL TO
    (b"ngsim", "\u{2275}"),   // NEITHER GREATER-THAN NOR EQUIVALENT TO
    (b"ngt", "\u{226F}"),     // NOT GREATER-THAN
    (b"nhArr", "\u{21CE}"),   // LEFT RIGHT DOUBLE ARROW WITH STROKE
    (b"nharr", "\u{21AE}"),   // LEFT RIGHT ARROW WITH STROKE
    (b"nhpar", "\u{2AF2}"),   // PARALLEL WITH HORIZONTAL STROKE
    (b"nis", "\u{22FC}"),     // SMALL CONTAINS WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    (b"nisd", "\u{22FA}"),    // CONTAINS WITH LONG HORIZONTAL STROKE
    (b"niv", "\u{220B}"),     // CONTAINS AS MEMBER
    (b"njcy", "\u{045A}"),    // CYRILLIC SMALL LETTER NJE
    (b"nlArr", "\u{21CD}"),   // LEFTWARDS DOUBLE ARROW WITH STROKE
    (b"nlarr", "\u{219A}"),   // LEFTWARDS ARROW WITH STROKE
    (b"nldr", "\u{2025}"),    // TWO DOT LEADER
    (b"nle", "\u{2270}"),     // NEITHER LESS-THAN NOR EQUAL TO
    (b"nlsim", "\u{2274}"),   // NEITHER LESS-THAN NOR EQUIVALENT TO
    (b"nlt", "\u{226E}"),     // NOT LESS-THAN
    (b"nltri", "\u{22EA}"),   // NOT NORMAL SUBGROUP OF
    (b"nltrie", "\u{22EC}"),  // NOT NORMAL SUBGROUP OF OR EQUAL TO
    (b"nmid", "\u{2224}"),    // DOES NOT DIVIDE
    (b"nopf", "\u{1D55F}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL N
    (b"not", "\u{00AC}"),     // NOT SIGN
    (b"notin", "\u{2209}"),   // NOT AN ELEMENT OF
    (b"notinvb", "\u{22F7}"), // SMALL ELEMENT OF WITH OVERBAR
    (b"notinvc", "\u{22F6}"), // ELEMENT OF WITH OVERBAR
    (b"notni", "\u{220C}"),   // DOES NOT CONTAIN AS MEMBER
    (b"notnivb", "\u{22FE}"), // SMALL CONTAINS WITH OVERBAR
    (b"notnivc", "\u{22FD}"), // CONTAINS WITH OVERBAR
    (b"npar", "\u{2226}"),    // NOT PARALLEL TO
    (b"npolint", "\u{2A14}"), // LINE INTEGRATION NOT INCLUDING THE POLE
    (b"npr", "\u{2280}"),     // DOES NOT PRECEDE
    (b"nprcue", "\u{22E0}"),  // DOES NOT PRECEDE OR EQUAL
    (b"nrArr", "\u{21CF}"),   // RIGHTWARDS DOUBLE ARROW WITH STROKE
    (b"nrarr", "\u{219B}"),   // RIGHTWARDS ARROW WITH STROKE
    (b"nrtri", "\u{22EB}"),   // DOES NOT CONTAIN AS NORMAL SUBGROUP
    (b"nrtrie", "\u{22ED}"),  // DOES NOT CONTAIN AS NORMAL SUBGROUP OR EQUAL
    (b"nsc", "\u{2281}"),     // DOES NOT SUCCEED
    (b"nsccue", "\u{22E1}"),  // DOES NOT SUCCEED OR EQUAL
    (b"nscr", "\u{1D4C3}"),   // MATHEMATICAL SCRIPT SMALL N
    (b"nsim", "\u{2241}"),    // NOT TILDE
    (b"nsime", "\u{2244}"),   // NOT ASYMPTOTICALLY EQUAL TO
    (b"nsqsube", "\u{22E2}"), // NOT SQUARE IMAGE OF OR EQUAL TO
    (b"nsqsupe", "\u{22E3}"), // NOT SQUARE ORIGINAL OF OR EQUAL TO
    (b"nsub", "\u{2284}"),    // NOT A SUBSET OF
    (b"nsube", "\u{2288}"),   // NEITHER A SUBSET OF NOR EQUAL TO
    (b"nsup", "\u{2285}"),    // NOT A SUPERSET OF
    (b"nsupe", "\u{2289}"),   // NEITHER A SUPERSET OF NOR EQUAL TO
    (b"ntgl", "\u{2279}"),    // NEITHER GREATER-THAN NOR LESS-THAN
    (b"ntilde", "\u{00F1}"),  // LATIN SMALL LETTER N WITH TILDE
    (b"ntlg", "\u{2278}"),    // NEITHER LESS-THAN NOR GREATER-THAN
    (b"nu", "\u{03BD}"),      // GREEK SMALL LETTER NU
    (b"num", "\u{0023}"),     // NUMBER SIGN
    (b"numero", "\u{2116}"),  // NUMERO SIGN
    (b"numsp", "\u{2007}"),   // FIGURE SPACE
    (b"nvDash", "\u{22AD}"),  // NOT TRUE
    (b"nvHarr", "\u{2904}"),  // LEFT RIGHT DOUBLE ARROW WITH VERTICAL STROKE
    (b"nvdash", "\u{22AC}"),  // DOES NOT PROVE
    (b"nvinfin", "\u{29DE}"), // INFINITY NEGATED WITH VERTICAL BAR
    (b"nvlArr", "\u{2902}"),  // LEFTWARDS DOUBLE ARROW WITH VERTICAL STROKE
    (b"nvrArr", "\u{2903}"),  // RIGHTWARDS DOUBLE ARROW WITH VERTICAL STROKE
    (b"nwArr", "\u{21D6}"),   // NORTH WEST DOUBLE ARROW
    (b"nwarhk", "\u{2923}"),  // NORTH WEST ARROW WITH HOOK
    (b"nwarr", "\u{2196}"),   // NORTH WEST ARROW
    (b"nwnear", "\u{2927}"),  // NORTH WEST ARROW AND NORTH EAST ARROW
    (b"oS", "\u{24C8}"),      // CIRCLED LATIN CAPITAL LETTER S
    (b"oacute", "\u{00F3}"),  // LATIN SMALL LETTER O WITH ACUTE
    (b"oast", "\u{229B}"),    // CIRCLED ASTERISK OPERATOR
    (b"ocir", "\u{229A}"),    // CIRCLED RING OPERATOR
    (b"ocirc", "\u{00F4}"),   // LATIN SMALL LETTER O WITH CIRCUMFLEX
    (b"ocy", "\u{043E}"),     // CYRILLIC SMALL LETTER O
    (b"odash", "\u{229D}"),   // CIRCLED DASH
    (b"odblac", "\u{0151}"),  // LATIN SMALL LETTER O WITH DOUBLE ACUTE
    (b"odiv", "\u{2A38}"),    // CIRCLED DIVISION SIGN
    (b"odot", "\u{2299}"),    // CIRCLED DOT OPERATOR
    (b"odsold", "\u{29BC}"),  // CIRCLED ANTICLOCKWISE-ROTATED DIVISION SIGN
    (b"oelig", "\u{0153}"),   // LATIN SMALL LIGATURE OE
    (b"ofcir", "\u{29BF}"),   // CIRCLED BULLET
    (b"ofr", "\u{1D52C}"),    // MATHEMATICAL FRAKTUR SMALL O
    (b"ogon", "\u{02DB}"),    // OGONEK
    (b"ograve", "\u{00F2}"),  // LATIN SMALL LETTER O WITH GRAVE
    (b"ogt", "\u{29C1}"),     // CIRCLED GREATER-THAN
    (b"ohbar", "\u{29B5}"),   // CIRCLE WITH HORIZONTAL BAR
    (b"ohm", "\u{2126}"),     // OHM SIGN
    (b"olarr", "\u{21BA}"),   // ANTICLOCKWISE OPEN CIRCLE ARROW
    (b"olcir", "\u{29BE}"),   // CIRCLED WHITE BULLET
    (b"olcross", "\u{29BB}"), // CIRCLE WITH SUPERIMPOSED X
    (b"oline", "\u{203E}"),   // OVERLINE
    (b"olt", "\u{29C0}"),     // CIRCLED LESS-THAN
    (b"omacr", "\u{014D}"),   // LATIN SMALL LETTER O WITH MACRON
    (b"omega", "\u{03C9}"),   // GREEK SMALL LETTER OMEGA
    (b"omicron", "\u{03BF}"), // GREEK SMALL LETTER OMICRON
    (b"omid", "\u{29B6}"),    // CIRCLED VERTICAL BAR
    (b"ominus", "\u{2296}"),  // CIRCLED MINUS
    (b"oopf", "\u{1D560}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL O
    (b"opar", "\u{29B7}"),    // CIRCLED PARALLEL
    (b"operp", "\u{29B9}"),   // CIRCLED PERPENDICULAR
    (b"oplus", "\u{2295}"),   // CIRCLED PLUS
    (b"or", "\u{2228}"),      // LOGICAL OR
    (b"orarr", "\u{21BB}"),   // CLOCKWISE OPEN CIRCLE ARROW
    (b"ord", "\u{2A5D}"),     // LOGICAL OR WITH HORIZONTAL DASH
    (b"order", "\u{2134}"),   // SCRIPT SMALL O
    (b"ordf", "\u{00AA}"),    // FEMININE ORDINAL INDICATOR
    (b"ordm", "\u{00BA}"),    // MASCULINE ORDINAL INDICATOR
    (b"origof", "\u{22B6}"),  // ORIGINAL OF
    (b"oror", "\u{2A56}"),    // TWO INTERSECTING LOGICAL OR
    (b"orslope", "\u{2A57}"), // SLOPING LARGE OR
    (b"orv", "\u{2A5B}"),     // LOGICAL OR WITH MIDDLE STEM
    (b"oslash", "\u{00F8}"),  // LATIN SMALL LETTER O WITH STROKE
    (b"osol", "\u{2298}"),    // CIRCLED DIVISION SLASH
    (b"otilde", "\u{00F5}"),  // LATIN SMALL LETTER O WITH TILDE
    (b"otimes", "\u{2297}"),  // CIRCLED TIMES
    (b"otimesas", "\u{2A36}"), // CIRCLED MULTIPLICATION SIGN WITH CIRCUMFLEX ACCENT
    (b"ouml", "\u{00F6}"),    // LATIN SMALL LETTER O WITH DIAERESIS
    (b"ovbar", "\u{233D}"),   // APL FUNCTIONAL SYMBOL CIRCLE STILE
    (b"par", "\u{2225}"),     // PARALLEL TO
    (b"para", "\u{00B6}"),    // PILCROW SIGN
    (b"parsim", "\u{2AF3}"),  // PARALLEL WITH TILDE OPERATOR
    (b"parsl", "\u{2AFD}"),   // DOUBLE SOLIDUS OPERATOR
    (b"part", "\u{2202}"),    // PARTIAL DIFFERENTIAL
    (b"pcy", "\u{043F}"),     // CYRILLIC SMALL LETTER PE
    (b"percnt", "\u{0025}"),  // PERCENT SIGN
    (b"period", "\u{002E}"),  // FULL STOP
    (b"permil", "\u{2030}"),  // PER MILLE SIGN
    (b"pertenk", "\u{2031}"), // PER TEN THOUSAND SIGN
    (b"pfr", "\u{1D52D}"),    // MATHEMATICAL FRAKTUR SMALL P
    (b"phi", "\u{03C6}"),     // GREEK SMALL LETTER PHI
    (b"phmmat", "\u{2133}"),  // SCRIPT CAPITAL M
    (b"phone", "\u{260E}"),   // BLACK TELEPHONE
    (b"pi", "\u{03C0}"),      // GREEK SMALL LETTER PI
    (b"piv", "\u{03D6}"),     // GREEK PI SYMBOL
    (b"planck", "\u{210F}"),  // PLANCK CONSTANT OVER TWO PI
    (b"planckh", "\u{210E}"), // PLANCK CONSTANT
    (b"plus", "\u{002B}"),    // PLUS SIGN
    (b"plusacir", "\u{2A23}"), // PLUS SIGN WITH CIRCUMFLEX ACCENT ABOVE
    (b"plusb", "\u{229E}"),   // SQUARED PLUS
    (b"pluscir", "\u{2A22}"), // PLUS SIGN WITH SMALL CIRCLE ABOVE
    (b"plusdo", "\u{2214}"),  // DOT PLUS
    (b"plusdu", "\u{2A25}"),  // PLUS SIGN WITH DOT BELOW
    (b"pluse", "\u{2A72}"),   // PLUS SIGN ABOVE EQUALS SIGN
    (b"plusmn", "\u{00B1}"),  // PLUS-MINUS SIGN
    (b"plussim", "\u{2A26}"), // PLUS SIGN WITH TILDE BELOW
    (b"plustwo", "\u{2A27}"), // PLUS SIGN WITH SUBSCRIPT TWO
    (b"pointint", "\u{2A15}"), // INTEGRAL AROUND A POINT OPERATOR
    (b"popf", "\u{1D561}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL P
    (b"pound", "\u{00A3}"),   // POUND SIGN
    (b"pr", "\u{227A}"),      // PRECEDES
    (b"prE", "\u{2AB3}"),     // PRECEDES ABOVE EQUALS SIGN
    (b"prap", "\u{2AB7}"),    // PRECEDES ABOVE ALMOST EQUAL TO
    (b"prcue", "\u{227C}"),   // PRECEDES OR EQUAL TO
    (b"pre", "\u{2AAF}"),     // PRECEDES ABOVE SINGLE-LINE EQUALS SIGN
    (b"prime", "\u{2032}"),   // PRIME
    (b"prnE", "\u{2AB5}"),    // PRECEDES ABOVE NOT EQUAL TO
    (b"prnap", "\u{2AB9}"),   // PRECEDES ABOVE NOT ALMOST EQUAL TO
    (b"prnsim", "\u{22E8}"),  // PRECEDES BUT NOT EQUIVALENT TO
    (b"prod", "\u{220F}"),    // N-ARY PRODUCT
    (b"profalar", "\u{232E}"), // ALL AROUND-PROFILE
    (b"profline", "\u{2312}"), // ARC
    (b"profsurf", "\u{2313}"), // SEGMENT
    (b"prop", "\u{221D}"),    // PROPORTIONAL TO
    (b"prsim", "\u{227E}"),   // PRECEDES OR EQUIVALENT TO
    (b"prurel", "\u{22B0}"),  // PRECEDES UNDER RELATION
    (b"pscr", "\u{1D4C5}"),   // MATHEMATICAL SCRIPT SMALL P
    (b"psi", "\u{03C8}"),     // GREEK SMALL LETTER PSI
    (b"puncsp", "\u{2008}"),  // PUNCTUATION SPACE
    (b"qfr", "\u{1D52E}"),    // MATHEMATICAL FRAKTUR SMALL Q
    (b"qint", "\u{2A0C}"),    // QUADRUPLE INTEGRAL OPERATOR
    (b"qopf", "\u{1D562}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL Q
    (b"qprime", "\u{2057}"),  // QUADRUPLE PRIME
    (b"qscr", "\u{1D4C6}"),   // MATHEMATICAL SCRIPT SMALL Q
    (b"quaternions", "\u{210D}"), // DOUBLE-STRUCK CAPITAL H
    (b"quatint", "\u{2A16}"), // QUATERNION INTEGRAL OPERATOR
    (b"quest", "\u{003F}"),   // QUESTION MARK
    (b"quot", "\u{0022}"),    // QUOTATION MARK
    (b"rAarr", "\u{21DB}"),   // RIGHTWARDS TRIPLE ARROW
    (b"rArr", "\u{21D2}"),    // RIGHTWARDS DOUBLE ARROW
    (b"rAtail", "\u{291C}"),  // RIGHTWARDS DOUBLE ARROW-TAIL
    (b"rBarr", "\u{290F}"),   // RIGHTWARDS TRIPLE DASH ARROW
    (b"rHar", "\u{2964}"), /* RIGHTWARDS HARPOON WITH BARB UP ABOVE RIGHTWARDS HARPOON WITH BARB DOWN */
    (b"race", "\u{29DA}"), // LEFT DOUBLE WIGGLY FENCE
    (b"racute", "\u{0155}"), // LATIN SMALL LETTER R WITH ACUTE
    (b"radic", "\u{221A}"), // SQUARE ROOT
    (b"raemptyv", "\u{29B3}"), // EMPTY SET WITH RIGHT ARROW ABOVE
    (b"rang", "\u{27E9}"), // MATHEMATICAL RIGHT ANGLE BRACKET
    (b"rangd", "\u{2992}"), // RIGHT ANGLE BRACKET WITH DOT
    (b"range", "\u{29A5}"), // REVERSED ANGLE WITH UNDERBAR
    (b"raquo", "\u{00BB}"), // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    (b"rarr", "\u{2192}"), // RIGHTWARDS ARROW
    (b"rarrap", "\u{2975}"), // RIGHTWARDS ARROW ABOVE ALMOST EQUAL TO
    (b"rarrb", "\u{21E5}"), // RIGHTWARDS ARROW TO BAR
    (b"rarrbfs", "\u{2920}"), // RIGHTWARDS ARROW FROM BAR TO BLACK DIAMOND
    (b"rarrc", "\u{2933}"), // WAVE ARROW POINTING DIRECTLY RIGHT
    (b"rarrfs", "\u{291E}"), // RIGHTWARDS ARROW TO BLACK DIAMOND
    (b"rarrhk", "\u{21AA}"), // RIGHTWARDS ARROW WITH HOOK
    (b"rarrlp", "\u{21AC}"), // RIGHTWARDS ARROW WITH LOOP
    (b"rarrpl", "\u{2945}"), // RIGHTWARDS ARROW WITH PLUS BELOW
    (b"rarrsim", "\u{2974}"), // RIGHTWARDS ARROW ABOVE TILDE OPERATOR
    (b"rarrtl", "\u{21A3}"), // RIGHTWARDS ARROW WITH TAIL
    (b"rarrw", "\u{219D}"), // RIGHTWARDS WAVE ARROW
    (b"ratail", "\u{291A}"), // RIGHTWARDS ARROW-TAIL
    (b"ratio", "\u{2236}"), // RATIO
    (b"rationals", "\u{211A}"), // DOUBLE-STRUCK CAPITAL Q
    (b"rbarr", "\u{290D}"), // RIGHTWARDS DOUBLE DASH ARROW
    (b"rbbrk", "\u{2773}"), // LIGHT RIGHT TORTOISE SHELL BRACKET ORNAMENT
    (b"rbrke", "\u{298C}"), // RIGHT SQUARE BRACKET WITH UNDERBAR
    (b"rbrksld", "\u{298E}"), // RIGHT SQUARE BRACKET WITH TICK IN BOTTOM CORNER
    (b"rbrkslu", "\u{2990}"), // RIGHT SQUARE BRACKET WITH TICK IN TOP CORNER
    (b"rcaron", "\u{0159}"), // LATIN SMALL LETTER R WITH CARON
    (b"rcedil", "\u{0157}"), // LATIN SMALL LETTER R WITH CEDILLA
    (b"rceil", "\u{2309}"), // RIGHT CEILING
    (b"rcub", "\u{007D}"), // RIGHT CURLY BRACKET
    (b"rcy", "\u{0440}"),  // CYRILLIC SMALL LETTER ER
    (b"rdca", "\u{2937}"), // ARROW POINTING DOWNWARDS THEN CURVING RIGHTWARDS
    (b"rdldhar", "\u{2969}"), /* RIGHTWARDS HARPOON WITH BARB DOWN ABOVE LEFTWARDS HARPOON WITH BARB DOWN */
    (b"rdquo", "\u{201D}"),   // RIGHT DOUBLE QUOTATION MARK
    (b"rdsh", "\u{21B3}"),    // DOWNWARDS ARROW WITH TIP RIGHTWARDS
    (b"real", "\u{211C}"),    // BLACK-LETTER CAPITAL R
    (b"reals", "\u{211D}"),   // DOUBLE-STRUCK CAPITAL R
    (b"rect", "\u{25AD}"),    // WHITE RECTANGLE
    (b"reg", "\u{00AE}"),     // REGISTERED SIGN
    (b"rfisht", "\u{297D}"),  // RIGHT FISH TAIL
    (b"rfloor", "\u{230B}"),  // RIGHT FLOOR
    (b"rfr", "\u{1D52F}"),    // MATHEMATICAL FRAKTUR SMALL R
    (b"rhard", "\u{21C1}"),   // RIGHTWARDS HARPOON WITH BARB DOWNWARDS
    (b"rharu", "\u{21C0}"),   // RIGHTWARDS HARPOON WITH BARB UPWARDS
    (b"rharul", "\u{296C}"),  // RIGHTWARDS HARPOON WITH BARB UP ABOVE LONG DASH
    (b"rho", "\u{03C1}"),     // GREEK SMALL LETTER RHO
    (b"rhov", "\u{03F1}"),    // GREEK RHO SYMBOL
    (b"ring", "\u{02DA}"),    // RING ABOVE
    (b"rlarr", "\u{21C4}"),   // RIGHTWARDS ARROW OVER LEFTWARDS ARROW
    (b"rlhar", "\u{21CC}"),   // RIGHTWARDS HARPOON OVER LEFTWARDS HARPOON
    (b"rlm", "\u{200F}"),     // RIGHT-TO-LEFT MARK
    (b"rmoust", "\u{23B1}"),  // UPPER RIGHT OR LOWER LEFT CURLY BRACKET SECTION
    (b"rnmid", "\u{2AEE}"),   // DOES NOT DIVIDE WITH REVERSED NEGATION SLASH
    (b"roang", "\u{27ED}"),   // MATHEMATICAL RIGHT WHITE TORTOISE SHELL BRACKET
    (b"roarr", "\u{21FE}"),   // RIGHTWARDS OPEN-HEADED ARROW
    (b"robrk", "\u{27E7}"),   // MATHEMATICAL RIGHT WHITE SQUARE BRACKET
    (b"ropar", "\u{2986}"),   // RIGHT WHITE PARENTHESIS
    (b"ropf", "\u{1D563}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL R
    (b"roplus", "\u{2A2E}"),  // PLUS SIGN IN RIGHT HALF CIRCLE
    (b"rotimes", "\u{2A35}"), // MULTIPLICATION SIGN IN RIGHT HALF CIRCLE
    (b"rpar", "\u{0029}"),    // RIGHT PARENTHESIS
    (b"rpargt", "\u{2994}"),  // RIGHT ARC GREATER-THAN BRACKET
    (b"rppolint", "\u{2A12}"), // LINE INTEGRATION WITH RECTANGULAR PATH AROUND POLE
    (b"rrarr", "\u{21C9}"),   // RIGHTWARDS PAIRED ARROWS
    (b"rsaquo", "\u{203A}"),  // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
    (b"rscr", "\u{1D4C7}"),   // MATHEMATICAL SCRIPT SMALL R
    (b"rsh", "\u{21B1}"),     // UPWARDS ARROW WITH TIP RIGHTWARDS
    (b"rsqb", "\u{005D}"),    // RIGHT SQUARE BRACKET
    (b"rsquo", "\u{2019}"),   // RIGHT SINGLE QUOTATION MARK
    (b"rthree", "\u{22CC}"),  // RIGHT SEMIDIRECT PRODUCT
    (b"rtimes", "\u{22CA}"),  // RIGHT NORMAL FACTOR SEMIDIRECT PRODUCT
    (b"rtri", "\u{25B9}"),    // WHITE RIGHT-POINTING SMALL TRIANGLE
    (b"rtrie", "\u{22B5}"),   // CONTAINS AS NORMAL SUBGROUP OR EQUAL TO
    (b"rtrif", "\u{25B8}"),   // BLACK RIGHT-POINTING SMALL TRIANGLE
    (b"rtriltri", "\u{29CE}"), // RIGHT TRIANGLE ABOVE LEFT TRIANGLE
    (b"ruluhar", "\u{2968}"), /* RIGHTWARDS HARPOON WITH BARB UP ABOVE LEFTWARDS HARPOON WITH BARB UP */
    (b"rx", "\u{211E}"),      // PRESCRIPTION TAKE
    (b"sacute", "\u{015B}"),  // LATIN SMALL LETTER S WITH ACUTE
    (b"sc", "\u{227B}"),      // SUCCEEDS
    (b"scE", "\u{2AB4}"),     // SUCCEEDS ABOVE EQUALS SIGN
    (b"scap", "\u{2AB8}"),    // SUCCEEDS ABOVE ALMOST EQUAL TO
    (b"scaron", "\u{0161}"),  // LATIN SMALL LETTER S WITH CARON
    (b"sccue", "\u{227D}"),   // SUCCEEDS OR EQUAL TO
    (b"sce", "\u{2AB0}"),     // SUCCEEDS ABOVE SINGLE-LINE EQUALS SIGN
    (b"scedil", "\u{015F}"),  // LATIN SMALL LETTER S WITH CEDILLA
    (b"scirc", "\u{015D}"),   // LATIN SMALL LETTER S WITH CIRCUMFLEX
    (b"scnE", "\u{2AB6}"),    // SUCCEEDS ABOVE NOT EQUAL TO
    (b"scnap", "\u{2ABA}"),   // SUCCEEDS ABOVE NOT ALMOST EQUAL TO
    (b"scnsim", "\u{22E9}"),  // SUCCEEDS BUT NOT EQUIVALENT TO
    (b"scpolint", "\u{2A13}"), // LINE INTEGRATION WITH SEMICIRCULAR PATH AROUND POLE
    (b"scsim", "\u{227F}"),   // SUCCEEDS OR EQUIVALENT TO
    (b"scy", "\u{0441}"),     // CYRILLIC SMALL LETTER ES
    (b"sdot", "\u{22C5}"),    // DOT OPERATOR
    (b"sdotb", "\u{22A1}"),   // SQUARED DOT OPERATOR
    (b"sdote", "\u{2A66}"),   // EQUALS SIGN WITH DOT BELOW
    (b"seArr", "\u{21D8}"),   // SOUTH EAST DOUBLE ARROW
    (b"searhk", "\u{2925}"),  // SOUTH EAST ARROW WITH HOOK
    (b"searr", "\u{2198}"),   // SOUTH EAST ARROW
    (b"sect", "\u{00A7}"),    // SECTION SIGN
    (b"semi", "\u{003B}"),    // SEMICOLON
    (b"seswar", "\u{2929}"),  // SOUTH EAST ARROW AND SOUTH WEST ARROW
    (b"setmn", "\u{2216}"),   // SET MINUS
    (b"sext", "\u{2736}"),    // SIX POINTED BLACK STAR
    (b"sfr", "\u{1D530}"),    // MATHEMATICAL FRAKTUR SMALL S
    (b"sharp", "\u{266F}"),   // MUSIC SHARP SIGN
    (b"shchcy", "\u{0449}"),  // CYRILLIC SMALL LETTER SHCHA
    (b"shcy", "\u{0448}"),    // CYRILLIC SMALL LETTER SHA
    (b"shy", "\u{00AD}"),     // SOFT HYPHEN
    (b"sigma", "\u{03C3}"),   // GREEK SMALL LETTER SIGMA
    (b"sigmav", "\u{03C2}"),  // GREEK SMALL LETTER FINAL SIGMA
    (b"sim", "\u{223C}"),     // TILDE OPERATOR
    (b"simdot", "\u{2A6A}"),  // TILDE OPERATOR WITH DOT ABOVE
    (b"sime", "\u{2243}"),    // ASYMPTOTICALLY EQUAL TO
    (b"simg", "\u{2A9E}"),    // SIMILAR OR GREATER-THAN
    (b"simgE", "\u{2AA0}"),   // SIMILAR ABOVE GREATER-THAN ABOVE EQUALS SIGN
    (b"siml", "\u{2A9D}"),    // SIMILAR OR LESS-THAN
    (b"simlE", "\u{2A9F}"),   // SIMILAR ABOVE LESS-THAN ABOVE EQUALS SIGN
    (b"simne", "\u{2246}"),   // APPROXIMATELY BUT NOT ACTUALLY EQUAL TO
    (b"simplus", "\u{2A24}"), // PLUS SIGN WITH TILDE ABOVE
    (b"simrarr", "\u{2972}"), // TILDE OPERATOR ABOVE RIGHTWARDS ARROW
    (b"smashp", "\u{2A33}"),  // SMASH PRODUCT
    (b"smeparsl", "\u{29E4}"), // EQUALS SIGN AND SLANTED PARALLEL WITH TILDE ABOVE
    (b"smile", "\u{2323}"),   // SMILE
    (b"smt", "\u{2AAA}"),     // SMALLER THAN
    (b"smte", "\u{2AAC}"),    // SMALLER THAN OR EQUAL TO
    (b"softcy", "\u{044C}"),  // CYRILLIC SMALL LETTER SOFT SIGN
    (b"sol", "\u{002F}"),     // SOLIDUS
    (b"solb", "\u{29C4}"),    // SQUARED RISING DIAGONAL SLASH
    (b"solbar", "\u{233F}"),  // APL FUNCTIONAL SYMBOL SLASH BAR
    (b"sopf", "\u{1D564}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL S
    (b"spades", "\u{2660}"),  // BLACK SPADE SUIT
    (b"sqcap", "\u{2293}"),   // SQUARE CAP
    (b"sqcup", "\u{2294}"),   // SQUARE CUP
    (b"sqsub", "\u{228F}"),   // SQUARE IMAGE OF
    (b"sqsube", "\u{2291}"),  // SQUARE IMAGE OF OR EQUAL TO
    (b"sqsup", "\u{2290}"),   // SQUARE ORIGINAL OF
    (b"sqsupe", "\u{2292}"),  // SQUARE ORIGINAL OF OR EQUAL TO
    (b"squ", "\u{25A1}"),     // WHITE SQUARE
    (b"squf", "\u{25AA}"),    // BLACK SMALL SQUARE
    (b"sscr", "\u{1D4C8}"),   // MATHEMATICAL SCRIPT SMALL S
    (b"sstarf", "\u{22C6}"),  // STAR OPERATOR
    (b"star", "\u{2606}"),    // WHITE STAR
    (b"starf", "\u{2605}"),   // BLACK STAR
    (b"straightphi", "\u{03D5}"), // GREEK PHI SYMBOL
    (b"sub", "\u{2282}"),     // SUBSET OF
    (b"subE", "\u{2AC5}"),    // SUBSET OF ABOVE EQUALS SIGN
    (b"subdot", "\u{2ABD}"),  // SUBSET WITH DOT
    (b"sube", "\u{2286}"),    // SUBSET OF OR EQUAL TO
    (b"subedot", "\u{2AC3}"), // SUBSET OF OR EQUAL TO WITH DOT ABOVE
    (b"submult", "\u{2AC1}"), // SUBSET WITH MULTIPLICATION SIGN BELOW
    (b"subnE", "\u{2ACB}"),   // SUBSET OF ABOVE NOT EQUAL TO
    (b"subne", "\u{228A}"),   // SUBSET OF WITH NOT EQUAL TO
    (b"subplus", "\u{2ABF}"), // SUBSET WITH PLUS SIGN BELOW
    (b"subrarr", "\u{2979}"), // SUBSET ABOVE RIGHTWARDS ARROW
    (b"subsim", "\u{2AC7}"),  // SUBSET OF ABOVE TILDE OPERATOR
    (b"subsub", "\u{2AD5}"),  // SUBSET ABOVE SUBSET
    (b"subsup", "\u{2AD3}"),  // SUBSET ABOVE SUPERSET
    (b"sum", "\u{2211}"),     // N-ARY SUMMATION
    (b"sung", "\u{266A}"),    // EIGHTH NOTE
    (b"sup", "\u{2283}"),     // SUPERSET OF
    (b"sup1", "\u{00B9}"),    // SUPERSCRIPT ONE
    (b"sup2", "\u{00B2}"),    // SUPERSCRIPT TWO
    (b"sup3", "\u{00B3}"),    // SUPERSCRIPT THREE
    (b"supE", "\u{2AC6}"),    // SUPERSET OF ABOVE EQUALS SIGN
    (b"supdot", "\u{2ABE}"),  // SUPERSET WITH DOT
    (b"supdsub", "\u{2AD8}"), // SUPERSET BESIDE AND JOINED BY DASH WITH SUBSET
    (b"supe", "\u{2287}"),    // SUPERSET OF OR EQUAL TO
    (b"supedot", "\u{2AC4}"), // SUPERSET OF OR EQUAL TO WITH DOT ABOVE
    (b"suphsub", "\u{2AD7}"), // SUPERSET BESIDE SUBSET
    (b"suplarr", "\u{297B}"), // SUPERSET ABOVE LEFTWARDS ARROW
    (b"supmult", "\u{2AC2}"), // SUPERSET WITH MULTIPLICATION SIGN BELOW
    (b"supnE", "\u{2ACC}"),   // SUPERSET OF ABOVE NOT EQUAL TO
    (b"supne", "\u{228B}"),   // SUPERSET OF WITH NOT EQUAL TO
    (b"supplus", "\u{2AC0}"), // SUPERSET WITH PLUS SIGN BELOW
    (b"supsim", "\u{2AC8}"),  // SUPERSET OF ABOVE TILDE OPERATOR
    (b"supsub", "\u{2AD4}"),  // SUPERSET ABOVE SUBSET
    (b"supsup", "\u{2AD6}"),  // SUPERSET ABOVE SUPERSET
    (b"swArr", "\u{21D9}"),   // SOUTH WEST DOUBLE ARROW
    (b"swarhk", "\u{2926}"),  // SOUTH WEST ARROW WITH HOOK
    (b"swarr", "\u{2199}"),   // SOUTH WEST ARROW
    (b"swnwar", "\u{292A}"),  // SOUTH WEST ARROW AND NORTH WEST ARROW
    (b"szlig", "\u{00DF}"),   // LATIN SMALL LETTER SHARP S
    (b"target", "\u{2316}"),  // POSITION INDICATOR
    (b"tau", "\u{03C4}"),     // GREEK SMALL LETTER TAU
    (b"tbrk", "\u{23B4}"),    // TOP SQUARE BRACKET
    (b"tcaron", "\u{0165}"),  // LATIN SMALL LETTER T WITH CARON
    (b"tcedil", "\u{0163}"),  // LATIN SMALL LETTER T WITH CEDILLA
    (b"tcy", "\u{0442}"),     // CYRILLIC SMALL LETTER TE
    (b"tdot", "\u{20DB}"),    // COMBINING THREE DOTS ABOVE
    (b"telrec", "\u{2315}"),  // TELEPHONE RECORDER
    (b"tfr", "\u{1D531}"),    // MATHEMATICAL FRAKTUR SMALL T
    (b"there4", "\u{2234}"),  // THEREFORE
    (b"theta", "\u{03B8}"),   // GREEK SMALL LETTER THETA
    (b"thetav", "\u{03D1}"),  // GREEK THETA SYMBOL
    (b"thinsp", "\u{2009}"),  // THIN SPACE
    (b"thorn", "\u{00FE}"),   // LATIN SMALL LETTER THORN
    (b"tilde", "\u{02DC}"),   // SMALL TILDE
    (b"times", "\u{00D7}"),   // MULTIPLICATION SIGN
    (b"timesb", "\u{22A0}"),  // SQUARED TIMES
    (b"timesbar", "\u{2A31}"), // MULTIPLICATION SIGN WITH UNDERBAR
    (b"timesd", "\u{2A30}"),  // MULTIPLICATION SIGN WITH DOT ABOVE
    (b"tint", "\u{222D}"),    // TRIPLE INTEGRAL
    (b"top", "\u{22A4}"),     // DOWN TACK
    (b"topbot", "\u{2336}"),  // APL FUNCTIONAL SYMBOL I-BEAM
    (b"topcir", "\u{2AF1}"),  // DOWN TACK WITH CIRCLE BELOW
    (b"topf", "\u{1D565}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL T
    (b"topfork", "\u{2ADA}"), // PITCHFORK WITH TEE TOP
    (b"tprime", "\u{2034}"),  // TRIPLE PRIME
    (b"trade", "\u{2122}"),   // TRADE MARK SIGN
    (b"tridot", "\u{25EC}"),  // WHITE UP-POINTING TRIANGLE WITH DOT
    (b"trie", "\u{225C}"),    // DELTA EQUAL TO
    (b"triminus", "\u{2A3A}"), // MINUS SIGN IN TRIANGLE
    (b"triplus", "\u{2A39}"), // PLUS SIGN IN TRIANGLE
    (b"trisb", "\u{29CD}"),   // TRIANGLE WITH SERIFS AT BOTTOM
    (b"tritime", "\u{2A3B}"), // MULTIPLICATION SIGN IN TRIANGLE
    (b"trpezium", "\u{23E2}"), // WHITE TRAPEZIUM
    (b"tscr", "\u{1D4C9}"),   // MATHEMATICAL SCRIPT SMALL T
    (b"tscy", "\u{0446}"),    // CYRILLIC SMALL LETTER TSE
    (b"tshcy", "\u{045B}"),   // CYRILLIC SMALL LETTER TSHE
    (b"tstrok", "\u{0167}"),  // LATIN SMALL LETTER T WITH STROKE
    (b"twixt", "\u{226C}"),   // BETWEEN
    (b"uArr", "\u{21D1}"),    // UPWARDS DOUBLE ARROW
    (b"uHar", "\u{2963}"), // UPWARDS HARPOON WITH BARB LEFT BESIDE UPWARDS HARPOON WITH BARB RIGHT
    (b"uacute", "\u{00FA}"), // LATIN SMALL LETTER U WITH ACUTE
    (b"uarr", "\u{2191}"), // UPWARDS ARROW
    (b"ubrcy", "\u{045E}"), // CYRILLIC SMALL LETTER SHORT U
    (b"ubreve", "\u{016D}"), // LATIN SMALL LETTER U WITH BREVE
    (b"ucirc", "\u{00FB}"), // LATIN SMALL LETTER U WITH CIRCUMFLEX
    (b"ucy", "\u{0443}"),  // CYRILLIC SMALL LETTER U
    (b"udarr", "\u{21C5}"), // UPWARDS ARROW LEFTWARDS OF DOWNWARDS ARROW
    (b"udblac", "\u{0171}"), // LATIN SMALL LETTER U WITH DOUBLE ACUTE
    (b"udhar", "\u{296E}"), /* UPWARDS HARPOON WITH BARB LEFT BESIDE DOWNWARDS HARPOON WITH BARB RIGHT */
    (b"ufisht", "\u{297E}"), // UP FISH TAIL
    (b"ufr", "\u{1D532}"),  // MATHEMATICAL FRAKTUR SMALL U
    (b"ugrave", "\u{00F9}"), // LATIN SMALL LETTER U WITH GRAVE
    (b"uharl", "\u{21BF}"), // UPWARDS HARPOON WITH BARB LEFTWARDS
    (b"uharr", "\u{21BE}"), // UPWARDS HARPOON WITH BARB RIGHTWARDS
    (b"uhblk", "\u{2580}"), // UPPER HALF BLOCK
    (b"ulcorn", "\u{231C}"), // TOP LEFT CORNER
    (b"ulcrop", "\u{230F}"), // TOP LEFT CROP
    (b"ultri", "\u{25F8}"), // UPPER LEFT TRIANGLE
    (b"umacr", "\u{016B}"), // LATIN SMALL LETTER U WITH MACRON
    (b"uogon", "\u{0173}"), // LATIN SMALL LETTER U WITH OGONEK
    (b"uopf", "\u{1D566}"), // MATHEMATICAL DOUBLE-STRUCK SMALL U
    (b"uplus", "\u{228E}"), // MULTISET UNION
    (b"upsi", "\u{03C5}"),  // GREEK SMALL LETTER UPSILON
    (b"urcorn", "\u{231D}"), // TOP RIGHT CORNER
    (b"urcrop", "\u{230E}"), // TOP RIGHT CROP
    (b"uring", "\u{016F}"), // LATIN SMALL LETTER U WITH RING ABOVE
    (b"urtri", "\u{25F9}"), // UPPER RIGHT TRIANGLE
    (b"uscr", "\u{1D4CA}"), // MATHEMATICAL SCRIPT SMALL U
    (b"utdot", "\u{22F0}"), // UP RIGHT DIAGONAL ELLIPSIS
    (b"utilde", "\u{0169}"), // LATIN SMALL LETTER U WITH TILDE
    (b"utri", "\u{25B5}"),  // WHITE UP-POINTING SMALL TRIANGLE
    (b"utrif", "\u{25B4}"), // BLACK UP-POINTING SMALL TRIANGLE
    (b"uuarr", "\u{21C8}"), // UPWARDS PAIRED ARROWS
    (b"uuml", "\u{00FC}"),  // LATIN SMALL LETTER U WITH DIAERESIS
    (b"uwangle", "\u{29A7}"), // OBLIQUE ANGLE OPENING DOWN
    (b"vArr", "\u{21D5}"),  // UP DOWN DOUBLE ARROW
    (b"vBar", "\u{2AE8}"),  // SHORT UP TACK WITH UNDERBAR
    (b"vBarv", "\u{2AE9}"), // SHORT UP TACK ABOVE SHORT DOWN TACK
    (b"vDash", "\u{22A8}"), // TRUE
    (b"vangrt", "\u{299C}"), // RIGHT ANGLE VARIANT WITH SQUARE
    (b"varr", "\u{2195}"),  // UP DOWN ARROW
    (b"vcy", "\u{0432}"),   // CYRILLIC SMALL LETTER VE
    (b"vdash", "\u{22A2}"), // RIGHT TACK
    (b"veebar", "\u{22BB}"), // XOR
    (b"veeeq", "\u{225A}"), // EQUIANGULAR TO
    (b"vellip", "\u{22EE}"), // VERTICAL ELLIPSIS
    (b"verbar", "\u{007C}"), // VERTICAL LINE
    (b"vfr", "\u{1D533}"),  // MATHEMATICAL FRAKTUR SMALL V
    (b"vltri", "\u{22B2}"), // NORMAL SUBGROUP OF
    (b"vopf", "\u{1D567}"), // MATHEMATICAL DOUBLE-STRUCK SMALL V
    (b"vrtri", "\u{22B3}"), // CONTAINS AS NORMAL SUBGROUP
    (b"vscr", "\u{1D4CB}"), // MATHEMATICAL SCRIPT SMALL V
    (b"vzigzag", "\u{299A}"), // VERTICAL ZIGZAG LINE
    (b"wcirc", "\u{0175}"), // LATIN SMALL LETTER W WITH CIRCUMFLEX
    (b"wedbar", "\u{2A5F}"), // LOGICAL AND WITH UNDERBAR
    (b"wedgeq", "\u{2259}"), // ESTIMATES
    (b"weierp", "\u{2118}"), // SCRIPT CAPITAL P
    (b"wfr", "\u{1D534}"),  // MATHEMATICAL FRAKTUR SMALL W
    (b"wopf", "\u{1D568}"), // MATHEMATICAL DOUBLE-STRUCK SMALL W
    (b"wreath", "\u{2240}"), // WREATH PRODUCT
    (b"wscr", "\u{1D4CC}"), // MATHEMATICAL SCRIPT SMALL W
    (b"xcap", "\u{22C2}"),  // N-ARY INTERSECTION
    (b"xcirc", "\u{25EF}"), // LARGE CIRCLE
    (b"xcup", "\u{22C3}"),  // N-ARY UNION
    (b"xdtri", "\u{25BD}"), // WHITE DOWN-POINTING TRIANGLE
    (b"xfr", "\u{1D535}"),  // MATHEMATICAL FRAKTUR SMALL X
    (b"xhArr", "\u{27FA}"), // LONG LEFT RIGHT DOUBLE ARROW
    (b"xharr", "\u{27F7}"), // LONG LEFT RIGHT ARROW
    (b"xi", "\u{03BE}"),    // GREEK SMALL LETTER XI
    (b"xlArr", "\u{27F8}"), // LONG LEFTWARDS DOUBLE ARROW
    (b"xlarr", "\u{27F5}"), // LONG LEFTWARDS ARROW
    (b"xmap", "\u{27FC}"),  // LONG RIGHTWARDS ARROW FROM BAR
    (b"xnis", "\u{22FB}"),  // CONTAINS WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    (b"xodot", "\u{2A00}"), // N-ARY CIRCLED DOT OPERATOR
    (b"xopf", "\u{1D569}"), // MATHEMATICAL DOUBLE-STRUCK SMALL X
    (b"xoplus", "\u{2A01}"), // N-ARY CIRCLED PLUS OPERATOR
    (b"xotime", "\u{2A02}"), // N-ARY CIRCLED TIMES OPERATOR
    (b"xrArr", "\u{27F9}"), // LONG RIGHTWARDS DOUBLE ARROW
    (b"xrarr", "\u{27F6}"), // LONG RIGHTWARDS ARROW
    (b"xscr", "\u{1D4CD}"), // MATHEMATICAL SCRIPT SMALL X
    (b"xsqcup", "\u{2A06}"), // N-ARY SQUARE UNION OPERATOR
    (b"xuplus", "\u{2A04}"), // N-ARY UNION OPERATOR WITH PLUS
    (b"xutri", "\u{25B3}"), // WHITE UP-POINTING TRIANGLE
    (b"xvee", "\u{22C1}"),  // N-ARY LOGICAL OR
    (b"xwedge", "\u{22C0}"), // N-ARY LOGICAL AND
    (b"yacute", "\u{00FD}"), // LATIN SMALL LETTER Y WITH ACUTE
    (b"yacy", "\u{044F}"),  // CYRILLIC SMALL LETTER YA
    (b"ycirc", "\u{0177}"), // LATIN SMALL LETTER Y WITH CIRCUMFLEX
    (b"ycy", "\u{044B}"),   // CYRILLIC SMALL LETTER YERU
    (b"yen", "\u{00A5}"),   // YEN SIGN
    (b"yfr", "\u{1D536}"),  // MATHEMATICAL FRAKTUR SMALL Y
    (b"yicy", "\u{0457}"),  // CYRILLIC SMALL LETTER YI
    (b"yopf", "\u{1D56A}"), // MATHEMATICAL DOUBLE-STRUCK SMALL Y
    (b"yscr", "\u{1D4CE}"), // MATHEMATICAL SCRIPT SMALL Y
    (b"yucy", "\u{044E}"),  // CYRILLIC SMALL LETTER YU
    (b"yuml", "\u{00FF}"),  // LATIN SMALL LETTER Y WITH DIAERESIS
    (b"zacute", "\u{017A}"), // LATIN SMALL LETTER Z WITH ACUTE
    (b"zcaron", "\u{017E}"), // LATIN SMALL LETTER Z WITH CARON
    (b"zcy", "\u{0437}"),   // CYRILLIC SMALL LETTER ZE
    (b"zdot", "\u{017C}"),  // LATIN SMALL LETTER Z WITH DOT ABOVE
    (b"zeta", "\u{03B6}"),  // GREEK SMALL LETTER ZETA
    (b"zfr", "\u{1D537}"),  // MATHEMATICAL FRAKTUR SMALL Z
    (b"zhcy", "\u{0436}"),  // CYRILLIC SMALL LETTER ZHE
    (b"zigrarr", "\u{21DD}"), // RIGHTWARDS SQUIGGLE ARROW
    (b"zopf", "\u{1D56B}"), // MATHEMATICAL DOUBLE-STRUCK SMALL Z
    (b"zscr", "\u{1D4CF}"), // MATHEMATICAL SCRIPT SMALL Z
    (b"zwj", "\u{200D}"),   // ZERO WIDTH JOINER
    (b"zwnj", "\u{200C}"),  // ZERO WIDTH NON-JOINER
];
