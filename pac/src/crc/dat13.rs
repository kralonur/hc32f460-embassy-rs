#[doc = "Register `DAT13` reader"]
pub type R = crate::R<Dat13Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT13\n\nYou can [`read`](crate::Reg::read) this register and get [`dat13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat13Spec;
impl crate::RegisterSpec for Dat13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat13::R`](R) reader structure"]
impl crate::Readable for Dat13Spec {}
#[doc = "`reset()` method sets DAT13 to value 0"]
impl crate::Resettable for Dat13Spec {}
