#[doc = "Register `RESP3` reader"]
pub type R = crate::R<Resp3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RESP3\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp3Spec;
impl crate::RegisterSpec for Resp3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for Resp3Spec {}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for Resp3Spec {}
