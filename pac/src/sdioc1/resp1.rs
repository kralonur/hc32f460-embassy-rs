#[doc = "Register `RESP1` reader"]
pub type R = crate::R<Resp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RESP1\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp1Spec;
impl crate::RegisterSpec for Resp1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`resp1::R`](R) reader structure"]
impl crate::Readable for Resp1Spec {}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for Resp1Spec {}
