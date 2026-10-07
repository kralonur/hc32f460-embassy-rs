#[doc = "Register `DATA2` reader"]
pub type R = crate::R<Data2Spec>;
#[doc = "Register `DATA2` writer"]
pub type W = crate::W<Data2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DATA2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data2Spec;
impl crate::RegisterSpec for Data2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2::R`](R) reader structure"]
impl crate::Readable for Data2Spec {}
#[doc = "`write(|w| ..)` method takes [`data2::W`](W) writer structure"]
impl crate::Writable for Data2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA2 to value 0"]
impl crate::Resettable for Data2Spec {}
