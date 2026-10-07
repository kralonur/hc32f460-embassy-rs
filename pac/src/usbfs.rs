#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gvbuscfg: Gvbuscfg,
    _reserved1: [u8; 0x04],
    gahbcfg: Gahbcfg,
    gusbcfg: Gusbcfg,
    grstctl: Grstctl,
    gintsts: Gintsts,
    gintmsk: Gintmsk,
    grxstsr: Grxstsr,
    grxstsp: Grxstsp,
    grxfsiz: Grxfsiz,
    hnptxfsiz: Hnptxfsiz,
    hnptxsts: Hnptxsts,
    _reserved11: [u8; 0x0c],
    cid: Cid,
    _reserved12: [u8; 0xc0],
    hptxfsiz: Hptxfsiz,
    dieptxf1: Dieptxf1,
    dieptxf2: Dieptxf2,
    dieptxf3: Dieptxf3,
    dieptxf4: Dieptxf4,
    dieptxf5: Dieptxf5,
    _reserved18: [u8; 0x02e8],
    hcfg: Hcfg,
    hfir: Hfir,
    hfnum: Hfnum,
    _reserved21: [u8; 0x04],
    hptxsts: Hptxsts,
    haint: Haint,
    haintmsk: Haintmsk,
    _reserved24: [u8; 0x24],
    hprt: Hprt,
    _reserved25: [u8; 0xbc],
    hcchar0: Hcchar0,
    _reserved26: [u8; 0x04],
    hcint0: Hcint0,
    hcintmsk0: Hcintmsk0,
    hctsiz0: Hctsiz0,
    hcdma0: Hcdma0,
    _reserved30: [u8; 0x08],
    hcchar1: Hcchar1,
    _reserved31: [u8; 0x04],
    hcint1: Hcint1,
    hcintmsk1: Hcintmsk1,
    hctsiz1: Hctsiz1,
    hcdma1: Hcdma1,
    _reserved35: [u8; 0x08],
    hcchar2: Hcchar2,
    _reserved36: [u8; 0x04],
    hcint2: Hcint2,
    hcintmsk2: Hcintmsk2,
    hctsiz2: Hctsiz2,
    hcdma2: Hcdma2,
    _reserved40: [u8; 0x08],
    hcchar3: Hcchar3,
    _reserved41: [u8; 0x04],
    hcint3: Hcint3,
    hcintmsk3: Hcintmsk3,
    hctsiz3: Hctsiz3,
    hcdma3: Hcdma3,
    _reserved45: [u8; 0x08],
    hcchar4: Hcchar4,
    _reserved46: [u8; 0x04],
    hcint4: Hcint4,
    hcintmsk4: Hcintmsk4,
    hctsiz4: Hctsiz4,
    hcdma4: Hcdma4,
    _reserved50: [u8; 0x08],
    hcchar5: Hcchar5,
    _reserved51: [u8; 0x04],
    hcint5: Hcint5,
    hcintmsk5: Hcintmsk5,
    hctsiz5: Hctsiz5,
    hcdma5: Hcdma5,
    _reserved55: [u8; 0x08],
    hcchar6: Hcchar6,
    _reserved56: [u8; 0x04],
    hcint6: Hcint6,
    hcintmsk6: Hcintmsk6,
    hctsiz6: Hctsiz6,
    hcdma6: Hcdma6,
    _reserved60: [u8; 0x08],
    hcchar7: Hcchar7,
    _reserved61: [u8; 0x04],
    hcint7: Hcint7,
    hcintmsk7: Hcintmsk7,
    hctsiz7: Hctsiz7,
    hcdma7: Hcdma7,
    _reserved65: [u8; 0x08],
    hcchar8: Hcchar8,
    _reserved66: [u8; 0x04],
    hcint8: Hcint8,
    hcintmsk8: Hcintmsk8,
    hctsiz8: Hctsiz8,
    hcdma8: Hcdma8,
    _reserved70: [u8; 0x08],
    hcchar9: Hcchar9,
    _reserved71: [u8; 0x04],
    hcint9: Hcint9,
    hcintmsk9: Hcintmsk9,
    hctsiz9: Hctsiz9,
    hcdma9: Hcdma9,
    _reserved75: [u8; 0x08],
    hcchar10: Hcchar10,
    _reserved76: [u8; 0x04],
    hcint10: Hcint10,
    hcintmsk10: Hcintmsk10,
    hctsiz10: Hctsiz10,
    hcdma10: Hcdma10,
    _reserved80: [u8; 0x08],
    hcchar11: Hcchar11,
    _reserved81: [u8; 0x04],
    hcint11: Hcint11,
    hcintmsk11: Hcintmsk11,
    hctsiz11: Hctsiz11,
    hcdma11: Hcdma11,
    _reserved85: [u8; 0x0188],
    dcfg: Dcfg,
    dctl: Dctl,
    dsts: Dsts,
    _reserved88: [u8; 0x04],
    diepmsk: Diepmsk,
    doepmsk: Doepmsk,
    daint: Daint,
    daintmsk: Daintmsk,
    _reserved92: [u8; 0x14],
    diepempmsk: Diepempmsk,
    _reserved93: [u8; 0xc8],
    diepctl0: Diepctl0,
    _reserved94: [u8; 0x04],
    diepint0: Diepint0,
    _reserved95: [u8; 0x04],
    dieptsiz0: Dieptsiz0,
    diepdma0: Diepdma0,
    dtxfsts0: Dtxfsts0,
    _reserved98: [u8; 0x04],
    diepctl1: Diepctl1,
    _reserved99: [u8; 0x04],
    diepint1: Diepint1,
    _reserved100: [u8; 0x04],
    dieptsiz1: Dieptsiz1,
    diepdma1: Diepdma1,
    dtxfsts1: Dtxfsts1,
    _reserved103: [u8; 0x04],
    diepctl2: Diepctl2,
    _reserved104: [u8; 0x04],
    diepint2: Diepint2,
    _reserved105: [u8; 0x04],
    dieptsiz2: Dieptsiz2,
    diepdma2: Diepdma2,
    dtxfsts2: Dtxfsts2,
    _reserved108: [u8; 0x04],
    diepctl3: Diepctl3,
    _reserved109: [u8; 0x04],
    diepint3: Diepint3,
    _reserved110: [u8; 0x04],
    dieptsiz3: Dieptsiz3,
    diepdma3: Diepdma3,
    dtxfsts3: Dtxfsts3,
    _reserved113: [u8; 0x04],
    diepctl4: Diepctl4,
    _reserved114: [u8; 0x04],
    diepint4: Diepint4,
    _reserved115: [u8; 0x04],
    dieptsiz4: Dieptsiz4,
    diepdma4: Diepdma4,
    dtxfsts4: Dtxfsts4,
    _reserved118: [u8; 0x04],
    diepctl5: Diepctl5,
    _reserved119: [u8; 0x04],
    diepint5: Diepint5,
    _reserved120: [u8; 0x04],
    dieptsiz5: Dieptsiz5,
    diepdma5: Diepdma5,
    dtxfsts5: Dtxfsts5,
    _reserved123: [u8; 0x0144],
    doepctl0: Doepctl0,
    _reserved124: [u8; 0x04],
    doepint0: Doepint0,
    _reserved125: [u8; 0x04],
    doeptsiz0: Doeptsiz0,
    doepdma0: Doepdma0,
    _reserved127: [u8; 0x08],
    doepctl1: Doepctl1,
    _reserved128: [u8; 0x04],
    doepint1: Doepint1,
    _reserved129: [u8; 0x04],
    doeptsiz1: Doeptsiz1,
    doepdma1: Doepdma1,
    _reserved131: [u8; 0x08],
    doepctl2: Doepctl2,
    _reserved132: [u8; 0x04],
    doepint2: Doepint2,
    _reserved133: [u8; 0x04],
    doeptsiz2: Doeptsiz2,
    doepdma2: Doepdma2,
    _reserved135: [u8; 0x08],
    doepctl3: Doepctl3,
    _reserved136: [u8; 0x04],
    doepint3: Doepint3,
    _reserved137: [u8; 0x04],
    doeptsiz3: Doeptsiz3,
    doepdma3: Doepdma3,
    _reserved139: [u8; 0x08],
    doepctl4: Doepctl4,
    _reserved140: [u8; 0x04],
    doepint4: Doepint4,
    _reserved141: [u8; 0x04],
    doeptsiz4: Doeptsiz4,
    doepdma4: Doepdma4,
    _reserved143: [u8; 0x08],
    doepctl5: Doepctl5,
    _reserved144: [u8; 0x04],
    doepint5: Doepint5,
    _reserved145: [u8; 0x04],
    doeptsiz5: Doeptsiz5,
    doepdma5: Doepdma5,
    _reserved147: [u8; 0x0248],
    gcctl: Gcctl,
}
impl RegisterBlock {
    #[doc = "0x00 - desc GVBUSCFG"]
    #[inline(always)]
    pub const fn gvbuscfg(&self) -> &Gvbuscfg {
        &self.gvbuscfg
    }
    #[doc = "0x08 - desc GAHBCFG"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &Gahbcfg {
        &self.gahbcfg
    }
    #[doc = "0x0c - desc GUSBCFG"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &Gusbcfg {
        &self.gusbcfg
    }
    #[doc = "0x10 - desc GRSTCTL"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &Grstctl {
        &self.grstctl
    }
    #[doc = "0x14 - desc GINTSTS"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &Gintsts {
        &self.gintsts
    }
    #[doc = "0x18 - desc GINTMSK"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &Gintmsk {
        &self.gintmsk
    }
    #[doc = "0x1c - desc GRXSTSR"]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &Grxstsr {
        &self.grxstsr
    }
    #[doc = "0x20 - desc GRXSTSP"]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &Grxstsp {
        &self.grxstsp
    }
    #[doc = "0x24 - desc GRXFSIZ"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &Grxfsiz {
        &self.grxfsiz
    }
    #[doc = "0x28 - desc HNPTXFSIZ"]
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &Hnptxfsiz {
        &self.hnptxfsiz
    }
    #[doc = "0x2c - desc HNPTXSTS"]
    #[inline(always)]
    pub const fn hnptxsts(&self) -> &Hnptxsts {
        &self.hnptxsts
    }
    #[doc = "0x3c - desc CID"]
    #[inline(always)]
    pub const fn cid(&self) -> &Cid {
        &self.cid
    }
    #[doc = "0x100 - desc HPTXFSIZ"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &Hptxfsiz {
        &self.hptxfsiz
    }
    #[doc = "0x104 - desc DIEPTXF1"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &Dieptxf1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - desc DIEPTXF2"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &Dieptxf2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - desc DIEPTXF3"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &Dieptxf3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - desc DIEPTXF4"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &Dieptxf4 {
        &self.dieptxf4
    }
    #[doc = "0x114 - desc DIEPTXF5"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &Dieptxf5 {
        &self.dieptxf5
    }
    #[doc = "0x400 - desc HCFG"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &Hcfg {
        &self.hcfg
    }
    #[doc = "0x404 - desc HFIR"]
    #[inline(always)]
    pub const fn hfir(&self) -> &Hfir {
        &self.hfir
    }
    #[doc = "0x408 - desc HFNUM"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &Hfnum {
        &self.hfnum
    }
    #[doc = "0x410 - desc HPTXSTS"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &Hptxsts {
        &self.hptxsts
    }
    #[doc = "0x414 - desc HAINT"]
    #[inline(always)]
    pub const fn haint(&self) -> &Haint {
        &self.haint
    }
    #[doc = "0x418 - desc HAINTMSK"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &Haintmsk {
        &self.haintmsk
    }
    #[doc = "0x440 - desc HPRT"]
    #[inline(always)]
    pub const fn hprt(&self) -> &Hprt {
        &self.hprt
    }
    #[doc = "0x500 - desc HCCHAR0"]
    #[inline(always)]
    pub const fn hcchar0(&self) -> &Hcchar0 {
        &self.hcchar0
    }
    #[doc = "0x508 - desc HCINT0"]
    #[inline(always)]
    pub const fn hcint0(&self) -> &Hcint0 {
        &self.hcint0
    }
    #[doc = "0x50c - desc HCINTMSK0"]
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &Hcintmsk0 {
        &self.hcintmsk0
    }
    #[doc = "0x510 - desc HCTSIZ0"]
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &Hctsiz0 {
        &self.hctsiz0
    }
    #[doc = "0x514 - desc HCDMA0"]
    #[inline(always)]
    pub const fn hcdma0(&self) -> &Hcdma0 {
        &self.hcdma0
    }
    #[doc = "0x520 - desc HCCHAR1"]
    #[inline(always)]
    pub const fn hcchar1(&self) -> &Hcchar1 {
        &self.hcchar1
    }
    #[doc = "0x528 - desc HCINT1"]
    #[inline(always)]
    pub const fn hcint1(&self) -> &Hcint1 {
        &self.hcint1
    }
    #[doc = "0x52c - desc HCINTMSK1"]
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &Hcintmsk1 {
        &self.hcintmsk1
    }
    #[doc = "0x530 - desc HCTSIZ1"]
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &Hctsiz1 {
        &self.hctsiz1
    }
    #[doc = "0x534 - desc HCDMA1"]
    #[inline(always)]
    pub const fn hcdma1(&self) -> &Hcdma1 {
        &self.hcdma1
    }
    #[doc = "0x540 - desc HCCHAR2"]
    #[inline(always)]
    pub const fn hcchar2(&self) -> &Hcchar2 {
        &self.hcchar2
    }
    #[doc = "0x548 - desc HCINT2"]
    #[inline(always)]
    pub const fn hcint2(&self) -> &Hcint2 {
        &self.hcint2
    }
    #[doc = "0x54c - desc HCINTMSK2"]
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &Hcintmsk2 {
        &self.hcintmsk2
    }
    #[doc = "0x550 - desc HCTSIZ2"]
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &Hctsiz2 {
        &self.hctsiz2
    }
    #[doc = "0x554 - desc HCDMA2"]
    #[inline(always)]
    pub const fn hcdma2(&self) -> &Hcdma2 {
        &self.hcdma2
    }
    #[doc = "0x560 - desc HCCHAR3"]
    #[inline(always)]
    pub const fn hcchar3(&self) -> &Hcchar3 {
        &self.hcchar3
    }
    #[doc = "0x568 - desc HCINT3"]
    #[inline(always)]
    pub const fn hcint3(&self) -> &Hcint3 {
        &self.hcint3
    }
    #[doc = "0x56c - desc HCINTMSK3"]
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &Hcintmsk3 {
        &self.hcintmsk3
    }
    #[doc = "0x570 - desc HCTSIZ3"]
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &Hctsiz3 {
        &self.hctsiz3
    }
    #[doc = "0x574 - desc HCDMA3"]
    #[inline(always)]
    pub const fn hcdma3(&self) -> &Hcdma3 {
        &self.hcdma3
    }
    #[doc = "0x580 - desc HCCHAR4"]
    #[inline(always)]
    pub const fn hcchar4(&self) -> &Hcchar4 {
        &self.hcchar4
    }
    #[doc = "0x588 - desc HCINT4"]
    #[inline(always)]
    pub const fn hcint4(&self) -> &Hcint4 {
        &self.hcint4
    }
    #[doc = "0x58c - desc HCINTMSK4"]
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &Hcintmsk4 {
        &self.hcintmsk4
    }
    #[doc = "0x590 - desc HCTSIZ4"]
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &Hctsiz4 {
        &self.hctsiz4
    }
    #[doc = "0x594 - desc HCDMA4"]
    #[inline(always)]
    pub const fn hcdma4(&self) -> &Hcdma4 {
        &self.hcdma4
    }
    #[doc = "0x5a0 - desc HCCHAR5"]
    #[inline(always)]
    pub const fn hcchar5(&self) -> &Hcchar5 {
        &self.hcchar5
    }
    #[doc = "0x5a8 - desc HCINT5"]
    #[inline(always)]
    pub const fn hcint5(&self) -> &Hcint5 {
        &self.hcint5
    }
    #[doc = "0x5ac - desc HCINTMSK5"]
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &Hcintmsk5 {
        &self.hcintmsk5
    }
    #[doc = "0x5b0 - desc HCTSIZ5"]
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &Hctsiz5 {
        &self.hctsiz5
    }
    #[doc = "0x5b4 - desc HCDMA5"]
    #[inline(always)]
    pub const fn hcdma5(&self) -> &Hcdma5 {
        &self.hcdma5
    }
    #[doc = "0x5c0 - desc HCCHAR6"]
    #[inline(always)]
    pub const fn hcchar6(&self) -> &Hcchar6 {
        &self.hcchar6
    }
    #[doc = "0x5c8 - desc HCINT6"]
    #[inline(always)]
    pub const fn hcint6(&self) -> &Hcint6 {
        &self.hcint6
    }
    #[doc = "0x5cc - desc HCINTMSK6"]
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &Hcintmsk6 {
        &self.hcintmsk6
    }
    #[doc = "0x5d0 - desc HCTSIZ6"]
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &Hctsiz6 {
        &self.hctsiz6
    }
    #[doc = "0x5d4 - desc HCDMA6"]
    #[inline(always)]
    pub const fn hcdma6(&self) -> &Hcdma6 {
        &self.hcdma6
    }
    #[doc = "0x5e0 - desc HCCHAR7"]
    #[inline(always)]
    pub const fn hcchar7(&self) -> &Hcchar7 {
        &self.hcchar7
    }
    #[doc = "0x5e8 - desc HCINT7"]
    #[inline(always)]
    pub const fn hcint7(&self) -> &Hcint7 {
        &self.hcint7
    }
    #[doc = "0x5ec - desc HCINTMSK7"]
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &Hcintmsk7 {
        &self.hcintmsk7
    }
    #[doc = "0x5f0 - desc HCTSIZ7"]
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &Hctsiz7 {
        &self.hctsiz7
    }
    #[doc = "0x5f4 - desc HCDMA7"]
    #[inline(always)]
    pub const fn hcdma7(&self) -> &Hcdma7 {
        &self.hcdma7
    }
    #[doc = "0x600 - desc HCCHAR8"]
    #[inline(always)]
    pub const fn hcchar8(&self) -> &Hcchar8 {
        &self.hcchar8
    }
    #[doc = "0x608 - desc HCINT8"]
    #[inline(always)]
    pub const fn hcint8(&self) -> &Hcint8 {
        &self.hcint8
    }
    #[doc = "0x60c - desc HCINTMSK8"]
    #[inline(always)]
    pub const fn hcintmsk8(&self) -> &Hcintmsk8 {
        &self.hcintmsk8
    }
    #[doc = "0x610 - desc HCTSIZ8"]
    #[inline(always)]
    pub const fn hctsiz8(&self) -> &Hctsiz8 {
        &self.hctsiz8
    }
    #[doc = "0x614 - desc HCDMA8"]
    #[inline(always)]
    pub const fn hcdma8(&self) -> &Hcdma8 {
        &self.hcdma8
    }
    #[doc = "0x620 - desc HCCHAR9"]
    #[inline(always)]
    pub const fn hcchar9(&self) -> &Hcchar9 {
        &self.hcchar9
    }
    #[doc = "0x628 - desc HCINT9"]
    #[inline(always)]
    pub const fn hcint9(&self) -> &Hcint9 {
        &self.hcint9
    }
    #[doc = "0x62c - desc HCINTMSK9"]
    #[inline(always)]
    pub const fn hcintmsk9(&self) -> &Hcintmsk9 {
        &self.hcintmsk9
    }
    #[doc = "0x630 - desc HCTSIZ9"]
    #[inline(always)]
    pub const fn hctsiz9(&self) -> &Hctsiz9 {
        &self.hctsiz9
    }
    #[doc = "0x634 - desc HCDMA9"]
    #[inline(always)]
    pub const fn hcdma9(&self) -> &Hcdma9 {
        &self.hcdma9
    }
    #[doc = "0x640 - desc HCCHAR10"]
    #[inline(always)]
    pub const fn hcchar10(&self) -> &Hcchar10 {
        &self.hcchar10
    }
    #[doc = "0x648 - desc HCINT10"]
    #[inline(always)]
    pub const fn hcint10(&self) -> &Hcint10 {
        &self.hcint10
    }
    #[doc = "0x64c - desc HCINTMSK10"]
    #[inline(always)]
    pub const fn hcintmsk10(&self) -> &Hcintmsk10 {
        &self.hcintmsk10
    }
    #[doc = "0x650 - desc HCTSIZ10"]
    #[inline(always)]
    pub const fn hctsiz10(&self) -> &Hctsiz10 {
        &self.hctsiz10
    }
    #[doc = "0x654 - desc HCDMA10"]
    #[inline(always)]
    pub const fn hcdma10(&self) -> &Hcdma10 {
        &self.hcdma10
    }
    #[doc = "0x660 - desc HCCHAR11"]
    #[inline(always)]
    pub const fn hcchar11(&self) -> &Hcchar11 {
        &self.hcchar11
    }
    #[doc = "0x668 - desc HCINT11"]
    #[inline(always)]
    pub const fn hcint11(&self) -> &Hcint11 {
        &self.hcint11
    }
    #[doc = "0x66c - desc HCINTMSK11"]
    #[inline(always)]
    pub const fn hcintmsk11(&self) -> &Hcintmsk11 {
        &self.hcintmsk11
    }
    #[doc = "0x670 - desc HCTSIZ11"]
    #[inline(always)]
    pub const fn hctsiz11(&self) -> &Hctsiz11 {
        &self.hctsiz11
    }
    #[doc = "0x674 - desc HCDMA11"]
    #[inline(always)]
    pub const fn hcdma11(&self) -> &Hcdma11 {
        &self.hcdma11
    }
    #[doc = "0x800 - desc DCFG"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0x804 - desc DCTL"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x808 - desc DSTS"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0x810 - desc DIEPMSK"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &Diepmsk {
        &self.diepmsk
    }
    #[doc = "0x814 - desc DOEPMSK"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &Doepmsk {
        &self.doepmsk
    }
    #[doc = "0x818 - desc DAINT"]
    #[inline(always)]
    pub const fn daint(&self) -> &Daint {
        &self.daint
    }
    #[doc = "0x81c - desc DAINTMSK"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &Daintmsk {
        &self.daintmsk
    }
    #[doc = "0x834 - desc DIEPEMPMSK"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &Diepempmsk {
        &self.diepempmsk
    }
    #[doc = "0x900 - desc DIEPCTL0"]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &Diepctl0 {
        &self.diepctl0
    }
    #[doc = "0x908 - desc DIEPINT0"]
    #[inline(always)]
    pub const fn diepint0(&self) -> &Diepint0 {
        &self.diepint0
    }
    #[doc = "0x910 - desc DIEPTSIZ0"]
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &Dieptsiz0 {
        &self.dieptsiz0
    }
    #[doc = "0x914 - desc DIEPDMA0"]
    #[inline(always)]
    pub const fn diepdma0(&self) -> &Diepdma0 {
        &self.diepdma0
    }
    #[doc = "0x918 - desc DTXFSTS0"]
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &Dtxfsts0 {
        &self.dtxfsts0
    }
    #[doc = "0x920 - desc DIEPCTL1"]
    #[inline(always)]
    pub const fn diepctl1(&self) -> &Diepctl1 {
        &self.diepctl1
    }
    #[doc = "0x928 - desc DIEPINT1"]
    #[inline(always)]
    pub const fn diepint1(&self) -> &Diepint1 {
        &self.diepint1
    }
    #[doc = "0x930 - desc DIEPTSIZ1"]
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &Dieptsiz1 {
        &self.dieptsiz1
    }
    #[doc = "0x934 - desc DIEPDMA1"]
    #[inline(always)]
    pub const fn diepdma1(&self) -> &Diepdma1 {
        &self.diepdma1
    }
    #[doc = "0x938 - desc DTXFSTS1"]
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &Dtxfsts1 {
        &self.dtxfsts1
    }
    #[doc = "0x940 - desc DIEPCTL2"]
    #[inline(always)]
    pub const fn diepctl2(&self) -> &Diepctl2 {
        &self.diepctl2
    }
    #[doc = "0x948 - desc DIEPINT2"]
    #[inline(always)]
    pub const fn diepint2(&self) -> &Diepint2 {
        &self.diepint2
    }
    #[doc = "0x950 - desc DIEPTSIZ2"]
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &Dieptsiz2 {
        &self.dieptsiz2
    }
    #[doc = "0x954 - desc DIEPDMA2"]
    #[inline(always)]
    pub const fn diepdma2(&self) -> &Diepdma2 {
        &self.diepdma2
    }
    #[doc = "0x958 - desc DTXFSTS2"]
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &Dtxfsts2 {
        &self.dtxfsts2
    }
    #[doc = "0x960 - desc DIEPCTL3"]
    #[inline(always)]
    pub const fn diepctl3(&self) -> &Diepctl3 {
        &self.diepctl3
    }
    #[doc = "0x968 - desc DIEPINT3"]
    #[inline(always)]
    pub const fn diepint3(&self) -> &Diepint3 {
        &self.diepint3
    }
    #[doc = "0x970 - desc DIEPTSIZ3"]
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &Dieptsiz3 {
        &self.dieptsiz3
    }
    #[doc = "0x974 - desc DIEPDMA3"]
    #[inline(always)]
    pub const fn diepdma3(&self) -> &Diepdma3 {
        &self.diepdma3
    }
    #[doc = "0x978 - desc DTXFSTS3"]
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &Dtxfsts3 {
        &self.dtxfsts3
    }
    #[doc = "0x980 - desc DIEPCTL4"]
    #[inline(always)]
    pub const fn diepctl4(&self) -> &Diepctl4 {
        &self.diepctl4
    }
    #[doc = "0x988 - desc DIEPINT4"]
    #[inline(always)]
    pub const fn diepint4(&self) -> &Diepint4 {
        &self.diepint4
    }
    #[doc = "0x990 - desc DIEPTSIZ4"]
    #[inline(always)]
    pub const fn dieptsiz4(&self) -> &Dieptsiz4 {
        &self.dieptsiz4
    }
    #[doc = "0x994 - desc DIEPDMA4"]
    #[inline(always)]
    pub const fn diepdma4(&self) -> &Diepdma4 {
        &self.diepdma4
    }
    #[doc = "0x998 - desc DTXFSTS4"]
    #[inline(always)]
    pub const fn dtxfsts4(&self) -> &Dtxfsts4 {
        &self.dtxfsts4
    }
    #[doc = "0x9a0 - desc DIEPCTL5"]
    #[inline(always)]
    pub const fn diepctl5(&self) -> &Diepctl5 {
        &self.diepctl5
    }
    #[doc = "0x9a8 - desc DIEPINT5"]
    #[inline(always)]
    pub const fn diepint5(&self) -> &Diepint5 {
        &self.diepint5
    }
    #[doc = "0x9b0 - desc DIEPTSIZ5"]
    #[inline(always)]
    pub const fn dieptsiz5(&self) -> &Dieptsiz5 {
        &self.dieptsiz5
    }
    #[doc = "0x9b4 - desc DIEPDMA5"]
    #[inline(always)]
    pub const fn diepdma5(&self) -> &Diepdma5 {
        &self.diepdma5
    }
    #[doc = "0x9b8 - desc DTXFSTS5"]
    #[inline(always)]
    pub const fn dtxfsts5(&self) -> &Dtxfsts5 {
        &self.dtxfsts5
    }
    #[doc = "0xb00 - desc DOEPCTL0"]
    #[inline(always)]
    pub const fn doepctl0(&self) -> &Doepctl0 {
        &self.doepctl0
    }
    #[doc = "0xb08 - desc DOEPINT0"]
    #[inline(always)]
    pub const fn doepint0(&self) -> &Doepint0 {
        &self.doepint0
    }
    #[doc = "0xb10 - desc DOEPTSIZ0"]
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &Doeptsiz0 {
        &self.doeptsiz0
    }
    #[doc = "0xb14 - desc DOEPDMA0"]
    #[inline(always)]
    pub const fn doepdma0(&self) -> &Doepdma0 {
        &self.doepdma0
    }
    #[doc = "0xb20 - desc DOEPCTL1"]
    #[inline(always)]
    pub const fn doepctl1(&self) -> &Doepctl1 {
        &self.doepctl1
    }
    #[doc = "0xb28 - desc DOEPINT1"]
    #[inline(always)]
    pub const fn doepint1(&self) -> &Doepint1 {
        &self.doepint1
    }
    #[doc = "0xb30 - desc DOEPTSIZ1"]
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &Doeptsiz1 {
        &self.doeptsiz1
    }
    #[doc = "0xb34 - desc DOEPDMA1"]
    #[inline(always)]
    pub const fn doepdma1(&self) -> &Doepdma1 {
        &self.doepdma1
    }
    #[doc = "0xb40 - desc DOEPCTL2"]
    #[inline(always)]
    pub const fn doepctl2(&self) -> &Doepctl2 {
        &self.doepctl2
    }
    #[doc = "0xb48 - desc DOEPINT2"]
    #[inline(always)]
    pub const fn doepint2(&self) -> &Doepint2 {
        &self.doepint2
    }
    #[doc = "0xb50 - desc DOEPTSIZ2"]
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &Doeptsiz2 {
        &self.doeptsiz2
    }
    #[doc = "0xb54 - desc DOEPDMA2"]
    #[inline(always)]
    pub const fn doepdma2(&self) -> &Doepdma2 {
        &self.doepdma2
    }
    #[doc = "0xb60 - desc DOEPCTL3"]
    #[inline(always)]
    pub const fn doepctl3(&self) -> &Doepctl3 {
        &self.doepctl3
    }
    #[doc = "0xb68 - desc DOEPINT3"]
    #[inline(always)]
    pub const fn doepint3(&self) -> &Doepint3 {
        &self.doepint3
    }
    #[doc = "0xb70 - desc DOEPTSIZ3"]
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &Doeptsiz3 {
        &self.doeptsiz3
    }
    #[doc = "0xb74 - desc DOEPDMA3"]
    #[inline(always)]
    pub const fn doepdma3(&self) -> &Doepdma3 {
        &self.doepdma3
    }
    #[doc = "0xb80 - desc DOEPCTL4"]
    #[inline(always)]
    pub const fn doepctl4(&self) -> &Doepctl4 {
        &self.doepctl4
    }
    #[doc = "0xb88 - desc DOEPINT4"]
    #[inline(always)]
    pub const fn doepint4(&self) -> &Doepint4 {
        &self.doepint4
    }
    #[doc = "0xb90 - desc DOEPTSIZ4"]
    #[inline(always)]
    pub const fn doeptsiz4(&self) -> &Doeptsiz4 {
        &self.doeptsiz4
    }
    #[doc = "0xb94 - desc DOEPDMA4"]
    #[inline(always)]
    pub const fn doepdma4(&self) -> &Doepdma4 {
        &self.doepdma4
    }
    #[doc = "0xba0 - desc DOEPCTL5"]
    #[inline(always)]
    pub const fn doepctl5(&self) -> &Doepctl5 {
        &self.doepctl5
    }
    #[doc = "0xba8 - desc DOEPINT5"]
    #[inline(always)]
    pub const fn doepint5(&self) -> &Doepint5 {
        &self.doepint5
    }
    #[doc = "0xbb0 - desc DOEPTSIZ5"]
    #[inline(always)]
    pub const fn doeptsiz5(&self) -> &Doeptsiz5 {
        &self.doeptsiz5
    }
    #[doc = "0xbb4 - desc DOEPDMA5"]
    #[inline(always)]
    pub const fn doepdma5(&self) -> &Doepdma5 {
        &self.doepdma5
    }
    #[doc = "0xe00 - desc GCCTL"]
    #[inline(always)]
    pub const fn gcctl(&self) -> &Gcctl {
        &self.gcctl
    }
}
#[doc = "GVBUSCFG (rw) register accessor: desc GVBUSCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`gvbuscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gvbuscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gvbuscfg`] module"]
#[doc(alias = "GVBUSCFG")]
pub type Gvbuscfg = crate::Reg<gvbuscfg::GvbuscfgSpec>;
#[doc = "desc GVBUSCFG"]
pub mod gvbuscfg;
#[doc = "GAHBCFG (rw) register accessor: desc GAHBCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`] module"]
#[doc(alias = "GAHBCFG")]
pub type Gahbcfg = crate::Reg<gahbcfg::GahbcfgSpec>;
#[doc = "desc GAHBCFG"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: desc GUSBCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`] module"]
#[doc(alias = "GUSBCFG")]
pub type Gusbcfg = crate::Reg<gusbcfg::GusbcfgSpec>;
#[doc = "desc GUSBCFG"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: desc GRSTCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`] module"]
#[doc(alias = "GRSTCTL")]
pub type Grstctl = crate::Reg<grstctl::GrstctlSpec>;
#[doc = "desc GRSTCTL"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: desc GINTSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`] module"]
#[doc(alias = "GINTSTS")]
pub type Gintsts = crate::Reg<gintsts::GintstsSpec>;
#[doc = "desc GINTSTS"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: desc GINTMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`] module"]
#[doc(alias = "GINTMSK")]
pub type Gintmsk = crate::Reg<gintmsk::GintmskSpec>;
#[doc = "desc GINTMSK"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: desc GRXSTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`] module"]
#[doc(alias = "GRXSTSR")]
pub type Grxstsr = crate::Reg<grxstsr::GrxstsrSpec>;
#[doc = "desc GRXSTSR"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: desc GRXSTSP\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`] module"]
#[doc(alias = "GRXSTSP")]
pub type Grxstsp = crate::Reg<grxstsp::GrxstspSpec>;
#[doc = "desc GRXSTSP"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: desc GRXFSIZ\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`] module"]
#[doc(alias = "GRXFSIZ")]
pub type Grxfsiz = crate::Reg<grxfsiz::GrxfsizSpec>;
#[doc = "desc GRXFSIZ"]
pub mod grxfsiz;
#[doc = "HNPTXFSIZ (rw) register accessor: desc HNPTXFSIZ\n\nYou can [`read`](crate::Reg::read) this register and get [`hnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxfsiz`] module"]
#[doc(alias = "HNPTXFSIZ")]
pub type Hnptxfsiz = crate::Reg<hnptxfsiz::HnptxfsizSpec>;
#[doc = "desc HNPTXFSIZ"]
pub mod hnptxfsiz;
#[doc = "HNPTXSTS (r) register accessor: desc HNPTXSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`hnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxsts`] module"]
#[doc(alias = "HNPTXSTS")]
pub type Hnptxsts = crate::Reg<hnptxsts::HnptxstsSpec>;
#[doc = "desc HNPTXSTS"]
pub mod hnptxsts;
#[doc = "CID (rw) register accessor: desc CID\n\nYou can [`read`](crate::Reg::read) this register and get [`cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`] module"]
#[doc(alias = "CID")]
pub type Cid = crate::Reg<cid::CidSpec>;
#[doc = "desc CID"]
pub mod cid;
#[doc = "HPTXFSIZ (rw) register accessor: desc HPTXFSIZ\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`] module"]
#[doc(alias = "HPTXFSIZ")]
pub type Hptxfsiz = crate::Reg<hptxfsiz::HptxfsizSpec>;
#[doc = "desc HPTXFSIZ"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: desc DIEPTXF1\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`] module"]
#[doc(alias = "DIEPTXF1")]
pub type Dieptxf1 = crate::Reg<dieptxf1::Dieptxf1Spec>;
#[doc = "desc DIEPTXF1"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: desc DIEPTXF2\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`] module"]
#[doc(alias = "DIEPTXF2")]
pub type Dieptxf2 = crate::Reg<dieptxf2::Dieptxf2Spec>;
#[doc = "desc DIEPTXF2"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: desc DIEPTXF3\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`] module"]
#[doc(alias = "DIEPTXF3")]
pub type Dieptxf3 = crate::Reg<dieptxf3::Dieptxf3Spec>;
#[doc = "desc DIEPTXF3"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: desc DIEPTXF4\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`] module"]
#[doc(alias = "DIEPTXF4")]
pub type Dieptxf4 = crate::Reg<dieptxf4::Dieptxf4Spec>;
#[doc = "desc DIEPTXF4"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: desc DIEPTXF5\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`] module"]
#[doc(alias = "DIEPTXF5")]
pub type Dieptxf5 = crate::Reg<dieptxf5::Dieptxf5Spec>;
#[doc = "desc DIEPTXF5"]
pub mod dieptxf5;
#[doc = "HCFG (rw) register accessor: desc HCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`] module"]
#[doc(alias = "HCFG")]
pub type Hcfg = crate::Reg<hcfg::HcfgSpec>;
#[doc = "desc HCFG"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: desc HFIR\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`] module"]
#[doc(alias = "HFIR")]
pub type Hfir = crate::Reg<hfir::HfirSpec>;
#[doc = "desc HFIR"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: desc HFNUM\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`] module"]
#[doc(alias = "HFNUM")]
pub type Hfnum = crate::Reg<hfnum::HfnumSpec>;
#[doc = "desc HFNUM"]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: desc HPTXSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`] module"]
#[doc(alias = "HPTXSTS")]
pub type Hptxsts = crate::Reg<hptxsts::HptxstsSpec>;
#[doc = "desc HPTXSTS"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: desc HAINT\n\nYou can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`] module"]
#[doc(alias = "HAINT")]
pub type Haint = crate::Reg<haint::HaintSpec>;
#[doc = "desc HAINT"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: desc HAINTMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`] module"]
#[doc(alias = "HAINTMSK")]
pub type Haintmsk = crate::Reg<haintmsk::HaintmskSpec>;
#[doc = "desc HAINTMSK"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: desc HPRT\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`] module"]
#[doc(alias = "HPRT")]
pub type Hprt = crate::Reg<hprt::HprtSpec>;
#[doc = "desc HPRT"]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: desc HCCHAR0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar0`] module"]
#[doc(alias = "HCCHAR0")]
pub type Hcchar0 = crate::Reg<hcchar0::Hcchar0Spec>;
#[doc = "desc HCCHAR0"]
pub mod hcchar0;
#[doc = "HCINT0 (rw) register accessor: desc HCINT0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint0`] module"]
#[doc(alias = "HCINT0")]
pub type Hcint0 = crate::Reg<hcint0::Hcint0Spec>;
#[doc = "desc HCINT0"]
pub mod hcint0;
#[doc = "HCINTMSK0 (rw) register accessor: desc HCINTMSK0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk0`] module"]
#[doc(alias = "HCINTMSK0")]
pub type Hcintmsk0 = crate::Reg<hcintmsk0::Hcintmsk0Spec>;
#[doc = "desc HCINTMSK0"]
pub mod hcintmsk0;
#[doc = "HCTSIZ0 (rw) register accessor: desc HCTSIZ0\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz0`] module"]
#[doc(alias = "HCTSIZ0")]
pub type Hctsiz0 = crate::Reg<hctsiz0::Hctsiz0Spec>;
#[doc = "desc HCTSIZ0"]
pub mod hctsiz0;
#[doc = "HCDMA0 (rw) register accessor: desc HCDMA0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma0`] module"]
#[doc(alias = "HCDMA0")]
pub type Hcdma0 = crate::Reg<hcdma0::Hcdma0Spec>;
#[doc = "desc HCDMA0"]
pub mod hcdma0;
#[doc = "HCCHAR1 (rw) register accessor: desc HCCHAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar1`] module"]
#[doc(alias = "HCCHAR1")]
pub type Hcchar1 = crate::Reg<hcchar1::Hcchar1Spec>;
#[doc = "desc HCCHAR1"]
pub mod hcchar1;
#[doc = "HCINT1 (rw) register accessor: desc HCINT1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint1`] module"]
#[doc(alias = "HCINT1")]
pub type Hcint1 = crate::Reg<hcint1::Hcint1Spec>;
#[doc = "desc HCINT1"]
pub mod hcint1;
#[doc = "HCINTMSK1 (rw) register accessor: desc HCINTMSK1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk1`] module"]
#[doc(alias = "HCINTMSK1")]
pub type Hcintmsk1 = crate::Reg<hcintmsk1::Hcintmsk1Spec>;
#[doc = "desc HCINTMSK1"]
pub mod hcintmsk1;
#[doc = "HCTSIZ1 (rw) register accessor: desc HCTSIZ1\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz1`] module"]
#[doc(alias = "HCTSIZ1")]
pub type Hctsiz1 = crate::Reg<hctsiz1::Hctsiz1Spec>;
#[doc = "desc HCTSIZ1"]
pub mod hctsiz1;
#[doc = "HCDMA1 (rw) register accessor: desc HCDMA1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma1`] module"]
#[doc(alias = "HCDMA1")]
pub type Hcdma1 = crate::Reg<hcdma1::Hcdma1Spec>;
#[doc = "desc HCDMA1"]
pub mod hcdma1;
#[doc = "HCCHAR2 (rw) register accessor: desc HCCHAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar2`] module"]
#[doc(alias = "HCCHAR2")]
pub type Hcchar2 = crate::Reg<hcchar2::Hcchar2Spec>;
#[doc = "desc HCCHAR2"]
pub mod hcchar2;
#[doc = "HCINT2 (rw) register accessor: desc HCINT2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint2`] module"]
#[doc(alias = "HCINT2")]
pub type Hcint2 = crate::Reg<hcint2::Hcint2Spec>;
#[doc = "desc HCINT2"]
pub mod hcint2;
#[doc = "HCINTMSK2 (rw) register accessor: desc HCINTMSK2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk2`] module"]
#[doc(alias = "HCINTMSK2")]
pub type Hcintmsk2 = crate::Reg<hcintmsk2::Hcintmsk2Spec>;
#[doc = "desc HCINTMSK2"]
pub mod hcintmsk2;
#[doc = "HCTSIZ2 (rw) register accessor: desc HCTSIZ2\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz2`] module"]
#[doc(alias = "HCTSIZ2")]
pub type Hctsiz2 = crate::Reg<hctsiz2::Hctsiz2Spec>;
#[doc = "desc HCTSIZ2"]
pub mod hctsiz2;
#[doc = "HCDMA2 (rw) register accessor: desc HCDMA2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma2`] module"]
#[doc(alias = "HCDMA2")]
pub type Hcdma2 = crate::Reg<hcdma2::Hcdma2Spec>;
#[doc = "desc HCDMA2"]
pub mod hcdma2;
#[doc = "HCCHAR3 (rw) register accessor: desc HCCHAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar3`] module"]
#[doc(alias = "HCCHAR3")]
pub type Hcchar3 = crate::Reg<hcchar3::Hcchar3Spec>;
#[doc = "desc HCCHAR3"]
pub mod hcchar3;
#[doc = "HCINT3 (rw) register accessor: desc HCINT3\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint3`] module"]
#[doc(alias = "HCINT3")]
pub type Hcint3 = crate::Reg<hcint3::Hcint3Spec>;
#[doc = "desc HCINT3"]
pub mod hcint3;
#[doc = "HCINTMSK3 (rw) register accessor: desc HCINTMSK3\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk3`] module"]
#[doc(alias = "HCINTMSK3")]
pub type Hcintmsk3 = crate::Reg<hcintmsk3::Hcintmsk3Spec>;
#[doc = "desc HCINTMSK3"]
pub mod hcintmsk3;
#[doc = "HCTSIZ3 (rw) register accessor: desc HCTSIZ3\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz3`] module"]
#[doc(alias = "HCTSIZ3")]
pub type Hctsiz3 = crate::Reg<hctsiz3::Hctsiz3Spec>;
#[doc = "desc HCTSIZ3"]
pub mod hctsiz3;
#[doc = "HCDMA3 (rw) register accessor: desc HCDMA3\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma3`] module"]
#[doc(alias = "HCDMA3")]
pub type Hcdma3 = crate::Reg<hcdma3::Hcdma3Spec>;
#[doc = "desc HCDMA3"]
pub mod hcdma3;
#[doc = "HCCHAR4 (rw) register accessor: desc HCCHAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar4`] module"]
#[doc(alias = "HCCHAR4")]
pub type Hcchar4 = crate::Reg<hcchar4::Hcchar4Spec>;
#[doc = "desc HCCHAR4"]
pub mod hcchar4;
#[doc = "HCINT4 (rw) register accessor: desc HCINT4\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint4`] module"]
#[doc(alias = "HCINT4")]
pub type Hcint4 = crate::Reg<hcint4::Hcint4Spec>;
#[doc = "desc HCINT4"]
pub mod hcint4;
#[doc = "HCINTMSK4 (rw) register accessor: desc HCINTMSK4\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk4`] module"]
#[doc(alias = "HCINTMSK4")]
pub type Hcintmsk4 = crate::Reg<hcintmsk4::Hcintmsk4Spec>;
#[doc = "desc HCINTMSK4"]
pub mod hcintmsk4;
#[doc = "HCTSIZ4 (rw) register accessor: desc HCTSIZ4\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz4`] module"]
#[doc(alias = "HCTSIZ4")]
pub type Hctsiz4 = crate::Reg<hctsiz4::Hctsiz4Spec>;
#[doc = "desc HCTSIZ4"]
pub mod hctsiz4;
#[doc = "HCDMA4 (rw) register accessor: desc HCDMA4\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma4`] module"]
#[doc(alias = "HCDMA4")]
pub type Hcdma4 = crate::Reg<hcdma4::Hcdma4Spec>;
#[doc = "desc HCDMA4"]
pub mod hcdma4;
#[doc = "HCCHAR5 (rw) register accessor: desc HCCHAR5\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar5`] module"]
#[doc(alias = "HCCHAR5")]
pub type Hcchar5 = crate::Reg<hcchar5::Hcchar5Spec>;
#[doc = "desc HCCHAR5"]
pub mod hcchar5;
#[doc = "HCINT5 (rw) register accessor: desc HCINT5\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint5`] module"]
#[doc(alias = "HCINT5")]
pub type Hcint5 = crate::Reg<hcint5::Hcint5Spec>;
#[doc = "desc HCINT5"]
pub mod hcint5;
#[doc = "HCINTMSK5 (rw) register accessor: desc HCINTMSK5\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk5`] module"]
#[doc(alias = "HCINTMSK5")]
pub type Hcintmsk5 = crate::Reg<hcintmsk5::Hcintmsk5Spec>;
#[doc = "desc HCINTMSK5"]
pub mod hcintmsk5;
#[doc = "HCTSIZ5 (rw) register accessor: desc HCTSIZ5\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz5`] module"]
#[doc(alias = "HCTSIZ5")]
pub type Hctsiz5 = crate::Reg<hctsiz5::Hctsiz5Spec>;
#[doc = "desc HCTSIZ5"]
pub mod hctsiz5;
#[doc = "HCDMA5 (rw) register accessor: desc HCDMA5\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma5`] module"]
#[doc(alias = "HCDMA5")]
pub type Hcdma5 = crate::Reg<hcdma5::Hcdma5Spec>;
#[doc = "desc HCDMA5"]
pub mod hcdma5;
#[doc = "HCCHAR6 (rw) register accessor: desc HCCHAR6\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar6`] module"]
#[doc(alias = "HCCHAR6")]
pub type Hcchar6 = crate::Reg<hcchar6::Hcchar6Spec>;
#[doc = "desc HCCHAR6"]
pub mod hcchar6;
#[doc = "HCINT6 (rw) register accessor: desc HCINT6\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint6`] module"]
#[doc(alias = "HCINT6")]
pub type Hcint6 = crate::Reg<hcint6::Hcint6Spec>;
#[doc = "desc HCINT6"]
pub mod hcint6;
#[doc = "HCINTMSK6 (rw) register accessor: desc HCINTMSK6\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk6`] module"]
#[doc(alias = "HCINTMSK6")]
pub type Hcintmsk6 = crate::Reg<hcintmsk6::Hcintmsk6Spec>;
#[doc = "desc HCINTMSK6"]
pub mod hcintmsk6;
#[doc = "HCTSIZ6 (rw) register accessor: desc HCTSIZ6\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz6`] module"]
#[doc(alias = "HCTSIZ6")]
pub type Hctsiz6 = crate::Reg<hctsiz6::Hctsiz6Spec>;
#[doc = "desc HCTSIZ6"]
pub mod hctsiz6;
#[doc = "HCDMA6 (rw) register accessor: desc HCDMA6\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma6`] module"]
#[doc(alias = "HCDMA6")]
pub type Hcdma6 = crate::Reg<hcdma6::Hcdma6Spec>;
#[doc = "desc HCDMA6"]
pub mod hcdma6;
#[doc = "HCCHAR7 (rw) register accessor: desc HCCHAR7\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar7`] module"]
#[doc(alias = "HCCHAR7")]
pub type Hcchar7 = crate::Reg<hcchar7::Hcchar7Spec>;
#[doc = "desc HCCHAR7"]
pub mod hcchar7;
#[doc = "HCINT7 (rw) register accessor: desc HCINT7\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint7`] module"]
#[doc(alias = "HCINT7")]
pub type Hcint7 = crate::Reg<hcint7::Hcint7Spec>;
#[doc = "desc HCINT7"]
pub mod hcint7;
#[doc = "HCINTMSK7 (rw) register accessor: desc HCINTMSK7\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk7`] module"]
#[doc(alias = "HCINTMSK7")]
pub type Hcintmsk7 = crate::Reg<hcintmsk7::Hcintmsk7Spec>;
#[doc = "desc HCINTMSK7"]
pub mod hcintmsk7;
#[doc = "HCTSIZ7 (rw) register accessor: desc HCTSIZ7\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz7`] module"]
#[doc(alias = "HCTSIZ7")]
pub type Hctsiz7 = crate::Reg<hctsiz7::Hctsiz7Spec>;
#[doc = "desc HCTSIZ7"]
pub mod hctsiz7;
#[doc = "HCDMA7 (rw) register accessor: desc HCDMA7\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma7`] module"]
#[doc(alias = "HCDMA7")]
pub type Hcdma7 = crate::Reg<hcdma7::Hcdma7Spec>;
#[doc = "desc HCDMA7"]
pub mod hcdma7;
#[doc = "HCCHAR8 (rw) register accessor: desc HCCHAR8\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar8`] module"]
#[doc(alias = "HCCHAR8")]
pub type Hcchar8 = crate::Reg<hcchar8::Hcchar8Spec>;
#[doc = "desc HCCHAR8"]
pub mod hcchar8;
#[doc = "HCINT8 (rw) register accessor: desc HCINT8\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint8`] module"]
#[doc(alias = "HCINT8")]
pub type Hcint8 = crate::Reg<hcint8::Hcint8Spec>;
#[doc = "desc HCINT8"]
pub mod hcint8;
#[doc = "HCINTMSK8 (rw) register accessor: desc HCINTMSK8\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk8`] module"]
#[doc(alias = "HCINTMSK8")]
pub type Hcintmsk8 = crate::Reg<hcintmsk8::Hcintmsk8Spec>;
#[doc = "desc HCINTMSK8"]
pub mod hcintmsk8;
#[doc = "HCTSIZ8 (rw) register accessor: desc HCTSIZ8\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz8`] module"]
#[doc(alias = "HCTSIZ8")]
pub type Hctsiz8 = crate::Reg<hctsiz8::Hctsiz8Spec>;
#[doc = "desc HCTSIZ8"]
pub mod hctsiz8;
#[doc = "HCDMA8 (rw) register accessor: desc HCDMA8\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma8`] module"]
#[doc(alias = "HCDMA8")]
pub type Hcdma8 = crate::Reg<hcdma8::Hcdma8Spec>;
#[doc = "desc HCDMA8"]
pub mod hcdma8;
#[doc = "HCCHAR9 (rw) register accessor: desc HCCHAR9\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar9`] module"]
#[doc(alias = "HCCHAR9")]
pub type Hcchar9 = crate::Reg<hcchar9::Hcchar9Spec>;
#[doc = "desc HCCHAR9"]
pub mod hcchar9;
#[doc = "HCINT9 (rw) register accessor: desc HCINT9\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint9`] module"]
#[doc(alias = "HCINT9")]
pub type Hcint9 = crate::Reg<hcint9::Hcint9Spec>;
#[doc = "desc HCINT9"]
pub mod hcint9;
#[doc = "HCINTMSK9 (rw) register accessor: desc HCINTMSK9\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk9`] module"]
#[doc(alias = "HCINTMSK9")]
pub type Hcintmsk9 = crate::Reg<hcintmsk9::Hcintmsk9Spec>;
#[doc = "desc HCINTMSK9"]
pub mod hcintmsk9;
#[doc = "HCTSIZ9 (rw) register accessor: desc HCTSIZ9\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz9`] module"]
#[doc(alias = "HCTSIZ9")]
pub type Hctsiz9 = crate::Reg<hctsiz9::Hctsiz9Spec>;
#[doc = "desc HCTSIZ9"]
pub mod hctsiz9;
#[doc = "HCDMA9 (rw) register accessor: desc HCDMA9\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma9`] module"]
#[doc(alias = "HCDMA9")]
pub type Hcdma9 = crate::Reg<hcdma9::Hcdma9Spec>;
#[doc = "desc HCDMA9"]
pub mod hcdma9;
#[doc = "HCCHAR10 (rw) register accessor: desc HCCHAR10\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar10`] module"]
#[doc(alias = "HCCHAR10")]
pub type Hcchar10 = crate::Reg<hcchar10::Hcchar10Spec>;
#[doc = "desc HCCHAR10"]
pub mod hcchar10;
#[doc = "HCINT10 (rw) register accessor: desc HCINT10\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint10`] module"]
#[doc(alias = "HCINT10")]
pub type Hcint10 = crate::Reg<hcint10::Hcint10Spec>;
#[doc = "desc HCINT10"]
pub mod hcint10;
#[doc = "HCINTMSK10 (rw) register accessor: desc HCINTMSK10\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk10`] module"]
#[doc(alias = "HCINTMSK10")]
pub type Hcintmsk10 = crate::Reg<hcintmsk10::Hcintmsk10Spec>;
#[doc = "desc HCINTMSK10"]
pub mod hcintmsk10;
#[doc = "HCTSIZ10 (rw) register accessor: desc HCTSIZ10\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz10`] module"]
#[doc(alias = "HCTSIZ10")]
pub type Hctsiz10 = crate::Reg<hctsiz10::Hctsiz10Spec>;
#[doc = "desc HCTSIZ10"]
pub mod hctsiz10;
#[doc = "HCDMA10 (rw) register accessor: desc HCDMA10\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma10`] module"]
#[doc(alias = "HCDMA10")]
pub type Hcdma10 = crate::Reg<hcdma10::Hcdma10Spec>;
#[doc = "desc HCDMA10"]
pub mod hcdma10;
#[doc = "HCCHAR11 (rw) register accessor: desc HCCHAR11\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar11`] module"]
#[doc(alias = "HCCHAR11")]
pub type Hcchar11 = crate::Reg<hcchar11::Hcchar11Spec>;
#[doc = "desc HCCHAR11"]
pub mod hcchar11;
#[doc = "HCINT11 (rw) register accessor: desc HCINT11\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint11`] module"]
#[doc(alias = "HCINT11")]
pub type Hcint11 = crate::Reg<hcint11::Hcint11Spec>;
#[doc = "desc HCINT11"]
pub mod hcint11;
#[doc = "HCINTMSK11 (rw) register accessor: desc HCINTMSK11\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk11`] module"]
#[doc(alias = "HCINTMSK11")]
pub type Hcintmsk11 = crate::Reg<hcintmsk11::Hcintmsk11Spec>;
#[doc = "desc HCINTMSK11"]
pub mod hcintmsk11;
#[doc = "HCTSIZ11 (rw) register accessor: desc HCTSIZ11\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz11`] module"]
#[doc(alias = "HCTSIZ11")]
pub type Hctsiz11 = crate::Reg<hctsiz11::Hctsiz11Spec>;
#[doc = "desc HCTSIZ11"]
pub mod hctsiz11;
#[doc = "HCDMA11 (rw) register accessor: desc HCDMA11\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma11`] module"]
#[doc(alias = "HCDMA11")]
pub type Hcdma11 = crate::Reg<hcdma11::Hcdma11Spec>;
#[doc = "desc HCDMA11"]
pub mod hcdma11;
#[doc = "DCFG (rw) register accessor: desc DCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`] module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "desc DCFG"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: desc DCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`] module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "desc DCTL"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: desc DSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`] module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "desc DSTS"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: desc DIEPMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`] module"]
#[doc(alias = "DIEPMSK")]
pub type Diepmsk = crate::Reg<diepmsk::DiepmskSpec>;
#[doc = "desc DIEPMSK"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: desc DOEPMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`] module"]
#[doc(alias = "DOEPMSK")]
pub type Doepmsk = crate::Reg<doepmsk::DoepmskSpec>;
#[doc = "desc DOEPMSK"]
pub mod doepmsk;
#[doc = "DAINT (rw) register accessor: desc DAINT\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`] module"]
#[doc(alias = "DAINT")]
pub type Daint = crate::Reg<daint::DaintSpec>;
#[doc = "desc DAINT"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: desc DAINTMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`] module"]
#[doc(alias = "DAINTMSK")]
pub type Daintmsk = crate::Reg<daintmsk::DaintmskSpec>;
#[doc = "desc DAINTMSK"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK (rw) register accessor: desc DIEPEMPMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`] module"]
#[doc(alias = "DIEPEMPMSK")]
pub type Diepempmsk = crate::Reg<diepempmsk::DiepempmskSpec>;
#[doc = "desc DIEPEMPMSK"]
pub mod diepempmsk;
#[doc = "DIEPCTL0 (rw) register accessor: desc DIEPCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`] module"]
#[doc(alias = "DIEPCTL0")]
pub type Diepctl0 = crate::Reg<diepctl0::Diepctl0Spec>;
#[doc = "desc DIEPCTL0"]
pub mod diepctl0;
#[doc = "DIEPINT0 (rw) register accessor: desc DIEPINT0\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint0`] module"]
#[doc(alias = "DIEPINT0")]
pub type Diepint0 = crate::Reg<diepint0::Diepint0Spec>;
#[doc = "desc DIEPINT0"]
pub mod diepint0;
#[doc = "DIEPTSIZ0 (rw) register accessor: desc DIEPTSIZ0\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz0`] module"]
#[doc(alias = "DIEPTSIZ0")]
pub type Dieptsiz0 = crate::Reg<dieptsiz0::Dieptsiz0Spec>;
#[doc = "desc DIEPTSIZ0"]
pub mod dieptsiz0;
#[doc = "DIEPDMA0 (rw) register accessor: desc DIEPDMA0\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma0`] module"]
#[doc(alias = "DIEPDMA0")]
pub type Diepdma0 = crate::Reg<diepdma0::Diepdma0Spec>;
#[doc = "desc DIEPDMA0"]
pub mod diepdma0;
#[doc = "DTXFSTS0 (r) register accessor: desc DTXFSTS0\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts0`] module"]
#[doc(alias = "DTXFSTS0")]
pub type Dtxfsts0 = crate::Reg<dtxfsts0::Dtxfsts0Spec>;
#[doc = "desc DTXFSTS0"]
pub mod dtxfsts0;
#[doc = "DIEPCTL1 (rw) register accessor: desc DIEPCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl1`] module"]
#[doc(alias = "DIEPCTL1")]
pub type Diepctl1 = crate::Reg<diepctl1::Diepctl1Spec>;
#[doc = "desc DIEPCTL1"]
pub mod diepctl1;
#[doc = "DIEPINT1 (rw) register accessor: desc DIEPINT1\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint1`] module"]
#[doc(alias = "DIEPINT1")]
pub type Diepint1 = crate::Reg<diepint1::Diepint1Spec>;
#[doc = "desc DIEPINT1"]
pub mod diepint1;
#[doc = "DIEPTSIZ1 (rw) register accessor: desc DIEPTSIZ1\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz1`] module"]
#[doc(alias = "DIEPTSIZ1")]
pub type Dieptsiz1 = crate::Reg<dieptsiz1::Dieptsiz1Spec>;
#[doc = "desc DIEPTSIZ1"]
pub mod dieptsiz1;
#[doc = "DIEPDMA1 (rw) register accessor: desc DIEPDMA1\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma1`] module"]
#[doc(alias = "DIEPDMA1")]
pub type Diepdma1 = crate::Reg<diepdma1::Diepdma1Spec>;
#[doc = "desc DIEPDMA1"]
pub mod diepdma1;
#[doc = "DTXFSTS1 (r) register accessor: desc DTXFSTS1\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts1`] module"]
#[doc(alias = "DTXFSTS1")]
pub type Dtxfsts1 = crate::Reg<dtxfsts1::Dtxfsts1Spec>;
#[doc = "desc DTXFSTS1"]
pub mod dtxfsts1;
#[doc = "DIEPCTL2 (rw) register accessor: desc DIEPCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl2`] module"]
#[doc(alias = "DIEPCTL2")]
pub type Diepctl2 = crate::Reg<diepctl2::Diepctl2Spec>;
#[doc = "desc DIEPCTL2"]
pub mod diepctl2;
#[doc = "DIEPINT2 (rw) register accessor: desc DIEPINT2\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint2`] module"]
#[doc(alias = "DIEPINT2")]
pub type Diepint2 = crate::Reg<diepint2::Diepint2Spec>;
#[doc = "desc DIEPINT2"]
pub mod diepint2;
#[doc = "DIEPTSIZ2 (rw) register accessor: desc DIEPTSIZ2\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz2`] module"]
#[doc(alias = "DIEPTSIZ2")]
pub type Dieptsiz2 = crate::Reg<dieptsiz2::Dieptsiz2Spec>;
#[doc = "desc DIEPTSIZ2"]
pub mod dieptsiz2;
#[doc = "DIEPDMA2 (rw) register accessor: desc DIEPDMA2\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma2`] module"]
#[doc(alias = "DIEPDMA2")]
pub type Diepdma2 = crate::Reg<diepdma2::Diepdma2Spec>;
#[doc = "desc DIEPDMA2"]
pub mod diepdma2;
#[doc = "DTXFSTS2 (r) register accessor: desc DTXFSTS2\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts2`] module"]
#[doc(alias = "DTXFSTS2")]
pub type Dtxfsts2 = crate::Reg<dtxfsts2::Dtxfsts2Spec>;
#[doc = "desc DTXFSTS2"]
pub mod dtxfsts2;
#[doc = "DIEPCTL3 (rw) register accessor: desc DIEPCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl3`] module"]
#[doc(alias = "DIEPCTL3")]
pub type Diepctl3 = crate::Reg<diepctl3::Diepctl3Spec>;
#[doc = "desc DIEPCTL3"]
pub mod diepctl3;
#[doc = "DIEPINT3 (rw) register accessor: desc DIEPINT3\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint3`] module"]
#[doc(alias = "DIEPINT3")]
pub type Diepint3 = crate::Reg<diepint3::Diepint3Spec>;
#[doc = "desc DIEPINT3"]
pub mod diepint3;
#[doc = "DIEPTSIZ3 (rw) register accessor: desc DIEPTSIZ3\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz3`] module"]
#[doc(alias = "DIEPTSIZ3")]
pub type Dieptsiz3 = crate::Reg<dieptsiz3::Dieptsiz3Spec>;
#[doc = "desc DIEPTSIZ3"]
pub mod dieptsiz3;
#[doc = "DIEPDMA3 (rw) register accessor: desc DIEPDMA3\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma3`] module"]
#[doc(alias = "DIEPDMA3")]
pub type Diepdma3 = crate::Reg<diepdma3::Diepdma3Spec>;
#[doc = "desc DIEPDMA3"]
pub mod diepdma3;
#[doc = "DTXFSTS3 (r) register accessor: desc DTXFSTS3\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts3`] module"]
#[doc(alias = "DTXFSTS3")]
pub type Dtxfsts3 = crate::Reg<dtxfsts3::Dtxfsts3Spec>;
#[doc = "desc DTXFSTS3"]
pub mod dtxfsts3;
#[doc = "DIEPCTL4 (rw) register accessor: desc DIEPCTL4\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl4`] module"]
#[doc(alias = "DIEPCTL4")]
pub type Diepctl4 = crate::Reg<diepctl4::Diepctl4Spec>;
#[doc = "desc DIEPCTL4"]
pub mod diepctl4;
#[doc = "DIEPINT4 (rw) register accessor: desc DIEPINT4\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint4`] module"]
#[doc(alias = "DIEPINT4")]
pub type Diepint4 = crate::Reg<diepint4::Diepint4Spec>;
#[doc = "desc DIEPINT4"]
pub mod diepint4;
#[doc = "DIEPTSIZ4 (rw) register accessor: desc DIEPTSIZ4\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz4`] module"]
#[doc(alias = "DIEPTSIZ4")]
pub type Dieptsiz4 = crate::Reg<dieptsiz4::Dieptsiz4Spec>;
#[doc = "desc DIEPTSIZ4"]
pub mod dieptsiz4;
#[doc = "DIEPDMA4 (rw) register accessor: desc DIEPDMA4\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma4`] module"]
#[doc(alias = "DIEPDMA4")]
pub type Diepdma4 = crate::Reg<diepdma4::Diepdma4Spec>;
#[doc = "desc DIEPDMA4"]
pub mod diepdma4;
#[doc = "DTXFSTS4 (r) register accessor: desc DTXFSTS4\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts4`] module"]
#[doc(alias = "DTXFSTS4")]
pub type Dtxfsts4 = crate::Reg<dtxfsts4::Dtxfsts4Spec>;
#[doc = "desc DTXFSTS4"]
pub mod dtxfsts4;
#[doc = "DIEPCTL5 (rw) register accessor: desc DIEPCTL5\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl5`] module"]
#[doc(alias = "DIEPCTL5")]
pub type Diepctl5 = crate::Reg<diepctl5::Diepctl5Spec>;
#[doc = "desc DIEPCTL5"]
pub mod diepctl5;
#[doc = "DIEPINT5 (rw) register accessor: desc DIEPINT5\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint5`] module"]
#[doc(alias = "DIEPINT5")]
pub type Diepint5 = crate::Reg<diepint5::Diepint5Spec>;
#[doc = "desc DIEPINT5"]
pub mod diepint5;
#[doc = "DIEPTSIZ5 (rw) register accessor: desc DIEPTSIZ5\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz5`] module"]
#[doc(alias = "DIEPTSIZ5")]
pub type Dieptsiz5 = crate::Reg<dieptsiz5::Dieptsiz5Spec>;
#[doc = "desc DIEPTSIZ5"]
pub mod dieptsiz5;
#[doc = "DIEPDMA5 (rw) register accessor: desc DIEPDMA5\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma5`] module"]
#[doc(alias = "DIEPDMA5")]
pub type Diepdma5 = crate::Reg<diepdma5::Diepdma5Spec>;
#[doc = "desc DIEPDMA5"]
pub mod diepdma5;
#[doc = "DTXFSTS5 (r) register accessor: desc DTXFSTS5\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts5`] module"]
#[doc(alias = "DTXFSTS5")]
pub type Dtxfsts5 = crate::Reg<dtxfsts5::Dtxfsts5Spec>;
#[doc = "desc DTXFSTS5"]
pub mod dtxfsts5;
#[doc = "DOEPCTL0 (rw) register accessor: desc DOEPCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl0`] module"]
#[doc(alias = "DOEPCTL0")]
pub type Doepctl0 = crate::Reg<doepctl0::Doepctl0Spec>;
#[doc = "desc DOEPCTL0"]
pub mod doepctl0;
#[doc = "DOEPINT0 (rw) register accessor: desc DOEPINT0\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint0`] module"]
#[doc(alias = "DOEPINT0")]
pub type Doepint0 = crate::Reg<doepint0::Doepint0Spec>;
#[doc = "desc DOEPINT0"]
pub mod doepint0;
#[doc = "DOEPTSIZ0 (rw) register accessor: desc DOEPTSIZ0\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz0`] module"]
#[doc(alias = "DOEPTSIZ0")]
pub type Doeptsiz0 = crate::Reg<doeptsiz0::Doeptsiz0Spec>;
#[doc = "desc DOEPTSIZ0"]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 (rw) register accessor: desc DOEPDMA0\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma0`] module"]
#[doc(alias = "DOEPDMA0")]
pub type Doepdma0 = crate::Reg<doepdma0::Doepdma0Spec>;
#[doc = "desc DOEPDMA0"]
pub mod doepdma0;
#[doc = "DOEPCTL1 (rw) register accessor: desc DOEPCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl1`] module"]
#[doc(alias = "DOEPCTL1")]
pub type Doepctl1 = crate::Reg<doepctl1::Doepctl1Spec>;
#[doc = "desc DOEPCTL1"]
pub mod doepctl1;
#[doc = "DOEPINT1 (rw) register accessor: desc DOEPINT1\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint1`] module"]
#[doc(alias = "DOEPINT1")]
pub type Doepint1 = crate::Reg<doepint1::Doepint1Spec>;
#[doc = "desc DOEPINT1"]
pub mod doepint1;
#[doc = "DOEPTSIZ1 (rw) register accessor: desc DOEPTSIZ1\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz1`] module"]
#[doc(alias = "DOEPTSIZ1")]
pub type Doeptsiz1 = crate::Reg<doeptsiz1::Doeptsiz1Spec>;
#[doc = "desc DOEPTSIZ1"]
pub mod doeptsiz1;
#[doc = "DOEPDMA1 (rw) register accessor: desc DOEPDMA1\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma1`] module"]
#[doc(alias = "DOEPDMA1")]
pub type Doepdma1 = crate::Reg<doepdma1::Doepdma1Spec>;
#[doc = "desc DOEPDMA1"]
pub mod doepdma1;
#[doc = "DOEPCTL2 (rw) register accessor: desc DOEPCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl2`] module"]
#[doc(alias = "DOEPCTL2")]
pub type Doepctl2 = crate::Reg<doepctl2::Doepctl2Spec>;
#[doc = "desc DOEPCTL2"]
pub mod doepctl2;
#[doc = "DOEPINT2 (rw) register accessor: desc DOEPINT2\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint2`] module"]
#[doc(alias = "DOEPINT2")]
pub type Doepint2 = crate::Reg<doepint2::Doepint2Spec>;
#[doc = "desc DOEPINT2"]
pub mod doepint2;
#[doc = "DOEPTSIZ2 (rw) register accessor: desc DOEPTSIZ2\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz2`] module"]
#[doc(alias = "DOEPTSIZ2")]
pub type Doeptsiz2 = crate::Reg<doeptsiz2::Doeptsiz2Spec>;
#[doc = "desc DOEPTSIZ2"]
pub mod doeptsiz2;
#[doc = "DOEPDMA2 (rw) register accessor: desc DOEPDMA2\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma2`] module"]
#[doc(alias = "DOEPDMA2")]
pub type Doepdma2 = crate::Reg<doepdma2::Doepdma2Spec>;
#[doc = "desc DOEPDMA2"]
pub mod doepdma2;
#[doc = "DOEPCTL3 (rw) register accessor: desc DOEPCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl3`] module"]
#[doc(alias = "DOEPCTL3")]
pub type Doepctl3 = crate::Reg<doepctl3::Doepctl3Spec>;
#[doc = "desc DOEPCTL3"]
pub mod doepctl3;
#[doc = "DOEPINT3 (rw) register accessor: desc DOEPINT3\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint3`] module"]
#[doc(alias = "DOEPINT3")]
pub type Doepint3 = crate::Reg<doepint3::Doepint3Spec>;
#[doc = "desc DOEPINT3"]
pub mod doepint3;
#[doc = "DOEPTSIZ3 (rw) register accessor: desc DOEPTSIZ3\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz3`] module"]
#[doc(alias = "DOEPTSIZ3")]
pub type Doeptsiz3 = crate::Reg<doeptsiz3::Doeptsiz3Spec>;
#[doc = "desc DOEPTSIZ3"]
pub mod doeptsiz3;
#[doc = "DOEPDMA3 (rw) register accessor: desc DOEPDMA3\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma3`] module"]
#[doc(alias = "DOEPDMA3")]
pub type Doepdma3 = crate::Reg<doepdma3::Doepdma3Spec>;
#[doc = "desc DOEPDMA3"]
pub mod doepdma3;
#[doc = "DOEPCTL4 (rw) register accessor: desc DOEPCTL4\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl4`] module"]
#[doc(alias = "DOEPCTL4")]
pub type Doepctl4 = crate::Reg<doepctl4::Doepctl4Spec>;
#[doc = "desc DOEPCTL4"]
pub mod doepctl4;
#[doc = "DOEPINT4 (rw) register accessor: desc DOEPINT4\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint4`] module"]
#[doc(alias = "DOEPINT4")]
pub type Doepint4 = crate::Reg<doepint4::Doepint4Spec>;
#[doc = "desc DOEPINT4"]
pub mod doepint4;
#[doc = "DOEPTSIZ4 (rw) register accessor: desc DOEPTSIZ4\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz4`] module"]
#[doc(alias = "DOEPTSIZ4")]
pub type Doeptsiz4 = crate::Reg<doeptsiz4::Doeptsiz4Spec>;
#[doc = "desc DOEPTSIZ4"]
pub mod doeptsiz4;
#[doc = "DOEPDMA4 (rw) register accessor: desc DOEPDMA4\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma4`] module"]
#[doc(alias = "DOEPDMA4")]
pub type Doepdma4 = crate::Reg<doepdma4::Doepdma4Spec>;
#[doc = "desc DOEPDMA4"]
pub mod doepdma4;
#[doc = "DOEPCTL5 (rw) register accessor: desc DOEPCTL5\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl5`] module"]
#[doc(alias = "DOEPCTL5")]
pub type Doepctl5 = crate::Reg<doepctl5::Doepctl5Spec>;
#[doc = "desc DOEPCTL5"]
pub mod doepctl5;
#[doc = "DOEPINT5 (rw) register accessor: desc DOEPINT5\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint5`] module"]
#[doc(alias = "DOEPINT5")]
pub type Doepint5 = crate::Reg<doepint5::Doepint5Spec>;
#[doc = "desc DOEPINT5"]
pub mod doepint5;
#[doc = "DOEPTSIZ5 (rw) register accessor: desc DOEPTSIZ5\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz5`] module"]
#[doc(alias = "DOEPTSIZ5")]
pub type Doeptsiz5 = crate::Reg<doeptsiz5::Doeptsiz5Spec>;
#[doc = "desc DOEPTSIZ5"]
pub mod doeptsiz5;
#[doc = "DOEPDMA5 (rw) register accessor: desc DOEPDMA5\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma5`] module"]
#[doc(alias = "DOEPDMA5")]
pub type Doepdma5 = crate::Reg<doepdma5::Doepdma5Spec>;
#[doc = "desc DOEPDMA5"]
pub mod doepdma5;
#[doc = "GCCTL (rw) register accessor: desc GCCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcctl`] module"]
#[doc(alias = "GCCTL")]
pub type Gcctl = crate::Reg<gcctl::GcctlSpec>;
#[doc = "desc GCCTL"]
pub mod gcctl;
