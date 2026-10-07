#[doc = "Register `DAT23` reader"]
pub type R = crate::R<Dat23Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT23\n\nYou can [`read`](crate::Reg::read) this register and get [`dat23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat23Spec;
impl crate::RegisterSpec for Dat23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat23::R`](R) reader structure"]
impl crate::Readable for Dat23Spec {}
#[doc = "`reset()` method sets DAT23 to value 0"]
impl crate::Resettable for Dat23Spec {}
