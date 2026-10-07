#[doc = "Register `RESP4` reader"]
pub type R = crate::R<Resp4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RESP4\n\nYou can [`read`](crate::Reg::read) this register and get [`resp4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp4Spec;
impl crate::RegisterSpec for Resp4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`resp4::R`](R) reader structure"]
impl crate::Readable for Resp4Spec {}
#[doc = "`reset()` method sets RESP4 to value 0"]
impl crate::Resettable for Resp4Spec {}
