#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    nmicr: Nmicr,
    nmienr: Nmienr,
    nmifr: Nmifr,
    nmicfr: Nmicfr,
    eirqcr0: Eirqcr0,
    eirqcr1: Eirqcr1,
    eirqcr2: Eirqcr2,
    eirqcr3: Eirqcr3,
    eirqcr4: Eirqcr4,
    eirqcr5: Eirqcr5,
    eirqcr6: Eirqcr6,
    eirqcr7: Eirqcr7,
    eirqcr8: Eirqcr8,
    eirqcr9: Eirqcr9,
    eirqcr10: Eirqcr10,
    eirqcr11: Eirqcr11,
    eirqcr12: Eirqcr12,
    eirqcr13: Eirqcr13,
    eirqcr14: Eirqcr14,
    eirqcr15: Eirqcr15,
    wupen: Wupen,
    eifr: Eifr,
    eifcr: Eifcr,
    sel0: Sel0,
    sel1: Sel1,
    sel2: Sel2,
    sel3: Sel3,
    sel4: Sel4,
    sel5: Sel5,
    sel6: Sel6,
    sel7: Sel7,
    sel8: Sel8,
    sel9: Sel9,
    sel10: Sel10,
    sel11: Sel11,
    sel12: Sel12,
    sel13: Sel13,
    sel14: Sel14,
    sel15: Sel15,
    sel16: Sel16,
    sel17: Sel17,
    sel18: Sel18,
    sel19: Sel19,
    sel20: Sel20,
    sel21: Sel21,
    sel22: Sel22,
    sel23: Sel23,
    sel24: Sel24,
    sel25: Sel25,
    sel26: Sel26,
    sel27: Sel27,
    sel28: Sel28,
    sel29: Sel29,
    sel30: Sel30,
    sel31: Sel31,
    sel32: Sel32,
    sel33: Sel33,
    sel34: Sel34,
    sel35: Sel35,
    sel36: Sel36,
    sel37: Sel37,
    sel38: Sel38,
    sel39: Sel39,
    sel40: Sel40,
    sel41: Sel41,
    sel42: Sel42,
    sel43: Sel43,
    sel44: Sel44,
    sel45: Sel45,
    sel46: Sel46,
    sel47: Sel47,
    sel48: Sel48,
    sel49: Sel49,
    sel50: Sel50,
    sel51: Sel51,
    sel52: Sel52,
    sel53: Sel53,
    sel54: Sel54,
    sel55: Sel55,
    sel56: Sel56,
    sel57: Sel57,
    sel58: Sel58,
    sel59: Sel59,
    sel60: Sel60,
    sel61: Sel61,
    sel62: Sel62,
    sel63: Sel63,
    sel64: Sel64,
    sel65: Sel65,
    sel66: Sel66,
    sel67: Sel67,
    sel68: Sel68,
    sel69: Sel69,
    sel70: Sel70,
    sel71: Sel71,
    sel72: Sel72,
    sel73: Sel73,
    sel74: Sel74,
    sel75: Sel75,
    sel76: Sel76,
    sel77: Sel77,
    sel78: Sel78,
    sel79: Sel79,
    sel80: Sel80,
    sel81: Sel81,
    sel82: Sel82,
    sel83: Sel83,
    sel84: Sel84,
    sel85: Sel85,
    sel86: Sel86,
    sel87: Sel87,
    sel88: Sel88,
    sel89: Sel89,
    sel90: Sel90,
    sel91: Sel91,
    sel92: Sel92,
    sel93: Sel93,
    sel94: Sel94,
    sel95: Sel95,
    sel96: Sel96,
    sel97: Sel97,
    sel98: Sel98,
    sel99: Sel99,
    sel100: Sel100,
    sel101: Sel101,
    sel102: Sel102,
    sel103: Sel103,
    sel104: Sel104,
    sel105: Sel105,
    sel106: Sel106,
    sel107: Sel107,
    sel108: Sel108,
    sel109: Sel109,
    sel110: Sel110,
    sel111: Sel111,
    sel112: Sel112,
    sel113: Sel113,
    sel114: Sel114,
    sel115: Sel115,
    sel116: Sel116,
    sel117: Sel117,
    sel118: Sel118,
    sel119: Sel119,
    sel120: Sel120,
    sel121: Sel121,
    sel122: Sel122,
    sel123: Sel123,
    sel124: Sel124,
    sel125: Sel125,
    sel126: Sel126,
    sel127: Sel127,
    vssel128: Vssel128,
    vssel129: Vssel129,
    vssel130: Vssel130,
    vssel131: Vssel131,
    vssel132: Vssel132,
    vssel133: Vssel133,
    vssel134: Vssel134,
    vssel135: Vssel135,
    vssel136: Vssel136,
    vssel137: Vssel137,
    vssel138: Vssel138,
    vssel139: Vssel139,
    vssel140: Vssel140,
    vssel141: Vssel141,
    vssel142: Vssel142,
    vssel143: Vssel143,
    swier: Swier,
    evter: Evter,
    ier: Ier,
}
impl RegisterBlock {
    #[doc = "0x00 - desc NMICR"]
    #[inline(always)]
    pub const fn nmicr(&self) -> &Nmicr {
        &self.nmicr
    }
    #[doc = "0x04 - desc NMIENR"]
    #[inline(always)]
    pub const fn nmienr(&self) -> &Nmienr {
        &self.nmienr
    }
    #[doc = "0x08 - desc NMIFR"]
    #[inline(always)]
    pub const fn nmifr(&self) -> &Nmifr {
        &self.nmifr
    }
    #[doc = "0x0c - desc NMICFR"]
    #[inline(always)]
    pub const fn nmicfr(&self) -> &Nmicfr {
        &self.nmicfr
    }
    #[doc = "0x10 - desc EIRQCR0"]
    #[inline(always)]
    pub const fn eirqcr0(&self) -> &Eirqcr0 {
        &self.eirqcr0
    }
    #[doc = "0x14 - desc EIRQCR1"]
    #[inline(always)]
    pub const fn eirqcr1(&self) -> &Eirqcr1 {
        &self.eirqcr1
    }
    #[doc = "0x18 - desc EIRQCR2"]
    #[inline(always)]
    pub const fn eirqcr2(&self) -> &Eirqcr2 {
        &self.eirqcr2
    }
    #[doc = "0x1c - desc EIRQCR3"]
    #[inline(always)]
    pub const fn eirqcr3(&self) -> &Eirqcr3 {
        &self.eirqcr3
    }
    #[doc = "0x20 - desc EIRQCR4"]
    #[inline(always)]
    pub const fn eirqcr4(&self) -> &Eirqcr4 {
        &self.eirqcr4
    }
    #[doc = "0x24 - desc EIRQCR5"]
    #[inline(always)]
    pub const fn eirqcr5(&self) -> &Eirqcr5 {
        &self.eirqcr5
    }
    #[doc = "0x28 - desc EIRQCR6"]
    #[inline(always)]
    pub const fn eirqcr6(&self) -> &Eirqcr6 {
        &self.eirqcr6
    }
    #[doc = "0x2c - desc EIRQCR7"]
    #[inline(always)]
    pub const fn eirqcr7(&self) -> &Eirqcr7 {
        &self.eirqcr7
    }
    #[doc = "0x30 - desc EIRQCR8"]
    #[inline(always)]
    pub const fn eirqcr8(&self) -> &Eirqcr8 {
        &self.eirqcr8
    }
    #[doc = "0x34 - desc EIRQCR9"]
    #[inline(always)]
    pub const fn eirqcr9(&self) -> &Eirqcr9 {
        &self.eirqcr9
    }
    #[doc = "0x38 - desc EIRQCR10"]
    #[inline(always)]
    pub const fn eirqcr10(&self) -> &Eirqcr10 {
        &self.eirqcr10
    }
    #[doc = "0x3c - desc EIRQCR11"]
    #[inline(always)]
    pub const fn eirqcr11(&self) -> &Eirqcr11 {
        &self.eirqcr11
    }
    #[doc = "0x40 - desc EIRQCR12"]
    #[inline(always)]
    pub const fn eirqcr12(&self) -> &Eirqcr12 {
        &self.eirqcr12
    }
    #[doc = "0x44 - desc EIRQCR13"]
    #[inline(always)]
    pub const fn eirqcr13(&self) -> &Eirqcr13 {
        &self.eirqcr13
    }
    #[doc = "0x48 - desc EIRQCR14"]
    #[inline(always)]
    pub const fn eirqcr14(&self) -> &Eirqcr14 {
        &self.eirqcr14
    }
    #[doc = "0x4c - desc EIRQCR15"]
    #[inline(always)]
    pub const fn eirqcr15(&self) -> &Eirqcr15 {
        &self.eirqcr15
    }
    #[doc = "0x50 - desc WUPEN"]
    #[inline(always)]
    pub const fn wupen(&self) -> &Wupen {
        &self.wupen
    }
    #[doc = "0x54 - desc EIFR"]
    #[inline(always)]
    pub const fn eifr(&self) -> &Eifr {
        &self.eifr
    }
    #[doc = "0x58 - desc EIFCR"]
    #[inline(always)]
    pub const fn eifcr(&self) -> &Eifcr {
        &self.eifcr
    }
    #[doc = "0x5c - desc SEL0"]
    #[inline(always)]
    pub const fn sel0(&self) -> &Sel0 {
        &self.sel0
    }
    #[doc = "0x60 - desc SEL1"]
    #[inline(always)]
    pub const fn sel1(&self) -> &Sel1 {
        &self.sel1
    }
    #[doc = "0x64 - desc SEL2"]
    #[inline(always)]
    pub const fn sel2(&self) -> &Sel2 {
        &self.sel2
    }
    #[doc = "0x68 - desc SEL3"]
    #[inline(always)]
    pub const fn sel3(&self) -> &Sel3 {
        &self.sel3
    }
    #[doc = "0x6c - desc SEL4"]
    #[inline(always)]
    pub const fn sel4(&self) -> &Sel4 {
        &self.sel4
    }
    #[doc = "0x70 - desc SEL5"]
    #[inline(always)]
    pub const fn sel5(&self) -> &Sel5 {
        &self.sel5
    }
    #[doc = "0x74 - desc SEL6"]
    #[inline(always)]
    pub const fn sel6(&self) -> &Sel6 {
        &self.sel6
    }
    #[doc = "0x78 - desc SEL7"]
    #[inline(always)]
    pub const fn sel7(&self) -> &Sel7 {
        &self.sel7
    }
    #[doc = "0x7c - desc SEL8"]
    #[inline(always)]
    pub const fn sel8(&self) -> &Sel8 {
        &self.sel8
    }
    #[doc = "0x80 - desc SEL9"]
    #[inline(always)]
    pub const fn sel9(&self) -> &Sel9 {
        &self.sel9
    }
    #[doc = "0x84 - desc SEL10"]
    #[inline(always)]
    pub const fn sel10(&self) -> &Sel10 {
        &self.sel10
    }
    #[doc = "0x88 - desc SEL11"]
    #[inline(always)]
    pub const fn sel11(&self) -> &Sel11 {
        &self.sel11
    }
    #[doc = "0x8c - desc SEL12"]
    #[inline(always)]
    pub const fn sel12(&self) -> &Sel12 {
        &self.sel12
    }
    #[doc = "0x90 - desc SEL13"]
    #[inline(always)]
    pub const fn sel13(&self) -> &Sel13 {
        &self.sel13
    }
    #[doc = "0x94 - desc SEL14"]
    #[inline(always)]
    pub const fn sel14(&self) -> &Sel14 {
        &self.sel14
    }
    #[doc = "0x98 - desc SEL15"]
    #[inline(always)]
    pub const fn sel15(&self) -> &Sel15 {
        &self.sel15
    }
    #[doc = "0x9c - desc SEL16"]
    #[inline(always)]
    pub const fn sel16(&self) -> &Sel16 {
        &self.sel16
    }
    #[doc = "0xa0 - desc SEL17"]
    #[inline(always)]
    pub const fn sel17(&self) -> &Sel17 {
        &self.sel17
    }
    #[doc = "0xa4 - desc SEL18"]
    #[inline(always)]
    pub const fn sel18(&self) -> &Sel18 {
        &self.sel18
    }
    #[doc = "0xa8 - desc SEL19"]
    #[inline(always)]
    pub const fn sel19(&self) -> &Sel19 {
        &self.sel19
    }
    #[doc = "0xac - desc SEL20"]
    #[inline(always)]
    pub const fn sel20(&self) -> &Sel20 {
        &self.sel20
    }
    #[doc = "0xb0 - desc SEL21"]
    #[inline(always)]
    pub const fn sel21(&self) -> &Sel21 {
        &self.sel21
    }
    #[doc = "0xb4 - desc SEL22"]
    #[inline(always)]
    pub const fn sel22(&self) -> &Sel22 {
        &self.sel22
    }
    #[doc = "0xb8 - desc SEL23"]
    #[inline(always)]
    pub const fn sel23(&self) -> &Sel23 {
        &self.sel23
    }
    #[doc = "0xbc - desc SEL24"]
    #[inline(always)]
    pub const fn sel24(&self) -> &Sel24 {
        &self.sel24
    }
    #[doc = "0xc0 - desc SEL25"]
    #[inline(always)]
    pub const fn sel25(&self) -> &Sel25 {
        &self.sel25
    }
    #[doc = "0xc4 - desc SEL26"]
    #[inline(always)]
    pub const fn sel26(&self) -> &Sel26 {
        &self.sel26
    }
    #[doc = "0xc8 - desc SEL27"]
    #[inline(always)]
    pub const fn sel27(&self) -> &Sel27 {
        &self.sel27
    }
    #[doc = "0xcc - desc SEL28"]
    #[inline(always)]
    pub const fn sel28(&self) -> &Sel28 {
        &self.sel28
    }
    #[doc = "0xd0 - desc SEL29"]
    #[inline(always)]
    pub const fn sel29(&self) -> &Sel29 {
        &self.sel29
    }
    #[doc = "0xd4 - desc SEL30"]
    #[inline(always)]
    pub const fn sel30(&self) -> &Sel30 {
        &self.sel30
    }
    #[doc = "0xd8 - desc SEL31"]
    #[inline(always)]
    pub const fn sel31(&self) -> &Sel31 {
        &self.sel31
    }
    #[doc = "0xdc - desc SEL32"]
    #[inline(always)]
    pub const fn sel32(&self) -> &Sel32 {
        &self.sel32
    }
    #[doc = "0xe0 - desc SEL33"]
    #[inline(always)]
    pub const fn sel33(&self) -> &Sel33 {
        &self.sel33
    }
    #[doc = "0xe4 - desc SEL34"]
    #[inline(always)]
    pub const fn sel34(&self) -> &Sel34 {
        &self.sel34
    }
    #[doc = "0xe8 - desc SEL35"]
    #[inline(always)]
    pub const fn sel35(&self) -> &Sel35 {
        &self.sel35
    }
    #[doc = "0xec - desc SEL36"]
    #[inline(always)]
    pub const fn sel36(&self) -> &Sel36 {
        &self.sel36
    }
    #[doc = "0xf0 - desc SEL37"]
    #[inline(always)]
    pub const fn sel37(&self) -> &Sel37 {
        &self.sel37
    }
    #[doc = "0xf4 - desc SEL38"]
    #[inline(always)]
    pub const fn sel38(&self) -> &Sel38 {
        &self.sel38
    }
    #[doc = "0xf8 - desc SEL39"]
    #[inline(always)]
    pub const fn sel39(&self) -> &Sel39 {
        &self.sel39
    }
    #[doc = "0xfc - desc SEL40"]
    #[inline(always)]
    pub const fn sel40(&self) -> &Sel40 {
        &self.sel40
    }
    #[doc = "0x100 - desc SEL41"]
    #[inline(always)]
    pub const fn sel41(&self) -> &Sel41 {
        &self.sel41
    }
    #[doc = "0x104 - desc SEL42"]
    #[inline(always)]
    pub const fn sel42(&self) -> &Sel42 {
        &self.sel42
    }
    #[doc = "0x108 - desc SEL43"]
    #[inline(always)]
    pub const fn sel43(&self) -> &Sel43 {
        &self.sel43
    }
    #[doc = "0x10c - desc SEL44"]
    #[inline(always)]
    pub const fn sel44(&self) -> &Sel44 {
        &self.sel44
    }
    #[doc = "0x110 - desc SEL45"]
    #[inline(always)]
    pub const fn sel45(&self) -> &Sel45 {
        &self.sel45
    }
    #[doc = "0x114 - desc SEL46"]
    #[inline(always)]
    pub const fn sel46(&self) -> &Sel46 {
        &self.sel46
    }
    #[doc = "0x118 - desc SEL47"]
    #[inline(always)]
    pub const fn sel47(&self) -> &Sel47 {
        &self.sel47
    }
    #[doc = "0x11c - desc SEL48"]
    #[inline(always)]
    pub const fn sel48(&self) -> &Sel48 {
        &self.sel48
    }
    #[doc = "0x120 - desc SEL49"]
    #[inline(always)]
    pub const fn sel49(&self) -> &Sel49 {
        &self.sel49
    }
    #[doc = "0x124 - desc SEL50"]
    #[inline(always)]
    pub const fn sel50(&self) -> &Sel50 {
        &self.sel50
    }
    #[doc = "0x128 - desc SEL51"]
    #[inline(always)]
    pub const fn sel51(&self) -> &Sel51 {
        &self.sel51
    }
    #[doc = "0x12c - desc SEL52"]
    #[inline(always)]
    pub const fn sel52(&self) -> &Sel52 {
        &self.sel52
    }
    #[doc = "0x130 - desc SEL53"]
    #[inline(always)]
    pub const fn sel53(&self) -> &Sel53 {
        &self.sel53
    }
    #[doc = "0x134 - desc SEL54"]
    #[inline(always)]
    pub const fn sel54(&self) -> &Sel54 {
        &self.sel54
    }
    #[doc = "0x138 - desc SEL55"]
    #[inline(always)]
    pub const fn sel55(&self) -> &Sel55 {
        &self.sel55
    }
    #[doc = "0x13c - desc SEL56"]
    #[inline(always)]
    pub const fn sel56(&self) -> &Sel56 {
        &self.sel56
    }
    #[doc = "0x140 - desc SEL57"]
    #[inline(always)]
    pub const fn sel57(&self) -> &Sel57 {
        &self.sel57
    }
    #[doc = "0x144 - desc SEL58"]
    #[inline(always)]
    pub const fn sel58(&self) -> &Sel58 {
        &self.sel58
    }
    #[doc = "0x148 - desc SEL59"]
    #[inline(always)]
    pub const fn sel59(&self) -> &Sel59 {
        &self.sel59
    }
    #[doc = "0x14c - desc SEL60"]
    #[inline(always)]
    pub const fn sel60(&self) -> &Sel60 {
        &self.sel60
    }
    #[doc = "0x150 - desc SEL61"]
    #[inline(always)]
    pub const fn sel61(&self) -> &Sel61 {
        &self.sel61
    }
    #[doc = "0x154 - desc SEL62"]
    #[inline(always)]
    pub const fn sel62(&self) -> &Sel62 {
        &self.sel62
    }
    #[doc = "0x158 - desc SEL63"]
    #[inline(always)]
    pub const fn sel63(&self) -> &Sel63 {
        &self.sel63
    }
    #[doc = "0x15c - desc SEL64"]
    #[inline(always)]
    pub const fn sel64(&self) -> &Sel64 {
        &self.sel64
    }
    #[doc = "0x160 - desc SEL65"]
    #[inline(always)]
    pub const fn sel65(&self) -> &Sel65 {
        &self.sel65
    }
    #[doc = "0x164 - desc SEL66"]
    #[inline(always)]
    pub const fn sel66(&self) -> &Sel66 {
        &self.sel66
    }
    #[doc = "0x168 - desc SEL67"]
    #[inline(always)]
    pub const fn sel67(&self) -> &Sel67 {
        &self.sel67
    }
    #[doc = "0x16c - desc SEL68"]
    #[inline(always)]
    pub const fn sel68(&self) -> &Sel68 {
        &self.sel68
    }
    #[doc = "0x170 - desc SEL69"]
    #[inline(always)]
    pub const fn sel69(&self) -> &Sel69 {
        &self.sel69
    }
    #[doc = "0x174 - desc SEL70"]
    #[inline(always)]
    pub const fn sel70(&self) -> &Sel70 {
        &self.sel70
    }
    #[doc = "0x178 - desc SEL71"]
    #[inline(always)]
    pub const fn sel71(&self) -> &Sel71 {
        &self.sel71
    }
    #[doc = "0x17c - desc SEL72"]
    #[inline(always)]
    pub const fn sel72(&self) -> &Sel72 {
        &self.sel72
    }
    #[doc = "0x180 - desc SEL73"]
    #[inline(always)]
    pub const fn sel73(&self) -> &Sel73 {
        &self.sel73
    }
    #[doc = "0x184 - desc SEL74"]
    #[inline(always)]
    pub const fn sel74(&self) -> &Sel74 {
        &self.sel74
    }
    #[doc = "0x188 - desc SEL75"]
    #[inline(always)]
    pub const fn sel75(&self) -> &Sel75 {
        &self.sel75
    }
    #[doc = "0x18c - desc SEL76"]
    #[inline(always)]
    pub const fn sel76(&self) -> &Sel76 {
        &self.sel76
    }
    #[doc = "0x190 - desc SEL77"]
    #[inline(always)]
    pub const fn sel77(&self) -> &Sel77 {
        &self.sel77
    }
    #[doc = "0x194 - desc SEL78"]
    #[inline(always)]
    pub const fn sel78(&self) -> &Sel78 {
        &self.sel78
    }
    #[doc = "0x198 - desc SEL79"]
    #[inline(always)]
    pub const fn sel79(&self) -> &Sel79 {
        &self.sel79
    }
    #[doc = "0x19c - desc SEL80"]
    #[inline(always)]
    pub const fn sel80(&self) -> &Sel80 {
        &self.sel80
    }
    #[doc = "0x1a0 - desc SEL81"]
    #[inline(always)]
    pub const fn sel81(&self) -> &Sel81 {
        &self.sel81
    }
    #[doc = "0x1a4 - desc SEL82"]
    #[inline(always)]
    pub const fn sel82(&self) -> &Sel82 {
        &self.sel82
    }
    #[doc = "0x1a8 - desc SEL83"]
    #[inline(always)]
    pub const fn sel83(&self) -> &Sel83 {
        &self.sel83
    }
    #[doc = "0x1ac - desc SEL84"]
    #[inline(always)]
    pub const fn sel84(&self) -> &Sel84 {
        &self.sel84
    }
    #[doc = "0x1b0 - desc SEL85"]
    #[inline(always)]
    pub const fn sel85(&self) -> &Sel85 {
        &self.sel85
    }
    #[doc = "0x1b4 - desc SEL86"]
    #[inline(always)]
    pub const fn sel86(&self) -> &Sel86 {
        &self.sel86
    }
    #[doc = "0x1b8 - desc SEL87"]
    #[inline(always)]
    pub const fn sel87(&self) -> &Sel87 {
        &self.sel87
    }
    #[doc = "0x1bc - desc SEL88"]
    #[inline(always)]
    pub const fn sel88(&self) -> &Sel88 {
        &self.sel88
    }
    #[doc = "0x1c0 - desc SEL89"]
    #[inline(always)]
    pub const fn sel89(&self) -> &Sel89 {
        &self.sel89
    }
    #[doc = "0x1c4 - desc SEL90"]
    #[inline(always)]
    pub const fn sel90(&self) -> &Sel90 {
        &self.sel90
    }
    #[doc = "0x1c8 - desc SEL91"]
    #[inline(always)]
    pub const fn sel91(&self) -> &Sel91 {
        &self.sel91
    }
    #[doc = "0x1cc - desc SEL92"]
    #[inline(always)]
    pub const fn sel92(&self) -> &Sel92 {
        &self.sel92
    }
    #[doc = "0x1d0 - desc SEL93"]
    #[inline(always)]
    pub const fn sel93(&self) -> &Sel93 {
        &self.sel93
    }
    #[doc = "0x1d4 - desc SEL94"]
    #[inline(always)]
    pub const fn sel94(&self) -> &Sel94 {
        &self.sel94
    }
    #[doc = "0x1d8 - desc SEL95"]
    #[inline(always)]
    pub const fn sel95(&self) -> &Sel95 {
        &self.sel95
    }
    #[doc = "0x1dc - desc SEL96"]
    #[inline(always)]
    pub const fn sel96(&self) -> &Sel96 {
        &self.sel96
    }
    #[doc = "0x1e0 - desc SEL97"]
    #[inline(always)]
    pub const fn sel97(&self) -> &Sel97 {
        &self.sel97
    }
    #[doc = "0x1e4 - desc SEL98"]
    #[inline(always)]
    pub const fn sel98(&self) -> &Sel98 {
        &self.sel98
    }
    #[doc = "0x1e8 - desc SEL99"]
    #[inline(always)]
    pub const fn sel99(&self) -> &Sel99 {
        &self.sel99
    }
    #[doc = "0x1ec - desc SEL100"]
    #[inline(always)]
    pub const fn sel100(&self) -> &Sel100 {
        &self.sel100
    }
    #[doc = "0x1f0 - desc SEL101"]
    #[inline(always)]
    pub const fn sel101(&self) -> &Sel101 {
        &self.sel101
    }
    #[doc = "0x1f4 - desc SEL102"]
    #[inline(always)]
    pub const fn sel102(&self) -> &Sel102 {
        &self.sel102
    }
    #[doc = "0x1f8 - desc SEL103"]
    #[inline(always)]
    pub const fn sel103(&self) -> &Sel103 {
        &self.sel103
    }
    #[doc = "0x1fc - desc SEL104"]
    #[inline(always)]
    pub const fn sel104(&self) -> &Sel104 {
        &self.sel104
    }
    #[doc = "0x200 - desc SEL105"]
    #[inline(always)]
    pub const fn sel105(&self) -> &Sel105 {
        &self.sel105
    }
    #[doc = "0x204 - desc SEL106"]
    #[inline(always)]
    pub const fn sel106(&self) -> &Sel106 {
        &self.sel106
    }
    #[doc = "0x208 - desc SEL107"]
    #[inline(always)]
    pub const fn sel107(&self) -> &Sel107 {
        &self.sel107
    }
    #[doc = "0x20c - desc SEL108"]
    #[inline(always)]
    pub const fn sel108(&self) -> &Sel108 {
        &self.sel108
    }
    #[doc = "0x210 - desc SEL109"]
    #[inline(always)]
    pub const fn sel109(&self) -> &Sel109 {
        &self.sel109
    }
    #[doc = "0x214 - desc SEL110"]
    #[inline(always)]
    pub const fn sel110(&self) -> &Sel110 {
        &self.sel110
    }
    #[doc = "0x218 - desc SEL111"]
    #[inline(always)]
    pub const fn sel111(&self) -> &Sel111 {
        &self.sel111
    }
    #[doc = "0x21c - desc SEL112"]
    #[inline(always)]
    pub const fn sel112(&self) -> &Sel112 {
        &self.sel112
    }
    #[doc = "0x220 - desc SEL113"]
    #[inline(always)]
    pub const fn sel113(&self) -> &Sel113 {
        &self.sel113
    }
    #[doc = "0x224 - desc SEL114"]
    #[inline(always)]
    pub const fn sel114(&self) -> &Sel114 {
        &self.sel114
    }
    #[doc = "0x228 - desc SEL115"]
    #[inline(always)]
    pub const fn sel115(&self) -> &Sel115 {
        &self.sel115
    }
    #[doc = "0x22c - desc SEL116"]
    #[inline(always)]
    pub const fn sel116(&self) -> &Sel116 {
        &self.sel116
    }
    #[doc = "0x230 - desc SEL117"]
    #[inline(always)]
    pub const fn sel117(&self) -> &Sel117 {
        &self.sel117
    }
    #[doc = "0x234 - desc SEL118"]
    #[inline(always)]
    pub const fn sel118(&self) -> &Sel118 {
        &self.sel118
    }
    #[doc = "0x238 - desc SEL119"]
    #[inline(always)]
    pub const fn sel119(&self) -> &Sel119 {
        &self.sel119
    }
    #[doc = "0x23c - desc SEL120"]
    #[inline(always)]
    pub const fn sel120(&self) -> &Sel120 {
        &self.sel120
    }
    #[doc = "0x240 - desc SEL121"]
    #[inline(always)]
    pub const fn sel121(&self) -> &Sel121 {
        &self.sel121
    }
    #[doc = "0x244 - desc SEL122"]
    #[inline(always)]
    pub const fn sel122(&self) -> &Sel122 {
        &self.sel122
    }
    #[doc = "0x248 - desc SEL123"]
    #[inline(always)]
    pub const fn sel123(&self) -> &Sel123 {
        &self.sel123
    }
    #[doc = "0x24c - desc SEL124"]
    #[inline(always)]
    pub const fn sel124(&self) -> &Sel124 {
        &self.sel124
    }
    #[doc = "0x250 - desc SEL125"]
    #[inline(always)]
    pub const fn sel125(&self) -> &Sel125 {
        &self.sel125
    }
    #[doc = "0x254 - desc SEL126"]
    #[inline(always)]
    pub const fn sel126(&self) -> &Sel126 {
        &self.sel126
    }
    #[doc = "0x258 - desc SEL127"]
    #[inline(always)]
    pub const fn sel127(&self) -> &Sel127 {
        &self.sel127
    }
    #[doc = "0x25c - desc VSSEL128"]
    #[inline(always)]
    pub const fn vssel128(&self) -> &Vssel128 {
        &self.vssel128
    }
    #[doc = "0x260 - desc VSSEL129"]
    #[inline(always)]
    pub const fn vssel129(&self) -> &Vssel129 {
        &self.vssel129
    }
    #[doc = "0x264 - desc VSSEL130"]
    #[inline(always)]
    pub const fn vssel130(&self) -> &Vssel130 {
        &self.vssel130
    }
    #[doc = "0x268 - desc VSSEL131"]
    #[inline(always)]
    pub const fn vssel131(&self) -> &Vssel131 {
        &self.vssel131
    }
    #[doc = "0x26c - desc VSSEL132"]
    #[inline(always)]
    pub const fn vssel132(&self) -> &Vssel132 {
        &self.vssel132
    }
    #[doc = "0x270 - desc VSSEL133"]
    #[inline(always)]
    pub const fn vssel133(&self) -> &Vssel133 {
        &self.vssel133
    }
    #[doc = "0x274 - desc VSSEL134"]
    #[inline(always)]
    pub const fn vssel134(&self) -> &Vssel134 {
        &self.vssel134
    }
    #[doc = "0x278 - desc VSSEL135"]
    #[inline(always)]
    pub const fn vssel135(&self) -> &Vssel135 {
        &self.vssel135
    }
    #[doc = "0x27c - desc VSSEL136"]
    #[inline(always)]
    pub const fn vssel136(&self) -> &Vssel136 {
        &self.vssel136
    }
    #[doc = "0x280 - desc VSSEL137"]
    #[inline(always)]
    pub const fn vssel137(&self) -> &Vssel137 {
        &self.vssel137
    }
    #[doc = "0x284 - desc VSSEL138"]
    #[inline(always)]
    pub const fn vssel138(&self) -> &Vssel138 {
        &self.vssel138
    }
    #[doc = "0x288 - desc VSSEL139"]
    #[inline(always)]
    pub const fn vssel139(&self) -> &Vssel139 {
        &self.vssel139
    }
    #[doc = "0x28c - desc VSSEL140"]
    #[inline(always)]
    pub const fn vssel140(&self) -> &Vssel140 {
        &self.vssel140
    }
    #[doc = "0x290 - desc VSSEL141"]
    #[inline(always)]
    pub const fn vssel141(&self) -> &Vssel141 {
        &self.vssel141
    }
    #[doc = "0x294 - desc VSSEL142"]
    #[inline(always)]
    pub const fn vssel142(&self) -> &Vssel142 {
        &self.vssel142
    }
    #[doc = "0x298 - desc VSSEL143"]
    #[inline(always)]
    pub const fn vssel143(&self) -> &Vssel143 {
        &self.vssel143
    }
    #[doc = "0x29c - desc SWIER"]
    #[inline(always)]
    pub const fn swier(&self) -> &Swier {
        &self.swier
    }
    #[doc = "0x2a0 - desc EVTER"]
    #[inline(always)]
    pub const fn evter(&self) -> &Evter {
        &self.evter
    }
    #[doc = "0x2a4 - desc IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
}
#[doc = "NMICR (rw) register accessor: desc NMICR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmicr`] module"]
#[doc(alias = "NMICR")]
pub type Nmicr = crate::Reg<nmicr::NmicrSpec>;
#[doc = "desc NMICR"]
pub mod nmicr;
#[doc = "NMIENR (rw) register accessor: desc NMIENR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmienr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmienr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmienr`] module"]
#[doc(alias = "NMIENR")]
pub type Nmienr = crate::Reg<nmienr::NmienrSpec>;
#[doc = "desc NMIENR"]
pub mod nmienr;
#[doc = "NMIFR (rw) register accessor: desc NMIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmifr`] module"]
#[doc(alias = "NMIFR")]
pub type Nmifr = crate::Reg<nmifr::NmifrSpec>;
#[doc = "desc NMIFR"]
pub mod nmifr;
#[doc = "NMICFR (rw) register accessor: desc NMICFR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmicfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmicfr`] module"]
#[doc(alias = "NMICFR")]
pub type Nmicfr = crate::Reg<nmicfr::NmicfrSpec>;
#[doc = "desc NMICFR"]
pub mod nmicfr;
#[doc = "EIRQCR0 (rw) register accessor: desc EIRQCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr0`] module"]
#[doc(alias = "EIRQCR0")]
pub type Eirqcr0 = crate::Reg<eirqcr0::Eirqcr0Spec>;
#[doc = "desc EIRQCR0"]
pub mod eirqcr0;
#[doc = "EIRQCR1 (rw) register accessor: desc EIRQCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr1`] module"]
#[doc(alias = "EIRQCR1")]
pub type Eirqcr1 = crate::Reg<eirqcr1::Eirqcr1Spec>;
#[doc = "desc EIRQCR1"]
pub mod eirqcr1;
#[doc = "EIRQCR2 (rw) register accessor: desc EIRQCR2\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr2`] module"]
#[doc(alias = "EIRQCR2")]
pub type Eirqcr2 = crate::Reg<eirqcr2::Eirqcr2Spec>;
#[doc = "desc EIRQCR2"]
pub mod eirqcr2;
#[doc = "EIRQCR3 (rw) register accessor: desc EIRQCR3\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr3`] module"]
#[doc(alias = "EIRQCR3")]
pub type Eirqcr3 = crate::Reg<eirqcr3::Eirqcr3Spec>;
#[doc = "desc EIRQCR3"]
pub mod eirqcr3;
#[doc = "EIRQCR4 (rw) register accessor: desc EIRQCR4\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr4`] module"]
#[doc(alias = "EIRQCR4")]
pub type Eirqcr4 = crate::Reg<eirqcr4::Eirqcr4Spec>;
#[doc = "desc EIRQCR4"]
pub mod eirqcr4;
#[doc = "EIRQCR5 (rw) register accessor: desc EIRQCR5\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr5`] module"]
#[doc(alias = "EIRQCR5")]
pub type Eirqcr5 = crate::Reg<eirqcr5::Eirqcr5Spec>;
#[doc = "desc EIRQCR5"]
pub mod eirqcr5;
#[doc = "EIRQCR6 (rw) register accessor: desc EIRQCR6\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr6`] module"]
#[doc(alias = "EIRQCR6")]
pub type Eirqcr6 = crate::Reg<eirqcr6::Eirqcr6Spec>;
#[doc = "desc EIRQCR6"]
pub mod eirqcr6;
#[doc = "EIRQCR7 (rw) register accessor: desc EIRQCR7\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr7`] module"]
#[doc(alias = "EIRQCR7")]
pub type Eirqcr7 = crate::Reg<eirqcr7::Eirqcr7Spec>;
#[doc = "desc EIRQCR7"]
pub mod eirqcr7;
#[doc = "EIRQCR8 (rw) register accessor: desc EIRQCR8\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr8`] module"]
#[doc(alias = "EIRQCR8")]
pub type Eirqcr8 = crate::Reg<eirqcr8::Eirqcr8Spec>;
#[doc = "desc EIRQCR8"]
pub mod eirqcr8;
#[doc = "EIRQCR9 (rw) register accessor: desc EIRQCR9\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr9`] module"]
#[doc(alias = "EIRQCR9")]
pub type Eirqcr9 = crate::Reg<eirqcr9::Eirqcr9Spec>;
#[doc = "desc EIRQCR9"]
pub mod eirqcr9;
#[doc = "EIRQCR10 (rw) register accessor: desc EIRQCR10\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr10`] module"]
#[doc(alias = "EIRQCR10")]
pub type Eirqcr10 = crate::Reg<eirqcr10::Eirqcr10Spec>;
#[doc = "desc EIRQCR10"]
pub mod eirqcr10;
#[doc = "EIRQCR11 (rw) register accessor: desc EIRQCR11\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr11`] module"]
#[doc(alias = "EIRQCR11")]
pub type Eirqcr11 = crate::Reg<eirqcr11::Eirqcr11Spec>;
#[doc = "desc EIRQCR11"]
pub mod eirqcr11;
#[doc = "EIRQCR12 (rw) register accessor: desc EIRQCR12\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr12`] module"]
#[doc(alias = "EIRQCR12")]
pub type Eirqcr12 = crate::Reg<eirqcr12::Eirqcr12Spec>;
#[doc = "desc EIRQCR12"]
pub mod eirqcr12;
#[doc = "EIRQCR13 (rw) register accessor: desc EIRQCR13\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr13`] module"]
#[doc(alias = "EIRQCR13")]
pub type Eirqcr13 = crate::Reg<eirqcr13::Eirqcr13Spec>;
#[doc = "desc EIRQCR13"]
pub mod eirqcr13;
#[doc = "EIRQCR14 (rw) register accessor: desc EIRQCR14\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr14`] module"]
#[doc(alias = "EIRQCR14")]
pub type Eirqcr14 = crate::Reg<eirqcr14::Eirqcr14Spec>;
#[doc = "desc EIRQCR14"]
pub mod eirqcr14;
#[doc = "EIRQCR15 (rw) register accessor: desc EIRQCR15\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr15`] module"]
#[doc(alias = "EIRQCR15")]
pub type Eirqcr15 = crate::Reg<eirqcr15::Eirqcr15Spec>;
#[doc = "desc EIRQCR15"]
pub mod eirqcr15;
#[doc = "WUPEN (rw) register accessor: desc WUPEN\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wupen`] module"]
#[doc(alias = "WUPEN")]
pub type Wupen = crate::Reg<wupen::WupenSpec>;
#[doc = "desc WUPEN"]
pub mod wupen;
#[doc = "EIFR (rw) register accessor: desc EIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eifr`] module"]
#[doc(alias = "EIFR")]
pub type Eifr = crate::Reg<eifr::EifrSpec>;
#[doc = "desc EIFR"]
pub mod eifr;
#[doc = "EIFCR (rw) register accessor: desc EIFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eifcr`] module"]
#[doc(alias = "EIFCR")]
pub type Eifcr = crate::Reg<eifcr::EifcrSpec>;
#[doc = "desc EIFCR"]
pub mod eifcr;
#[doc = "SEL0 (rw) register accessor: desc SEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel0`] module"]
#[doc(alias = "SEL0")]
pub type Sel0 = crate::Reg<sel0::Sel0Spec>;
#[doc = "desc SEL0"]
pub mod sel0;
#[doc = "SEL1 (rw) register accessor: desc SEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel1`] module"]
#[doc(alias = "SEL1")]
pub type Sel1 = crate::Reg<sel1::Sel1Spec>;
#[doc = "desc SEL1"]
pub mod sel1;
#[doc = "SEL2 (rw) register accessor: desc SEL2\n\nYou can [`read`](crate::Reg::read) this register and get [`sel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel2`] module"]
#[doc(alias = "SEL2")]
pub type Sel2 = crate::Reg<sel2::Sel2Spec>;
#[doc = "desc SEL2"]
pub mod sel2;
#[doc = "SEL3 (rw) register accessor: desc SEL3\n\nYou can [`read`](crate::Reg::read) this register and get [`sel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel3`] module"]
#[doc(alias = "SEL3")]
pub type Sel3 = crate::Reg<sel3::Sel3Spec>;
#[doc = "desc SEL3"]
pub mod sel3;
#[doc = "SEL4 (rw) register accessor: desc SEL4\n\nYou can [`read`](crate::Reg::read) this register and get [`sel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel4`] module"]
#[doc(alias = "SEL4")]
pub type Sel4 = crate::Reg<sel4::Sel4Spec>;
#[doc = "desc SEL4"]
pub mod sel4;
#[doc = "SEL5 (rw) register accessor: desc SEL5\n\nYou can [`read`](crate::Reg::read) this register and get [`sel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel5`] module"]
#[doc(alias = "SEL5")]
pub type Sel5 = crate::Reg<sel5::Sel5Spec>;
#[doc = "desc SEL5"]
pub mod sel5;
#[doc = "SEL6 (rw) register accessor: desc SEL6\n\nYou can [`read`](crate::Reg::read) this register and get [`sel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel6`] module"]
#[doc(alias = "SEL6")]
pub type Sel6 = crate::Reg<sel6::Sel6Spec>;
#[doc = "desc SEL6"]
pub mod sel6;
#[doc = "SEL7 (rw) register accessor: desc SEL7\n\nYou can [`read`](crate::Reg::read) this register and get [`sel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel7`] module"]
#[doc(alias = "SEL7")]
pub type Sel7 = crate::Reg<sel7::Sel7Spec>;
#[doc = "desc SEL7"]
pub mod sel7;
#[doc = "SEL8 (rw) register accessor: desc SEL8\n\nYou can [`read`](crate::Reg::read) this register and get [`sel8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel8`] module"]
#[doc(alias = "SEL8")]
pub type Sel8 = crate::Reg<sel8::Sel8Spec>;
#[doc = "desc SEL8"]
pub mod sel8;
#[doc = "SEL9 (rw) register accessor: desc SEL9\n\nYou can [`read`](crate::Reg::read) this register and get [`sel9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel9`] module"]
#[doc(alias = "SEL9")]
pub type Sel9 = crate::Reg<sel9::Sel9Spec>;
#[doc = "desc SEL9"]
pub mod sel9;
#[doc = "SEL10 (rw) register accessor: desc SEL10\n\nYou can [`read`](crate::Reg::read) this register and get [`sel10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel10`] module"]
#[doc(alias = "SEL10")]
pub type Sel10 = crate::Reg<sel10::Sel10Spec>;
#[doc = "desc SEL10"]
pub mod sel10;
#[doc = "SEL11 (rw) register accessor: desc SEL11\n\nYou can [`read`](crate::Reg::read) this register and get [`sel11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel11`] module"]
#[doc(alias = "SEL11")]
pub type Sel11 = crate::Reg<sel11::Sel11Spec>;
#[doc = "desc SEL11"]
pub mod sel11;
#[doc = "SEL12 (rw) register accessor: desc SEL12\n\nYou can [`read`](crate::Reg::read) this register and get [`sel12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel12`] module"]
#[doc(alias = "SEL12")]
pub type Sel12 = crate::Reg<sel12::Sel12Spec>;
#[doc = "desc SEL12"]
pub mod sel12;
#[doc = "SEL13 (rw) register accessor: desc SEL13\n\nYou can [`read`](crate::Reg::read) this register and get [`sel13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel13`] module"]
#[doc(alias = "SEL13")]
pub type Sel13 = crate::Reg<sel13::Sel13Spec>;
#[doc = "desc SEL13"]
pub mod sel13;
#[doc = "SEL14 (rw) register accessor: desc SEL14\n\nYou can [`read`](crate::Reg::read) this register and get [`sel14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel14`] module"]
#[doc(alias = "SEL14")]
pub type Sel14 = crate::Reg<sel14::Sel14Spec>;
#[doc = "desc SEL14"]
pub mod sel14;
#[doc = "SEL15 (rw) register accessor: desc SEL15\n\nYou can [`read`](crate::Reg::read) this register and get [`sel15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel15`] module"]
#[doc(alias = "SEL15")]
pub type Sel15 = crate::Reg<sel15::Sel15Spec>;
#[doc = "desc SEL15"]
pub mod sel15;
#[doc = "SEL16 (rw) register accessor: desc SEL16\n\nYou can [`read`](crate::Reg::read) this register and get [`sel16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel16`] module"]
#[doc(alias = "SEL16")]
pub type Sel16 = crate::Reg<sel16::Sel16Spec>;
#[doc = "desc SEL16"]
pub mod sel16;
#[doc = "SEL17 (rw) register accessor: desc SEL17\n\nYou can [`read`](crate::Reg::read) this register and get [`sel17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel17`] module"]
#[doc(alias = "SEL17")]
pub type Sel17 = crate::Reg<sel17::Sel17Spec>;
#[doc = "desc SEL17"]
pub mod sel17;
#[doc = "SEL18 (rw) register accessor: desc SEL18\n\nYou can [`read`](crate::Reg::read) this register and get [`sel18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel18`] module"]
#[doc(alias = "SEL18")]
pub type Sel18 = crate::Reg<sel18::Sel18Spec>;
#[doc = "desc SEL18"]
pub mod sel18;
#[doc = "SEL19 (rw) register accessor: desc SEL19\n\nYou can [`read`](crate::Reg::read) this register and get [`sel19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel19`] module"]
#[doc(alias = "SEL19")]
pub type Sel19 = crate::Reg<sel19::Sel19Spec>;
#[doc = "desc SEL19"]
pub mod sel19;
#[doc = "SEL20 (rw) register accessor: desc SEL20\n\nYou can [`read`](crate::Reg::read) this register and get [`sel20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel20`] module"]
#[doc(alias = "SEL20")]
pub type Sel20 = crate::Reg<sel20::Sel20Spec>;
#[doc = "desc SEL20"]
pub mod sel20;
#[doc = "SEL21 (rw) register accessor: desc SEL21\n\nYou can [`read`](crate::Reg::read) this register and get [`sel21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel21`] module"]
#[doc(alias = "SEL21")]
pub type Sel21 = crate::Reg<sel21::Sel21Spec>;
#[doc = "desc SEL21"]
pub mod sel21;
#[doc = "SEL22 (rw) register accessor: desc SEL22\n\nYou can [`read`](crate::Reg::read) this register and get [`sel22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel22`] module"]
#[doc(alias = "SEL22")]
pub type Sel22 = crate::Reg<sel22::Sel22Spec>;
#[doc = "desc SEL22"]
pub mod sel22;
#[doc = "SEL23 (rw) register accessor: desc SEL23\n\nYou can [`read`](crate::Reg::read) this register and get [`sel23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel23`] module"]
#[doc(alias = "SEL23")]
pub type Sel23 = crate::Reg<sel23::Sel23Spec>;
#[doc = "desc SEL23"]
pub mod sel23;
#[doc = "SEL24 (rw) register accessor: desc SEL24\n\nYou can [`read`](crate::Reg::read) this register and get [`sel24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel24`] module"]
#[doc(alias = "SEL24")]
pub type Sel24 = crate::Reg<sel24::Sel24Spec>;
#[doc = "desc SEL24"]
pub mod sel24;
#[doc = "SEL25 (rw) register accessor: desc SEL25\n\nYou can [`read`](crate::Reg::read) this register and get [`sel25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel25`] module"]
#[doc(alias = "SEL25")]
pub type Sel25 = crate::Reg<sel25::Sel25Spec>;
#[doc = "desc SEL25"]
pub mod sel25;
#[doc = "SEL26 (rw) register accessor: desc SEL26\n\nYou can [`read`](crate::Reg::read) this register and get [`sel26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel26`] module"]
#[doc(alias = "SEL26")]
pub type Sel26 = crate::Reg<sel26::Sel26Spec>;
#[doc = "desc SEL26"]
pub mod sel26;
#[doc = "SEL27 (rw) register accessor: desc SEL27\n\nYou can [`read`](crate::Reg::read) this register and get [`sel27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel27`] module"]
#[doc(alias = "SEL27")]
pub type Sel27 = crate::Reg<sel27::Sel27Spec>;
#[doc = "desc SEL27"]
pub mod sel27;
#[doc = "SEL28 (rw) register accessor: desc SEL28\n\nYou can [`read`](crate::Reg::read) this register and get [`sel28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel28`] module"]
#[doc(alias = "SEL28")]
pub type Sel28 = crate::Reg<sel28::Sel28Spec>;
#[doc = "desc SEL28"]
pub mod sel28;
#[doc = "SEL29 (rw) register accessor: desc SEL29\n\nYou can [`read`](crate::Reg::read) this register and get [`sel29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel29`] module"]
#[doc(alias = "SEL29")]
pub type Sel29 = crate::Reg<sel29::Sel29Spec>;
#[doc = "desc SEL29"]
pub mod sel29;
#[doc = "SEL30 (rw) register accessor: desc SEL30\n\nYou can [`read`](crate::Reg::read) this register and get [`sel30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel30`] module"]
#[doc(alias = "SEL30")]
pub type Sel30 = crate::Reg<sel30::Sel30Spec>;
#[doc = "desc SEL30"]
pub mod sel30;
#[doc = "SEL31 (rw) register accessor: desc SEL31\n\nYou can [`read`](crate::Reg::read) this register and get [`sel31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel31`] module"]
#[doc(alias = "SEL31")]
pub type Sel31 = crate::Reg<sel31::Sel31Spec>;
#[doc = "desc SEL31"]
pub mod sel31;
#[doc = "SEL32 (rw) register accessor: desc SEL32\n\nYou can [`read`](crate::Reg::read) this register and get [`sel32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel32`] module"]
#[doc(alias = "SEL32")]
pub type Sel32 = crate::Reg<sel32::Sel32Spec>;
#[doc = "desc SEL32"]
pub mod sel32;
#[doc = "SEL33 (rw) register accessor: desc SEL33\n\nYou can [`read`](crate::Reg::read) this register and get [`sel33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel33`] module"]
#[doc(alias = "SEL33")]
pub type Sel33 = crate::Reg<sel33::Sel33Spec>;
#[doc = "desc SEL33"]
pub mod sel33;
#[doc = "SEL34 (rw) register accessor: desc SEL34\n\nYou can [`read`](crate::Reg::read) this register and get [`sel34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel34`] module"]
#[doc(alias = "SEL34")]
pub type Sel34 = crate::Reg<sel34::Sel34Spec>;
#[doc = "desc SEL34"]
pub mod sel34;
#[doc = "SEL35 (rw) register accessor: desc SEL35\n\nYou can [`read`](crate::Reg::read) this register and get [`sel35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel35`] module"]
#[doc(alias = "SEL35")]
pub type Sel35 = crate::Reg<sel35::Sel35Spec>;
#[doc = "desc SEL35"]
pub mod sel35;
#[doc = "SEL36 (rw) register accessor: desc SEL36\n\nYou can [`read`](crate::Reg::read) this register and get [`sel36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel36`] module"]
#[doc(alias = "SEL36")]
pub type Sel36 = crate::Reg<sel36::Sel36Spec>;
#[doc = "desc SEL36"]
pub mod sel36;
#[doc = "SEL37 (rw) register accessor: desc SEL37\n\nYou can [`read`](crate::Reg::read) this register and get [`sel37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel37`] module"]
#[doc(alias = "SEL37")]
pub type Sel37 = crate::Reg<sel37::Sel37Spec>;
#[doc = "desc SEL37"]
pub mod sel37;
#[doc = "SEL38 (rw) register accessor: desc SEL38\n\nYou can [`read`](crate::Reg::read) this register and get [`sel38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel38`] module"]
#[doc(alias = "SEL38")]
pub type Sel38 = crate::Reg<sel38::Sel38Spec>;
#[doc = "desc SEL38"]
pub mod sel38;
#[doc = "SEL39 (rw) register accessor: desc SEL39\n\nYou can [`read`](crate::Reg::read) this register and get [`sel39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel39`] module"]
#[doc(alias = "SEL39")]
pub type Sel39 = crate::Reg<sel39::Sel39Spec>;
#[doc = "desc SEL39"]
pub mod sel39;
#[doc = "SEL40 (rw) register accessor: desc SEL40\n\nYou can [`read`](crate::Reg::read) this register and get [`sel40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel40`] module"]
#[doc(alias = "SEL40")]
pub type Sel40 = crate::Reg<sel40::Sel40Spec>;
#[doc = "desc SEL40"]
pub mod sel40;
#[doc = "SEL41 (rw) register accessor: desc SEL41\n\nYou can [`read`](crate::Reg::read) this register and get [`sel41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel41`] module"]
#[doc(alias = "SEL41")]
pub type Sel41 = crate::Reg<sel41::Sel41Spec>;
#[doc = "desc SEL41"]
pub mod sel41;
#[doc = "SEL42 (rw) register accessor: desc SEL42\n\nYou can [`read`](crate::Reg::read) this register and get [`sel42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel42`] module"]
#[doc(alias = "SEL42")]
pub type Sel42 = crate::Reg<sel42::Sel42Spec>;
#[doc = "desc SEL42"]
pub mod sel42;
#[doc = "SEL43 (rw) register accessor: desc SEL43\n\nYou can [`read`](crate::Reg::read) this register and get [`sel43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel43`] module"]
#[doc(alias = "SEL43")]
pub type Sel43 = crate::Reg<sel43::Sel43Spec>;
#[doc = "desc SEL43"]
pub mod sel43;
#[doc = "SEL44 (rw) register accessor: desc SEL44\n\nYou can [`read`](crate::Reg::read) this register and get [`sel44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel44`] module"]
#[doc(alias = "SEL44")]
pub type Sel44 = crate::Reg<sel44::Sel44Spec>;
#[doc = "desc SEL44"]
pub mod sel44;
#[doc = "SEL45 (rw) register accessor: desc SEL45\n\nYou can [`read`](crate::Reg::read) this register and get [`sel45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel45`] module"]
#[doc(alias = "SEL45")]
pub type Sel45 = crate::Reg<sel45::Sel45Spec>;
#[doc = "desc SEL45"]
pub mod sel45;
#[doc = "SEL46 (rw) register accessor: desc SEL46\n\nYou can [`read`](crate::Reg::read) this register and get [`sel46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel46`] module"]
#[doc(alias = "SEL46")]
pub type Sel46 = crate::Reg<sel46::Sel46Spec>;
#[doc = "desc SEL46"]
pub mod sel46;
#[doc = "SEL47 (rw) register accessor: desc SEL47\n\nYou can [`read`](crate::Reg::read) this register and get [`sel47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel47`] module"]
#[doc(alias = "SEL47")]
pub type Sel47 = crate::Reg<sel47::Sel47Spec>;
#[doc = "desc SEL47"]
pub mod sel47;
#[doc = "SEL48 (rw) register accessor: desc SEL48\n\nYou can [`read`](crate::Reg::read) this register and get [`sel48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel48`] module"]
#[doc(alias = "SEL48")]
pub type Sel48 = crate::Reg<sel48::Sel48Spec>;
#[doc = "desc SEL48"]
pub mod sel48;
#[doc = "SEL49 (rw) register accessor: desc SEL49\n\nYou can [`read`](crate::Reg::read) this register and get [`sel49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel49`] module"]
#[doc(alias = "SEL49")]
pub type Sel49 = crate::Reg<sel49::Sel49Spec>;
#[doc = "desc SEL49"]
pub mod sel49;
#[doc = "SEL50 (rw) register accessor: desc SEL50\n\nYou can [`read`](crate::Reg::read) this register and get [`sel50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel50`] module"]
#[doc(alias = "SEL50")]
pub type Sel50 = crate::Reg<sel50::Sel50Spec>;
#[doc = "desc SEL50"]
pub mod sel50;
#[doc = "SEL51 (rw) register accessor: desc SEL51\n\nYou can [`read`](crate::Reg::read) this register and get [`sel51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel51`] module"]
#[doc(alias = "SEL51")]
pub type Sel51 = crate::Reg<sel51::Sel51Spec>;
#[doc = "desc SEL51"]
pub mod sel51;
#[doc = "SEL52 (rw) register accessor: desc SEL52\n\nYou can [`read`](crate::Reg::read) this register and get [`sel52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel52`] module"]
#[doc(alias = "SEL52")]
pub type Sel52 = crate::Reg<sel52::Sel52Spec>;
#[doc = "desc SEL52"]
pub mod sel52;
#[doc = "SEL53 (rw) register accessor: desc SEL53\n\nYou can [`read`](crate::Reg::read) this register and get [`sel53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel53`] module"]
#[doc(alias = "SEL53")]
pub type Sel53 = crate::Reg<sel53::Sel53Spec>;
#[doc = "desc SEL53"]
pub mod sel53;
#[doc = "SEL54 (rw) register accessor: desc SEL54\n\nYou can [`read`](crate::Reg::read) this register and get [`sel54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel54`] module"]
#[doc(alias = "SEL54")]
pub type Sel54 = crate::Reg<sel54::Sel54Spec>;
#[doc = "desc SEL54"]
pub mod sel54;
#[doc = "SEL55 (rw) register accessor: desc SEL55\n\nYou can [`read`](crate::Reg::read) this register and get [`sel55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel55`] module"]
#[doc(alias = "SEL55")]
pub type Sel55 = crate::Reg<sel55::Sel55Spec>;
#[doc = "desc SEL55"]
pub mod sel55;
#[doc = "SEL56 (rw) register accessor: desc SEL56\n\nYou can [`read`](crate::Reg::read) this register and get [`sel56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel56`] module"]
#[doc(alias = "SEL56")]
pub type Sel56 = crate::Reg<sel56::Sel56Spec>;
#[doc = "desc SEL56"]
pub mod sel56;
#[doc = "SEL57 (rw) register accessor: desc SEL57\n\nYou can [`read`](crate::Reg::read) this register and get [`sel57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel57`] module"]
#[doc(alias = "SEL57")]
pub type Sel57 = crate::Reg<sel57::Sel57Spec>;
#[doc = "desc SEL57"]
pub mod sel57;
#[doc = "SEL58 (rw) register accessor: desc SEL58\n\nYou can [`read`](crate::Reg::read) this register and get [`sel58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel58`] module"]
#[doc(alias = "SEL58")]
pub type Sel58 = crate::Reg<sel58::Sel58Spec>;
#[doc = "desc SEL58"]
pub mod sel58;
#[doc = "SEL59 (rw) register accessor: desc SEL59\n\nYou can [`read`](crate::Reg::read) this register and get [`sel59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel59`] module"]
#[doc(alias = "SEL59")]
pub type Sel59 = crate::Reg<sel59::Sel59Spec>;
#[doc = "desc SEL59"]
pub mod sel59;
#[doc = "SEL60 (rw) register accessor: desc SEL60\n\nYou can [`read`](crate::Reg::read) this register and get [`sel60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel60`] module"]
#[doc(alias = "SEL60")]
pub type Sel60 = crate::Reg<sel60::Sel60Spec>;
#[doc = "desc SEL60"]
pub mod sel60;
#[doc = "SEL61 (rw) register accessor: desc SEL61\n\nYou can [`read`](crate::Reg::read) this register and get [`sel61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel61`] module"]
#[doc(alias = "SEL61")]
pub type Sel61 = crate::Reg<sel61::Sel61Spec>;
#[doc = "desc SEL61"]
pub mod sel61;
#[doc = "SEL62 (rw) register accessor: desc SEL62\n\nYou can [`read`](crate::Reg::read) this register and get [`sel62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel62`] module"]
#[doc(alias = "SEL62")]
pub type Sel62 = crate::Reg<sel62::Sel62Spec>;
#[doc = "desc SEL62"]
pub mod sel62;
#[doc = "SEL63 (rw) register accessor: desc SEL63\n\nYou can [`read`](crate::Reg::read) this register and get [`sel63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel63`] module"]
#[doc(alias = "SEL63")]
pub type Sel63 = crate::Reg<sel63::Sel63Spec>;
#[doc = "desc SEL63"]
pub mod sel63;
#[doc = "SEL64 (rw) register accessor: desc SEL64\n\nYou can [`read`](crate::Reg::read) this register and get [`sel64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel64`] module"]
#[doc(alias = "SEL64")]
pub type Sel64 = crate::Reg<sel64::Sel64Spec>;
#[doc = "desc SEL64"]
pub mod sel64;
#[doc = "SEL65 (rw) register accessor: desc SEL65\n\nYou can [`read`](crate::Reg::read) this register and get [`sel65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel65`] module"]
#[doc(alias = "SEL65")]
pub type Sel65 = crate::Reg<sel65::Sel65Spec>;
#[doc = "desc SEL65"]
pub mod sel65;
#[doc = "SEL66 (rw) register accessor: desc SEL66\n\nYou can [`read`](crate::Reg::read) this register and get [`sel66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel66`] module"]
#[doc(alias = "SEL66")]
pub type Sel66 = crate::Reg<sel66::Sel66Spec>;
#[doc = "desc SEL66"]
pub mod sel66;
#[doc = "SEL67 (rw) register accessor: desc SEL67\n\nYou can [`read`](crate::Reg::read) this register and get [`sel67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel67`] module"]
#[doc(alias = "SEL67")]
pub type Sel67 = crate::Reg<sel67::Sel67Spec>;
#[doc = "desc SEL67"]
pub mod sel67;
#[doc = "SEL68 (rw) register accessor: desc SEL68\n\nYou can [`read`](crate::Reg::read) this register and get [`sel68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel68`] module"]
#[doc(alias = "SEL68")]
pub type Sel68 = crate::Reg<sel68::Sel68Spec>;
#[doc = "desc SEL68"]
pub mod sel68;
#[doc = "SEL69 (rw) register accessor: desc SEL69\n\nYou can [`read`](crate::Reg::read) this register and get [`sel69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel69`] module"]
#[doc(alias = "SEL69")]
pub type Sel69 = crate::Reg<sel69::Sel69Spec>;
#[doc = "desc SEL69"]
pub mod sel69;
#[doc = "SEL70 (rw) register accessor: desc SEL70\n\nYou can [`read`](crate::Reg::read) this register and get [`sel70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel70`] module"]
#[doc(alias = "SEL70")]
pub type Sel70 = crate::Reg<sel70::Sel70Spec>;
#[doc = "desc SEL70"]
pub mod sel70;
#[doc = "SEL71 (rw) register accessor: desc SEL71\n\nYou can [`read`](crate::Reg::read) this register and get [`sel71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel71`] module"]
#[doc(alias = "SEL71")]
pub type Sel71 = crate::Reg<sel71::Sel71Spec>;
#[doc = "desc SEL71"]
pub mod sel71;
#[doc = "SEL72 (rw) register accessor: desc SEL72\n\nYou can [`read`](crate::Reg::read) this register and get [`sel72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel72`] module"]
#[doc(alias = "SEL72")]
pub type Sel72 = crate::Reg<sel72::Sel72Spec>;
#[doc = "desc SEL72"]
pub mod sel72;
#[doc = "SEL73 (rw) register accessor: desc SEL73\n\nYou can [`read`](crate::Reg::read) this register and get [`sel73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel73`] module"]
#[doc(alias = "SEL73")]
pub type Sel73 = crate::Reg<sel73::Sel73Spec>;
#[doc = "desc SEL73"]
pub mod sel73;
#[doc = "SEL74 (rw) register accessor: desc SEL74\n\nYou can [`read`](crate::Reg::read) this register and get [`sel74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel74`] module"]
#[doc(alias = "SEL74")]
pub type Sel74 = crate::Reg<sel74::Sel74Spec>;
#[doc = "desc SEL74"]
pub mod sel74;
#[doc = "SEL75 (rw) register accessor: desc SEL75\n\nYou can [`read`](crate::Reg::read) this register and get [`sel75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel75`] module"]
#[doc(alias = "SEL75")]
pub type Sel75 = crate::Reg<sel75::Sel75Spec>;
#[doc = "desc SEL75"]
pub mod sel75;
#[doc = "SEL76 (rw) register accessor: desc SEL76\n\nYou can [`read`](crate::Reg::read) this register and get [`sel76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel76`] module"]
#[doc(alias = "SEL76")]
pub type Sel76 = crate::Reg<sel76::Sel76Spec>;
#[doc = "desc SEL76"]
pub mod sel76;
#[doc = "SEL77 (rw) register accessor: desc SEL77\n\nYou can [`read`](crate::Reg::read) this register and get [`sel77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel77`] module"]
#[doc(alias = "SEL77")]
pub type Sel77 = crate::Reg<sel77::Sel77Spec>;
#[doc = "desc SEL77"]
pub mod sel77;
#[doc = "SEL78 (rw) register accessor: desc SEL78\n\nYou can [`read`](crate::Reg::read) this register and get [`sel78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel78`] module"]
#[doc(alias = "SEL78")]
pub type Sel78 = crate::Reg<sel78::Sel78Spec>;
#[doc = "desc SEL78"]
pub mod sel78;
#[doc = "SEL79 (rw) register accessor: desc SEL79\n\nYou can [`read`](crate::Reg::read) this register and get [`sel79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel79`] module"]
#[doc(alias = "SEL79")]
pub type Sel79 = crate::Reg<sel79::Sel79Spec>;
#[doc = "desc SEL79"]
pub mod sel79;
#[doc = "SEL80 (rw) register accessor: desc SEL80\n\nYou can [`read`](crate::Reg::read) this register and get [`sel80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel80`] module"]
#[doc(alias = "SEL80")]
pub type Sel80 = crate::Reg<sel80::Sel80Spec>;
#[doc = "desc SEL80"]
pub mod sel80;
#[doc = "SEL81 (rw) register accessor: desc SEL81\n\nYou can [`read`](crate::Reg::read) this register and get [`sel81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel81`] module"]
#[doc(alias = "SEL81")]
pub type Sel81 = crate::Reg<sel81::Sel81Spec>;
#[doc = "desc SEL81"]
pub mod sel81;
#[doc = "SEL82 (rw) register accessor: desc SEL82\n\nYou can [`read`](crate::Reg::read) this register and get [`sel82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel82`] module"]
#[doc(alias = "SEL82")]
pub type Sel82 = crate::Reg<sel82::Sel82Spec>;
#[doc = "desc SEL82"]
pub mod sel82;
#[doc = "SEL83 (rw) register accessor: desc SEL83\n\nYou can [`read`](crate::Reg::read) this register and get [`sel83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel83`] module"]
#[doc(alias = "SEL83")]
pub type Sel83 = crate::Reg<sel83::Sel83Spec>;
#[doc = "desc SEL83"]
pub mod sel83;
#[doc = "SEL84 (rw) register accessor: desc SEL84\n\nYou can [`read`](crate::Reg::read) this register and get [`sel84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel84`] module"]
#[doc(alias = "SEL84")]
pub type Sel84 = crate::Reg<sel84::Sel84Spec>;
#[doc = "desc SEL84"]
pub mod sel84;
#[doc = "SEL85 (rw) register accessor: desc SEL85\n\nYou can [`read`](crate::Reg::read) this register and get [`sel85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel85`] module"]
#[doc(alias = "SEL85")]
pub type Sel85 = crate::Reg<sel85::Sel85Spec>;
#[doc = "desc SEL85"]
pub mod sel85;
#[doc = "SEL86 (rw) register accessor: desc SEL86\n\nYou can [`read`](crate::Reg::read) this register and get [`sel86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel86`] module"]
#[doc(alias = "SEL86")]
pub type Sel86 = crate::Reg<sel86::Sel86Spec>;
#[doc = "desc SEL86"]
pub mod sel86;
#[doc = "SEL87 (rw) register accessor: desc SEL87\n\nYou can [`read`](crate::Reg::read) this register and get [`sel87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel87`] module"]
#[doc(alias = "SEL87")]
pub type Sel87 = crate::Reg<sel87::Sel87Spec>;
#[doc = "desc SEL87"]
pub mod sel87;
#[doc = "SEL88 (rw) register accessor: desc SEL88\n\nYou can [`read`](crate::Reg::read) this register and get [`sel88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel88`] module"]
#[doc(alias = "SEL88")]
pub type Sel88 = crate::Reg<sel88::Sel88Spec>;
#[doc = "desc SEL88"]
pub mod sel88;
#[doc = "SEL89 (rw) register accessor: desc SEL89\n\nYou can [`read`](crate::Reg::read) this register and get [`sel89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel89`] module"]
#[doc(alias = "SEL89")]
pub type Sel89 = crate::Reg<sel89::Sel89Spec>;
#[doc = "desc SEL89"]
pub mod sel89;
#[doc = "SEL90 (rw) register accessor: desc SEL90\n\nYou can [`read`](crate::Reg::read) this register and get [`sel90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel90`] module"]
#[doc(alias = "SEL90")]
pub type Sel90 = crate::Reg<sel90::Sel90Spec>;
#[doc = "desc SEL90"]
pub mod sel90;
#[doc = "SEL91 (rw) register accessor: desc SEL91\n\nYou can [`read`](crate::Reg::read) this register and get [`sel91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel91`] module"]
#[doc(alias = "SEL91")]
pub type Sel91 = crate::Reg<sel91::Sel91Spec>;
#[doc = "desc SEL91"]
pub mod sel91;
#[doc = "SEL92 (rw) register accessor: desc SEL92\n\nYou can [`read`](crate::Reg::read) this register and get [`sel92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel92`] module"]
#[doc(alias = "SEL92")]
pub type Sel92 = crate::Reg<sel92::Sel92Spec>;
#[doc = "desc SEL92"]
pub mod sel92;
#[doc = "SEL93 (rw) register accessor: desc SEL93\n\nYou can [`read`](crate::Reg::read) this register and get [`sel93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel93`] module"]
#[doc(alias = "SEL93")]
pub type Sel93 = crate::Reg<sel93::Sel93Spec>;
#[doc = "desc SEL93"]
pub mod sel93;
#[doc = "SEL94 (rw) register accessor: desc SEL94\n\nYou can [`read`](crate::Reg::read) this register and get [`sel94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel94`] module"]
#[doc(alias = "SEL94")]
pub type Sel94 = crate::Reg<sel94::Sel94Spec>;
#[doc = "desc SEL94"]
pub mod sel94;
#[doc = "SEL95 (rw) register accessor: desc SEL95\n\nYou can [`read`](crate::Reg::read) this register and get [`sel95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel95`] module"]
#[doc(alias = "SEL95")]
pub type Sel95 = crate::Reg<sel95::Sel95Spec>;
#[doc = "desc SEL95"]
pub mod sel95;
#[doc = "SEL96 (rw) register accessor: desc SEL96\n\nYou can [`read`](crate::Reg::read) this register and get [`sel96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel96`] module"]
#[doc(alias = "SEL96")]
pub type Sel96 = crate::Reg<sel96::Sel96Spec>;
#[doc = "desc SEL96"]
pub mod sel96;
#[doc = "SEL97 (rw) register accessor: desc SEL97\n\nYou can [`read`](crate::Reg::read) this register and get [`sel97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel97`] module"]
#[doc(alias = "SEL97")]
pub type Sel97 = crate::Reg<sel97::Sel97Spec>;
#[doc = "desc SEL97"]
pub mod sel97;
#[doc = "SEL98 (rw) register accessor: desc SEL98\n\nYou can [`read`](crate::Reg::read) this register and get [`sel98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel98`] module"]
#[doc(alias = "SEL98")]
pub type Sel98 = crate::Reg<sel98::Sel98Spec>;
#[doc = "desc SEL98"]
pub mod sel98;
#[doc = "SEL99 (rw) register accessor: desc SEL99\n\nYou can [`read`](crate::Reg::read) this register and get [`sel99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel99`] module"]
#[doc(alias = "SEL99")]
pub type Sel99 = crate::Reg<sel99::Sel99Spec>;
#[doc = "desc SEL99"]
pub mod sel99;
#[doc = "SEL100 (rw) register accessor: desc SEL100\n\nYou can [`read`](crate::Reg::read) this register and get [`sel100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel100`] module"]
#[doc(alias = "SEL100")]
pub type Sel100 = crate::Reg<sel100::Sel100Spec>;
#[doc = "desc SEL100"]
pub mod sel100;
#[doc = "SEL101 (rw) register accessor: desc SEL101\n\nYou can [`read`](crate::Reg::read) this register and get [`sel101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel101`] module"]
#[doc(alias = "SEL101")]
pub type Sel101 = crate::Reg<sel101::Sel101Spec>;
#[doc = "desc SEL101"]
pub mod sel101;
#[doc = "SEL102 (rw) register accessor: desc SEL102\n\nYou can [`read`](crate::Reg::read) this register and get [`sel102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel102`] module"]
#[doc(alias = "SEL102")]
pub type Sel102 = crate::Reg<sel102::Sel102Spec>;
#[doc = "desc SEL102"]
pub mod sel102;
#[doc = "SEL103 (rw) register accessor: desc SEL103\n\nYou can [`read`](crate::Reg::read) this register and get [`sel103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel103`] module"]
#[doc(alias = "SEL103")]
pub type Sel103 = crate::Reg<sel103::Sel103Spec>;
#[doc = "desc SEL103"]
pub mod sel103;
#[doc = "SEL104 (rw) register accessor: desc SEL104\n\nYou can [`read`](crate::Reg::read) this register and get [`sel104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel104`] module"]
#[doc(alias = "SEL104")]
pub type Sel104 = crate::Reg<sel104::Sel104Spec>;
#[doc = "desc SEL104"]
pub mod sel104;
#[doc = "SEL105 (rw) register accessor: desc SEL105\n\nYou can [`read`](crate::Reg::read) this register and get [`sel105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel105`] module"]
#[doc(alias = "SEL105")]
pub type Sel105 = crate::Reg<sel105::Sel105Spec>;
#[doc = "desc SEL105"]
pub mod sel105;
#[doc = "SEL106 (rw) register accessor: desc SEL106\n\nYou can [`read`](crate::Reg::read) this register and get [`sel106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel106`] module"]
#[doc(alias = "SEL106")]
pub type Sel106 = crate::Reg<sel106::Sel106Spec>;
#[doc = "desc SEL106"]
pub mod sel106;
#[doc = "SEL107 (rw) register accessor: desc SEL107\n\nYou can [`read`](crate::Reg::read) this register and get [`sel107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel107`] module"]
#[doc(alias = "SEL107")]
pub type Sel107 = crate::Reg<sel107::Sel107Spec>;
#[doc = "desc SEL107"]
pub mod sel107;
#[doc = "SEL108 (rw) register accessor: desc SEL108\n\nYou can [`read`](crate::Reg::read) this register and get [`sel108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel108`] module"]
#[doc(alias = "SEL108")]
pub type Sel108 = crate::Reg<sel108::Sel108Spec>;
#[doc = "desc SEL108"]
pub mod sel108;
#[doc = "SEL109 (rw) register accessor: desc SEL109\n\nYou can [`read`](crate::Reg::read) this register and get [`sel109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel109`] module"]
#[doc(alias = "SEL109")]
pub type Sel109 = crate::Reg<sel109::Sel109Spec>;
#[doc = "desc SEL109"]
pub mod sel109;
#[doc = "SEL110 (rw) register accessor: desc SEL110\n\nYou can [`read`](crate::Reg::read) this register and get [`sel110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel110`] module"]
#[doc(alias = "SEL110")]
pub type Sel110 = crate::Reg<sel110::Sel110Spec>;
#[doc = "desc SEL110"]
pub mod sel110;
#[doc = "SEL111 (rw) register accessor: desc SEL111\n\nYou can [`read`](crate::Reg::read) this register and get [`sel111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel111`] module"]
#[doc(alias = "SEL111")]
pub type Sel111 = crate::Reg<sel111::Sel111Spec>;
#[doc = "desc SEL111"]
pub mod sel111;
#[doc = "SEL112 (rw) register accessor: desc SEL112\n\nYou can [`read`](crate::Reg::read) this register and get [`sel112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel112`] module"]
#[doc(alias = "SEL112")]
pub type Sel112 = crate::Reg<sel112::Sel112Spec>;
#[doc = "desc SEL112"]
pub mod sel112;
#[doc = "SEL113 (rw) register accessor: desc SEL113\n\nYou can [`read`](crate::Reg::read) this register and get [`sel113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel113`] module"]
#[doc(alias = "SEL113")]
pub type Sel113 = crate::Reg<sel113::Sel113Spec>;
#[doc = "desc SEL113"]
pub mod sel113;
#[doc = "SEL114 (rw) register accessor: desc SEL114\n\nYou can [`read`](crate::Reg::read) this register and get [`sel114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel114`] module"]
#[doc(alias = "SEL114")]
pub type Sel114 = crate::Reg<sel114::Sel114Spec>;
#[doc = "desc SEL114"]
pub mod sel114;
#[doc = "SEL115 (rw) register accessor: desc SEL115\n\nYou can [`read`](crate::Reg::read) this register and get [`sel115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel115`] module"]
#[doc(alias = "SEL115")]
pub type Sel115 = crate::Reg<sel115::Sel115Spec>;
#[doc = "desc SEL115"]
pub mod sel115;
#[doc = "SEL116 (rw) register accessor: desc SEL116\n\nYou can [`read`](crate::Reg::read) this register and get [`sel116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel116`] module"]
#[doc(alias = "SEL116")]
pub type Sel116 = crate::Reg<sel116::Sel116Spec>;
#[doc = "desc SEL116"]
pub mod sel116;
#[doc = "SEL117 (rw) register accessor: desc SEL117\n\nYou can [`read`](crate::Reg::read) this register and get [`sel117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel117`] module"]
#[doc(alias = "SEL117")]
pub type Sel117 = crate::Reg<sel117::Sel117Spec>;
#[doc = "desc SEL117"]
pub mod sel117;
#[doc = "SEL118 (rw) register accessor: desc SEL118\n\nYou can [`read`](crate::Reg::read) this register and get [`sel118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel118`] module"]
#[doc(alias = "SEL118")]
pub type Sel118 = crate::Reg<sel118::Sel118Spec>;
#[doc = "desc SEL118"]
pub mod sel118;
#[doc = "SEL119 (rw) register accessor: desc SEL119\n\nYou can [`read`](crate::Reg::read) this register and get [`sel119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel119`] module"]
#[doc(alias = "SEL119")]
pub type Sel119 = crate::Reg<sel119::Sel119Spec>;
#[doc = "desc SEL119"]
pub mod sel119;
#[doc = "SEL120 (rw) register accessor: desc SEL120\n\nYou can [`read`](crate::Reg::read) this register and get [`sel120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel120`] module"]
#[doc(alias = "SEL120")]
pub type Sel120 = crate::Reg<sel120::Sel120Spec>;
#[doc = "desc SEL120"]
pub mod sel120;
#[doc = "SEL121 (rw) register accessor: desc SEL121\n\nYou can [`read`](crate::Reg::read) this register and get [`sel121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel121`] module"]
#[doc(alias = "SEL121")]
pub type Sel121 = crate::Reg<sel121::Sel121Spec>;
#[doc = "desc SEL121"]
pub mod sel121;
#[doc = "SEL122 (rw) register accessor: desc SEL122\n\nYou can [`read`](crate::Reg::read) this register and get [`sel122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel122`] module"]
#[doc(alias = "SEL122")]
pub type Sel122 = crate::Reg<sel122::Sel122Spec>;
#[doc = "desc SEL122"]
pub mod sel122;
#[doc = "SEL123 (rw) register accessor: desc SEL123\n\nYou can [`read`](crate::Reg::read) this register and get [`sel123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel123`] module"]
#[doc(alias = "SEL123")]
pub type Sel123 = crate::Reg<sel123::Sel123Spec>;
#[doc = "desc SEL123"]
pub mod sel123;
#[doc = "SEL124 (rw) register accessor: desc SEL124\n\nYou can [`read`](crate::Reg::read) this register and get [`sel124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel124`] module"]
#[doc(alias = "SEL124")]
pub type Sel124 = crate::Reg<sel124::Sel124Spec>;
#[doc = "desc SEL124"]
pub mod sel124;
#[doc = "SEL125 (rw) register accessor: desc SEL125\n\nYou can [`read`](crate::Reg::read) this register and get [`sel125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel125`] module"]
#[doc(alias = "SEL125")]
pub type Sel125 = crate::Reg<sel125::Sel125Spec>;
#[doc = "desc SEL125"]
pub mod sel125;
#[doc = "SEL126 (rw) register accessor: desc SEL126\n\nYou can [`read`](crate::Reg::read) this register and get [`sel126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel126`] module"]
#[doc(alias = "SEL126")]
pub type Sel126 = crate::Reg<sel126::Sel126Spec>;
#[doc = "desc SEL126"]
pub mod sel126;
#[doc = "SEL127 (rw) register accessor: desc SEL127\n\nYou can [`read`](crate::Reg::read) this register and get [`sel127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel127`] module"]
#[doc(alias = "SEL127")]
pub type Sel127 = crate::Reg<sel127::Sel127Spec>;
#[doc = "desc SEL127"]
pub mod sel127;
#[doc = "VSSEL128 (rw) register accessor: desc VSSEL128\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel128`] module"]
#[doc(alias = "VSSEL128")]
pub type Vssel128 = crate::Reg<vssel128::Vssel128Spec>;
#[doc = "desc VSSEL128"]
pub mod vssel128;
#[doc = "VSSEL129 (rw) register accessor: desc VSSEL129\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel129`] module"]
#[doc(alias = "VSSEL129")]
pub type Vssel129 = crate::Reg<vssel129::Vssel129Spec>;
#[doc = "desc VSSEL129"]
pub mod vssel129;
#[doc = "VSSEL130 (rw) register accessor: desc VSSEL130\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel130`] module"]
#[doc(alias = "VSSEL130")]
pub type Vssel130 = crate::Reg<vssel130::Vssel130Spec>;
#[doc = "desc VSSEL130"]
pub mod vssel130;
#[doc = "VSSEL131 (rw) register accessor: desc VSSEL131\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel131`] module"]
#[doc(alias = "VSSEL131")]
pub type Vssel131 = crate::Reg<vssel131::Vssel131Spec>;
#[doc = "desc VSSEL131"]
pub mod vssel131;
#[doc = "VSSEL132 (rw) register accessor: desc VSSEL132\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel132::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel132::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel132`] module"]
#[doc(alias = "VSSEL132")]
pub type Vssel132 = crate::Reg<vssel132::Vssel132Spec>;
#[doc = "desc VSSEL132"]
pub mod vssel132;
#[doc = "VSSEL133 (rw) register accessor: desc VSSEL133\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel133::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel133::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel133`] module"]
#[doc(alias = "VSSEL133")]
pub type Vssel133 = crate::Reg<vssel133::Vssel133Spec>;
#[doc = "desc VSSEL133"]
pub mod vssel133;
#[doc = "VSSEL134 (rw) register accessor: desc VSSEL134\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel134`] module"]
#[doc(alias = "VSSEL134")]
pub type Vssel134 = crate::Reg<vssel134::Vssel134Spec>;
#[doc = "desc VSSEL134"]
pub mod vssel134;
#[doc = "VSSEL135 (rw) register accessor: desc VSSEL135\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel135::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel135::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel135`] module"]
#[doc(alias = "VSSEL135")]
pub type Vssel135 = crate::Reg<vssel135::Vssel135Spec>;
#[doc = "desc VSSEL135"]
pub mod vssel135;
#[doc = "VSSEL136 (rw) register accessor: desc VSSEL136\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel136::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel136::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel136`] module"]
#[doc(alias = "VSSEL136")]
pub type Vssel136 = crate::Reg<vssel136::Vssel136Spec>;
#[doc = "desc VSSEL136"]
pub mod vssel136;
#[doc = "VSSEL137 (rw) register accessor: desc VSSEL137\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel137::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel137::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel137`] module"]
#[doc(alias = "VSSEL137")]
pub type Vssel137 = crate::Reg<vssel137::Vssel137Spec>;
#[doc = "desc VSSEL137"]
pub mod vssel137;
#[doc = "VSSEL138 (rw) register accessor: desc VSSEL138\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel138`] module"]
#[doc(alias = "VSSEL138")]
pub type Vssel138 = crate::Reg<vssel138::Vssel138Spec>;
#[doc = "desc VSSEL138"]
pub mod vssel138;
#[doc = "VSSEL139 (rw) register accessor: desc VSSEL139\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel139::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel139::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel139`] module"]
#[doc(alias = "VSSEL139")]
pub type Vssel139 = crate::Reg<vssel139::Vssel139Spec>;
#[doc = "desc VSSEL139"]
pub mod vssel139;
#[doc = "VSSEL140 (rw) register accessor: desc VSSEL140\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel140`] module"]
#[doc(alias = "VSSEL140")]
pub type Vssel140 = crate::Reg<vssel140::Vssel140Spec>;
#[doc = "desc VSSEL140"]
pub mod vssel140;
#[doc = "VSSEL141 (rw) register accessor: desc VSSEL141\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel141::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel141::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel141`] module"]
#[doc(alias = "VSSEL141")]
pub type Vssel141 = crate::Reg<vssel141::Vssel141Spec>;
#[doc = "desc VSSEL141"]
pub mod vssel141;
#[doc = "VSSEL142 (rw) register accessor: desc VSSEL142\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel142::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel142::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel142`] module"]
#[doc(alias = "VSSEL142")]
pub type Vssel142 = crate::Reg<vssel142::Vssel142Spec>;
#[doc = "desc VSSEL142"]
pub mod vssel142;
#[doc = "VSSEL143 (rw) register accessor: desc VSSEL143\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel143::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel143::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel143`] module"]
#[doc(alias = "VSSEL143")]
pub type Vssel143 = crate::Reg<vssel143::Vssel143Spec>;
#[doc = "desc VSSEL143"]
pub mod vssel143;
#[doc = "SWIER (rw) register accessor: desc SWIER\n\nYou can [`read`](crate::Reg::read) this register and get [`swier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier`] module"]
#[doc(alias = "SWIER")]
pub type Swier = crate::Reg<swier::SwierSpec>;
#[doc = "desc SWIER"]
pub mod swier;
#[doc = "EVTER (rw) register accessor: desc EVTER\n\nYou can [`read`](crate::Reg::read) this register and get [`evter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evter`] module"]
#[doc(alias = "EVTER")]
pub type Evter = crate::Reg<evter::EvterSpec>;
#[doc = "desc EVTER"]
pub mod evter;
#[doc = "IER (rw) register accessor: desc IER\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "desc IER"]
pub mod ier;
