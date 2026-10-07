#[doc = "Register `DAT6` reader"]
pub type R = crate::R<Dat6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT6\n\nYou can [`read`](crate::Reg::read) this register and get [`dat6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat6Spec;
impl crate::RegisterSpec for Dat6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat6::R`](R) reader structure"]
impl crate::Readable for Dat6Spec {}
#[doc = "`reset()` method sets DAT6 to value 0"]
impl crate::Resettable for Dat6Spec {}
