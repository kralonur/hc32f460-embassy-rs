#[doc = "Register `FSTP` reader"]
pub type R = crate::R<FstpSpec>;
#[doc = "Register `FSTP` writer"]
pub type W = crate::W<FstpSpec>;
#[doc = "Field `FSTP` reader - desc FSTP"]
pub type FstpR = crate::BitReader;
#[doc = "Field `FSTP` writer - desc FSTP"]
pub type FstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc FSTP"]
    #[inline(always)]
    pub fn fstp(&self) -> FstpR {
        FstpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc FSTP"]
    #[inline(always)]
    pub fn fstp(&mut self) -> FstpW<'_, FstpSpec> {
        FstpW::new(self, 0)
    }
}
#[doc = "desc FSTP\n\nYou can [`read`](crate::Reg::read) this register and get [`fstp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstpSpec;
impl crate::RegisterSpec for FstpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstp::R`](R) reader structure"]
impl crate::Readable for FstpSpec {}
#[doc = "`write(|w| ..)` method takes [`fstp::W`](W) writer structure"]
impl crate::Writable for FstpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSTP to value 0"]
impl crate::Resettable for FstpSpec {}
