#[doc = "Register `DCONR` reader"]
pub type R = crate::R<DconrSpec>;
#[doc = "Register `DCONR` writer"]
pub type W = crate::W<DconrSpec>;
#[doc = "Field `DTCEN` reader - desc DTCEN"]
pub type DtcenR = crate::BitReader;
#[doc = "Field `DTCEN` writer - desc DTCEN"]
pub type DtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBENU` reader - desc DTBENU"]
pub type DtbenuR = crate::BitReader;
#[doc = "Field `DTBENU` writer - desc DTBENU"]
pub type DtbenuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBEND` reader - desc DTBEND"]
pub type DtbendR = crate::BitReader;
#[doc = "Field `DTBEND` writer - desc DTBEND"]
pub type DtbendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEPA` reader - desc SEPA"]
pub type SepaR = crate::BitReader;
#[doc = "Field `SEPA` writer - desc SEPA"]
pub type SepaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DTCEN"]
    #[inline(always)]
    pub fn dtcen(&self) -> DtcenR {
        DtcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - desc DTBENU"]
    #[inline(always)]
    pub fn dtbenu(&self) -> DtbenuR {
        DtbenuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DTBEND"]
    #[inline(always)]
    pub fn dtbend(&self) -> DtbendR {
        DtbendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SEPA"]
    #[inline(always)]
    pub fn sepa(&self) -> SepaR {
        SepaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DTCEN"]
    #[inline(always)]
    pub fn dtcen(&mut self) -> DtcenW<'_, DconrSpec> {
        DtcenW::new(self, 0)
    }
    #[doc = "Bit 4 - desc DTBENU"]
    #[inline(always)]
    pub fn dtbenu(&mut self) -> DtbenuW<'_, DconrSpec> {
        DtbenuW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DTBEND"]
    #[inline(always)]
    pub fn dtbend(&mut self) -> DtbendW<'_, DconrSpec> {
        DtbendW::new(self, 5)
    }
    #[doc = "Bit 8 - desc SEPA"]
    #[inline(always)]
    pub fn sepa(&mut self) -> SepaW<'_, DconrSpec> {
        SepaW::new(self, 8)
    }
}
#[doc = "desc DCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`dconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DconrSpec;
impl crate::RegisterSpec for DconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dconr::R`](R) reader structure"]
impl crate::Readable for DconrSpec {}
#[doc = "`write(|w| ..)` method takes [`dconr::W`](W) writer structure"]
impl crate::Writable for DconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCONR to value 0"]
impl crate::Resettable for DconrSpec {}
