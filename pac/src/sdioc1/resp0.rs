#[doc = "Register `RESP0` reader"]
pub type R = crate::R<Resp0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RESP0\n\nYou can [`read`](crate::Reg::read) this register and get [`resp0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp0Spec;
impl crate::RegisterSpec for Resp0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`resp0::R`](R) reader structure"]
impl crate::Readable for Resp0Spec {}
#[doc = "`reset()` method sets RESP0 to value 0"]
impl crate::Resettable for Resp0Spec {}
