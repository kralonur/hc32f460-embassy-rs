#[doc = "Register `AWDDR0` reader"]
pub type R = crate::R<Awddr0Spec>;
#[doc = "Register `AWDDR0` writer"]
pub type W = crate::W<Awddr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc AWDDR0\n\nYou can [`read`](crate::Reg::read) this register and get [`awddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awddr0Spec;
impl crate::RegisterSpec for Awddr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`awddr0::R`](R) reader structure"]
impl crate::Readable for Awddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`awddr0::W`](W) writer structure"]
impl crate::Writable for Awddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDDR0 to value 0"]
impl crate::Resettable for Awddr0Spec {}
