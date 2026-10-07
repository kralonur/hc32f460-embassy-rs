#[doc = "Register `PIDRA` reader"]
pub type R = crate::R<PidraSpec>;
#[doc = "Field `PIN00` reader - desc PIN00"]
pub type Pin00R = crate::BitReader;
#[doc = "Field `PIN01` reader - desc PIN01"]
pub type Pin01R = crate::BitReader;
#[doc = "Field `PIN02` reader - desc PIN02"]
pub type Pin02R = crate::BitReader;
#[doc = "Field `PIN03` reader - desc PIN03"]
pub type Pin03R = crate::BitReader;
#[doc = "Field `PIN04` reader - desc PIN04"]
pub type Pin04R = crate::BitReader;
#[doc = "Field `PIN05` reader - desc PIN05"]
pub type Pin05R = crate::BitReader;
#[doc = "Field `PIN06` reader - desc PIN06"]
pub type Pin06R = crate::BitReader;
#[doc = "Field `PIN07` reader - desc PIN07"]
pub type Pin07R = crate::BitReader;
#[doc = "Field `PIN08` reader - desc PIN08"]
pub type Pin08R = crate::BitReader;
#[doc = "Field `PIN09` reader - desc PIN09"]
pub type Pin09R = crate::BitReader;
#[doc = "Field `PIN10` reader - desc PIN10"]
pub type Pin10R = crate::BitReader;
#[doc = "Field `PIN11` reader - desc PIN11"]
pub type Pin11R = crate::BitReader;
#[doc = "Field `PIN12` reader - desc PIN12"]
pub type Pin12R = crate::BitReader;
#[doc = "Field `PIN13` reader - desc PIN13"]
pub type Pin13R = crate::BitReader;
#[doc = "Field `PIN14` reader - desc PIN14"]
pub type Pin14R = crate::BitReader;
#[doc = "Field `PIN15` reader - desc PIN15"]
pub type Pin15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc PIN00"]
    #[inline(always)]
    pub fn pin00(&self) -> Pin00R {
        Pin00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PIN01"]
    #[inline(always)]
    pub fn pin01(&self) -> Pin01R {
        Pin01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PIN02"]
    #[inline(always)]
    pub fn pin02(&self) -> Pin02R {
        Pin02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PIN03"]
    #[inline(always)]
    pub fn pin03(&self) -> Pin03R {
        Pin03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PIN04"]
    #[inline(always)]
    pub fn pin04(&self) -> Pin04R {
        Pin04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PIN05"]
    #[inline(always)]
    pub fn pin05(&self) -> Pin05R {
        Pin05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PIN06"]
    #[inline(always)]
    pub fn pin06(&self) -> Pin06R {
        Pin06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PIN07"]
    #[inline(always)]
    pub fn pin07(&self) -> Pin07R {
        Pin07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PIN08"]
    #[inline(always)]
    pub fn pin08(&self) -> Pin08R {
        Pin08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PIN09"]
    #[inline(always)]
    pub fn pin09(&self) -> Pin09R {
        Pin09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PIN10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PIN11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PIN12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "desc PIDRA\n\nYou can [`read`](crate::Reg::read) this register and get [`pidra::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidraSpec;
impl crate::RegisterSpec for PidraSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pidra::R`](R) reader structure"]
impl crate::Readable for PidraSpec {}
#[doc = "`reset()` method sets PIDRA to value 0"]
impl crate::Resettable for PidraSpec {}
