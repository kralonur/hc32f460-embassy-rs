#[doc = "Register `ICG2` reader"]
pub type R = crate::R<Icg2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc ICG2\n\nYou can [`read`](crate::Reg::read) this register and get [`icg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg2Spec;
impl crate::RegisterSpec for Icg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg2::R`](R) reader structure"]
impl crate::Readable for Icg2Spec {}
#[doc = "`reset()` method sets ICG2 to value 0xffff_ffff"]
impl crate::Resettable for Icg2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
