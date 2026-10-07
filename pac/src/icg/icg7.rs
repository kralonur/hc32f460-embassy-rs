#[doc = "Register `ICG7` reader"]
pub type R = crate::R<Icg7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc ICG7\n\nYou can [`read`](crate::Reg::read) this register and get [`icg7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg7Spec;
impl crate::RegisterSpec for Icg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg7::R`](R) reader structure"]
impl crate::Readable for Icg7Spec {}
#[doc = "`reset()` method sets ICG7 to value 0xffff_ffff"]
impl crate::Resettable for Icg7Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
