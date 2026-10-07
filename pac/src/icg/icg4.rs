#[doc = "Register `ICG4` reader"]
pub type R = crate::R<Icg4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc ICG4\n\nYou can [`read`](crate::Reg::read) this register and get [`icg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg4Spec;
impl crate::RegisterSpec for Icg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg4::R`](R) reader structure"]
impl crate::Readable for Icg4Spec {}
#[doc = "`reset()` method sets ICG4 to value 0xffff_ffff"]
impl crate::Resettable for Icg4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
