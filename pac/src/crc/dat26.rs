#[doc = "Register `DAT26` reader"]
pub type R = crate::R<Dat26Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT26\n\nYou can [`read`](crate::Reg::read) this register and get [`dat26::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat26Spec;
impl crate::RegisterSpec for Dat26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat26::R`](R) reader structure"]
impl crate::Readable for Dat26Spec {}
#[doc = "`reset()` method sets DAT26 to value 0"]
impl crate::Resettable for Dat26Spec {}
