#[doc = "Register `DAT25` reader"]
pub type R = crate::R<Dat25Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT25\n\nYou can [`read`](crate::Reg::read) this register and get [`dat25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat25Spec;
impl crate::RegisterSpec for Dat25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat25::R`](R) reader structure"]
impl crate::Readable for Dat25Spec {}
#[doc = "`reset()` method sets DAT25 to value 0"]
impl crate::Resettable for Dat25Spec {}
