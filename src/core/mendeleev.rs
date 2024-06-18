// Code stolen from : https://github.com/TheRealSalmon/pertable/blob/main/src/lib.rs


use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]

pub enum Error {
    InvalidAtomicNumber(u8),
    InvalidAtomicSymbol(String),
    InvalidIsotope(String, u16),
    InvalidFormalCharge(String, i8),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Element {
    Unknown,
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,

    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
}

impl Element {
    pub fn symbol(&self) -> &'static str {
        match self {
            Element::Unknown => "*",
            Element::H => "H",
            Element::He => "He",
            Element::Li => "Li",
            Element::Be => "Be",
            Element::B => "B",
            Element::C => "C",
            Element::N => "N",
            Element::O => "O",
            Element::F => "F",
            Element::Ne => "Ne",
            Element::Na => "Na",
            Element::Mg => "Mg",
            Element::Al => "Al",
            Element::Si => "Si",
            Element::P => "P",
            Element::S => "S",
            Element::Cl => "Cl",
            Element::Ar => "Ar",
            Element::K => "K",
            Element::Ca => "Ca",
            Element::Sc => "Sc",
            Element::Ti => "Ti",
            Element::V => "V",
            Element::Cr => "Cr",
            Element::Mn => "Mn",
            Element::Fe => "Fe",
            Element::Co => "Co",
            Element::Ni => "Ni",
            Element::Cu => "Cu",
            Element::Zn => "Zn",
            Element::Ga => "Ga",
            Element::Ge => "Ge",
            Element::As => "As",
            Element::Se => "Se",
            Element::Br => "Br",
            Element::Kr => "Kr",
            Element::Rb => "Rb",
            Element::Sr => "Sr",
            Element::Y => "Y",
            Element::Zr => "Zr",
            Element::Nb => "Nb",
            Element::Mo => "Mo",
            Element::Tc => "Tc",
            Element::Ru => "Ru",
            Element::Rh => "Rh",
            Element::Pd => "Pd",
            Element::Ag => "Ag",
            Element::Cd => "Cd",
            Element::In => "In",
            Element::Sn => "Sn",
            Element::Sb => "Sb",
            Element::Te => "Te",
            Element::I => "I",
            Element::Xe => "Xe",
            Element::Cs => "Cs",
            Element::Ba => "Ba",
            Element::La => "La",
            Element::Ce => "Ce",
            Element::Pr => "Pr",
            Element::Nd => "Nd",
            Element::Pm => "Pm",
            Element::Sm => "Sm",
            Element::Eu => "Eu",
            Element::Gd => "Gd",
            Element::Tb => "Tb",
            Element::Dy => "Dy",
            Element::Ho => "Ho",
            Element::Er => "Er",
            Element::Tm => "Tm",
            Element::Yb => "Yb",
            Element::Lu => "Lu",
            Element::Hf => "Hf",
            Element::Ta => "Ta",
            Element::W => "W",
            Element::Re => "Re",
            Element::Os => "Os",
            Element::Ir => "Ir",
            Element::Pt => "Pt",
            Element::Au => "Au",
            Element::Hg => "Hg",
            Element::Tl => "Tl",
            Element::Pb => "Pb",
            Element::Bi => "Bi",
            Element::Po => "Po",
            Element::At => "At",
            Element::Rn => "Rn",
            Element::Fr => "Fr",
            Element::Ra => "Ra",
            Element::Ac => "Ac",
            Element::Th => "Th",
            Element::Pa => "Pa",
            Element::U => "U",
            Element::Np => "Np",
            Element::Pu => "Pu",
            Element::Am => "Am",
            Element::Cm => "Cm",
            Element::Bk => "Bk",
            Element::Cf => "Cf",
            Element::Es => "Es",
            Element::Fm => "Fm",
            Element::Md => "Md",
            Element::No => "No",
            Element::Lr => "Lr",
            Element::Rf => "Rf",
            Element::Db => "Db",
            Element::Sg => "Sg",
            Element::Bh => "Bh",
            Element::Hs => "Hs",
            Element::Mt => "Mt",
            Element::Ds => "Ds",
            Element::Rg => "Rg",
            Element::Cn => "Cn",
            Element::Nh => "Nh",
            Element::Fl => "Fl",
            Element::Mc => "Mc",
            Element::Lv => "Lv",
            Element::Ts => "Ts",
            Element::Og => "Og",
        }
    }

    pub fn atomic_symbol(&self) -> String {
        self.to_string()
    }

