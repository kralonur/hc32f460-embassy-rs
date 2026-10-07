#[doc = "Register `RR` reader"]
pub type R = crate::R<RrSpec>;
#[doc = "Register `RR` writer"]
pub type W = crate::W<RrSpec>;
#[doc = "Field `RF` reader - desc RF"]
pub type RfR = crate::FieldReader<u16>;
#[doc = "Field `RF` writer - desc RF"]
pub type RfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc RF"]
    #[inline(always)]
    pub fn rf(&self) -> RfR {
        RfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc RF"]
    #[inline(always)]
    pub fn rf(&mut self) -> RfW<'_, RrSpec> {
        RfW::new(self, 0)
    }
}
#[doc = "desc RR\n\nYou can [`read`](crate::Reg::read) this register and get [`rr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrSpec;
impl crate::RegisterSpec for RrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rr::R`](R) reader structure"]
impl crate::Readable for RrSpec {}
#[doc = "`write(|w| ..)` method takes [`rr::W`](W) writer structure"]
impl crate::Writable for RrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RR to value 0"]
impl crate::Resettable for RrSpec {}
