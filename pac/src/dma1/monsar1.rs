#[doc = "Register `MONSAR1` reader"]
pub type R = crate::R<Monsar1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONSAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Monsar1Spec;
impl crate::RegisterSpec for Monsar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monsar1::R`](R) reader structure"]
impl crate::Readable for Monsar1Spec {}
#[doc = "`reset()` method sets MONSAR1 to value 0"]
impl crate::Resettable for Monsar1Spec {}
