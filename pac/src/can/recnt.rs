#[doc = "Register `RECNT` reader"]
pub type R = crate::R<RecntSpec>;
#[doc = "Register `RECNT` writer"]
pub type W = crate::W<RecntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc RECNT\n\nYou can [`read`](crate::Reg::read) this register and get [`recnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`recnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RecntSpec;
impl crate::RegisterSpec for RecntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`recnt::R`](R) reader structure"]
impl crate::Readable for RecntSpec {}
#[doc = "`write(|w| ..)` method takes [`recnt::W`](W) writer structure"]
impl crate::Writable for RecntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RECNT to value 0"]
impl crate::Resettable for RecntSpec {}