    pub fn atomic_number(&self) -> i32 {
        match self {
            Element::Unknown => 0,
            Element::H => 1,
            Element::He => 2,
            Element::Li => 3,
            Element::Be => 4,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Ne => 10,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Al => 13,
            Element::Si => 14,
            Element::P => 15,
            Element::S => 16,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::K => 19,
            Element::Ca => 20,
            Element::Sc => 21,
            Element::Ti => 22,
            Element::V => 23,
            Element::Cr => 24,
            Element::Mn => 25,
            Element::Fe => 26,
            Element::Co => 27,
            Element::Ni => 28,
            Element::Cu => 29,
            Element::Zn => 30,
            Element::Ga => 31,
            Element::Ge => 32,
            Element::As => 33,
            Element::Se => 34,
            Element::Br => 35,
            Element::Kr => 36,
            Element::Rb => 37,
            Element::Sr => 38,
            Element::Y => 39,
            Element::Zr => 40,
            Element::Nb => 41,
            Element::Mo => 42,
            Element::Tc => 43,
            Element::Ru => 44,
            Element::Rh => 45,
            Element::Pd => 46,
            Element::Ag => 47,
            Element::Cd => 48,
            Element::In => 49,
            Element::Sn => 50,
            Element::Sb => 51,
            Element::Te => 52,
            Element::I => 53,
            Element::Xe => 54,
            Element::Cs => 55,
            Element::Ba => 56,
            Element::La => 57,
            Element::Ce => 58,
            Element::Pr => 59,
            Element::Nd => 60,
            Element::Pm => 61,
            Element::Sm => 62,
            Element::Eu => 63,
            Element::Gd => 64,
            Element::Tb => 65,
            Element::Dy => 66,
            Element::Ho => 67,
            Element::Er => 68,
            Element::Tm => 69,
            Element::Yb => 70,
            Element::Lu => 71,
            Element::Hf => 72,
            Element::Ta => 73,
            Element::W => 74,
            Element::Re => 75,
            Element::Os => 76,
            Element::Ir => 77,
            Element::Pt => 78,
            Element::Au => 79,
            Element::Hg => 80,
            Element::Tl => 81,
            Element::Pb => 82,
            Element::Bi => 83,
            Element::Po => 84,
            Element::At => 85,
            Element::Rn => 86,
            Element::Fr => 87,
            Element::Ra => 88,
            Element::Ac => 89,
            Element::Th => 90,
            Element::Pa => 91,
            Element::U => 92,
            Element::Np => 93,
            Element::Pu => 94,
            Element::Am => 95,
            Element::Cm => 96,
            Element::Bk => 97,
            Element::Cf => 98,
            Element::Es => 99,
            Element::Fm => 100,
            Element::Md => 101,
            Element::No => 102,
            Element::Lr => 103,
            Element::Rf => 104,
            Element::Db => 105,
            Element::Sg => 106,
            Element::Bh => 107,
            Element::Hs => 108,
            Element::Mt => 109,
            Element::Ds => 110,
            Element::Rg => 111,
            Element::Cn => 112,
            Element::Nh => 113,
            Element::Fl => 114,
            Element::Mc => 115,
            Element::Lv => 116,
            Element::Ts => 117,
            Element::Og => 118,
        }
    }

