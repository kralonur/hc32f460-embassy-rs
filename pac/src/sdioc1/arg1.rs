#[doc = "Register `ARG1` reader"]
pub type R = crate::R<Arg1Spec>;
#[doc = "Register `ARG1` writer"]
pub type W = crate::W<Arg1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc ARG1\n\nYou can [`read`](crate::Reg::read) this register and get [`arg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Arg1Spec;
impl crate::RegisterSpec for Arg1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`arg1::R`](R) reader structure"]
impl crate::Readable for Arg1Spec {}
#[doc = "`write(|w| ..)` method takes [`arg1::W`](W) writer structure"]
impl crate::Writable for Arg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARG1 to value 0"]
impl crate::Resettable for Arg1Spec {}
