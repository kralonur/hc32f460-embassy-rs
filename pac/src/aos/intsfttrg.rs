#[doc = "Register `INTSFTTRG` writer"]
pub type W = crate::W<IntsfttrgSpec>;
#[doc = "Field `STRG` writer - desc STRG"]
pub type StrgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - desc STRG"]
    #[inline(always)]
    pub fn strg(&mut self) -> StrgW<'_, IntsfttrgSpec> {
        StrgW::new(self, 0)
    }
}
#[doc = "desc INTSFTTRG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsfttrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsfttrgSpec;
impl crate::RegisterSpec for IntsfttrgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intsfttrg::W`](W) writer structure"]
impl crate::Writable for IntsfttrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSFTTRG to value 0"]
impl crate::Resettable for IntsfttrgSpec {}
