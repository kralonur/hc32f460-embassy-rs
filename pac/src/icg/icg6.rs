#[doc = "Register `ICG6` reader"]
pub type R = crate::R<Icg6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc ICG6\n\nYou can [`read`](crate::Reg::read) this register and get [`icg6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg6Spec;
impl crate::RegisterSpec for Icg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg6::R`](R) reader structure"]
impl crate::Readable for Icg6Spec {}
#[doc = "`reset()` method sets ICG6 to value 0xffff_ffff"]
impl crate::Resettable for Icg6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
