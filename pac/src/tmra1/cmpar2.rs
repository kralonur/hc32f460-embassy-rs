#[doc = "Register `CMPAR2` reader"]
pub type R = crate::R<Cmpar2Spec>;
#[doc = "Register `CMPAR2` writer"]
pub type W = crate::W<Cmpar2Spec>;
#[doc = "Field `CMP` reader - desc CMP"]
pub type CmpR = crate::FieldReader<u16>;
#[doc = "Field `CMP` writer - desc CMP"]
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CMP"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, Cmpar2Spec> {
        CmpW::new(self, 0)
    }
}
#[doc = "desc CMPAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpar2Spec;
impl crate::RegisterSpec for Cmpar2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpar2::R`](R) reader structure"]
impl crate::Readable for Cmpar2Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpar2::W`](W) writer structure"]
impl crate::Writable for Cmpar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPAR2 to value 0xffff"]
impl crate::Resettable for Cmpar2Spec {
    const RESET_VALUE: u16 = 0xffff;
}
