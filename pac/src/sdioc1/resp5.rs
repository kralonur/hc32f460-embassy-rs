#[doc = "Register `RESP5` reader"]
pub type R = crate::R<Resp5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc RESP5\n\nYou can [`read`](crate::Reg::read) this register and get [`resp5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp5Spec;
impl crate::RegisterSpec for Resp5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`resp5::R`](R) reader structure"]
impl crate::Readable for Resp5Spec {}
#[doc = "`reset()` method sets RESP5 to value 0"]
impl crate::Resettable for Resp5Spec {}
