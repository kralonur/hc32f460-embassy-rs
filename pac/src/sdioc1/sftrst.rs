#[doc = "Register `SFTRST` reader"]
pub type R = crate::R<SftrstSpec>;
#[doc = "Register `SFTRST` writer"]
pub type W = crate::W<SftrstSpec>;
#[doc = "Field `RSTA` reader - desc RSTA"]
pub type RstaR = crate::BitReader;
#[doc = "Field `RSTA` writer - desc RSTA"]
pub type RstaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTC` reader - desc RSTC"]
pub type RstcR = crate::BitReader;
#[doc = "Field `RSTC` writer - desc RSTC"]
pub type RstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTD` reader - desc RSTD"]
pub type RstdR = crate::BitReader;
#[doc = "Field `RSTD` writer - desc RSTD"]
pub type RstdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc RSTA"]
    #[inline(always)]
    pub fn rsta(&self) -> RstaR {
        RstaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RSTC"]
    #[inline(always)]
    pub fn rstc(&self) -> RstcR {
        RstcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RSTD"]
    #[inline(always)]
    pub fn rstd(&self) -> RstdR {
        RstdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RSTA"]
    #[inline(always)]
    pub fn rsta(&mut self) -> RstaW<'_, SftrstSpec> {
        RstaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RSTC"]
    #[inline(always)]
    pub fn rstc(&mut self) -> RstcW<'_, SftrstSpec> {
        RstcW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RSTD"]
    #[inline(always)]
    pub fn rstd(&mut self) -> RstdW<'_, SftrstSpec> {
        RstdW::new(self, 2)
    }
}
#[doc = "desc SFTRST\n\nYou can [`read`](crate::Reg::read) this register and get [`sftrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sftrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SftrstSpec;
impl crate::RegisterSpec for SftrstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sftrst::R`](R) reader structure"]
impl crate::Readable for SftrstSpec {}
#[doc = "`write(|w| ..)` method takes [`sftrst::W`](W) writer structure"]
impl crate::Writable for SftrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFTRST to value 0"]
impl crate::Resettable for SftrstSpec {}
