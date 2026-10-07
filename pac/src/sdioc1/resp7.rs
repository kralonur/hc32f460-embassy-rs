#[doc = "Register `RESP7` reader"]
pub type R = crate::R<Resp7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RESP7\n\nYou can [`read`](crate::Reg::read) this register and get [`resp7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp7Spec;
impl crate::RegisterSpec for Resp7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`resp7::R`](R) reader structure"]
impl crate::Readable for Resp7Spec {}
#[doc = "`reset()` method sets RESP7 to value 0"]
impl crate::Resettable for Resp7Spec {}
