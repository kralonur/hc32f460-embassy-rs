#[doc = "Register `DAT22` reader"]
pub type R = crate::R<Dat22Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT22\n\nYou can [`read`](crate::Reg::read) this register and get [`dat22::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat22Spec;
impl crate::RegisterSpec for Dat22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat22::R`](R) reader structure"]
impl crate::Readable for Dat22Spec {}
#[doc = "`reset()` method sets DAT22 to value 0"]
impl crate::Resettable for Dat22Spec {}
