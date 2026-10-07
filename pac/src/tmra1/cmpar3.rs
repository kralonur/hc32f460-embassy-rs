#[doc = "Register `CMPAR3` reader"]
pub type R = crate::R<Cmpar3Spec>;
#[doc = "Register `CMPAR3` writer"]
pub type W = crate::W<Cmpar3Spec>;
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
    pub fn cmp(&mut self) -> CmpW<'_, Cmpar3Spec> {
        CmpW::new(self, 0)
    }
}
#[doc = "desc CMPAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpar3Spec;
impl crate::RegisterSpec for Cmpar3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpar3::R`](R) reader structure"]
impl crate::Readable for Cmpar3Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpar3::W`](W) writer structure"]
impl crate::Writable for Cmpar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPAR3 to value 0xffff"]
impl crate::Resettable for Cmpar3Spec {
    const RESET_VALUE: u16 = 0xffff;
}
