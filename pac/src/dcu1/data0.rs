#[doc = "Register `DATA0` reader"]
pub type R = crate::R<Data0Spec>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<Data0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0Spec;
impl crate::RegisterSpec for Data0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for Data0Spec {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for Data0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for Data0Spec {}
