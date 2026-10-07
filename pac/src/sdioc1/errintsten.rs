#[doc = "Register `ERRINTSTEN` reader"]
pub type R = crate::R<ErrintstenSpec>;
#[doc = "Register `ERRINTSTEN` writer"]
pub type W = crate::W<ErrintstenSpec>;
#[doc = "Field `CTOEEN` reader - desc CTOEEN"]
pub type CtoeenR = crate::BitReader;
#[doc = "Field `CTOEEN` writer - desc CTOEEN"]
pub type CtoeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEEN` reader - desc CCEEN"]
pub type CceenR = crate::BitReader;
#[doc = "Field `CCEEN` writer - desc CCEEN"]
pub type CceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEBEEN` reader - desc CEBEEN"]
pub type CebeenR = crate::BitReader;
#[doc = "Field `CEBEEN` writer - desc CEBEEN"]
pub type CebeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIEEN` reader - desc CIEEN"]
pub type CieenR = crate::BitReader;
#[doc = "Field `CIEEN` writer - desc CIEEN"]
pub type CieenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOEEN` reader - desc DTOEEN"]
pub type DtoeenR = crate::BitReader;
#[doc = "Field `DTOEEN` writer - desc DTOEEN"]
pub type DtoeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEEN` reader - desc DCEEN"]
pub type DceenR = crate::BitReader;
#[doc = "Field `DCEEN` writer - desc DCEEN"]
pub type DceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBEEN` reader - desc DEBEEN"]
pub type DebeenR = crate::BitReader;
#[doc = "Field `DEBEEN` writer - desc DEBEEN"]
pub type DebeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACEEN` reader - desc ACEEN"]
pub type AceenR = crate::BitReader;
#[doc = "Field `ACEEN` writer - desc ACEEN"]
pub type AceenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CTOEEN"]
    #[inline(always)]
    pub fn ctoeen(&self) -> CtoeenR {
        CtoeenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CCEEN"]
    #[inline(always)]
    pub fn cceen(&self) -> CceenR {
        CceenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CEBEEN"]
    #[inline(always)]
    pub fn cebeen(&self) -> CebeenR {
        CebeenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CIEEN"]
    #[inline(always)]
    pub fn cieen(&self) -> CieenR {
        CieenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DTOEEN"]
    #[inline(always)]
    pub fn dtoeen(&self) -> DtoeenR {
        DtoeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DCEEN"]
    #[inline(always)]
    pub fn dceen(&self) -> DceenR {
        DceenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DEBEEN"]
    #[inline(always)]
    pub fn debeen(&self) -> DebeenR {
        DebeenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ACEEN"]
    #[inline(always)]
    pub fn aceen(&self) -> AceenR {
        AceenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CTOEEN"]
    #[inline(always)]
    pub fn ctoeen(&mut self) -> CtoeenW<'_, ErrintstenSpec> {
        CtoeenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CCEEN"]
    #[inline(always)]
    pub fn cceen(&mut self) -> CceenW<'_, ErrintstenSpec> {
        CceenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CEBEEN"]
    #[inline(always)]
    pub fn cebeen(&mut self) -> CebeenW<'_, ErrintstenSpec> {
        CebeenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CIEEN"]
    #[inline(always)]
    pub fn cieen(&mut self) -> CieenW<'_, ErrintstenSpec> {
        CieenW::new(self, 3)
    }
    #[doc = "Bit 4 - desc DTOEEN"]
    #[inline(always)]
    pub fn dtoeen(&mut self) -> DtoeenW<'_, ErrintstenSpec> {
        DtoeenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DCEEN"]
    #[inline(always)]
    pub fn dceen(&mut self) -> DceenW<'_, ErrintstenSpec> {
        DceenW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DEBEEN"]
    #[inline(always)]
    pub fn debeen(&mut self) -> DebeenW<'_, ErrintstenSpec> {
        DebeenW::new(self, 6)
    }
    #[doc = "Bit 8 - desc ACEEN"]
    #[inline(always)]
    pub fn aceen(&mut self) -> AceenW<'_, ErrintstenSpec> {
        AceenW::new(self, 8)
    }
}
#[doc = "desc ERRINTSTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`errintsten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errintsten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrintstenSpec;
impl crate::RegisterSpec for ErrintstenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`errintsten::R`](R) reader structure"]
impl crate::Readable for ErrintstenSpec {}
#[doc = "`write(|w| ..)` method takes [`errintsten::W`](W) writer structure"]
impl crate::Writable for ErrintstenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRINTSTEN to value 0"]
impl crate::Resettable for ErrintstenSpec {}
