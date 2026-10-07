#[doc = "Register `DAT16` reader"]
pub type R = crate::R<Dat16Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT16\n\nYou can [`read`](crate::Reg::read) this register and get [`dat16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat16Spec;
impl crate::RegisterSpec for Dat16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat16::R`](R) reader structure"]
impl crate::Readable for Dat16Spec {}
#[doc = "`reset()` method sets DAT16 to value 0"]
impl crate::Resettable for Dat16Spec {}
