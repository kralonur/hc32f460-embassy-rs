#[doc = "Register `PERCR` reader"]
pub type R = crate::R<PercrSpec>;
#[doc = "Register `PERCR` writer"]
pub type W = crate::W<PercrSpec>;
#[doc = "Field `PERC` reader - desc PERC"]
pub type PercR = crate::FieldReader<u16>;
#[doc = "Field `PERC` writer - desc PERC"]
pub type PercW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc PERC"]
    #[inline(always)]
    pub fn perc(&self) -> PercR {
        PercR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PERC"]
    #[inline(always)]
    pub fn perc(&mut self) -> PercW<'_, PercrSpec> {
        PercW::new(self, 0)
    }
}
#[doc = "desc PERCR\n\nYou can [`read`](crate::Reg::read) this register and get [`percr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`percr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PercrSpec;
impl crate::RegisterSpec for PercrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`percr::R`](R) reader structure"]
impl crate::Readable for PercrSpec {}
#[doc = "`write(|w| ..)` method takes [`percr::W`](W) writer structure"]
impl crate::Writable for PercrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERCR to value 0xffff"]
impl crate::Resettable for PercrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
