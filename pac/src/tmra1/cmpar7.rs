#[doc = "Register `CMPAR7` reader"]
pub type R = crate::R<Cmpar7Spec>;
#[doc = "Register `CMPAR7` writer"]
pub type W = crate::W<Cmpar7Spec>;
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
    pub fn cmp(&mut self) -> CmpW<'_, Cmpar7Spec> {
        CmpW::new(self, 0)
    }
}
#[doc = "desc CMPAR7\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpar7Spec;
impl crate::RegisterSpec for Cmpar7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpar7::R`](R) reader structure"]
impl crate::Readable for Cmpar7Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpar7::W`](W) writer structure"]
impl crate::Writable for Cmpar7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPAR7 to value 0xffff"]
impl crate::Resettable for Cmpar7Spec {
    const RESET_VALUE: u16 = 0xffff;
}
