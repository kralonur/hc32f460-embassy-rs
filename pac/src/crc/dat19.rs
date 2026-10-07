#[doc = "Register `DAT19` reader"]
pub type R = crate::R<Dat19Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT19\n\nYou can [`read`](crate::Reg::read) this register and get [`dat19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat19Spec;
impl crate::RegisterSpec for Dat19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat19::R`](R) reader structure"]
impl crate::Readable for Dat19Spec {}
#[doc = "`reset()` method sets DAT19 to value 0"]
impl crate::Resettable for Dat19Spec {}
