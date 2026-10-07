#[doc = "Register `MONSAR2` reader"]
pub type R = crate::R<Monsar2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONSAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Monsar2Spec;
impl crate::RegisterSpec for Monsar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monsar2::R`](R) reader structure"]
impl crate::Readable for Monsar2Spec {}
#[doc = "`reset()` method sets MONSAR2 to value 0"]
impl crate::Resettable for Monsar2Spec {}
