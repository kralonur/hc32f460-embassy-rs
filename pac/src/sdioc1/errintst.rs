#[doc = "Register `ERRINTST` reader"]
pub type R = crate::R<ErrintstSpec>;
#[doc = "Register `ERRINTST` writer"]
pub type W = crate::W<ErrintstSpec>;
#[doc = "Field `CTOE` reader - desc CTOE"]
pub type CtoeR = crate::BitReader;
#[doc = "Field `CTOE` writer - desc CTOE"]
pub type CtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - desc CCE"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - desc CCE"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEBE` reader - desc CEBE"]
pub type CebeR = crate::BitReader;
#[doc = "Field `CEBE` writer - desc CEBE"]
pub type CebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIE` reader - desc CIE"]
pub type CieR = crate::BitReader;
#[doc = "Field `CIE` writer - desc CIE"]
pub type CieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` reader - desc DTOE"]
pub type DtoeR = crate::BitReader;
#[doc = "Field `DTOE` writer - desc DTOE"]
pub type DtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCE` reader - desc DCE"]
pub type DceR = crate::BitReader;
#[doc = "Field `DCE` writer - desc DCE"]
pub type DceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBE` reader - desc DEBE"]
pub type DebeR = crate::BitReader;
#[doc = "Field `DEBE` writer - desc DEBE"]
pub type DebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACE` reader - desc ACE"]
pub type AceR = crate::BitReader;
#[doc = "Field `ACE` writer - desc ACE"]
pub type AceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CTOE"]
    #[inline(always)]
    pub fn ctoe(&self) -> CtoeR {
        CtoeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CCE"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CEBE"]
    #[inline(always)]
    pub fn cebe(&self) -> CebeR {
        CebeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CIE"]
    #[inline(always)]
    pub fn cie(&self) -> CieR {
        CieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DTOE"]
    #[inline(always)]
    pub fn dtoe(&self) -> DtoeR {
        DtoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DCE"]
    #[inline(always)]
    pub fn dce(&self) -> DceR {
        DceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DEBE"]
    #[inline(always)]
    pub fn debe(&self) -> DebeR {
        DebeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ACE"]
    #[inline(always)]
    pub fn ace(&self) -> AceR {
        AceR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CTOE"]
    #[inline(always)]
    pub fn ctoe(&mut self) -> CtoeW<'_, ErrintstSpec> {
        CtoeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CCE"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<'_, ErrintstSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CEBE"]
    #[inline(always)]
    pub fn cebe(&mut self) -> CebeW<'_, ErrintstSpec> {
        CebeW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CIE"]
    #[inline(always)]
    pub fn cie(&mut self) -> CieW<'_, ErrintstSpec> {
        CieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc DTOE"]
    #[inline(always)]
    pub fn dtoe(&mut self) -> DtoeW<'_, ErrintstSpec> {
        DtoeW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DCE"]
    #[inline(always)]
    pub fn dce(&mut self) -> DceW<'_, ErrintstSpec> {
        DceW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DEBE"]
    #[inline(always)]
    pub fn debe(&mut self) -> DebeW<'_, ErrintstSpec> {
        DebeW::new(self, 6)
    }
    #[doc = "Bit 8 - desc ACE"]
    #[inline(always)]
    pub fn ace(&mut self) -> AceW<'_, ErrintstSpec> {
        AceW::new(self, 8)
    }
}
#[doc = "desc ERRINTST\n\nYou can [`read`](crate::Reg::read) this register and get [`errintst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errintst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrintstSpec;
impl crate::RegisterSpec for ErrintstSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`errintst::R`](R) reader structure"]
impl crate::Readable for ErrintstSpec {}
#[doc = "`write(|w| ..)` method takes [`errintst::W`](W) writer structure"]
impl crate::Writable for ErrintstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRINTST to value 0"]
impl crate::Resettable for ErrintstSpec {}
