#[doc = "Register `AWDDR1` reader"]
pub type R = crate::R<Awddr1Spec>;
#[doc = "Register `AWDDR1` writer"]
pub type W = crate::W<Awddr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc AWDDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`awddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awddr1Spec;
impl crate::RegisterSpec for Awddr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`awddr1::R`](R) reader structure"]
impl crate::Readable for Awddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`awddr1::W`](W) writer structure"]
impl crate::Writable for Awddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDDR1 to value 0"]
impl crate::Resettable for Awddr1Spec {}
