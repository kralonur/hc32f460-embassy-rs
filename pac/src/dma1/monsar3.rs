#[doc = "Register `MONSAR3` reader"]
pub type R = crate::R<Monsar3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONSAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Monsar3Spec;
impl crate::RegisterSpec for Monsar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monsar3::R`](R) reader structure"]
impl crate::Readable for Monsar3Spec {}
#[doc = "`reset()` method sets MONSAR3 to value 0"]
impl crate::Resettable for Monsar3Spec {}
