#[doc = "Register `MONDAR2` reader"]
pub type R = crate::R<Mondar2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONDAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondar2Spec;
impl crate::RegisterSpec for Mondar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondar2::R`](R) reader structure"]
impl crate::Readable for Mondar2Spec {}
#[doc = "`reset()` method sets MONDAR2 to value 0"]
impl crate::Resettable for Mondar2Spec {}