    pub fn atomic_weight(&self, isotope: Option<u16>) -> Result<f64, Error> {
        match self {
            Element::Unknown => Ok(0.0),
            Element::H => match isotope {
                None => Ok(1.007_975),
                Some(isotope) => match isotope {
                    1 => Ok(1.007_825),
                    2 => Ok(2.014_102),
                    3 => Ok(3.016_049),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::He => match isotope {
                None => Ok(4.002_602),
                Some(isotope) => match isotope {
                    3 => Ok(3.016_029),
                    4 => Ok(4.002_603),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Li => match isotope {
                None => Ok(6.967_5),
                Some(isotope) => match isotope {
                    6 => Ok(6.015_123),
                    7 => Ok(7.016_003),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Be => match isotope {
                None => Ok(9.012_183),
                Some(isotope) => match isotope {
                    9 => Ok(9.012_183),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::B => match isotope {
                None => Ok(10.813_5),
                Some(isotope) => match isotope {
                    10 => Ok(10.012_937),
                    11 => Ok(11.009_305),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::C => match isotope {
                None => Ok(12.010_6),
                Some(isotope) => match isotope {
                    12 => Ok(12.000_000),
                    13 => Ok(13.003_355),
                    14 => Ok(14.003_242),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::N => match isotope {
                None => Ok(14.006_855),
                Some(isotope) => match isotope {
                    14 => Ok(14.003_074),
                    15 => Ok(15.000_109),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::O => match isotope {
                None => Ok(15.9994),
                Some(isotope) => match isotope {
                    16 => Ok(15.994_915),
                    17 => Ok(16.999_132),
                    18 => Ok(17.999_160),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::F => match isotope {
                None => Ok(18.998_403),
                Some(isotope) => match isotope {
                    19 => Ok(18.998_403),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ne => match isotope {
                None => Ok(20.179_7),
                Some(isotope) => match isotope {
                    20 => Ok(19.997_440),
                    21 => Ok(20.993_847),
                    22 => Ok(21.991_385),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Na => match isotope {
                None => Ok(22.989_769),
                Some(isotope) => match isotope {
                    23 => Ok(22.989_769),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Mg => match isotope {
                None => Ok(24.305_5),
                Some(isotope) => match isotope {
                    24 => Ok(23.985_042),
                    25 => Ok(24.985_837),
                    26 => Ok(25.982_593),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Al => match isotope {
                None => Ok(26.981_539),
                Some(isotope) => match isotope {
                    27 => Ok(26.981_538),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Si => match isotope {
                None => Ok(28.085),
                Some(isotope) => match isotope {
                    28 => Ok(27.976_927),
                    29 => Ok(28.976_495),
                    30 => Ok(29.973_770),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::P => match isotope {
                None => Ok(30.973_762),
                Some(isotope) => match isotope {
                    31 => Ok(30.973_762),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::S => match isotope {
                None => Ok(32.0675),
                Some(isotope) => match isotope {
                    32 => Ok(31.972_071),
                    33 => Ok(32.971_459),
                    34 => Ok(33.967_867),
                    36 => Ok(35.967_081),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cl => match isotope {
                None => Ok(35.4515),
                Some(isotope) => match isotope {
                    35 => Ok(34.968_853),
                    37 => Ok(36.965_093),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ar => match isotope {
                None => Ok(39.948_1),
                Some(isotope) => match isotope {
                    36 => Ok(35.967_545),
                    38 => Ok(37.962_732),
                    40 => Ok(39.962_383),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::K => match isotope {
                None => Ok(39.098_3),
                Some(isotope) => match isotope {
                    39 => Ok(38.963_707),
                    40 => Ok(39.963_999),
                    41 => Ok(40.961_826),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ca => match isotope {
                None => Ok(40.078),
                Some(isotope) => match isotope {
                    40 => Ok(39.962_591),
                    42 => Ok(41.958_618),
                    43 => Ok(42.958_767),
                    44 => Ok(43.955_481),
                    46 => Ok(45.953_693),
                    48 => Ok(47.952_534),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Sc => match isotope {
                None => Ok(44.955_908),
                Some(isotope) => match isotope {
                    45 => Ok(44.955_910),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ti => match isotope {
                None => Ok(47.867),
                Some(isotope) => match isotope {
                    46 => Ok(45.952_629),
                    47 => Ok(46.951_764),
                    48 => Ok(47.947_947),
                    49 => Ok(48.947_871),
                    50 => Ok(49.944_792),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::V => match isotope {
                None => Ok(50.941_5),
                Some(isotope) => match isotope {
                    50 => Ok(49.947_163),
                    51 => Ok(50.943_964),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cr => match isotope {
                None => Ok(51.996_1),
                Some(isotope) => match isotope {
                    50 => Ok(49.946_050),
                    52 => Ok(51.940_512),
                    53 => Ok(52.940_654),
                    54 => Ok(53.938_885),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Mn => match isotope {
                None => Ok(54.938_045),
                Some(isotope) => match isotope {
                    55 => Ok(54.938_050),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Fe => match isotope {
                None => Ok(55.845),
                Some(isotope) => match isotope {
                    54 => Ok(53.939_615),
                    56 => Ok(55.934_942),
                    57 => Ok(56.935_399),
                    58 => Ok(57.933_280),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Co => match isotope {
                None => Ok(58.933_200),
                Some(isotope) => match isotope {
                    59 => Ok(58.933_200),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ni => match isotope {
                None => Ok(57.935_348),
                Some(isotope) => match isotope {
                    58 => Ok(57.935_348),
                    60 => Ok(59.930_791),
                    61 => Ok(60.931_060),
                    62 => Ok(61.928_349),
                    64 => Ok(63.927_970),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cu => match isotope {
                None => Ok(63.929_601),
                Some(isotope) => match isotope {
                    63 => Ok(62.929_601),
                    65 => Ok(64.927_793),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Zn => match isotope {
                None => Ok(65.929_147),
                Some(isotope) => match isotope {
                    64 => Ok(63.929_147),
                    66 => Ok(65.926_037),
                    67 => Ok(66.927_131),
                    68 => Ok(67.924_848),
                    70 => Ok(69.925_319),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ga => match isotope {
                None => Ok(69.723),
                Some(isotope) => match isotope {
                    69 => Ok(68.925_581),
                    71 => Ok(70.924_705),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ge => match isotope {
                None => Ok(69.924_250),
                Some(isotope) => match isotope {
                    70 => Ok(69.924_250),
                    72 => Ok(71.922_076),
                    73 => Ok(72.923_459),
                    74 => Ok(73.921_178),
                    76 => Ok(75.921_403),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::As => match isotope {
                None => Ok(74.921_596),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Se => match isotope {
                None => Ok(73.922_477),
                Some(isotope) => match isotope {
                    74 => Ok(73.922_477),
                    76 => Ok(75.919_214),
                    77 => Ok(76.919_915),
                    78 => Ok(77.917_310),
                    80 => Ok(79.916_522),
                    82 => Ok(81.916_700),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Br => match isotope {
                None => Ok(78.918_337),
                Some(isotope) => match isotope {
                    79 => Ok(78.918_337),
                    81 => Ok(80.916_291),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Kr => match isotope {
                None => Ok(83.911_507),
                Some(isotope) => match isotope {
                    78 => Ok(77.920_386),
                    80 => Ok(79.916_738),
                    82 => Ok(81.913_485),
                    83 => Ok(82.914_136),
                    84 => Ok(83.911_507),
                    86 => Ok(85.910_610),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Rb => match isotope {
                None => Ok(85.4678),
                Some(isotope) => match isotope {
                    85 => Ok(84.911_789),
                    87 => Ok(86.909_183),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Sr => match isotope {
                None => Ok(87.62),
                Some(isotope) => match isotope {
                    84 => Ok(83.913_430),
                    86 => Ok(85.909_267),
                    87 => Ok(86.908_884),
                    88 => Ok(87.905_614),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Y => match isotope {
                None => Ok(88.905_848),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Zr => match isotope {
                None => Ok(91.224),
                Some(isotope) => match isotope {
                    90 => Ok(89.904704),
                    91 => Ok(90.905_645),
                    92 => Ok(91.905_040),
                    94 => Ok(93.906_316),
                    96 => Ok(95.908_276),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Nb => match isotope {
                None => Ok(92.906_378),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Mo => match isotope {
                None => Ok(95.95),
                Some(isotope) => match isotope {
                    92 => Ok(91.906810),
                    94 => Ok(93.905088),
                    95 => Ok(94.905841),
                    96 => Ok(95.904679),
                    97 => Ok(96.906021),
                    98 => Ok(97.905408),
                    100 => Ok(99.907477),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Tc => match isotope {
                None => Ok(97.907_216),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ru => match isotope {
                None => Ok(101.07),
                Some(isotope) => match isotope {
                    96 => Ok(95.907598),
                    98 => Ok(97.905287),
                    99 => Ok(98.905939),
                    100 => Ok(99.904220),
                    101 => Ok(100.905582),
                    102 => Ok(101.904350),
                    104 => Ok(103.905430),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Rh => match isotope {
                None => Ok(102.905_504),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Pd => match isotope {
                None => Ok(106.42),
                Some(isotope) => match isotope {
                    102 => Ok(101.905608),
                    104 => Ok(103.904035),
                    105 => Ok(104.905084),
                    106 => Ok(105.903483),
                    108 => Ok(107.903894),
                    110 => Ok(109.905152),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ag => match isotope {
                None => Ok(107.904756),
                Some(isotope) => match isotope {
                    107 => Ok(106.905093),
                    109 => Ok(108.904756),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cd => match isotope {
                None => Ok(112.414),
                Some(isotope) => match isotope {
                    106 => Ok(105.906458),
                    108 => Ok(107.904183),
                    110 => Ok(109.903006),
                    111 => Ok(110.904182),
                    112 => Ok(111.902757),
                    113 => Ok(112.904401),
                    114 => Ok(113.903358),
                    116 => Ok(115.904755),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::In => match isotope {
                None => Ok(114.818),
                Some(isotope) => match isotope {
                    113 => Ok(112.904061),
                    115 => Ok(114.903878),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Sn => match isotope {
                None => Ok(118.710),
                Some(isotope) => match isotope {
                    112 => Ok(111.904821),
                    114 => Ok(113.902782),
                    115 => Ok(114.903346),
                    116 => Ok(115.901744),
                    117 => Ok(116.902954),
                    118 => Ok(117.901606),
                    119 => Ok(118.903309),
                    120 => Ok(119.902197),
                    122 => Ok(121.903440),
                    124 => Ok(123.905275),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Sb => match isotope {
                None => Ok(121.760),
                Some(isotope) => match isotope {
                    121 => Ok(120.903818),
                    123 => Ok(122.904216),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Te => match isotope {
                None => Ok(127.60),
                Some(isotope) => match isotope {
                    120 => Ok(119.904020),
                    122 => Ok(121.903047),
                    123 => Ok(122.904273),
                    124 => Ok(123.902819),
                    125 => Ok(124.904425),
                    126 => Ok(125.903306),
                    128 => Ok(127.904461),
                    130 => Ok(129.906223),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::I => match isotope {
                None => Ok(126.904468),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Xe => match isotope {
                None => Ok(131.293),
                Some(isotope) => match isotope {
                    123 => Ok(123.905896),
                    126 => Ok(125.904269),
                    128 => Ok(127.903530),
                    129 => Ok(128.904779),
                    130 => Ok(129.903508),
                    131 => Ok(130.905082),
                    132 => Ok(131.904154),
                    134 => Ok(133.905395),
                    136 => Ok(135.907220),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cs => match isotope {
                None => Ok(132.905447),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ba => match isotope {
                None => Ok(137.327),
                Some(isotope) => match isotope {
                    130 => Ok(129.906310),
                    132 => Ok(131.905056),
                    134 => Ok(133.905056),
                    135 => Ok(134.905683),
                    136 => Ok(135.904570),
                    137 => Ok(136.905821),
                    138 => Ok(137.905241),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::La => match isotope {
                None => Ok(138.905_47),
                Some(isotope) => match isotope {
                    138 => Ok(137.907_144),
                    139 => Ok(138.906_348),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ce => match isotope {
                None => Ok(140.116),
                Some(isotope) => match isotope {
                    136 => Ok(135.907144),
                    138 => Ok(137.905986),
                    140 => Ok(139.905434),
                    142 => Ok(141.909240),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Pr => match isotope {
                None => Ok(140.907_648),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Nd => match isotope {
                None => Ok(144.24),
                Some(isotope) => match isotope {
                    142 => Ok(141.907719),
                    143 => Ok(142.909810),
                    144 => Ok(143.910083),
                    145 => Ok(144.912569),
                    146 => Ok(145.913112),
                    148 => Ok(147.916889),
                    150 => Ok(149.920887),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Pm => match isotope {
                None => Ok(144.912744),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Sm => match isotope {
                None => Ok(150.36),
                Some(isotope) => match isotope {
                    144 => Ok(143.911998),
                    147 => Ok(146.914893),
                    148 => Ok(147.914818),
                    149 => Ok(148.917180),
                    150 => Ok(149.917271),
                    152 => Ok(151.919728),
                    154 => Ok(153.922205),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Eu => match isotope {
                None => Ok(151.964),
                Some(isotope) => match isotope {
                    151 => Ok(150.919846),
                    153 => Ok(152.921226),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Gd => match isotope {
                None => Ok(157.25),
                Some(isotope) => match isotope {
                    152 => Ok(151.919788),
                    154 => Ok(153.920862),
                    155 => Ok(154.922619),
                    156 => Ok(155.922120),
                    157 => Ok(156.923957),
                    158 => Ok(157.924101),
                    160 => Ok(159.927051),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Tb => match isotope {
                None => Ok(158.925343),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Dy => match isotope {
                None => Ok(162.50),
                Some(isotope) => match isotope {
                    156 => Ok(155.924278),
                    158 => Ok(157.924405),
                    160 => Ok(159.925194),
                    161 => Ok(160.926930),
                    162 => Ok(161.926795),
                    163 => Ok(162.928728),
                    164 => Ok(163.929171),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ho => match isotope {
                None => Ok(164.930319),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Er => match isotope {
                None => Ok(167.26),
                Some(isotope) => match isotope {
                    162 => Ok(161.928775),
                    164 => Ok(163.929197),
                    166 => Ok(165.930290),
                    167 => Ok(166.932045),
                    168 => Ok(167.932368),
                    170 => Ok(169.935460),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Yb => match isotope {
                None => Ok(173.04),
                Some(isotope) => match isotope {
                    168 => Ok(167.933894),
                    170 => Ok(169.934759),
                    171 => Ok(170.936322),
                    172 => Ok(171.936377),
                    173 => Ok(172.938207),
                    174 => Ok(173.938858),
                    176 => Ok(175.942568),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            
            },
            Element::Lu => match isotope {
                None => Ok(174.9668),
                Some(isotope) => match isotope {
                    175 => Ok(174.940768),
                    176 => Ok(174.942682),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Hf => match isotope {
                None => Ok(178.49),
                Some(isotope) => match isotope {
                    174 => Ok(173.940040),
                    176 => Ok(175.941402),
                    177 => Ok(176.943220),
                    178 => Ok(177.943698),
                    179 => Ok(178.945815),
                    180 => Ok(179.946549),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ta => match isotope {
                None => Ok(180.947_88),
                Some(isotope) => match isotope {
                    180 => Ok(179.947466),
                    181 => Ok(180.947996),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::W => match isotope {
                None => Ok(183.84),
                Some(isotope) => match isotope {
                    180 => Ok(179.946706),
                    182 => Ok(181.948206),
                    183 => Ok(182.950224),
                    184 => Ok(183.950933),
                    186 => Ok(185.954362),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Re => match isotope {
                None => Ok(186.207),
                Some(isotope) => match isotope {
                    185 => Ok(184.952956),
                    187 => Ok(186.955751),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Os => match isotope {
                None => Ok(190.23),
                Some(isotope) => match isotope {
                    184 => Ok(183.952491),
                    186 => Ok(185.953838),
                    187 => Ok(186.955748),
                    188 => Ok(187.955836),
                    189 => Ok(188.958145),
                    190 => Ok(189.958445),
                    192 => Ok(191.961479),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ir => match isotope {
                None => Ok(192.217),
                Some(isotope) => match isotope {
                    191 => Ok(190.960591),
                    193 => Ok(192.962924),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Pt => match isotope {
                None => Ok(195.084),
                Some(isotope) => match isotope {
                    190 => Ok(189.959930),
                    192 => Ok(191.961035),
                    194 => Ok(193.962664),
                    195 => Ok(194.964774),
                    196 => Ok(195.964935),
                    198 => Ok(197.967876),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Au => match isotope {
                None => Ok(196.966_552),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Hg => match isotope {
                None => Ok(200.592),
                Some(isotope) => match isotope {
                    196 => Ok(195.965815),
                    198 => Ok(197.966752),
                    199 => Ok(198.968262),
                    200 => Ok(199.968309),
                    201 => Ok(200.970285),
                    202 => Ok(201.970626),
                    204 => Ok(203.973476),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Tl => match isotope {
                None => Ok(204.38),
                Some(isotope) => match isotope {
                    203 => Ok(202.972329),
                    205 => Ok(204.974412),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Pb => match isotope {
                None => Ok(207.2),
                Some(isotope) => match isotope {
                    204 => Ok(203.973_029),
                    206 => Ok(205.974_449),
                    207 => Ok(206.975_881),
                    208 => Ok(207.976_636),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Bi => match isotope {
                None => Ok(208.980_383),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Po => match isotope {
                None => Ok(209.982416),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::At => match isotope {
                None => Ok(210.987131),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Rn => match isotope {
                None => Ok(222.017570),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Fr => match isotope {
                None => Ok(223.019731),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ra => match isotope {
                None => Ok(226.025403),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Ac => match isotope {
                None => Ok(227.027747),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Th => match isotope {
                None => Ok(232.038_050),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Pa => match isotope {
                None => Ok(231.035_879),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::U => match isotope {
                None => Ok(238.028_910),
                Some(isotope) => match isotope {
                    234 => Ok(234.040_946),
                    235 => Ok(235.043_923),
                    238 => Ok(238.050_783),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Np => match isotope {
                None => Ok(237.0),
                Some(isotope) => match isotope {
                    237 => Ok(237.0),
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Pu => match isotope {
                None => Ok(244.048167),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Am => match isotope {
                None => Ok(243.064198),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cm => match isotope {
                None => Ok(247.070347),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Bk => match isotope {
                None => Ok(247.070299),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Cf => match isotope {
                None => Ok(251.079580),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Es => match isotope {
                None => Ok(252.082972),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Fm => match isotope {
                None => Ok(257.095099),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Md => match isotope {
                None => Ok(258.098425),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::No => match isotope {
                None => Ok(259.101024),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Lr => match isotope {
                None => Ok(262.109692),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Rf => match isotope {
                None => Ok(263.118313),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            Element::Db => match isotope {
                None => Ok(262.011437),
                Some(isotope) => match isotope {
                    _ => Err(Error::InvalidIsotope(self.atomic_symbol(), isotope)),
                },
            },
            _ => Err(Error::InvalidAtomicSymbol(self.atomic_symbol())),
        }
    }

    pub fn covalent_radius(&self) -> Option<f64> {
        match self {
            Element::Unknown => None,
            Element::H => Some(0.37),
            Element::He => Some(0.32),
            Element::Li => Some(1.34),
            Element::Be => Some(0.90),
            Element::B => Some(0.82),
            Element::C => Some(0.77),
            Element::N => Some(0.75),
            Element::O => Some(0.73),
            Element::F => Some(0.71),
            Element::Ne => Some(0.69),
            Element::Na => Some(1.54),
            Element::Mg => Some(1.30),
            Element::Al => Some(1.18),
            Element::Si => Some(1.11),
            Element::P => Some(1.06),
            Element::S => Some(1.02),
            Element::Cl => Some(0.99),
            Element::Ar => Some(0.97),
            Element::K => Some(1.96),
            Element::Ca => Some(1.74),
            Element::Sc => Some(1.44),
            Element::Ti => Some(1.36),
            Element::V => Some(1.25),
            Element::Cr => Some(1.27),
            Element::Mn => Some(1.39),
            Element::Fe => Some(2.05),
            Element::Co => Some(1.26),
            Element::Ni => Some(1.21),
            Element::Cu => Some(1.38),
            Element::Zn => Some(1.31),
            Element::Ga => Some(1.26),
            Element::Ge => Some(1.22),
            Element::As => Some(1.19),
            Element::Br => Some(1.14),
            Element::Kr => Some(1.10),
            Element::Rb => Some(2.11),
            Element::Sr => Some(1.92),
            Element::Y => Some(1.62),
            Element::Zr => Some(1.48),
            Element::Nb => Some(1.37),
            Element::Mo => Some(1.45),
            Element::Tc => Some(1.56),
            Element::Ru => Some(1.26),
            Element::Rh => Some(1.35),
            Element::Pd => Some(1.31),
            Element::Ag => Some(1.53),
            Element::Cd => Some(1.48),
            Element::In => Some(1.44),
            Element::Sn => Some(1.41),
            Element::Sb => Some(1.38),
            Element::Te => Some(1.35),
            Element::I => Some(1.33),
            Element::Xe => Some(2.16),
            Element::Cs => Some(2.25),
            Element::Ba => Some(1.98),
            Element::La => Some(1.69),
            Element::Ce => None,
            Element::Pr => None,
            Element::Nd => None,
            Element::Pm => None,
            Element::Sm => None,
            Element::Eu => Some(2.40),
            Element::Gd => None,
            Element::Tb => None,
            Element::Dy => None,
            Element::Ho => None,
            Element::Er => None,
            Element::Tm => None,
            Element::Yb => None,
            Element::Lu => Some(1.60),
            Element::Hf => Some(1.50),
            Element::Ta => Some(1.38),
            Element::W => Some(1.46),
            Element::Re => Some(1.59),
            Element::Os => Some(1.28),
            Element::Ir => Some(1.37),
            Element::Pt => Some(1.28),
            Element::Au => Some(1.44),
            Element::Hg => Some(1.49),
            Element::Tl => Some(1.48),
            Element::Pb => Some(2.30),
            Element::Bi => Some(1.46),
            Element::Po => None,
            Element::At => None,
            Element::Rn => None,
            Element::Fr => None,
            Element::Ra => None,
            Element::Ac => None,
            Element::Th => None,
            Element::Pa => None,
            Element::U => None,
            Element::Np => None,
            Element::Pu => None,
            Element::Am => None,
            Element::Cm => None,
            Element::Bk => None,
            Element::Cf => None,
            Element::Es => None,
            Element::Fm => None,
            Element::Md => None,
            Element::No => None,
            Element::Lr => None,
            Element::Rf => None,
            Element::Db => None,
            Element::Sg => None,
            Element::Bh => None,
            Element::Hs => None,
            Element::Mt => None,
            Element::Ds => None,
            Element::Rg => None,
            Element::Cn => None,
            Element::Nh => None,
            Element::Fl => None,
            Element::Mc => None,
            Element::Lv => None,
            Element::Ts => None,
            Element::Og => None,
            _ => None,
        }
    }

    pub fn vdw_radius(&self) -> Option<f64> {
        match self {
            Element::Unknown => None,
            Element::H => Some(1.20),
            Element::He => Some(1.40),
            Element::Li => Some(2.20),
            Element::Be => Some(1.90),
            Element::B => Some(1.80),
            Element::C => Some(1.70),
            Element::N => Some(1.60),
            Element::O => Some(1.55),
            Element::F => Some(1.50),
            Element::Ne => Some(1.54),

            Element::Na => Some(2.40),
            Element::Mg => Some(2.20),
            Element::Al => Some(2.10),
            Element::Si => Some(2.10),
            Element::P => Some(1.95),
            Element::S => Some(2.58),
            Element::Cl => Some(1.80),
            Element::Ar => Some(1.88),
            Element::K => Some(2.80),
            Element::Ca => Some(2.40),
            Element::Sc => Some(2.30),
            Element::Ti => Some(2.15),

            Element::V => Some(2.05),
            Element::Cr => Some(2.05),
            Element::Mn => Some(2.05),
            Element::Fe => Some(2.05),
            Element::Co => None,
            Element::Ni => None,
            Element::Cu => None,
            Element::Zn => Some(2.10),
            Element::Ga => Some(2.10),
            Element::Ge => Some(2.10),
            Element::As => Some(2.05),
            Element::Se => Some(1.90),
            Element::Br => Some(1.90),
            Element::Kr => Some(2.02),
            Element::Rb => Some(2.90),
            Element::Sr => Some(2.55),
            Element::Y => Some(2.40),
            Element::Zr => Some(2.30),
            Element::Nb => Some(2.15),
            Element::Mo => Some(2.10),
            Element::Tc => Some(2.05),
            Element::Ru => Some(2.05),
            Element::Rh => None,
            Element::Pd => Some(2.05),
            Element::Ag => Some(2.10),
            Element::Cd => Some(2.20),
            Element::In => Some(2.20),
            Element::Sn => Some(2.25),
            Element::Sb => Some(2.20),
            Element::Te => Some(2.10),
            Element::I => Some(2.16),
            Element::Xe => Some(3.00),
            Element::Cs => Some(3.00),
            Element::Ba => Some(2.70),

            Element::La => Some(2.50),
            Element::Ce => Some(2.48),
            Element::Pr => Some(2.47),
            Element::Nd => Some(2.45),
            Element::Pm => Some(2.43),
            Element::Sm => Some(2.42),
            Element::Eu => Some(2.40),
            Element::Gd => Some(2.38),
            Element::Tb => Some(2.37),
            Element::Dy => Some(2.35),
            Element::Ho => Some(2.33),
            Element::Er => Some(2.32),
            Element::Tm => Some(2.30),
            Element::Yb => Some(2.28),
            Element::Lu => Some(2.27),
            Element::Hf => Some(2.25),
            Element::Ta => Some(2.20),
            Element::W => Some(2.10),
            Element::Re => Some(2.05),
            Element::Os => None,
            Element::Ir => None,
            Element::Pt => Some(2.05),
            Element::Au => Some(2.10),
            Element::Hg => Some(2.05),
            Element::Tl => Some(2.20),
            Element::Pb => Some(2.30),
            Element::Bi => Some(2.30),
            Element::Po => None,
            Element::At => None,
            Element::Rn => None,
            Element::Fr => None,
            Element::Ra => None,
            Element::Ac => None,
            Element::Th => Some(2.40),
            Element::Pa => None,

            Element::U => Some(2.30),
            Element::Np => None,
            Element::Pu => None,
            Element::Am => None,
            Element::Cm => None,
            Element::Bk => None,
            Element::Cf => None,
            Element::Es => None,
            Element::Fm => None,
            Element::Md => None,
            Element::No => None,
            Element::Lr => None,
            Element::Rf => None,
            Element::Db => None,
            Element::Sg => None,
            Element::Bh => None,
            Element::Hs => None,
            Element::Mt => None,
            Element::Ds => None,
            Element::Rg => None,
            Element::Cn => None,
            Element::Nh => None,
            Element::Fl => None,
            Element::Mc => None,
            Element::Lv => None,
            Element::Ts => None,
            Element::Og => None,
            Element::Cn => None,
            _ => None,
        }
    }

    pub fn electronegativity(&self) -> Option<f64> {
        match self {
            Element::Unknown => None,
            Element::H => Some(2.20),
            Element::He => None,
            Element::Li => Some(0.98),
            Element::Be => Some(1.57),
            Element::B => Some(2.04),
            Element::C => Some(2.55),
            Element::N => Some(3.04),
            Element::O => Some(3.44),
            Element::F => Some(3.98),
            Element::Ne => None,
            Element::Na => Some(0.93),
            Element::Mg => Some(1.31),
            Element::Al => Some(1.18),
            Element::Si => Some(1.90),
            Element::P => Some(2.19),
            Element::S => Some(2.58),
            Element::Cl => Some(3.16),
            Element::Ar => Some(0.0),
            Element::K => Some(0.82),
            Element::Ca => Some(1.00),
            Element::Sc => Some(1.36),
            Element::Ti => Some(1.54),
            Element::V => Some(1.63),
            Element::Cr => Some(1.66),
            Element::Mn => Some(1.55),
            Element::Fe => Some(1.83),
            Element::Co => Some(1.88),
            Element::Ni => Some(1.91),
            Element::Cu => Some(1.90),
            Element::Zn => Some(1.65),
            Element::Ga => Some(1.81),
            Element::Ge => Some(2.01),
            Element::As => Some(2.18),
            Element::Se => Some(2.55),
            Element::Br => Some(2.96),
            Element::Kr => Some(3.00),
            Element::Rb => Some(0.82),
            Element::Sr => Some(0.95),
            Element::Y => Some(1.22),
            Element::Zr => Some(1.33),
            Element::Nb => Some(1.60),
            Element::Mo => Some(2.16),
            Element::Tc => Some(1.90),
            Element::Ru => Some(2.20),
            Element::Rh => Some(2.28),
            Element::Pd => Some(2.20),
            Element::Ag => Some(1.93),
            Element::Cd => Some(1.69),
            Element::In => Some(1.78),
            Element::Sn => Some(1.96),
            Element::Sb => Some(2.05),
            Element::Te => Some(2.10),
            Element::I => Some(2.66),
            Element::Xe => Some(2.60),
            Element::Cs => Some(0.70),
            Element::Ba => Some(0.89),
            Element::La => Some(1.10),
            Element::Ce => Some(1.12),
            Element::Pr => Some(1.13),
            Element::Nd => Some(1.14),
            Element::Pm => None,
            Element::Sm => Some(1.17),
            Element::Eu => None,
            Element::Gd => Some(1.20),
            Element::Tb => None,
            Element::Dy => Some(1.22),
            Element::Ho => Some(1.23),
            Element::Er => Some(1.24),
            Element::Tm => Some(1.25),
            Element::Yb => None,
            Element::Lu => Some(1.27),
            Element::Hf => Some(1.30),
            Element::Ta => Some(1.50),
            Element::W => Some(2.36),
            Element::Re => Some(1.90),
            Element::Os => Some(2.20),
            Element::Ir => Some(2.20),
            Element::Pt => Some(2.28),
            Element::Au => Some(2.54),
            Element::Hg => Some(2.00),
            Element::Tl => Some(1.62),
            Element::Pb => Some(2.33),
            Element::Bi => Some(2.02),
            Element::Po => Some(2.00),
            Element::At => Some(2.20),
            Element::Rn => None,
            Element::Fr => Some(0.70),
            Element::Ra => Some(0.90),
            Element::Ac => Some(1.10),
            Element::Th => Some(1.30),
            Element::Pa => Some(1.50),
            Element::U => Some(1.38),
            Element::Np => Some(1.36),
            Element::Pu => Some(1.28),
            Element::Am => Some(1.30),
            Element::Cm => Some(1.30),
            Element::Bk => Some(1.30),
            Element::Cf => Some(1.30),
            Element::Es => Some(1.30),
            Element::Fm => Some(1.30),
            Element::Md => Some(1.30),
            Element::No => Some(1.30),
            Element::Lr => None,
            Element::Rf => None,
            Element::Db => None,
            Element::Sg => None,
            Element::Bh => None,
            Element::Hs => None,
            Element::Mt => None,
            Element::Ds => None,
            Element::Rg => None,
            Element::Cn => None,
            _ => None,
        }
    }

    /// Return the period in the periodic table this element belongs to.
    /// If the element is Unknown, its period is 0.
    pub fn period(&self) -> i32 {
        match self {
            Element::Unknown => 0,
            Element::H | Element::He => 1,
            Element::Li
            | Element::Be
            | Element::B
            | Element::C
            | Element::N
            | Element::O
            | Element::F
            | Element::Ne => 2,
            Element::Na
            | Element::Mg
            | Element::Al
            | Element::Si
            | Element::P
            | Element::S
            | Element::Cl
            | Element::Ar => 3,
            Element::K
            | Element::Ca
            | Element::Sc
            | Element::Ti
            | Element::V
            | Element::Cr
            | Element::Mn
            | Element::Fe
            | Element::Co
            | Element::Ni
            | Element::Cu
            | Element::Zn
            | Element::Ga
            | Element::Ge
            | Element::As
            | Element::Se
            | Element::Br
            | Element::Kr => 4,
            Element::Rb
            | Element::Sr
            | Element::Y
            | Element::Zr
            | Element::Nb
            | Element::Mo
            | Element::Tc
            | Element::Ru
            | Element::Rh
            | Element::Pd
            | Element::Ag
            | Element::Cd
            | Element::In
            | Element::Sn
            | Element::Sb
            | Element::Te
            | Element::I
            | Element::Xe => 5,
            Element::Cs
            | Element::Ba
            | Element::La
            | Element::Ce
            | Element::Pr
            | Element::Nd
            | Element::Pm
            | Element::Sm
            | Element::Eu
            | Element::Gd
            | Element::Tb
            | Element::Dy
            | Element::Ho
            | Element::Er
            | Element::Tm
            | Element::Yb
            | Element::Lu
            | Element::Hf
            | Element::Ta
            | Element::W
            | Element::Re
            | Element::Os
            | Element::Ir
            | Element::Pt
            | Element::Au
            | Element::Hg
            | Element::Tl
            | Element::Pb
            | Element::Bi
            | Element::Po
            | Element::At
            | Element::Rn => 6,
            _ => 7,
        }
    }

    /// Return the group in the periodic table this element belongs to.
    /// If the element does not belong to a group, its group is 0.
    pub fn group(&self) -> i32 {
        match self {
            Element::H => 1,
            Element::He
            | Element::Ne
            | Element::Ar
            | Element::Kr
            | Element::Xe
            | Element::Rn
            | Element::Og => 18,
            Element::Li | Element::Na | Element::K | Element::Rb | Element::Cs | Element::Fr => 1,
            Element::Be | Element::Mg | Element::Ca | Element::Sr | Element::Ba | Element::Ra => 2,
            Element::Sc | Element::Y | Element::La | Element::Ac => 3,
            Element::Ti | Element::Zr | Element::Hf | Element::Rf => 4,
            Element::V | Element::Nb | Element::Ta | Element::Db => 5,
            Element::Cr | Element::Mo | Element::W | Element::Sg => 6,
            Element::Mn | Element::Tc | Element::Re | Element::Bh => 7,
            Element::Fe | Element::Ru | Element::Os | Element::Hs => 8,
            Element::Co | Element::Rh | Element::Ir | Element::Mt => 9,
            Element::Ni | Element::Pd | Element::Pt | Element::Ds => 10,
            Element::Cu | Element::Ag | Element::Au | Element::Rg => 11,
            Element::Zn | Element::Cd | Element::Hg | Element::Cn => 12,
            Element::B | Element::Al | Element::Ga | Element::In | Element::Tl | Element::Nh => 13,
            Element::C | Element::Si | Element::Ge | Element::Sn | Element::Pb | Element::Fl => 14,
            Element::N | Element::P | Element::As | Element::Sb | Element::Bi | Element::Mc => 15,
            Element::O | Element::S | Element::Se | Element::Te | Element::Po | Element::Lv => 16,
            Element::F | Element::Cl | Element::Br | Element::I | Element::At | Element::Ts => 17,
            Element::Ce
            | Element::Pr
            | Element::Nd
            | Element::Pm
            | Element::Sm
            | Element::Eu
            | Element::Gd
            | Element::Tb
            | Element::Dy
            | Element::Ho
            | Element::Er
            | Element::Tm
            | Element::Yb
            | Element::Lu
            | Element::Th
            | Element::Pa
            | Element::U
            | Element::Np
            | Element::Pu
            | Element::Am
            | Element::Cm
            | Element::Bk
            | Element::Cf
            | Element::Es
            | Element::Fm
            | Element::Md
            | Element::No
            | Element::Lr => 0,
            Element::Unknown => 420,
        }
    }

    pub fn n_valence_electrons(&self, formal_charge: i8) -> Result<u8, Error> {
        let mut n_valence_electrons = match self {
            Element::H => 1,
            Element::B => 3,
            Element::C => 4,
            Element::N => 5,
            Element::O => 6,
            Element::F => 7,
            Element::P => 5,
            Element::S => 6,
            Element::Cl => 7,
            Element::Br => 7,
            Element::I => 7,
            _ => unimplemented!(),
            _ => return Err(Error::InvalidAtomicSymbol(self.atomic_symbol())),
        };

        n_valence_electrons -= formal_charge;
        if !(0..=8).contains(&n_valence_electrons) {
            return Err(Error::InvalidFormalCharge(
                self.atomic_symbol(),
                formal_charge,
            ));
        }

        Ok(n_valence_electrons as u8)
    }

    pub fn valence(&self, formal_charge: i8) -> Result<u8, Error> {
        let n_valence_electrons = self.n_valence_electrons(formal_charge)?;

        match n_valence_electrons {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(2),
            3 => Ok(3),
            4 => Ok(4),
            5 => Ok(3),
            6 => Ok(2),
            7 => Ok(1),
            8 => Ok(0),
            _ => unreachable!(),
        }
    }

    pub fn is_metal(&self) -> bool {
        match self {
            Element::H
            | Element::He
            | Element::C
            | Element::N
            | Element::O
            | Element::F
            | Element::Ne
            | Element::P
            | Element::S
            | Element::Cl
            | Element::Ar
            | Element::Se
            | Element::Br
            | Element::Kr
            | Element::I
            | Element::Xe
            | Element::Rn => false,
            Element::B
            | Element::Si
            | Element::Ge
            | Element::As
            | Element::Sb
            | Element::Te
            | Element::At => false,
            _ => true,
        }
    }

    pub fn is_metalloid(&self) -> bool {
        matches!(
            self,
            Element::B
                | Element::Si
                | Element::Ge
                | Element::As
                | Element::Sb
                | Element::Te
                | Element::At
        )
    }
    pub fn valence_electrons(&self) -> i32 {
        let mut result = self.atomic_number();

        if let Some(core) = self.core() {
            result -= core.atomic_number();
        }
        result
    }
    fn core(&self) -> Option<Self> {
        if self.atomic_number() < 3 {
            None
        } else if self.atomic_number() < 11 {
            Some(Element::He)
        } else if self.atomic_number() < 19 {
            Some(Element::Ne)
        } else if self.atomic_number() < 37 {
            Some(Element::Ar)
        } else if self.atomic_number() < 55 {
            Some(Element::Kr)
        } else if self.atomic_number() < 87 {
            Some(Element::Xe)
        } else {
            Some(Element::Rn)
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

pub struct NaturalElement {
    symbol: String,
    number: i32,
}

impl NaturalElement {
    pub fn new(symbol: &str, number: i32) -> Self {
        NaturalElement {
            symbol: symbol.to_string(),
            number,
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_atomic_number(&self) -> i32 {
        self.number
    }
}