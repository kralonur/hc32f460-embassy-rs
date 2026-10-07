#[doc = "Register `ICG5` reader"]
pub type R = crate::R<Icg5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc ICG5\n\nYou can [`read`](crate::Reg::read) this register and get [`icg5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg5Spec;
impl crate::RegisterSpec for Icg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg5::R`](R) reader structure"]
impl crate::Readable for Icg5Spec {}
#[doc = "`reset()` method sets ICG5 to value 0xffff_ffff"]
impl crate::Resettable for Icg5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
