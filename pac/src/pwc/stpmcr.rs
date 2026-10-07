#[doc = "Register `STPMCR` reader"]
pub type R = crate::R<StpmcrSpec>;
#[doc = "Register `STPMCR` writer"]
pub type W = crate::W<StpmcrSpec>;
#[doc = "Field `FLNWT` reader - desc FLNWT"]
pub type FlnwtR = crate::BitReader;
#[doc = "Field `FLNWT` writer - desc FLNWT"]
pub type FlnwtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSMRC` reader - desc CKSMRC"]
pub type CksmrcR = crate::BitReader;
#[doc = "Field `CKSMRC` writer - desc CKSMRC"]
pub type CksmrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc FLNWT"]
    #[inline(always)]
    pub fn flnwt(&self) -> FlnwtR {
        FlnwtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CKSMRC"]
    #[inline(always)]
    pub fn cksmrc(&self) -> CksmrcR {
        CksmrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc FLNWT"]
    #[inline(always)]
    pub fn flnwt(&mut self) -> FlnwtW<'_, StpmcrSpec> {
        FlnwtW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CKSMRC"]
    #[inline(always)]
    pub fn cksmrc(&mut self) -> CksmrcW<'_, StpmcrSpec> {
        CksmrcW::new(self, 1)
    }
    #[doc = "Bit 15 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, StpmcrSpec> {
        StopW::new(self, 15)
    }
}
#[doc = "desc STPMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`stpmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stpmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StpmcrSpec;
impl crate::RegisterSpec for StpmcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`stpmcr::R`](R) reader structure"]
impl crate::Readable for StpmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`stpmcr::W`](W) writer structure"]
impl crate::Writable for StpmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STPMCR to value 0x4000"]
impl crate::Resettable for StpmcrSpec {
    const RESET_VALUE: u16 = 0x4000;
}
