#[doc = "Register `DAT31` reader"]
pub type R = crate::R<Dat31Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT31\n\nYou can [`read`](crate::Reg::read) this register and get [`dat31::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat31Spec;
impl crate::RegisterSpec for Dat31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat31::R`](R) reader structure"]
impl crate::Readable for Dat31Spec {}
#[doc = "`reset()` method sets DAT31 to value 0"]
impl crate::Resettable for Dat31Spec {}
