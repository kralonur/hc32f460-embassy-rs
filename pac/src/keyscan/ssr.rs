#[doc = "Register `SSR` reader"]
pub type R = crate::R<SsrSpec>;
#[doc = "Register `SSR` writer"]
pub type W = crate::W<SsrSpec>;
#[doc = "Field `INDEX` reader - desc INDEX"]
pub type IndexR = crate::FieldReader;
#[doc = "Field `INDEX` writer - desc INDEX"]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc INDEX"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc INDEX"]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, SsrSpec> {
        IndexW::new(self, 0)
    }
}
#[doc = "desc SSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrSpec;
impl crate::RegisterSpec for SsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssr::R`](R) reader structure"]
impl crate::Readable for SsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssr::W`](W) writer structure"]
impl crate::Writable for SsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SsrSpec {}
