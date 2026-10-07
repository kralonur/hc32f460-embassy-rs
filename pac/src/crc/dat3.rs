#[doc = "Register `DAT3` reader"]
pub type R = crate::R<Dat3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT3\n\nYou can [`read`](crate::Reg::read) this register and get [`dat3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat3Spec;
impl crate::RegisterSpec for Dat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat3::R`](R) reader structure"]
impl crate::Readable for Dat3Spec {}
#[doc = "`reset()` method sets DAT3 to value 0"]
impl crate::Resettable for Dat3Spec {}
