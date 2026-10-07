#[doc = "Register `MONDAR0` reader"]
pub type R = crate::R<Mondar0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONDAR0\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondar0Spec;
impl crate::RegisterSpec for Mondar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondar0::R`](R) reader structure"]
impl crate::Readable for Mondar0Spec {}
#[doc = "`reset()` method sets MONDAR0 to value 0"]
impl crate::Resettable for Mondar0Spec {}
